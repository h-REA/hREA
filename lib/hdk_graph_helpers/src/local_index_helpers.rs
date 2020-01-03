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
use hdk::{
    holochain_json_api::{ json::JsonString },
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::AppEntryValue,
    },
    error::{ ZomeApiResult },
    link_entries,
    remove_link,
};

use super::{
    entries::{
        get_entries_by_address,
        get_entries_by_key_index,
    },
    links::{
        get_linked_addresses,
    },
    keys::{
        determine_key_index_address,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Load any set of records of type `R` that are:
/// - linked locally (in the same DNA) from the `base_address`
/// - linked directly to the `base_address`, without any indirection
/// - linked via `link_type` and `link_name`
///
/// :TODO: return errors, improve error handling
///
pub fn query_direct_index<R, F, A>(
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
    get_entries_by_address(addrs_result.unwrap())
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
