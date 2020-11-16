/**
 * Helpers related to management of low-level Holochain entries.
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
pub fn try_entry_from_element<'a>(element: &'a Option<Element>) -> GraphAPIResult<&'a Entry> {
    element
        .as_ref()
        .and_then(|el| el.entry().as_option())
        .ok_or(DataIntegrityError::EntryNotFound)
}

/// Helper for handling decoding of entry data to requested entry struct type
///
pub (crate) fn try_decode_entry<'a, T>(entry: &'a Entry) -> GraphAPIResult<&'a T>
    where SerializedBytes: TryInto<&'a T, Error = SerializedBytesError>,
{
    match entry {
        Entry::App(content) => {
            let decoded = T::try_from(content.into_sb());
            match decoded {
                Ok(e) => Ok(&e),
                Err(_) => Err(DataIntegrityError::EntryWrongType),
            }
        },
        _ => Err(DataIntegrityError::EntryNotFound),
    }
}

//--------------------------------[ READ ]--------------------------------------

/// Reads an entry from the DHT by its `EntryHash`. The latest live version of the entry will be returned.
///
pub (crate) fn get_entry_by_address<'a, R, A>(address: &'a A) -> GraphAPIResult<&'a R>
    where A: Into<EntryHash>,
        SerializedBytes: TryInto<&'a R, Error = SerializedBytesError>,
{
    // :DUPE: identical to below, only type signature differs
    try_decode_entry(try_entry_from_element(&get((*address).into(), GetOptions)?)?)
}

/// Reads an entry from the DHT by its `HeaderHash`. The specific requested version of the entry will be returned.
///
pub (crate) fn get_entry_by_header<'a, R, A>(address: &'a A) -> GraphAPIResult<&'a R>
    where A: Into<HeaderHash>,
        SerializedBytes: TryInto<&'a R, Error = SerializedBytesError>,
{
    // :DUPE: identical to above, only type signature differs
    try_decode_entry(try_entry_from_element(&get((*address).into(), GetOptions)?)?)
}

/// Loads up all entry data for the input list of `EntryHash` and returns a vector
/// of results corresponding to the deserialized entry data.
///
/// If your calling code needs to assocate hashes with results, it is recommended
/// that your next step be to `zip` the return value of this function onto the input
/// `addresses`.
///
pub (crate) fn get_entries_by_address<'a, R, A>(addresses: &'a Vec<A>) -> Vec<GraphAPIResult<&'a R>>
    where A: Into<EntryHash>,
        SerializedBytes: TryInto<&'a R, Error = SerializedBytesError>,
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
    create_payload: C,
) -> GraphAPIResult<(HeaderHash, EntryHash, E)>
    where C: Into<E>,
        EntryDefId: From<&'a E>,
        SerializedBytes: TryFrom<&'a E, Error = SerializedBytesError>,
{
    // convert the type's CREATE payload into internal storage struct
    let entry_struct: E = create_payload.into();

    // get entry address
    let address = hash_entry(&entry_struct)?;

    // write underlying entry
    let header_address = hdk_create_entry(&entry_struct)?;

    Ok((header_address, address, entry_struct))
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
    address: &A,
    update_payload: &U,
) -> GraphAPIResult<(HeaderHash, EntryHash, E)>
    where A: Into<HeaderHash>,
        EntryDefId: From<&'a E>,
        SerializedBytes: TryFrom<&'a E, Error = SerializedBytesError>,
        E: Into<SerializedBytes> + Updateable<U>,
{
    // read previous record data
    let prev_entry: &E = get_entry_by_header(address)?;

    // apply the update payload to the previously retrievable version
    let new_entry: E = (*prev_entry).update_with(update_payload);

    // get initial address
    let entry_address = hash_entry(&new_entry)?;

    // perform update logic
    let updated_header = hdk_update_entry((*address).into(), &new_entry)?;

    Ok((updated_header, entry_address, new_entry))
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<T, A>(
    address: &A,
) -> GraphAPIResult<bool>
    where T: TryFrom<SerializedBytes>,
        A: Into<HeaderHash>,
{
    // typecheck the record before deleting, to prevent any accidental or malicious cross-type deletions
    let _prev_entry: &T = get_entry_by_header(address)?;

    hdk_delete_entry((*address).into())?;

    Ok(true)
}
