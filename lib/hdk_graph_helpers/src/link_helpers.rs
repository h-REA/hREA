/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */

use std::borrow::Cow;
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        link::LinkMatch,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksOptions,
    error::{ ZomeApiResult },
    get_links_with_options,
};

use super::{
    keys::{
        get_key_index_address,
    },
};

// HDK re-exports
pub use hdk::link_entries;

//--------------------------------[ READ ]--------------------------------------

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
pub fn get_linked_addresses_with_foreign_key_as_type<'a, T, I>(
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
