/**
 * Helpers related to `local indexes`.
 *
 * A `local index` is a simple set of links between Holochain entries. These are
 * appropriate for linking directly between entries within the same DNA.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use std::convert::{ TryFrom };
use hdk3::prelude::*;

use crate::{
    MaybeUndefined,
    GraphAPIResult,
    entries::{
        get_entries_by_address,
        // get_entries_by_key_index,
    },
    links::{
        get_linked_addresses,
        // get_linked_addresses_as_type,
    },
    identity_helpers::{
        calculate_identity_address,
    },
    internals::{
        wipe_links_from_origin,
        link_matches,
        link_does_not_match,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Given a base address to query from, returns a Vec of tuples of all target
/// `EntryHash`es referenced via the given link tag, bound to the result of
/// attempting to decode each referenced entry into the requested type `R`.
///
pub fn query_direct_index<R, F, A>(
    base_address: &F,
    link_tag: &str,
) -> GraphAPIResult<Vec<(A, GraphAPIResult<R>)>>
    where A: From<EntryHash>,
        F: AsRef<EntryHash>,
        R: Clone,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    let addrs_result = get_linked_addresses(base_address.as_ref(), LinkTag::new(link_tag))?;
    let entries = get_entries_by_address(&addrs_result);

    Ok(addrs_result
        .iter()
        .map(|h| { (*h).into() })
        .zip(entries)
        .collect())
}

/// Load any set of records of type `R` that are:
/// - linked locally (in the same DNA) from the `base_address`
/// - linked via their own local indirect indexes (`base_address` -> entry base -> entry data)
/// - linked via `link_type` and `link_name`
///
/// Results are automatically deserialized into `R` as they are retrieved from the DHT.
/// Any entries that either fail to load or cannot be converted to the type will be dropped.
///
/// :TODO: return errors, improve error handling
///
pub fn query_direct_index_with_foreign_key<R, F, A>(
    base_address: &F,
    link_type: &str,
    link_name: &str,
) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
        F: AsRef<Address>,
{
    let addrs_result = get_linked_addresses(base_address.as_ref(), link_type, link_name);
    if let Err(get_links_err) = addrs_result {
        return Err(get_links_err);
    }
    get_entries_by_key_index(addrs_result.unwrap())
}

/// Load any set of records of type `R` that are:
/// - linked remotely (from an external DNA) from the `base_address`
/// - linked via their own local indirect indexes (`base_address` -> entry base -> entry data)
/// - linked via `link_type` and `link_name`
///
/// Results are automatically deserialized into `R` as they are retrieved from the DHT.
/// Any entries that either fail to load or cannot be converted to the type will be dropped.
///
/// :TODO: return errors, improve error handling
///
pub fn query_direct_remote_index_with_foreign_key<'a, R, F, A>(
    base_address: &F,
    base_entry_type: &'a str,
    link_type: &str,
    link_name: &str,
) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
        F: AsRef<Address> + Into<JsonString> + Clone,
{
    let query_address = determine_key_index_address(base_entry_type.to_string(), base_address.as_ref());
    if let Err(resolve_remote_err) = query_address {
        return Err(resolve_remote_err);
    }

    let addrs_result = get_linked_addresses(&query_address.unwrap(), link_type, link_name);
    if let Err(get_links_err) = addrs_result {
        return Err(get_links_err);
    }
    get_entries_by_key_index(addrs_result.unwrap())
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the addresses of the (respectively) forward & reciprocal links created.
pub fn create_direct_index<S: Into<String>>(
    source: &Address,
    dest: &Address,
    link_type: S,
    link_name: S,
    link_type_reciprocal: S,
    link_name_reciprocal: S,
) -> Vec<ZomeApiResult<Address>> {
    vec! [
        link_entries(source, dest, link_type, link_name),
        link_entries(dest, source, link_type_reciprocal, link_name_reciprocal),
    ]
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Remove any links of `link_type`/`link_name` and their reciprocal links of
/// `link_type_reciprocal`/`link_name_reciprocal` that might be present on `source`;
/// then add new links of the given types & names that instead point from `source`
/// to `new_dest`.
///
/// If `new_dest` is `MaybeUndefined::None`, the links are simply removed.
/// If `new_dest` is `MaybeUndefined::Undefined`, this is a no-op.
///
/// Returns the addresses of the previously erased link targets, if any.
///
/// :TODO: update to accept multiple targets for the replacement links
/// :TODO: propagate errors to allow non-critical failures in calling code
///
pub fn replace_direct_index<A, B>(
    source: &A,
    new_dest: &MaybeUndefined<B>,
    link_type: &str,
    link_name: &str,
    link_type_reciprocal: &str,
    link_name_reciprocal: &str,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    // if not updating, skip operation
    if let MaybeUndefined::Undefined = new_dest {
        return Ok(vec![]);
    }

    // load any existing linked entries from the originating address
    let existing_links: Vec<B> = get_linked_addresses_as_type(source, link_type, link_name).into_owned();

    // determine links to erase
    let to_erase: Vec<B> = existing_links.iter()
        .filter(link_does_not_match(new_dest)).map(|x| { (*x).clone() }).collect();

    // wipe stale links
    // :TODO: propagate errors
    let _erased: Vec<ZomeApiResult<()>> = to_erase.iter().flat_map(wipe_links_from_origin(
        link_type, link_name,
        link_type_reciprocal, link_name_reciprocal,
        source,
    )).collect();

    // get base addresses of erased items
    let erased: Vec<ZomeApiResult<Address>> = to_erase.iter().map(|addr| { Ok((*addr).as_ref().clone()) }).collect();

    // run insert if needed
    match new_dest {
        MaybeUndefined::Some(new_link) => {
            let already_present = existing_links.iter().filter(link_matches(new_dest)).count() > 0;

            if already_present {
                Ok(erased)
            } else {
                create_direct_index(
                    source.as_ref(), new_link.as_ref(),
                    link_type, link_name,
                    link_type_reciprocal, link_name_reciprocal
                );
                Ok(erased)
            }
        },
        _ => Ok(erased),
    }
}

//-------------------------------[ DELETE ]-------------------------------------

/// Deletes a bidirectional link between two entry addresses, and returns any errors encountered
/// to the caller.
///
/// :TODO: filter empty success tuples from results and return as flattened error array
///
pub fn delete_direct_index<S: Into<String>>(
    source: &Address,
    dest: &Address,
    link_type: S,
    link_name: S,
    link_type_reciprocal: S,
    link_name_reciprocal: S,
) -> Vec<ZomeApiResult<()>> {
    vec! [
        remove_link(source, dest, link_type, link_name),
        remove_link(dest, source, link_type_reciprocal, link_name_reciprocal),
    ]
}
