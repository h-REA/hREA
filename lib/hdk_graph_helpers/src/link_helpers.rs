/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */
use std::fmt::Debug;
use std::borrow::Cow;
use std::convert::{ TryFrom };
use hdk::{
    holochain_json_api::{ json::JsonString },
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry::App as AppEntry,
        entry::AppEntryValue,
        entry::entry_type::AppEntryType,
        link::LinkMatch,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksOptions,
    error::{ ZomeApiError, ZomeApiResult },
    link_entries,
    entry_address,
    get_entry,
    get_links_with_options,
    remove_link,
};

use super::{
    MaybeUndefined,
    records::{
        create_base_entry,
        get_dereferenced_address,
    },
};



// CREATE

/// Creates a 'remote' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of a `base` address for the remote content and bidirectional
/// links between it and its target_base_addresses.
pub fn create_remote_query_index<'a, A, B>(
    remote_base_entry_type: &'a str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &A,
    target_base_addresses: Vec<B>,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    // create a base entry pointer for the referenced origin record
    let base_entry: AppEntryType = remote_base_entry_type.to_string().into();
    let base_resp = create_base_entry(&base_entry, source_base_address.as_ref());
    if let Err(base_creation_failure) = base_resp {
        return Err(base_creation_failure);
    }
    let base_address = base_resp.unwrap();

    // link all referenced records to our pointer to the remote origin record
    Ok(target_base_addresses.iter()
        .map(|target_address| {
            // link origin record to local records by specified edge
            link_entries_bidir(
                &base_address, target_address.as_ref(),
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            Ok(target_address.as_ref().clone())
        })
        .collect()
    )
}

/// Creates a 'local' query index used for fetching and querying pointers to other
/// records that are stored externally to this DNA / zome.
///
/// In the local DNA, this consists of `base` addresses for all referenced foreign
/// content, bidirectionally linked to the originating record for querying in either direction.
///
/// In the remote DNA, a corresponding remote query index is built via `create_remote_query_index`,
/// which is presumed to be linked to the other end of the specified `remote_zome_method`.
pub fn create_local_query_index(
    remote_base_entry_type: &str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: Vec<Address>,
) -> Vec<ZomeApiResult<Address>> {
    // abort if target_base_addresses are empty
    if target_base_addresses.len() == 0 { return vec![] }

    // Build local index first (for reading linked record IDs from the `source_base_address`)
    let results: Vec<ZomeApiResult<Address>> = target_base_addresses.iter()
        .map(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_entry_result = create_base_entry(&(remote_base_entry_type.to_string().into()), base_entry_addr);

            match &base_entry_result {
                Ok(base_address) => {
                    // link event to commitment by `fulfilled`/`fulfilledBy` edge
                    link_entries_bidir(
                        &source_base_address, base_address,
                        origin_relationship_link_type, origin_relationship_link_tag,
                        destination_relationship_link_type, destination_relationship_link_tag
                    );
                },
                _ => (),
            }

            base_entry_result
        })
        .collect();

    results
}

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the addresses of the (respectively) forward & reciprocal links created.
pub fn link_entries_bidir<S: Into<String>>(
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



// READ

/// Load any set of records of type `R` that are linked from the
/// `base_address` entry via `link_type` and `link_name`.
///
/// Results are automatically deserialized into `R` as they are retrieved from the DHT.
/// Any entries that either fail to load or cannot be converted to the type will be dropped.
///
/// :TODO: return errors, improve error handling
///
pub fn get_links_and_load_entry_data<R, F, A>(
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
    load_entry_data(addrs_result.unwrap())
}

/// Load any set of records of type `R` that are linked from the
/// `base_address` entry via `link_type` and `link_name`, where the
/// `base_address` is incoming from an external DNA.
///
pub fn get_remote_links_and_load_entry_data<'a, R, F, A>(
    base_address: &F,
    base_entry_type: &'a str,
    link_type: &str,
    link_name: &str,
) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
        F: AsRef<Address> + Into<JsonString> + Clone,
{
    let query_address = get_index_address(base_entry_type.to_string(), base_address.as_ref());
    if let Err(resolve_remote_err) = query_address {
        return Err(resolve_remote_err);
    }

    let addrs_result = get_linked_addresses(&query_address.unwrap(), link_type, link_name);
    if let Err(get_links_err) = addrs_result {
        return Err(get_links_err);
    }
    load_entry_data(addrs_result.unwrap())
}

