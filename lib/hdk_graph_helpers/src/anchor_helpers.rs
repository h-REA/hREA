/**
 * Helper methods related to entry `anchors`.
 *
 * Anchors are named entries that provide lookup functionality for locating un-named entries.
 * They are most often used like "primary keys" which can be retrieved by a predictable ID
 * in order to lookup the underlying referenced entry.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2020-01-04
 */
use std::convert::{ TryFrom };
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry::App as AppEntry,
        entry::entry_type::AppEntryType,
        entry::AppEntryValue,
    },
    error::{ ZomeApiResult, ZomeApiError },
    entry_address,
    commit_entry,
    remove_entry,
    link_entries,
    remove_link,
};

use super::{
    identifiers::{ ANCHOR_POINTER_LINK_TAG, ERR_MSG_ENTRY_NOT_FOUND },
    links::{
        get_linked_addresses,
    },
    local_indexes::query_direct_index_with_foreign_key,
    type_wrappers::Addressable,
};

//--------------------------------[ READ ]--------------------------------------

pub fn get_anchor_index_entry_address<E>(
    id_entry_type: &E,
    id_link_type: &str,
    id_string: &String,
) -> ZomeApiResult<Option<Address>>
    where E: Into<AppEntryType> + Clone,
{
    // determine anchor entry address
    let anchor_address = determine_anchor_index_address(id_entry_type, id_string)?;

    // query linked entry
    let mut entries: Vec<Address> = get_linked_addresses(&anchor_address, id_link_type, ANCHOR_POINTER_LINK_TAG)?;
    // :TODO: ensure only 1 anchor per entry?
    Ok(entries.pop())
}

/// Reads a set of entries which have been referenced from a base `anchor index`.
///
/// Follows an anchor identified by `anchor_entry_type`, `anchor_link_type` and
/// some well-known `anchor_string` to retrieve the set of entries of type `T`
/// that are linked via their own `key indexes` following `record_link_type`
/// and `record_link_name`.
///
/// Works like reading an unfiltered list of records from a database table.
///
/// :TODO: this is very much a higher-order level of abstraction and should probably be
/// moved somewhere else or replaced with a DAG implementation.
///
/// @see local_index_helpers.rs
///
pub fn read_anchored_record_entries<T, E, A>(
    anchor_entry_type: &E,
    anchor_link_type: &str,
    anchor_string: &String,
) -> ZomeApiResult<Vec<(A, Option<T>)>>
    where E: Into<AppEntryType> + Clone,
        A: From<Address>,
        T: Clone + TryFrom<AppEntryValue>,
{
    // determine ID anchor entry address
    let anchor_address = determine_anchor_index_address(anchor_entry_type, anchor_string)?;
    // retrieve the indexed records by querying the anchor index
    query_direct_index_with_foreign_key(&Addressable::from(anchor_address), anchor_link_type, ANCHOR_POINTER_LINK_TAG)
}

fn determine_anchor_index_address<E>(
    id_entry_type: &E,
    id_string: &String,
) -> ZomeApiResult<Address>
    where E: Into<AppEntryType> + Clone,
{
    let anchor_entry = AppEntry(id_entry_type.to_owned().into(), Some((*id_string).to_owned()).into());
    entry_address(&anchor_entry)
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates an `anchor index` - an entry which stores a well-known data payload which
/// can be determined programatically in order to locate it; and a link which points
/// from the `anchor` entry to the target `entry_address`.
///
/// Returns the address of the new anchor index on success.
///
pub fn create_anchor_index<E>(
    id_entry_type: &E,
    id_link_type: &str,
    id_string: &String,
    entry_address: &Address,
) -> ZomeApiResult<Address>
    where E: Into<AppEntryType> + Clone,
{
    let anchor_entry = AppEntry(id_entry_type.to_owned().into(), Some((*id_string).to_owned()).into());
    let maybe_anchor_address = commit_entry(&anchor_entry);

    match maybe_anchor_address {
        Ok(anchor_address) => {
            link_entries(&anchor_address, entry_address, id_link_type, ANCHOR_POINTER_LINK_TAG)?;  // :TODO: error handling? Probably fine to treat as critical?
            Ok(anchor_address)
        },
        Err(e) => Err(e),
    }
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Update an `anchor index` by changing the identifier from `old_id_string` to
/// `new_id_string` for the given `entry_address`.
///
/// Returns the address of the new anchor index on success.
///
pub fn update_anchor_index<E>(
    id_entry_type: &E,
    id_link_type: &str,
    entry_address: &Address,
    old_id_string: &String,
    new_id_string: &String,
) -> ZomeApiResult<Address>
    where E: Into<AppEntryType> + Clone,
{
    // determine anchor entry address
    let old_anchor_address = determine_anchor_index_address(id_entry_type, old_id_string)?;

    // no-op if ID has not changed
    if &old_id_string[..] == &new_id_string[..] {
        return Ok(old_anchor_address);
    }

    // wipe old anchor
    remove_link(&old_anchor_address, entry_address, id_link_type, ANCHOR_POINTER_LINK_TAG)?;
    remove_entry(&old_anchor_address)?;

    // create new anchor
    create_anchor_index(id_entry_type, id_link_type, new_id_string, entry_address)
}

//-------------------------------[ DELETE ]-------------------------------------

pub fn delete_anchor_index<E>(
    id_entry_type: &E,
    id_link_type: &str,
    id_string: &String,
) -> ZomeApiResult<bool>
    where E: Into<AppEntryType> + Clone,
{
    // determine anchor entry address
    let anchor_address = determine_anchor_index_address(id_entry_type, id_string)?;

    // determine underlying entry address
    let check_entry_addr = get_anchor_index_entry_address(id_entry_type, id_link_type, id_string)?;

    // if all validates, wipe anchoring entry & corresponding link
    match check_entry_addr {
        None => Err(ZomeApiError::Internal(ERR_MSG_ENTRY_NOT_FOUND.to_string())),
        Some(entry_addr) => {
            remove_link(&anchor_address, &entry_addr, id_link_type, ANCHOR_POINTER_LINK_TAG)?;
            remove_entry(&anchor_address)?;
            Ok(true)
        },
    }
}
