/**
 * Helpers related to management of low-level Holochain entries.
 *
 * :TODO:   General performance overhaul. Currently, some data
 *          is being cloned to deal with rough edges in the HDK.
 *
 *          @see traits bound to `Clone + Into`.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use std::convert::{ TryFrom };
use hdk3::prelude::*;
use hdk3::prelude::{
    create_entry as hdk_create_entry,
    update_entry as hdk_update_entry,
    delete_entry as hdk_delete_entry,
};

use crate::{
    record_interface::Updateable,
    GraphAPIResult, DataIntegrityError,
};

/// Helper to handle retrieving linked element entry from an element
///
pub fn try_entry_from_element<'a>(element: Option<&'a Element>) -> GraphAPIResult<&'a Entry> {
    element
        .and_then(|el| el.entry().as_option())
        .ok_or(DataIntegrityError::EntryNotFound)
}

/// Helper for handling decoding of entry data to requested entry struct type
///
/// :TODO: check the performance of this function, into_sb() is copying data
///
pub (crate) fn try_decode_entry<T>(entry: Entry) -> GraphAPIResult<T>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    match entry {
        Entry::App(content) => {
            let decoded: T = content.into_sb().try_into()?;
            Ok(decoded)
        },
        _ => Err(DataIntegrityError::EntryNotFound),
    }
}

//--------------------------------[ READ ]--------------------------------------

/// Reads an entry from the DHT by its `EntryHash`. The latest live version of the entry will be returned.
///
pub (crate) fn get_entry_by_address<'a, R, A>(address: &'a A) -> GraphAPIResult<R>
    where A: Clone + Into<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Clone,
{
    // :DUPE: identical to below, only type signature differs
    let result = get((*address).clone().into(), GetOptions)?;
    let entry = try_entry_from_element(result.as_ref())?;
    try_decode_entry(entry.to_owned())
}

/// Reads an entry from the DHT by its `HeaderHash`. The specific requested version of the entry will be returned.
///
pub (crate) fn get_entry_by_header<'a, R, A>(address: &'a A) -> GraphAPIResult<R>
    where A: Clone + Into<HeaderHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Clone,
{
    // :DUPE: identical to above, only type signature differs
    let result = get((*address).clone().into(), GetOptions)?;
    let entry = try_entry_from_element(result.as_ref())?;
    try_decode_entry(entry.to_owned())
}

/// Loads up all entry data for the input list of `EntryHash` and returns a vector
/// of results corresponding to the deserialized entry data.
///
/// If your calling code needs to assocate hashes with results, it is recommended
/// that your next step be to `zip` the return value of this function onto the input
/// `addresses`.
///
pub (crate) fn get_entries_by_address<'a, R, A>(addresses: &'a Vec<A>) -> Vec<GraphAPIResult<R>>
    where A: Clone + Into<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Clone,
{
    addresses.iter()
        .map(get_entry_by_address)
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
pub fn create_entry<'a, E: 'a, C>(
    create_payload: &C,
) -> GraphAPIResult<(HeaderHash, EntryHash, E)>
    where C: Clone + Into<E>,
        E: Clone + AsRef<&'a E>,
        EntryDefId: From<&'a E>,
        SerializedBytes: TryFrom<&'a E, Error = SerializedBytesError>,
{
    // convert the type's CREATE payload into internal storage struct
    let entry_struct: E = (*create_payload).clone().into();

    let entry_hash = hash_entry(entry_struct.as_ref())?;
    let header_hash = hdk_create_entry(entry_struct.as_ref())?;

    Ok((
        header_hash,
        entry_hash,
        entry_struct.to_owned(),
    ))
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Updates a record in the DHT directly. Appropriate for entries which do not have
/// "base address" indexing and rely on other custom indexing logic (eg. anchor indexes).
///
/// The way in which the input update payload is applied to the existing
/// entry data is up to the implementor of `Updateable<U>` for the entry type.
///
/// :TODO: determine how to implement some best-possible validation to alleviate at
///        least non-malicious forks in the hashchain of a datum.
///
pub fn update_entry<'a, E: 'a, U, A>(
    address: &'a A,
    update_payload: &U,
) -> GraphAPIResult<(HeaderHash, EntryHash, E)>
    where A: Clone + Into<HeaderHash>,
        E: Clone + AsRef<&'a E> + Updateable<U>,
        EntryDefId: From<&'a E>,
        SerializedBytes: TryFrom<&'a E, Error = SerializedBytesError>,
        SerializedBytes: TryInto<E, Error = SerializedBytesError>,
{
    // read previous record data
    let prev_entry: E = get_entry_by_header(address)?;

    // apply the update payload to the previously retrievable version
    let new_entry = prev_entry.update_with(update_payload);

    // get initial address
    let entry_address = hash_entry(new_entry.as_ref())?;

    // perform update logic
    let updated_header = hdk_update_entry((*address).clone().into(), new_entry.as_ref())?;

    Ok((updated_header, entry_address, new_entry))
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<'a, T: 'a, A>(
    address: &'a A,
) -> GraphAPIResult<bool>
    where A: Clone + Into<HeaderHash>,
        SerializedBytes: TryInto<T, Error = SerializedBytesError>,
        T: Clone,
{
    // typecheck the record before deleting, to prevent any accidental or malicious cross-type deletions
    let _prev_entry: T = get_entry_by_header(address.into())?;

    hdk_delete_entry((*address).clone().into())?;

    Ok(true)
}
