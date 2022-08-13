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
    ActionHash,
    RecordAPIResult, DataIntegrityError,
};

/// Helper to handle retrieving linked record entry from an record
///
pub fn try_entry_from_record<'a>(record: &'a Record) -> RecordAPIResult<&'a Entry> {
    record.entry().as_option().ok_or(DataIntegrityError::EntryNotFound)
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
pub fn get_entry_by_address<R>(address: &EntryHash) -> RecordAPIResult<(SignedActionHashed, R)>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    let maybe_result = get((*address).clone(), GetOptions { strategy: GetStrategy::Latest });
    let record = match maybe_result {
        Ok(Some(el)) => el,
        _ => return Err(DataIntegrityError::EntryNotFound),
    };

    let entry = try_entry_from_record(&record)?;
    let decoded = try_decode_entry(entry.to_owned());
    match decoded {
        Err(DataIntegrityError::Serialization(_)) => Err(DataIntegrityError::EntryWrongType),
        Err(_) => Err(DataIntegrityError::EntryNotFound),
        _ => Ok((record.signed_action().to_owned(), decoded?)),
    }
}

/// Reads an entry from the DHT by its `ActionHash`. The specific requested version of the entry will be returned.
///
/// :DUPE: identical to above, only type signature differs
///
pub fn get_entry_by_action<R>(address: &ActionHash) -> RecordAPIResult<(SignedActionHashed, R)>
    where SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    let maybe_result = get(address.clone(), GetOptions { strategy: GetStrategy::Latest });
    let record = match maybe_result {
        Ok(Some(el)) => el,
        _ => return Err(DataIntegrityError::EntryNotFound),
    };

    let entry = try_entry_from_record(&record)?;
    let decoded = try_decode_entry(entry.to_owned());
    match decoded {
        Err(DataIntegrityError::Serialization(_)) => Err(DataIntegrityError::EntryWrongType),
        Err(_) => Err(DataIntegrityError::EntryNotFound),
        _ => Ok((record.signed_action().to_owned(), decoded?)),
    }
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new entry in the DHT and returns a tuple of
/// the `action address` and `entry address`.
///
/// It is recommended that you include a creation timestamp or source of randomness
/// in newly created records, to avoid them conflicting with previously entered
/// entries that may be of the same content.
///
/// @see hdk::prelude::random_bytes
///
pub fn create_entry<T, I: Clone, E>(
    entry_struct: I,
) -> RecordAPIResult<(SignedActionHashed, EntryHash)>
    where WasmError: From<E>,
        Entry: TryFrom<I, Error = E>,
        T: From<I>,
        ScopedEntryDefIndex: for<'a> TryFrom<&'a T, Error = E>,
        EntryVisibility: for<'a> From<&'a T>,
{
    // use conversion traits to load HDK `EntryTypes` def for input entry data
    let wrapped_entry_struct: T = entry_struct.to_owned().into();
    let ScopedEntryDefIndex {
        zome_id, zome_type,
    } = (&wrapped_entry_struct).try_into().map_err(|e: E| DataIntegrityError::Wasm(e.into()))?;
    let visibility = EntryVisibility::from(&wrapped_entry_struct);

    // build a `CreateInput` for writing the entry, aborting on any `Entry` encoding errors
    let create_input = CreateInput::new(
        EntryDefLocation::app(zome_id, zome_type),
        visibility,
        entry_struct.to_owned().try_into().map_err(|e: E| DataIntegrityError::Wasm(e.into()))?,
        ChainTopOrdering::default(),
    );

    // write the entry data
    let action_hash = hdk_create(create_input)?;

    // retrieve written `Record` for returning signature information
    let maybe_result = get(action_hash, GetOptions { strategy: GetStrategy::Latest });
    let record = match maybe_result {
        Ok(Some(el)) => el,
        _ => return Err(DataIntegrityError::EntryNotFound),
    };

    // return `Record` signature and hash of `Entry`
    Ok((record.signed_action().to_owned(), hash_entry(entry_struct)?))
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
pub fn update_entry<'a, I: Clone, E>(
    address: &ActionHash,
    new_entry: I,
) -> RecordAPIResult<(SignedActionHashed, EntryHash)>
    where WasmError: From<E>,
        Entry: TryFrom<I, Error = E>
{
    // get initial address
    let entry_address = hash_entry(new_entry.clone())?;

    // perform update logic
    let entry_data: Result<Entry, E> = new_entry.try_into();
    match entry_data {
        Ok(entry) => {
            // let entry: ExternResult<Entry> = new_entry.try_into();
            let input = UpdateInput {
                original_action_address: address.clone(),
                entry,
                chain_top_ordering: ChainTopOrdering::default(),
            };
            let updated_action = hdk_update(input)?;

            let maybe_result = get(updated_action, GetOptions { strategy: GetStrategy::Latest });
            let record = match maybe_result {
                Ok(Some(el)) => el,
                _ => return Err(DataIntegrityError::EntryNotFound),
            };

            Ok((record.signed_action().to_owned(), entry_address))
        },
        Err(e) => Err(DataIntegrityError::Wasm(WasmError::from(e))),
    }
}