/// Loads up all entry data for the input list of addresses and returns a vector
/// of tuples corresponding to the entry address and deserialized entry data.
///
fn load_entry_data<R, A>(addresses: Vec<Address>) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
{
    let entries: Vec<Option<R>> = addresses.iter()
        .map(|address| {
            let entry_address = get_entry(&address)?;
            let entry = match entry_address {
                Some(AppEntry(_, entry_address_value)) => {
                    get_entry(&Address::try_from(entry_address_value)?)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            };
            match entry {
                Ok(Some(AppEntry(_, entry_value))) => {
                    match R::try_from(entry_value.to_owned()) {
                        Ok(val) => Ok(Some(val)),
                        Err(_) => Err(ZomeApiError::Internal("Could not convert get_links result to requested type".to_string())),
                    }
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            }
        })
        .filter_map(Result::ok)
        .collect();

    Ok(addresses.iter()
        .map(|address| {
            address.to_owned().into()
        })
        .zip(entries)
        .collect()
    )
}

/// Load a set of addresses of type `T` and automatically coerce them to the
/// provided newtype wrapper.
///
/// The `Address` array is returned wrapped in a copy-on-write smart pointer
/// such that it can be cheaply passed by reference into Serde output
/// structs & functions via `Cow::into_owned`.
///
/// :TODO: propagate errors in link retrieval
///
/// @see `type_aliases.rs`
///
pub fn get_linked_addresses_as_type<'a, T, I>(
    base_address: I,
    link_type: &str,
    link_tag: &str,
) -> Cow<'a, Vec<T>>
    where T: From<Address> + Clone, I: AsRef<Address>
{
    let addrs = get_linked_addresses(base_address.as_ref(), link_type, link_tag);
    if let Err(_get_links_err) = addrs {
        return Cow::Owned(vec![]);  // :TODO: improve error handling
    }

    Cow::Owned(addrs.unwrap().iter()
        .map(|addr| { T::from(addr.to_owned()) })
        .collect())
}

/// Similar to `get_linked_addresses_as_type` except that the returned addresses
/// refer to remote entries from external networks.
///
/// This means that the addresses are "dereferenced" by way of a `base` entry
/// which contains the address of the link target as entry data.
///
pub fn get_linked_remote_addresses_as_type<'a, T, I>(
    base_address: I,
    link_type: &str,
    link_tag: &str,
) -> Cow<'a, Vec<T>>
    where T: From<Address> + Clone, I: AsRef<Address>
{
    let addrs = get_linked_addresses(base_address.as_ref(), link_type, link_tag);
    if let Err(_get_links_err) = addrs {
        return Cow::Owned(vec![]);  // :TODO: improve error handling
    }

    Cow::Owned(addrs.unwrap().iter()
        .filter_map(|addr| {
            let base_addr_request = get_dereferenced_address(&addr);
            match base_addr_request {
                Ok(remote_addr) => Some(T::from(remote_addr)),
                Err(_) => None,     // :TODO: handle errors gracefully
            }
        })
        .collect())
}

/// Load any set of addresses that are linked from the
/// `base_address` entry via `link_type` and `link_name`.
///
fn get_linked_addresses(
    base_address: &Address,
    link_type: &str,
    link_tag: &str,
) -> ZomeApiResult<Vec<Address>> {
    let get_links_result = get_links_with_options(
        base_address,
        LinkMatch::Exactly(link_type),
        LinkMatch::Exactly(link_tag),
        GetLinksOptions::default(),
    );
    if let Err(get_links_err) = get_links_result {
        return Err(get_links_err);
    }

    Ok(get_links_result.unwrap().addresses())
}

/// Query the 'index' address for a given external `base_address`.
/// The `base_entry_type` must be provided in order to calculate the entry hash.
fn get_index_address<A, S>(base_entry_type: S, base_address: &Address) -> ZomeApiResult<A>
    where S: Into<AppEntryType>,
        A: From<Address>,
{
    entry_address(&AppEntry(base_entry_type.into(), (*base_address).clone().into()))
        .map(|addr| { addr.into() })
}



