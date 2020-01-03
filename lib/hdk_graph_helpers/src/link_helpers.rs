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
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::entry_type::AppEntryType,
        link::LinkMatch,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksOptions,
    error::{ ZomeApiResult },
    get_links_with_options,
};

use super::{
    MaybeUndefined,
    keys::{
        create_key_index,
        get_key_index_address,
        determine_key_index_address,
    },
    local_indexes::{
        create_direct_index,
        delete_direct_index,
    },
};

// HDK re-exports
pub use hdk::link_entries;


// READ

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
            let base_addr_request = get_key_index_address(&addr);
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
pub (crate) fn get_linked_addresses(
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
    let erased: Vec<ZomeApiResult<Address>> = to_erase.iter().map(|addr| { get_key_index_address(addr.as_ref()) }).collect();

    // run insert if needed
    match new_dest {
        MaybeUndefined::Some(new_link) => {
            let already_present = existing_links.iter().filter(dereferenced_link_matches(new_dest)).count() > 0;

            if already_present {
                Ok(erased)
            } else {
                let new_dest_pointer = create_key_index(&(base_entry_type.into()), new_link.as_ref());
                if let Err(e) = new_dest_pointer {
                    return Err(e);
                }
                create_direct_index(
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
        let existing_target = get_key_index_address(existing_link.as_ref());
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
        delete_direct_index(
            source.as_ref(), remove_link.as_ref(),
            link_type, link_name,
            link_type_reciprocal, link_name_reciprocal,
        )
    })
}



// DELETE

/// Deletes a set of links between a remote record reference and some set
/// of local target addresses.
///
/// The 'base' entry representing the remote target is not
/// affected in the removal, and is simply left dangling in the
/// DHT space as an indicator of previously linked items.
///
pub fn remove_remote_entry_link_set<'a, A, B>(
    source: &A,
    remove_targets: Vec<B>,
    base_entry_type: &'a str,
    link_type: &str,
    link_name: &str,
    link_type_reciprocal: &str,
    link_name_reciprocal: &str,
) -> Vec<ZomeApiResult<()>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq + Debug,
        Address: From<B>,
{
    let dereferenced_source: ZomeApiResult<A> = determine_key_index_address(base_entry_type.to_string(), source.as_ref());
    if let Err(e) = dereferenced_source {
        return vec![Err(e)]
    }

    let index_address = dereferenced_source.unwrap();
    remove_targets.iter()
        .flat_map(wipe_links_from_origin(
            link_type, link_name,
            link_type_reciprocal, link_name_reciprocal,
            &index_address,
        ))
        .collect()
}
