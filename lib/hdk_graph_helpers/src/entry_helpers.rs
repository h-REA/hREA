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

use crate::{GraphAPIResult, DataIntegrityError};

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
pub (crate) fn get_entry_by_address<R>(address: EntryHash) -> GraphAPIResult<R>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    // :DUPE: identical to below, only type signature differs
    let result = get(address, GetOptions)?;
    let entry = try_entry_from_element(result.as_ref())?;
    try_decode_entry(entry.to_owned())
}

/// Reads an entry from the DHT by its `HeaderHash`. The specific requested version of the entry will be returned.
///
pub (crate) fn get_entry_by_header<R>(address: HeaderHash) -> GraphAPIResult<R>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    // :DUPE: identical to above, only type signature differs
    let result = get(address, GetOptions)?;
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
pub (crate) fn get_entries_by_address<'a, R>(addresses: &[EntryHash]) -> Vec<GraphAPIResult<R>>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    addresses.iter().cloned()
        .map(get_entry_by_address)
        .collect()
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new entry in the DHT and returns a tuple of
/// the `header address` and `entry address`.
///
/// It is recommended that you include a creation timestamp or source of randomness
/// in newly created records, to avoid them conflicting with previously entered
/// entries that may be of the same content.
///
/// @see hdk::prelude::random_bytes
///
pub fn create_entry<'a, E: 'a>(
    entry_struct: &'a E,
) -> GraphAPIResult<(HeaderHash, EntryHash)>
    where EntryDefId: From<&'a E>,
        SerializedBytes: TryFrom<&'a E, Error = SerializedBytesError>,
{
    let entry_hash = hash_entry(entry_struct)?;
    let header_hash = hdk_create_entry(entry_struct)?;

    Ok((header_hash, entry_hash))
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
pub fn update_entry<'a, E>(
    address: &'a HeaderHash,
    new_entry: &'a E,
) -> GraphAPIResult<(HeaderHash, EntryHash)>
    where EntryDefId: From<&'a E>,
        SerializedBytes: TryFrom<&'a E, Error = SerializedBytesError>,
{
    // get initial address
    let entry_address = hash_entry(&new_entry)?;

    // perform update logic
    let updated_header = hdk_update_entry((*address).clone(), &new_entry)?;

    Ok((updated_header, entry_address))
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<T>(
    address: &HeaderHash,
) -> GraphAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    // typecheck the record before deleting, to prevent any accidental or malicious cross-type deletions
    let _prev_entry: T = get_entry_by_header((*address).clone())?;

    hdk_delete_entry((*address).clone())?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[hdk_entry(id="test_entry")]
    #[derive(Clone, PartialEq, Debug)]
    pub struct TestEntry {
        field: Option<String>,
    }

    #[test]
    fn test_roundtrip() {
        let entry = TestEntry { field: None };

        // CREATE
        let (header_hash, entry_hash) = create_entry(&entry.clone()).unwrap();

        // READ
        let e1: TestEntry = get_entry_by_address(entry_hash.clone()).unwrap();
        let e2: TestEntry = get_entry_by_header(header_hash.clone()).unwrap();

        assert_eq!(e1, entry, "failed to read entry by EntryHash");
        assert_eq!(e2, entry, "failed to read entry by HeaderHash");
        assert_eq!(e1, e2, "unexpected different entry at HeaderHash vs EntryHash");

        // UPDATE
        let new_entry = TestEntry { field: Some("val".to_string()) };
        let (updated_header, updated_entry) = update_entry(&header_hash, &new_entry).unwrap();

        assert_ne!(updated_header, header_hash, "update HeaderHash did not change");
        assert_ne!(updated_entry, entry_hash, "update EntryHash did not change");

        let u1: TestEntry = get_entry_by_address(updated_entry).unwrap();
        let u2: TestEntry = get_entry_by_header(updated_header.clone()).unwrap();

        assert_ne!(u1, entry, "failed to read entry by EntryHash");
        assert_ne!(u2, entry, "failed to read entry by HeaderHash");
        assert_eq!(u1, u2, "unexpected different entry at HeaderHash vs EntryHash after update");

        let o1: TestEntry = get_entry_by_address(entry_hash.clone()).unwrap();
        assert_eq!(o1, entry, "retrieving entry by old hash should return original data");

        // DELETE
        let success = delete_entry::<TestEntry>(&updated_header).unwrap();

        assert!(success, "entry deletion failed");

        let try_retrieve = get_entry_by_address::<TestEntry>(entry_hash);

        assert!(try_retrieve.is_err(), "entry retrieval after deletion should error");
    }
}
