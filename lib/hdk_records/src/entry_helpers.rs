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
use hdk::prelude::*;
use hdk::prelude::{
    create as hdk_create,
    update as hdk_update,
    delete_entry as hdk_delete_entry,
};

use crate::{
    HeaderHash,
    RecordAPIResult, DataIntegrityError,
    metadata_helpers::RevisionMeta,
};

/// Helper to handle retrieving linked element entry from an element
///
pub fn try_entry_from_element<'a>(element: &'a Element) -> RecordAPIResult<&'a Entry> {
    element.entry().as_option().ok_or(DataIntegrityError::EntryNotFound)
}

/// Helper for handling decoding of entry data to requested entry struct type
///
/// :TODO: check the performance of this function, into_sb() is copying data
///
pub (crate) fn try_decode_entry<T>(entry: Entry) -> RecordAPIResult<T>
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
/// :DUPE: identical to below, only type signature differs
///
pub fn get_entry_by_address<R>(address: &EntryHash) -> RecordAPIResult<(RevisionMeta, R)>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    let maybe_result = get((*address).clone(), GetOptions { strategy: GetStrategy::Latest });
    let element = match maybe_result {
        Ok(Some(el)) => el,
        _ => return Err(DataIntegrityError::EntryNotFound),
    };

    let entry = try_entry_from_element(&element)?;
    let decoded = try_decode_entry(entry.to_owned());
    match decoded {
        Err(DataIntegrityError::Serialization(_)) => Err(DataIntegrityError::EntryWrongType),
        Err(_) => Err(DataIntegrityError::EntryNotFound),
        _ => Ok((element.into(), decoded?)),
    }
}

/// Reads an entry from the DHT by its `HeaderHash`. The specific requested version of the entry will be returned.
///
/// :DUPE: identical to above, only type signature differs
///
pub fn get_entry_by_header<R>(address: &HeaderHash) -> RecordAPIResult<(RevisionMeta, R)>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    let maybe_result = get(address.clone(), GetOptions { strategy: GetStrategy::Latest });
    let element = match maybe_result {
        Ok(Some(el)) => el,
        _ => return Err(DataIntegrityError::EntryNotFound),
    };

    let entry = try_entry_from_element(&element)?;
    let decoded = try_decode_entry(entry.to_owned());
    match decoded {
        Err(DataIntegrityError::Serialization(_)) => Err(DataIntegrityError::EntryWrongType),
        Err(_) => Err(DataIntegrityError::EntryNotFound),
        _ => Ok((element.into(), decoded?)),
    }
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
pub fn create_entry<I: Clone, E, S: AsRef<str>>(
    entry_def_id: S,
    entry_struct: I,
) -> RecordAPIResult<(RevisionMeta, EntryHash)>
    where WasmError: From<E>,
        Entry: TryFrom<I, Error = E>,
{
    let entry_hash = hash_entry(entry_struct.clone())?;

    let entry_data: Result<Entry, E> = entry_struct.try_into();
    match entry_data {
        Ok(entry) => {
            let header_hash = hdk_create(CreateInput::new(EntryDefId::App(entry_def_id.as_ref().to_string()), entry, ChainTopOrdering::default()))?;

            let maybe_result = get(header_hash, GetOptions { strategy: GetStrategy::Latest });
            let element = match maybe_result {
                Ok(Some(el)) => el,
                _ => return Err(DataIntegrityError::EntryNotFound),
            };

            Ok((element.into(), entry_hash))
        },
        Err(e) => Err(DataIntegrityError::Wasm(WasmError::from(e))),
    }
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
pub fn update_entry<'a, I: Clone, E, S: AsRef<str>>(
    entry_def_id: S,
    address: &HeaderHash,
    new_entry: I,
) -> RecordAPIResult<(RevisionMeta, EntryHash)>
    where WasmError: From<E>,
        Entry: TryFrom<I, Error = E>,
{
    // get initial address
    let entry_address = hash_entry(new_entry.clone())?;

    // perform update logic
    let entry_data: Result<Entry, E> = new_entry.try_into();
    match entry_data {
        Ok(entry) => {
            let updated_header = hdk_update(address.clone(), CreateInput::new(EntryDefId::App(entry_def_id.as_ref().to_string()), entry, ChainTopOrdering::default()))?;

            let maybe_result = get(updated_header, GetOptions { strategy: GetStrategy::Latest });
            let element = match maybe_result {
                Ok(Some(el)) => el,
                _ => return Err(DataIntegrityError::EntryNotFound),
            };

            Ok((element.into(), entry_address))
        },
        Err(e) => Err(DataIntegrityError::Wasm(WasmError::from(e))),
    }
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<T>(
    address: &HeaderHash,
) -> RecordAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    // typecheck the record before deleting, to prevent any accidental or malicious cross-type deletions
    let (_meta, _prev_entry): (RevisionMeta, T) = get_entry_by_header(address)?;

    hdk_delete_entry(address.clone())?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[hdk_entry(id="test_entry")]
    #[derive(Clone, PartialEq)]
    pub struct TestEntry {
        field: Option<String>,
    }

    #[test]
    fn test_roundtrip() {
        let entry = TestEntry { field: None };

        // CREATE
        let (header_hash, entry_hash) = create_entry("test_entry", entry.clone()).unwrap();

        // READ
        let e1: TestEntry = get_entry_by_address(&entry_hash).unwrap();
        let e2: TestEntry = get_entry_by_header(&header_hash).unwrap();

        assert_eq!(e1, entry, "failed to read entry by EntryHash");
        assert_eq!(e2, entry, "failed to read entry by HeaderHash");
        assert_eq!(e1, e2, "unexpected different entry at HeaderHash vs EntryHash");

        // UPDATE
        let new_entry = TestEntry { field: Some("val".to_string()) };
        let (updated_header, updated_entry) = update_entry("test_entry", &header_hash, new_entry).unwrap();

        assert_ne!(updated_header, header_hash, "update HeaderHash did not change");
        assert_ne!(updated_entry, entry_hash, "update EntryHash did not change");

        let u1: TestEntry = get_entry_by_address(&updated_entry).unwrap();
        let u2: TestEntry = get_entry_by_header(&updated_header).unwrap();

        assert_ne!(u1, entry, "failed to read entry by EntryHash");
        assert_ne!(u2, entry, "failed to read entry by HeaderHash");
        assert_eq!(u1, u2, "unexpected different entry at HeaderHash vs EntryHash after update");

        let o1: TestEntry = get_entry_by_address(&entry_hash).unwrap();
        assert_eq!(o1, entry, "retrieving entry by old hash should return original data");

        // DELETE
        let success = delete_entry::<TestEntry>(&updated_header).unwrap();

        assert!(success, "entry deletion failed");

        let try_retrieve = get_entry_by_address::<TestEntry>(&entry_hash);

        assert!(try_retrieve.is_err(), "entry retrieval after deletion should error");

        let try_retrieve_old = get_entry_by_header::<TestEntry>(&header_hash);
        let try_retrieve_deleted = get_entry_by_header::<TestEntry>(&updated_header);

        assert_eq!(try_retrieve_old.unwrap(), entry, "archival entry retrieval by header after deletion should return successfully");
        assert!(try_retrieve_deleted.is_err(), "entry retrieval by header after deletion should error");
    }
}
