/**
 * Record-handling abstractions for Holochain apps
 *
 * Allows for data layers which behave like a traditional graph database, where
 * 'records' are the core abstraction, managed by complex arrangements of DHT
 * entries and links.
 *
 * @package HoloREA
 * @since   2019-07-02
 */
use hdk::prelude::*;
use hdk::info::dna_info;

use crate::{
    DnaAddressable,
    RecordAPIResult, DataIntegrityError,
    record_interface::{Identifiable, Identified, Updateable},
    entries::{
        get_entry_by_action,
        create_entry,
        update_entry,
        delete_entry,
    },
    identity_helpers::{
        create_entry_identity,
        infer_local_entry_identity,
        calculate_identity_address,
    },
    metadata_helpers::{
        get_action_hash,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Retrieve the latest available ActionHash for a given EntryHash.
///
/// Useful in coordinating updates between different entry types.
///
/// :TODO: this is a very naive recursive algorithm that basically assumes full network
/// connectivity between everyone at all times, and Updates form a Linked List, rather
/// than a multi-branching tree. This should be updated during other 'conflict resolution' related
/// changes outlined in issue https://github.com/h-REA/hREA/issues/196
///
pub fn get_latest_action_hash(entry_hash: EntryHash) -> RecordAPIResult<ActionHash> {
    match get_details(entry_hash.clone(), GetOptions { strategy: GetStrategy::Latest })? {
        Some(Details::Entry(details)) => match details.entry_dht_status {
            metadata::EntryDhtStatus::Live => match details.updates.len() {
                0 => {
                    // https://docs.rs/hdk/latest/hdk/prelude/struct.EntryDetails.html#structfield.actions
                    Ok(get_action_hash(details.actions.first().unwrap()))
                },
                _ => {
                    // updates exist, find most recent action
                    let mut sortlist = details.updates.to_vec();
                    sortlist.sort_by_key(|update| update.action().timestamp().as_micros());
                    let last = sortlist.last().unwrap();
                    // (unwrap should be safe because these are Update actions)
                    let last_entry_hash = last.action().entry_hash().unwrap().clone();
                    if entry_hash == last_entry_hash {
                      Ok(get_action_hash(last))
                    } else {
                      // recurse
                      get_latest_action_hash(last_entry_hash)
                    }
                },
            },
            _ => Err(DataIntegrityError::EntryNotFound),
        },
        _ => Err(DataIntegrityError::EntryNotFound),
    }
}

/// Retrive the specific version of an entry specified by the given `ActionHash`
///
pub fn read_record_entry_by_action<T, R, B>(
    action_hash: &ActionHash,
) -> RecordAPIResult<(SignedActionHashed, B, T)>
    where T: std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, B>,
{
    let (meta, storage_entry): (_, R) = get_entry_by_action(&action_hash)?;
    Ok((meta, storage_entry.identity()?, storage_entry.entry()))
}

/// Read a record's entry data by its identity index
///
/// :TODO: Currently, the most recent version of the given entry will
///        be provided instead of the exact entry specified.
///        We should also check for multiple live actions, and throw a
///        conflict error if necessary. But core may implement this for
///        us eventually. (@see EntryDhtStatus)
///
pub (crate) fn read_record_entry_by_identity<T, R, B>(
    identity_address: &EntryHash,
) -> RecordAPIResult<(SignedActionHashed, B, T)>
    where T: std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, B>,
{
    // read active links to current version
    let identifier: B = infer_local_entry_identity(identity_address)?;
    // pull details of the current version, to ensure we have the most recent
    let entry_hash: &EntryHash = identifier.as_ref();
    let latest_action_hash = get_latest_action_hash(entry_hash.to_owned())?;

    let (meta, read_entry_hash, entry_data) = read_record_entry_by_action(&latest_action_hash)?;

    Ok((meta, read_entry_hash, entry_data))
}

/// Read a record's entry data by locating it via an anchor `Path` composed
/// of some root component and (uniquely identifying) initial identity address.
///
/// Presumes that the record is to be fetched from the current DNA and naturally errors
/// if attempted on an `EntryHash` that only exists in a foreign cell.
///
pub fn read_record_entry<T, R, B>(
    address: &EntryHash,
) -> RecordAPIResult<(SignedActionHashed, B, T)>
    where T: std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, B>,
{
    let identity_address = calculate_identity_address(&B::new(dna_info()?.hash, address.clone()))?;
    read_record_entry_by_identity::<T, R, B>(&identity_address)
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new record in the DHT, assigns it an identity index (@see identity_helpers.rs)
/// and returns a tuple of this version's `ActionHash`, the identity `EntryHash` and initial record `entry` data.
///
pub fn create_record<T, I, R: Clone, B, C, E, S, F, G>(
    indexing_zome_name_from_config: F,
    entry_def_id: S,
    create_payload: C,
) -> RecordAPIResult<(SignedActionHashed, B, I)>
    where S: AsRef<str> + std::fmt::Display,
        B: DnaAddressable<EntryHash>,
        C: TryInto<I, Error = DataIntegrityError>,
        I: Identifiable<R>,
        WasmError: From<E>,
        Entry: TryFrom<R, Error = E>,
        T: From<R>,
        ScopedEntryDefIndex: for<'a> TryFrom<&'a T, Error = E>,
        EntryVisibility: for<'a> From<&'a T>,
        R: Identified<I, B>,
        F: FnOnce(G) -> Option<String>,
        G: std::fmt::Debug,
        SerializedBytes: TryInto<G, Error = SerializedBytesError>,
{
    // convert the type's CREATE payload into internal storage struct
    let entry_data: I = create_payload.try_into()?;
    // wrap data with null identity for origin record
    let storage = entry_data.with_identity(None);

    // write underlying entry
    let (meta, entry_hash) = create_entry::<T,_,_>(storage)?;

    // create an identifier for the new entry in companion index zome
    // :TODO: move this to a postcommit hook in coordination zome; see #264
    let identity = B::new(dna_info()?.hash, entry_hash.clone());
    create_entry_identity(
        indexing_zome_name_from_config,
        &entry_def_id, &identity,
    )?;

    Ok((meta, identity, entry_data))
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Updates a record in the DHT by its `ActionHash` (revision ID)
///
/// The way in which the input update payload is applied to the existing
/// entry data is up to the implementor of `Updateable<U>` for the entry type.
///
/// :TODO: prevent multiple updates to the same ActionHash under standard operations
///
/// @see hdk_records::record_interface::Updateable
///
pub fn update_record<I, R: Clone, B, U, E>(
    address: &ActionHash,
    update_payload: U,
) -> RecordAPIResult<(SignedActionHashed, B, I, I)>
    where B: DnaAddressable<EntryHash>,
        I: Identifiable<R> + Updateable<U>,
        WasmError: From<E>,
        Entry: TryFrom<R, Error = E>,
        R: Clone + Identified<I, B>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    // get referenced entry for the given action
    let (_meta, previous): (_, R) = get_entry_by_action(address)?;
    let prev_entry = previous.entry();
    let identity = previous.identity()?;
    let identity_hash: &EntryHash = identity.as_ref();

    // apply update payload
    let new_entry = prev_entry.update_with(update_payload)?;
    let storage: R = new_entry.with_identity(Some(identity_hash.clone()));

    // perform regular entry update using internal address
    let (meta, _entry_addr) = update_entry(address, storage)?;

    Ok((meta, identity, new_entry, prev_entry))
}

//-------------------------------[ DELETE ]-------------------------------------

/// Removes a record of the given `ActionHash` from the DHT by marking it as deleted.
///
/// Links are not affected so as to retain a link to the referencing information, which may now need to be updated.
///
pub fn delete_record<T>(address: &ActionHash) -> RecordAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    // :TODO: handle deletion of the identity `Path` for the referenced entry if this is the last action being deleted

    delete_entry::<T>(address)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdk_uuid_types::{ simple_alias, addressable_identifier };
    use crate::{generate_record_entry};

    simple_alias!(EntryId => EntryHash);

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    pub struct Entry {
        field: Option<String>,
    }
    generate_record_entry!(Entry, EntryWithIdentity);

    #[derive(Clone)]
    pub struct CreateRequest {
        field: Option<String>,
    }

    impl From<CreateRequest> for Entry {
        fn from(e: CreateRequest) -> Entry {
            Entry {
                field: e.field.into(),
            }
        }
    }

    #[derive(Clone)]
    pub struct UpdateRequest {
        field: Option<String>,
    }

    impl Updateable<UpdateRequest> for Entry {
        fn update_with(&self, e: UpdateRequest) -> Entry {
            Entry {
                field: e.field.to_owned(),
            }
        }
    }

    fn indexing_zome_name_from_config(_: ()) -> Option<String> {
        Some("".to_string())
    }

    #[test]
    fn test_roundtrip() {
        let entry_type: String = "testing".to_string();

        // CREATE
        let (action_addr, base_address, initial_entry): (_, EntryId, Entry) = create_record(indexing_zome_name_from_config, &entry_type, CreateRequest { field: None }).unwrap();

        // Verify read
        let (action_addr_2, returned_address, first_entry) = read_record_entry::<Entry, EntryWithIdentity, EntryId,_,_>(&base_address).unwrap();
        assert_eq!(action_addr, action_addr_2, "record should have same action ID on read as for creation");
        assert_eq!(base_address.as_ref(), returned_address.as_ref(), "record should have same identifier ID on read as for creation");
        assert_eq!(initial_entry, first_entry, "record from creation output should be same as read data");

        // UPDATE
        let (updated_action_addr, identity_address, updated_entry): (_, EntryId, Entry) = update_record(&action_addr, UpdateRequest { field: Some("value".into()) }).unwrap();

        // Verify update & read
        assert_eq!(base_address.as_ref(), identity_address.as_ref(), "record should have consistent ID over updates");
        assert_ne!(action_addr, updated_action_addr, "record revision should change after update");
        assert_eq!(updated_entry, Entry { field: Some("value".into()) }, "returned record should be changed after update");
        let (action_addr_3, returned_address_3, third_entry) = read_record_entry::<Entry, EntryWithIdentity, EntryId,_,_>(&identity_address).unwrap();
        assert_eq!(base_address.as_ref(), returned_address_3.as_ref(), "record should have consistent ID over updates");
        assert_eq!(action_addr_3, updated_action_addr, "record revision should be same as latest update");
        assert_eq!(third_entry, Entry { field: Some("value".into()) }, "retrieved record should be changed after update");

        // DELETE
        let _ = delete_record::<Entry>(&updated_action_addr);

        // Verify read failure
        let _failure = read_record_entry::<Entry, EntryWithIdentity, EntryId,_,_>(&identity_address).err().unwrap();
    }
}
