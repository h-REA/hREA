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

use std::convert::TryFrom;
use hdk3::prelude::*;

use crate::{
    GraphAPIResult, DataIntegrityError,
    record_interface::{Identifiable, Identified, Updateable},
    entries::{
        get_entry_by_header,
        create_entry,
        update_entry,
        delete_entry,
    },
    identity_helpers::{
        create_entry_identity,
        read_entry_identity,
        calculate_identity_address,
    },
};

/// Helper to retrieve the HeaderHash for an Element
///
fn get_header_hash(shh: element::SignedHeaderHashed) -> HeaderHash {
    shh.header_hashed().as_hash().to_owned()
}

//--------------------------------[ READ ]--------------------------------------

/// Read a record's entry data by its identity index
///
/// :TODO: Currently, the most recent version of the given entry will
///        be provided instead of the exact entry specified.
///        We should also check for multiple live headers, and throw a
///        conflict error if necessary. But core may implement this for
///        us eventually. (@see EntryDhtStatus)
///
pub (crate) fn read_record_entry_by_identity<T, R, O>(
    identity_address: &EntryHash,
) -> GraphAPIResult<(HeaderHash, O, T)>
    where O: From<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Identified<T>,
{
    // read active links to current version
    let entry_hash = read_entry_identity(identity_address)?;

    // pull details of the current version, to ensure we have the most recent
    let latest_header_hash = (match get_details(entry_hash, GetOptions)? {
        Some(Details::Entry(details)) => match details.entry_dht_status {
            metadata::EntryDhtStatus::Live => match details.updates.len() {
                0 => {
                    // no updates yet, latest header hash is the first one
                    Ok(get_header_hash(details.headers.first().unwrap().to_owned()))
                },
                _ => {
                    // updates exist, find most recent header
                    let mut sortlist = details.updates.to_vec();
                    sortlist.sort_by_key(|update| update.header().timestamp().0);
                    let last = sortlist.last().unwrap().to_owned();
                    Ok(get_header_hash(last))
                },
            },
            _ => Err(DataIntegrityError::EntryNotFound),
        },
        _ => Err(DataIntegrityError::EntryNotFound),
    })?;

    let out_header_hash = latest_header_hash.to_owned();

    let storage_entry: R = get_entry_by_header(&latest_header_hash)?;

    Ok((out_header_hash, storage_entry.identity()?.into(), storage_entry.entry()))
}

/// Read a record's entry data by locating it via an anchor `Path` composed
/// of some root component and (uniquely identifying) initial identity address.
///
pub fn read_record_entry<T, R, O, A, S>(
    entry_type_root_path: &S,
    address: &A,
) -> GraphAPIResult<(HeaderHash, O, T)>
    where S: AsRef<str>,
        A: AsRef<EntryHash>,
        O: From<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Identified<T>,
{
    let identity_address = calculate_identity_address(entry_type_root_path, address.as_ref())?;
    read_record_entry_by_identity::<T, R, O>(&identity_address)
}

/// Reads an entry via its `anchor index`.
///
/// Follows an anchor identified by `id_entry_type`, `id_link_type` and
/// its well-known `id_string` to retrieve whichever entry of type `T` resides
/// at the anchored address.
///
/// @see anchor_helpers.rs
///
pub fn read_anchored_record_entry<T, E>(
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
                _ => Err(DataIntegrityError::EntryNotFound),
            }
        },
        None => Err(DataIntegrityError::EntryNotFound),
    }
}*/

/// Fetches all referenced record entries found corresponding to the input
/// identity addresses.
///
/// Useful in loading the results of indexed data, where indexes link identity `Path`s for different records.
///
pub (crate) fn get_records_by_identity_address<'a, T, R, A>(addresses: &'a Vec<EntryHash>) -> Vec<GraphAPIResult<(HeaderHash, A, T)>>
    where A: From<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Identified<T>,
{
    addresses.iter()
        .map(read_record_entry_by_identity)
        .collect()
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new record in the DHT, assigns it an identity index (@see identity_helpers.rs)
/// and returns a tuple of this version's `HeaderHash`, the identity `EntryHash` and initial record `entry` data.
///
pub fn create_record<'a, E: 'a, R: 'a, C, A, S: AsRef<str>>(
    entry_type: S,
    create_payload: C,
) -> GraphAPIResult<(HeaderHash, A, E)>
    where A: From<EntryHash>,
        C: Into<E>,
        E: Identifiable<R>,
        R: Identified<E>,
        EntryDefId: From<&'a R>, SerializedBytes: TryFrom<&'a R, Error = SerializedBytesError>,
{
    // convert the type's CREATE payload into internal storage struct
    let entry_data: E = create_payload.into();
    // wrap data with null identity for origin record
    let storage = entry_data.with_identity(None);

    // write underlying entry
    let (header_hash, entry_hash) = create_entry(&storage)?;

    // create an identifier for the new entry
    let base_address = create_entry_identity(entry_type, &entry_hash)?;

    // link the identifier to the actual entry
    create_link(base_address, entry_hash.clone(), LinkTag::new(crate::identifiers::RECORD_INITIAL_ENTRY_LINK_TAG))?;

    Ok((header_hash, entry_hash.into(), entry_data))
}

