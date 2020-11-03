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

/// Reads an entry from the DHT by its `EntryHash`. The latest live version of the entry will be returned.
///
pub (crate) fn get_entry_by_address<'a, R, A>(address: &'a A) -> ExternResult<&'a R>
    where R: TryFrom<SerializedBytes>,
        A: Into<EntryHash>,
{
    // :DUPE: identical to below, only type signature differs
    try_decode_entry(try_entry_from_element(&get!((*address).into())?)?)
}

/// Reads an entry from the DHT by its `HeaderHash`. The specific requested version of the entry will be returned.
///
pub (crate) fn get_entry_by_header<'a, R, A>(address: &'a A) -> ExternResult<&'a R>
    where R: TryFrom<SerializedBytes>,
        A: Into<HeaderHash>,
{
    // :DUPE: identical to above, only type signature differs
    try_decode_entry(try_entry_from_element(&get!((*address).into())?)?)
}

/// Loads up all entry data for the input list of `EntryHash` and returns a vector
/// of results corresponding to the deserialized entry data.
///
/// If your calling code needs to assocate hashes with results, it is recommended
/// that your next step be to `zip` the return value of this function onto the input
/// `addresses`.
///
pub (crate) fn get_entries_by_address<'a, R, A>(addresses: &'a Vec<A>) -> Vec<ExternResult<&'a R>>
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
/// the `header address`, `entry address` and initial record `entry` data.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
/// @see random_bytes!()
///
pub fn create_entry<E, C>(
    create_payload: C,
) -> ExternResult<(HeaderHash, EntryHash, E)>
    where E: Clone + Into<SerializedBytes>,
        C: Into<E>,
{
    // convert the type's CREATE payload into internal storage struct
    let entry_struct: E = create_payload.into();

    // clone entry for returning to caller
    // :TODO: should not need to do this if AppEntry stops consuming the value
    let entry_resp = entry_struct.clone();

    // get entry address
    let address = hash_entry!(entry_resp)?;

    // write underlying entry
    let header_address = create_entry!(entry_struct)?;

    Ok((header_address, address, entry_resp))
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Updates a record in the DHT directly. Appropriate for entries which do not have
/// "base address" indexing and rely on other custom indexing logic (eg. anchor indexes).
///
/// The way in which the input update payload is applied to the existing
/// entry data is up to the implementor of `Updateable<U>` for the entry type.
///
pub fn update_entry<E, U, A>(
    address: &A,
    update_payload: &U,
) -> ExternResult<(HeaderHash, EntryHash, E)>
    where E: Clone + TryFrom<SerializedBytes> + Into<SerializedBytes> + Updateable<U>,
        A: Into<HeaderHash>,
{
    // read previous record data
    let prev_entry: &E = get_entry_by_header(address)?;

    // apply the update payload to the previously retrievable version
    let new_entry: E = (*prev_entry).update_with(update_payload);

    // get initial address
    let entry_address = hash_entry!(new_entry.clone())?; // :TODO: optimise memory

    // perform update logic
    let updated_header = update_entry!(*address, new_entry.clone())?; // :TODO: optimise memory

    Ok((updated_header, entry_address, new_entry))
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<T>(
    address: &HeaderHash,
) -> ExternResult<bool>
    where T: TryFrom<SerializedBytes>
{
    // typecheck the record before deleting, to prevent any accidental or malicious cross-type deletions
    let _prev_entry: &T = get_entry_by_header(address)?;

    delete_entry!(*address)?;

    Ok(true)
}