// UPDATE

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
pub fn replace_entry_link_set<A, B>(
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
                link_entries_bidir(
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

/// Same as `replace_entry_link_set` except that the replaced links
/// are matched against dereferenced addresses pointing to entries in other DNAs.
///
/// Returns the addresses of the previously erased link targets, if any.
///
/// :TODO: update to accept multiple targets for the replacement links
///
pub fn replace_remote_entry_link_set<A, B, S>(
    source: &A,
    new_dest: &MaybeUndefined<B>,
    base_entry_type: S,
    link_type: &str,
    link_name: &str,
    link_type_reciprocal: &str,
    link_name_reciprocal: &str,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq + Debug,
        S: Into<AppEntryType>,
{
    // if not updating, skip operation
    if let MaybeUndefined::Undefined = new_dest {
        return Ok(vec![]);
    }

    // load any existing links from the originating address
    let existing_links: Vec<B> = get_linked_addresses_as_type(source, link_type, link_name).into_owned();

    // determine links to erase
    let to_erase: Vec<B> = existing_links.iter()
        .filter(dereferenced_link_does_not_match(new_dest)).map(|x| { (*x).clone() }).collect();

    // wipe stale links. Note we don't remove the base addresses, dangling remnants do no harm.
    // :TODO: propagate errors
    let _erased: Vec<ZomeApiResult<()>> = to_erase.iter().flat_map(wipe_links_from_origin(
        link_type, link_name,
        link_type_reciprocal, link_name_reciprocal,
        source,
    )).collect();

    // get base addresses of erased items
    let erased: Vec<ZomeApiResult<Address>> = to_erase.iter().map(|addr| { get_dereferenced_address(addr.as_ref()) }).collect();

    // run insert if needed
    match new_dest {
        MaybeUndefined::Some(new_link) => {
            let already_present = existing_links.iter().filter(dereferenced_link_matches(new_dest)).count() > 0;

            if already_present {
                Ok(erased)
            } else {
                let new_dest_pointer = create_base_entry(&(base_entry_type.into()), new_link.as_ref());
                if let Err(e) = new_dest_pointer {
                    return Err(e);
                }
                link_entries_bidir(
                    source.as_ref(), &(new_dest_pointer.unwrap()),
                    link_type, link_name,
                    link_type_reciprocal, link_name_reciprocal
                );  // :TODO: error handling
                Ok(erased)
            }
        },
        _ => Ok(erased),
    }
}

// internals for link update logic

/// Filter predicate to include all links which do not match the destination link
fn link_does_not_match<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    Box::new(move |&existing_link| {
        match &new_dest {
            // return comparison of existing value & newly provided value
            &MaybeUndefined::Some(new_link) => {
                *new_link != *existing_link
            },
            // nothing matches if new value is None or Undefined
            _ => true,
        }
    })
}

/// Filter predicate to include all links which match the destination link
fn link_matches<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    let inverse_filter = link_does_not_match(new_dest);
    Box::new(move |&existing_link| {
        !inverse_filter(&existing_link)
    })
}

/// Filter predicate to include all links which do not reference an entry containing the destination link
fn dereferenced_link_does_not_match<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    Box::new(move |&existing_link| {
        let existing_target = get_dereferenced_address(existing_link.as_ref());
        match &new_dest {
            &MaybeUndefined::Some(new_link) => {
                // ignore any current values which encounter errors
                if let Err(_) = existing_target {
                    return false;   // :NOTE: this should never happen if all write logic goes OK
                }
                // return comparison of existing (dereferenced) value & newly provided value
                *new_link != existing_target.unwrap().into()
            },
            // nothing matches if new value is None or Undefined
            _ => true,
        }
    })
}

/// Filter predicate to include all links which reference an entry containing the destination link
fn dereferenced_link_matches<'a, B>(new_dest: &'a MaybeUndefined<B>) -> Box<dyn for<'r> Fn(&'r &'a B) -> bool + 'a>
    where B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    let inverse_filter = dereferenced_link_does_not_match(new_dest);
    Box::new(move |&existing_link| {
        !inverse_filter(&existing_link)
    })
}


/// Iterator processor to wipe all links originating from a given `source` address
fn wipe_links_from_origin<'a, A, B>(
    link_type: &'a str,
    link_name: &'a str,
    link_type_reciprocal: &'a str,
    link_name_reciprocal: &'a str,
    source: &'a A,
) -> Box<dyn Fn(&'a B) -> Vec<ZomeApiResult<()>> + 'a>
    where A: AsRef<Address>,
        B: AsRef<Address> + From<Address> + Clone + PartialEq,
{
    Box::new(move |remove_link| {
        remove_links_bidir(
            source.as_ref(), remove_link.as_ref(),
            link_type, link_name,
            link_type_reciprocal, link_name_reciprocal,
        )
    })
}



// DELETE

/// Deletes a bidirectional link between two entry addresses, and returns any errors encountered
/// to the caller.
///
/// :TODO: filter empty success tuples from results and return as flattened error array
///
fn remove_links_bidir<S: Into<String>>(
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