/// Creates a new record in the DHT and assigns it a manually specified `anchor index`
/// that can be used like a primary key. The `create_payload` must also implement
/// `UniquelyIdentifiable` in order to derive the unique `anchor index` value.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_anchored_record<E, C, S>(
    base_entry_type: S,
    id_link_type: &str,
    entry_type: S,
    create_payload: C,
) -> ZomeApiResult<(String, E)>
    where E: Clone + Into<AppEntryValue>,
        C: Into<E> + UniquelyIdentifiable,
        S: Into<AppEntryType>,
{
    // determine unique anchor index key
    // :TODO: deal with collisions
    let entry_id = create_payload.get_anchor_key();

    // write underlying entry
    let (address, entry_resp) = create_entry(entry_type, create_payload)?;

    // write primary key index
    let _ = create_anchor_index(&base_entry_type.into(), id_link_type, &entry_id, &address)?;

    Ok((entry_id, entry_resp))
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Updates a record in the DHT by its `HeaderHash` (revision ID)
///
/// The way in which the input update payload is applied to the existing
/// entry data is up to the implementor of `Updateable<U>` for the entry type.
///
/// :TODO: prevent multiple updates to the same HeaderHash under standard operations
///
/// @see hdk_graph_helpers::record_interface::Updateable
///
pub fn update_record<'a, E: 'a, R: 'a, U, I>(
    address: &'a HeaderHash,
    update_payload: U,
) -> GraphAPIResult<(HeaderHash, I, E)>
    where I: From<EntryHash>,
        E: Identifiable<R> + Updateable<U>,
        R: Identified<E>,
        EntryDefId: From<&'a R>,
        SerializedBytes: TryFrom<&'a R, Error = SerializedBytesError>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    // get referenced entry for the given header
    let previous: R = get_entry_by_header(address)?;
    let prev_entry = previous.entry();
    let identity_hash = previous.identity()?;

    // apply update payload
    let new_entry = prev_entry.update_with(update_payload);
    let storage: R = new_entry.with_identity(Some(identity_hash.clone()));

    // perform regular entry update using internal address
    let (header_addr, _entry_addr) = update_entry(address, &storage)?;

    Ok((header_addr, identity_hash.into(), new_entry))
}

/// Updates a record via references to its `anchor index`.
///
/// The `update_payload` must contain all data necessary to determine both the existing
/// `anchor index` ID of the record, and the new `anchor index` that it has been moved to (if any).
///
/// @see hdk_graph_helpers::record_interface::UpdateableIdentifier
///
pub fn update_anchored_record<E, U, S>(
    id_entry_type: &str,
    id_link_type: &str,
    entry_type: S,
    update_payload: &U,
) -> ZomeApiResult<(String, E)>
    where E: Clone + TryFrom<AppEntryValue> + Into<AppEntryValue> + Updateable<U>,
        S: Into<AppEntryType> + Clone,
        U: UpdateableIdentifier,
{
    let current_id = update_payload.get_anchor_key();
    let maybe_new_id = update_payload.get_new_anchor_key();

    // determine entry address
    let entry_address = get_anchor_index_entry_address(&id_entry_type.to_string(), id_link_type, &current_id)?;
    match entry_address {
        Some(entry_addr) => {
            let mut final_id = current_id.clone();

            // check if ID has changed
            match maybe_new_id {
                Some(new_id) => {
                    // update the anchor index
                    let _anchor_updated = update_anchor_index(&id_entry_type.to_string(), id_link_type, &entry_addr, &current_id, &new_id)?;
                    final_id = new_id.into();
                },
                None => (),
            }

            // perform update of actual entry object
            let (_new_addr, new_entry) = update_entry(entry_type, &Addressable::from(entry_addr), update_payload)?;

            // return updated record to caller
            Ok((final_id, new_entry))
        },
        None => Err(ZomeApiError::Internal(ERR_MSG_ENTRY_NOT_FOUND.into())),
    }
}