//-------------------------------[ DELETE ]-------------------------------------

/// Wrapper for `hdk::remove_entry` that ensures that the entry is of the specified type before deleting.
///
pub fn delete_entry<T>(
    address: &ActionHash,
) -> RecordAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    // typecheck the record before deleting, to prevent any accidental or malicious cross-type deletions
    let (_meta, _prev_entry): (_, T) = get_entry_by_action(address)?;

    hdk_delete_entry(address.clone())?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[hdk_entry_defs]
    #[derive(Clone)]
    #[unit_enum(UnitTypes)]
    enum EntryTypes {
        TestEntry(TestEntry),
    }
    #[hdk_entry_helper]
    #[derive(Clone, PartialEq)]
    pub struct TestEntry {
        field: Option<String>,
    }

    #[test]
    fn test_roundtrip() {
        let entry = TestEntry { field: None };
        let wrapped_entry = EntryTypes::TestEntry(entry);

        // CREATE
        let (RevisionMeta { id: action_hash, .. }, entry_hash) = create_entry(wrapped_entry).unwrap();

        // READ
        let e1: TestEntry = get_entry_by_address(&entry_hash).unwrap().1;
        let e2: TestEntry = get_entry_by_action(&action_hash).unwrap().1;

        assert_eq!(e1, entry, "failed to read entry by EntryHash");
        assert_eq!(e2, entry, "failed to read entry by ActionHash");
        assert_eq!(e1, e2, "unexpected different entry at ActionHash vs EntryHash");

        // UPDATE
        let new_entry = TestEntry { field: Some("val".to_string()) };
        let (RevisionMeta {id: updated_action, .. }, updated_entry) = update_entry(&action_hash, new_entry).unwrap();

        assert_ne!(updated_action, action_hash, "update ActionHash did not change");
        assert_ne!(updated_entry, entry_hash, "update EntryHash did not change");

        let u1: TestEntry = get_entry_by_address(&updated_entry).unwrap().1;
        let u2: TestEntry = get_entry_by_action(&updated_action).unwrap().1;

        assert_ne!(u1, entry, "failed to read entry by EntryHash");
        assert_ne!(u2, entry, "failed to read entry by ActionHash");
        assert_eq!(u1, u2, "unexpected different entry at ActionHash vs EntryHash after update");

        let o1: TestEntry = get_entry_by_address(&entry_hash).unwrap().1;
        assert_eq!(o1, entry, "retrieving entry by old hash should return original data");

        // DELETE
        let success = delete_entry::<TestEntry>(&updated_action).unwrap();

        assert!(success, "entry deletion failed");

        let try_retrieve = get_entry_by_address::<TestEntry>(&entry_hash);

        assert!(try_retrieve.is_err(), "entry retrieval after deletion should error");

        let try_retrieve_old = get_entry_by_action::<TestEntry>(&action_hash);
        let try_retrieve_deleted = get_entry_by_action::<TestEntry>(&updated_action);

        assert_eq!(try_retrieve_old.unwrap().1, entry, "archival entry retrieval by action after deletion should return successfully");
        assert!(try_retrieve_deleted.is_err(), "entry retrieval by action after deletion should error");
    }
}
