/**
 * Helpers related to management of low-level Holochain entries.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use std::convert::{ TryFrom };
use hdk3::prelude::*;

use super::{
    identifiers::{ ERR_MSG_ENTRY_NOT_FOUND, ERR_MSG_ENTRY_WRONG_TYPE },
    record_interface::Updateable,
};

/// Helper to handle retrieving linked element entry from an element
///
pub fn try_entry_from_element<'a>(element: &'a Option<Element>) -> ExternResult<&'a Entry> {
    element
        .as_ref()
        .and_then(|el| el.entry().as_option())
        .ok_or(crate::error(ERR_MSG_ENTRY_NOT_FOUND))
}

/// Helper for handling decoding of entry data to requested entry struct type
///
pub (crate) fn try_decode_entry<'a, T: TryFrom<SerializedBytes>>(entry: &'a Entry) -> ExternResult<&'a T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(&e),
            Err(_) => Err(crate::error(ERR_MSG_ENTRY_WRONG_TYPE)),
        },
        _ => Err(crate::error(ERR_MSG_ENTRY_NOT_FOUND)),
    }
}

//--------------------------------[ READ ]--------------------------------------

pub fn get_entry_by_address<'a, R, A>(address: &A) -> ExternResult<&'a R>
    where R: TryFrom<SerializedBytes>,
        A: Into<EntryHash>,
{
    try_decode_entry(try_entry_from_element(&get!((*address).into())?)?)
}

/// Loads up all entry data for the input list of `EntryHash` and returns a vector
/// of results corresponding to the deserialized entry data.
///
/// If your calling code needs to assocate hashes with results, it is recommended
/// that your next step be to `zip` the return value of this function onto the input
/// `addresses`.
///
pub (crate) fn get_entries_by_address<'a, R, A>(addresses: Vec<A>) -> Vec<ExternResult<&'a R>>
    where R: TryFrom<SerializedBytes>,
        A: Into<EntryHash>,
{
    addresses.iter()
        .map(get_entry_by_address)
        .collect()
}

/// Loads up all entry data for the input list of `key indexes` and returns a vector
/// of results corresponding to the deserialized entry data.
///
/// If your calling code needs to assocate hashes with results, it is recommended
/// that your next step be to `zip` the return value of this function onto the input
/// `addresses`.
///
pub (crate) fn get_entries_by_key_index<'a, R, A>(addresses: Vec<EntryHash>) -> Vec<ExternResult<&'a R>>
    where R: TryFrom<SerializedBytes>,
        A: From<EntryHash>,
{
    addresses.iter()
        .map(|address| {
            let entry_address: &EntryHash = get_entry_by_address(address)?;
            get_entry_by_address(entry_address)
        })
        .collect()
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
