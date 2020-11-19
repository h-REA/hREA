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
    record_interface::{Identifiable, Identified, Updateable, },//UniquelyIdentifiable, UpdateableIdentifier },
    entries::{
        get_entry_by_header,
        // get_entry_by_address,
        create_entry,
        update_entry,
        delete_entry,
    },
    ids::{
        create_entry_identity,
        // get_identity_address,
        calculate_identity_address,
    },
    links::{
        get_linked_addresses,
    },
    // anchors::{
    //     create_anchor_index,
    //     get_anchor_index_entry_address,
    //     update_anchor_index,
    //     delete_anchor_index,
    // },
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
pub fn read_record_entry<T, A>(
    identity_address: &A,
) -> GraphAPIResult<(HeaderHash, T)>
    where A: Clone + Into<EntryHash>,
        SerializedBytes: TryInto<T, Error = SerializedBytesError>,
        T: Clone,
{
    // read active links to current version
    let addrs = get_linked_addresses(&(*identity_address).clone().into(), LinkTag::new(crate::identifiers::RECORD_INITIAL_ENTRY_LINK_TAG))?;
    let entry_hash = addrs.first().ok_or(DataIntegrityError::EntryNotFound)?;

    // pull details of the current version, to ensure we have the most recent
    let latest_header_hash = (match get_details((*entry_hash).clone(), GetOptions)? {
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

    let out_header_hash = latest_header_hash.clone();

    Ok((out_header_hash, get_entry_by_header(&latest_header_hash)?))
}

/// Read a record's entry data by locating it via an anchor `Path` composed
/// of some root component and (uniquely identifying) initial identity address.
///
pub fn locate_record_entry<T, A, S>(
    entry_type_root_path: &S,
    address: &A,
) -> GraphAPIResult<(HeaderHash, T)>
    where S: Clone + Into<String>,
        A: Clone + Into<EntryHash>,
        SerializedBytes: TryInto<T, Error = SerializedBytesError>,
        T: Clone,
{
    let identity_address = calculate_identity_address(entry_type_root_path, address)?;
    read_record_entry(&identity_address)
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

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a new record in the DHT, assigns it an identity index (@see identity_helpers.rs)
/// and returns a tuple of this version's `HeaderHash`, the identity `EntryHash` and initial record `entry` data.
///
pub fn create_record<'a, R: 'a, E: 'a, C, A, S: Clone + Into<String>>(
    entry_type: &S,
    create_payload: &C,
) -> GraphAPIResult<(HeaderHash, A, E)>
    where A: From<EntryHash>,
        C: Clone + Into<E>,
        E: Clone + Identifiable,
        R: Clone + Identified<E> + AsRef<&'a R> + From<<E as Identifiable>::StorageType>,
        EntryDefId: From<&'a R>,
        SerializedBytes: TryFrom<&'a R, Error = SerializedBytesError>,
{
    // convert the type's CREATE payload into internal storage struct
    let entry_data: E = (*create_payload).clone().into();
    // wrap data with null identity for origin record
    let storage: R = entry_data.with_identity(None).into();

    // write underlying entry
    let (header_hash, entry_hash) = create_entry(storage.as_ref())?;

    // create an identifier for the new entry
    let base_address = create_entry_identity(entry_type, &entry_hash)?;

    // link the identifier to the actual entry
    create_link(base_address.clone(), entry_hash, LinkTag::new(crate::identifiers::RECORD_INITIAL_ENTRY_LINK_TAG))?;

    Ok((header_hash, A::from(base_address), entry_data))
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
pub fn update_record<'a, E: 'a, U, R: 'a, A>(
    address: &'a A,
    update_payload: &U,
) -> GraphAPIResult<(HeaderHash, EntryHash, E)>
    where A: Clone + Into<HeaderHash>,
        E: Clone + Identifiable + Updateable<U>,
        EntryDefId: From<&'a R>,
        SerializedBytes: TryFrom<&'a R, Error = SerializedBytesError>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        R: Clone + Identified<E> + AsRef<&'a R> + From<<E as Identifiable>::StorageType>,
{
    // get referenced entry for the given header
    let previous: R = get_entry_by_header(address)?;
    let prev_entry = previous.entry();

    // apply update payload
    let new_entry = prev_entry.update_with(update_payload);
    let storage: R = new_entry.with_identity(previous.identity().as_ref().ok()).into();

    // perform regular entry update using internal address
    let (header_addr, entry_addr) = update_entry(address, storage.as_ref())?;

    Ok((header_addr, entry_addr, new_entry))
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
pub fn delete_record<'a, T: 'a, A>(address: &'a A) -> GraphAPIResult<bool>
    where A: Clone + Into<HeaderHash>,
        SerializedBytes: TryInto<T, Error = SerializedBytesError>,
        T: Clone,
{
    // :TODO: handle deletion of the identity `Path` for the referenced entry if this is the last header being deleted

    delete_entry::<T, A>(address)?;
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
    use crate::bind_identity;

    #[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
    pub struct Entry {
        field: Option<String>,
    }
    bind_identity!(Entry: id="test");

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

    #[test]
    fn test_creation() {
        let (header_addr, base_address, entry_resp): (_, _, Entry) = create_record::<EntryWithIdentity,_,_,_,_>("testing".to_string().as_ref(), &CreateRequest { field: None }).unwrap();
    }
}