//-------------------------------[ DELETE ]-------------------------------------

/// Removes a record of the given `HeaderHash` from the DHT by marking it as deleted.
///
/// Links are not affected so as to retain a link to the referencing information, which may now need to be updated.
///
pub fn delete_record<T>(address: &HeaderHash) -> GraphAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    // :TODO: handle deletion of the identity `Path` for the referenced entry if this is the last header being deleted

    delete_entry::<T>(address)?;
    Ok(true)
}

/// Removes a record via references to its `anchor index`.
///
/// The index as well as the record's entry data will both be deleted; any failures
/// are considered an error.
///
pub fn delete_anchored_record<E>(
    id_entry_type: &str,
    id_link_type: &str,
    entry_id: &String,
) -> ZomeApiResult<bool>
    where E: TryFrom<AppEntryValue>,
{
    // determine entry address
    let entry_address = get_anchor_index_entry_address(&id_entry_type.to_string(), id_link_type, entry_id)?;

    match entry_address {
        Some(entry_addr) => {
            // wipe the anchor index
            let _anchor_deleted = delete_anchor_index(&id_entry_type.to_string(), id_link_type, entry_id)?;

            // remove the entry
            // :NOTE: this is done second as links pointing to entry must be cleared before the entry itself
            let entry_result = delete_entry::<E>(&entry_addr);

            entry_result
        },
        None => Err(ZomeApiError::Internal(ERR_MSG_ENTRY_NOT_FOUND.into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bind_identity, simple_alias};

    simple_alias!(EntryId => EntryHash);

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    pub struct Entry {
        field: Option<String>,
    }
    bind_identity!(Entry);
    entry_def!(EntryWithIdentity EntryDef {
        id: "test_entry".into(),
        visibility: EntryVisibility::Public,
        crdt_type: CrdtType,
        required_validations: 1.into(),
        required_validation_type: RequiredValidationType::default(),
    });

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

    #[test]
    fn test_roundtrip() {
        let entry_type: String = "testing".to_string();

        // CREATE
        let (header_addr, base_address, initial_entry): (_, EntryId, Entry) = create_record(&entry_type, CreateRequest { field: None }).unwrap();

        // Verify read
        let (header_addr_2, returned_address, first_entry) = read_record_entry::<Entry, EntryWithIdentity, EntryId,_,_>(&entry_type, &base_address).unwrap();
        assert_eq!(header_addr, header_addr_2, "record should have same header ID on read as for creation");
        assert_eq!(base_address.as_ref(), returned_address.as_ref(), "record should have same identifier ID on read as for creation");
        assert_eq!(initial_entry, first_entry, "record from creation output should be same as read data");

        // UPDATE
        let (updated_header_addr, identity_address, updated_entry): (_, EntryId, Entry) = update_record(&header_addr, UpdateRequest { field: Some("value".into()) }).unwrap();

        // Verify update & read
        assert_eq!(base_address.as_ref(), identity_address.as_ref(), "record should have consistent ID over updates");
        assert_ne!(header_addr, updated_header_addr, "record revision should change after update");
        assert_eq!(updated_entry, Entry { field: Some("value".into()) }, "returned record should be changed after update");
        let (header_addr_3, returned_address_3, third_entry) = read_record_entry::<Entry, EntryWithIdentity, EntryId,_,_>(&entry_type, &identity_address).unwrap();
        assert_eq!(base_address.as_ref(), returned_address_3.as_ref(), "record should have consistent ID over updates");
        assert_eq!(header_addr_3, updated_header_addr, "record revision should be same as latest update");
        assert_eq!(third_entry, Entry { field: Some("value".into()) }, "retrieved record should be changed after update");

        // DELETE
        delete_record::<Entry>(&updated_header_addr);

        // Verify read failure
        let _failure = read_record_entry::<Entry, EntryWithIdentity, EntryId,_,_>(&entry_type, &identity_address).err().unwrap();
    }
}
