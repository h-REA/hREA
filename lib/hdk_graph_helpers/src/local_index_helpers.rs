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
use hdk3::prelude::*;

use crate::{
    GraphAPIResult,
    record_interface::Identified,
    records::{
        get_records_by_identity_address,
    },
    links::{
        get_linked_headers,
        get_linked_addresses,
    },
    identity_helpers::{
        calculate_identity_address,
        read_entry_identity,
    },
    internals::{
        link_matches,
        link_does_not_match,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Reads and returns all entry identities referenced by the given index from
/// (`base_entry_type.base_address` via `link_tag`.
///
/// Use this method to query associated IDs for a query edge, without retrieving
/// the records themselves.
///
pub fn read_index<'a, A, S: 'a + Into<Vec<u8>>, I: AsRef<str>>(
    base_entry_type: &I,
    base_address: &EntryHash,
    link_tag: S,
) -> GraphAPIResult<Vec<GraphAPIResult<A>>>
    where A: From<EntryHash>,
{
    let index_address = calculate_identity_address(base_entry_type, base_address)?;
    let refd_index_addresses = get_linked_addresses(&index_address, LinkTag::new(link_tag))?;

    Ok(refd_index_addresses.iter()
        .map(|i| {
            Ok(read_entry_identity(i)?.into())
        })
        .collect())
}

/// Given a base address to query from, returns a Vec of tuples of all target
/// `EntryHash`es referenced via the given link tag, bound to the result of
/// attempting to decode each referenced entry into the requested type `R`.
///
/// Use this method to query associated records for a query edge in full.
///
pub fn query_index<'a, T, R, A, S: 'a + Into<Vec<u8>>, I: AsRef<str>>(
    base_entry_type: &I,
    base_address: &EntryHash,
    link_tag: S,
) -> GraphAPIResult<Vec<GraphAPIResult<(HeaderHash, A, T)>>>
    where A: From<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Identified<T>,
{
    let index_address = calculate_identity_address(base_entry_type, base_address)?;
    let addrs_result = get_linked_addresses(&index_address, LinkTag::new(link_tag))?;
    let entries = get_records_by_identity_address::<T, R, A>(&addrs_result);
    Ok(entries)
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
) -> GraphAPIResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<EntryHash>,
        F: AsRef<EntryHash>,
{
    let addrs_result = get_linked_addresses(base_address.as_ref(), LinkTag::new(link_tag));
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
) -> GraphAPIResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<EntryHash>,
        F: AsRef<EntryHash> + Into<JsonString> + Clone,
{
    let query_address = calculate_identity_address(base_entry_type.to_string(), base_address.as_ref());
    if let Err(resolve_remote_err) = query_address {
        return Err(resolve_remote_err);
    }

    let addrs_result = get_linked_addresses(&query_address.unwrap(), LinkTag::new(link_tag));
    if let Err(get_links_err) = addrs_result {
        return Err(get_links_err);
    }
    get_entries_by_key_index(addrs_result.unwrap())
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the `HeaderHash`es of the (respectively) forward & reciprocal links created.
pub fn create_index<'a, S: 'a + Into<Vec<u8>>, I: AsRef<str>>(
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest: &EntryHash,
    link_tag: S,
    link_tag_reciprocal: S,
) -> GraphAPIResult<Vec<HeaderHash>> {
    let source_hash = calculate_identity_address(source_entry_type, source)?;
    let dest_hash = calculate_identity_address(dest_entry_type, dest)?;

    Ok(vec! [
        create_link(source_hash.clone(), dest_hash.clone(), LinkTag::new(link_tag))?,
        create_link(dest_hash, source_hash, LinkTag::new(link_tag_reciprocal))?,
    ])
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
) -> GraphAPIResult<Vec<GraphAPIResult<EntryHash>>>
    where A: AsRef<EntryHash> + From<EntryHash> + Clone,
        B: AsRef<EntryHash> + From<EntryHash> + Clone + PartialEq,
{
    // if not updating, skip operation
    if let MaybeUndefined::Undefined = new_dest {
        return Ok(vec![]);
    }

    // load any existing linked entries from the originating address
    let existing_links: Vec<B> = get_linked_addresses_as_type(source, LinkTag::new(link_tag)).into_owned();

    // determine links to erase
    let to_erase: Vec<B> = existing_links.iter()
        .filter(link_does_not_match(new_dest)).map(|x| { (*x).clone() }).collect();

    // wipe stale links
    // :TODO: propagate errors
    let _erased: Vec<GraphAPIResult<()>> = to_erase.iter().flat_map(wipe_links_from_origin(
        LinkTag::new(link_tag),
        link_type_reciprocal, link_name_reciprocal,
        source,
    )).collect();

    // get base addresses of erased items
    let erased: Vec<GraphAPIResult<EntryHash>> = to_erase.iter().map(|addr| { Ok((*addr).as_ref().clone()) }).collect();

    // run insert if needed
    match new_dest {
        MaybeUndefined::Some(new_link) => {
            let already_present = existing_links.iter().filter(link_matches(new_dest)).count() > 0;

            if already_present {
                Ok(erased)
            } else {
                create_direct_index(
                    source.as_ref(), new_link.as_ref(),
                    LinkTag::new(link_tag),
                    link_type_reciprocal, link_name_reciprocal
                );
                Ok(erased)
            }
        },
        _ => Ok(erased),
    }
}

//-------------------------------[ DELETE ]-------------------------------------

/// Deletes a bidirectional link between two entry addresses. Any active links between
/// the given addresses using the given tags will be deleted.
///
pub fn delete_index<'a, S: 'a + Into<Vec<u8>>, I: AsRef<str>>(
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest: &EntryHash,
    link_tag: S,
    link_tag_reciprocal: S,
) -> GraphAPIResult<Vec<GraphAPIResult<HeaderHash>>> {
    let tag_source = LinkTag::new(link_tag);
    let tag_dest = LinkTag::new(link_tag_reciprocal);
    let address_source = calculate_identity_address(source_entry_type, source)?;
    let address_dest = calculate_identity_address(dest_entry_type, dest)?;

    let mut links = get_linked_headers(&address_source, tag_source)?;
    links.append(& mut get_linked_headers(&address_dest, tag_dest)?);

    Ok(links
        .iter()
        .map(|l| { Ok(delete_link((*l).clone())?) })
        .collect()
    )
}
