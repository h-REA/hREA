/**
 * Helpers related to management of low-level Holochain entries.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use std::convert::{ TryFrom };
use hdk::{
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry,
        entry::Entry::App as AppEntry,
        entry::entry_type::AppEntryType,
        entry::AppEntryValue,
    },
    error::{ ZomeApiError, ZomeApiResult },
    entry_address,
    get_entry,
    commit_entry,
    update_entry as hdk_update_entry,
    remove_entry,
    utils:: {
        get_as_type,    // :TODO: switch this method to one which doesn't consume the input
    },
};

use super::{
    record_interface::Updateable,
    anchors::{
        get_anchor_index_entry_address,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Reads an entry via its `anchor index`.
///
/// Follows an anchor identified by `id_entry_type`, `id_link_type` and
/// its well-known `id_string` to retrieve whichever entry of type `T` resides
/// at the anchored address.
///
/// @see anchor_helpers.rs
///
pub fn get_entry_by_anchor_index<T, E>(
    id_entry_type: &E,
    id_link_type: &str,
    id_string: &String,
) -> ZomeApiResult<T>
    where E: Into<AppEntryType> + Clone,
        T: TryFrom<AppEntryValue>,
{
    // determine ID anchor entry address
    let entry_address = get_anchor_index_entry_address(id_entry_type, id_link_type, id_string)?;
    match entry_address {
        Some(address) => {
            let entry = get_entry(&address);
            let decoded = try_decode_entry(entry);
            match decoded {
                Ok(Some(entry)) => {
                    Ok(entry)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            }
        },
        None => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
    }
}

/// Loads up all entry data for the input list of `Addresses` and returns a vector
/// of tuples corresponding to the entry address and deserialized entry data.
///
pub (crate) fn get_entries_by_address<R, A>(addresses: Vec<Address>) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
{
    let entries: Vec<Option<R>> = addresses.iter()
        .map(|address| {
            let entry = get_entry(&address);
            try_decode_entry(entry)
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

/// Loads up all entry data for the input list of `key indexes` and returns a vector
/// of tuples corresponding to the entry key's address and deserialized entry data.
///
pub (crate) fn get_entries_by_key_index<R, A>(addresses: Vec<Address>) -> ZomeApiResult<Vec<(A, Option<R>)>>
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

            try_decode_entry(entry)
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

/// Helper for handling decoding of entry data to requested entry struct type
///
pub (crate) fn try_decode_entry<R>(entry: ZomeApiResult<Option<Entry>>) -> ZomeApiResult<Option<R>>
    where R: TryFrom<AppEntryValue>,
{
    match entry {
        Ok(Some(AppEntry(_, entry_value))) => {
            match R::try_from(entry_value.to_owned()) {
                Ok(val) => Ok(Some(val)),
                Err(_) => Err(ZomeApiError::Internal("Could not convert entry to requested type".to_string())),
            }
        },
        _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
    }
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new entry in the DHT and returns a tuple of
/// the `entry address` and initial record `entry` data.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_entry<E, C, S>(
    entry_type: S,
    create_payload: C,
) -> ZomeApiResult<(Address, E)>
    where E: Clone + Into<AppEntryValue>,
        C: Into<E>,
        S: Into<AppEntryType>,
{
    // convert the type's CREATE payload into internal struct via built-in conversion trait
    let entry_struct: E = create_payload.into();
    // clone entry for returning to caller
    // :TODO: should not need to do this if AppEntry stops consuming the value
    let entry_resp = entry_struct.clone();

    // write underlying entry and get initial address
    let entry = AppEntry(entry_type.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    Ok((address, entry_resp))
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Updates a record in the DHT directly. Appropriate for entries which do not have
/// "base address" indexing and rely on other custom indexing logic (eg. anchor indexes).
///
/// The way in which the input update payload is applied to the existing
/// entry data is up to the implementor of `Updateable<U>` for the entry type.
///
pub fn update_entry<E, U, A, S>(
    entry_type: S,
    address: &A,
    update_payload: &U,
) -> ZomeApiResult<(Address, E)>
    where E: Clone + TryFrom<AppEntryValue> + Into<AppEntryValue> + Updateable<U>,
        S: Into<AppEntryType> + Clone,
        A: AsRef<Address>,
{
    let prev_entry: E = get_as_type((*(address.as_ref())).clone())?;
    // :NOTE: to handle update checks we need the *exact* most recent entry address, not that of the head of the entry chain
    let data_address = entry_address(&(AppEntry(entry_type.clone().into(), prev_entry.to_owned().into())))?;

    // perform update logic
    let new_entry = prev_entry.update_with(update_payload);

    // clone entry for returning to caller
    // :TODO: should not need to do this if AppEntry stops consuming the value
    let entry_resp = new_entry.clone();

    // store updated entry back to the DHT if there was a change
    let entry = AppEntry(entry_type.into(), new_entry.into());

    // :IMPORTANT: only write if data has changed, otherwise core gets in infinite loops
    // @see https://github.com/holochain/holochain-rust/issues/1662
    let new_hash = entry_address(&entry)?;
    if new_hash != data_address {
        hdk_update_entry(entry, &data_address)?;
    }

    Ok((new_hash, entry_resp))
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<T>(
    addr: &Address,
) -> ZomeApiResult<bool>
    where T: TryFrom<AppEntryValue>
{
    let entry_data: ZomeApiResult<T> = get_as_type(addr.to_owned());
    match entry_data {
        Ok(_) => {
            remove_entry(&addr)?;
            Ok(true)
        },
        Err(_) => Err(ZomeApiError::ValidationFailed("incorrect record type specified for deletion".to_string())),
    }
}
