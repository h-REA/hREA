/**
 * Helpers for managing records which are associated with manually assigned
 * string-based identifiers, similar to UNIQUE keys in relational databases.
 *
 * :TODO: this code is pretty rough around the edges, needs a major review
 * and thought given to efficiency. Probably a lot of duplicated logic that
 * could be cleaned up too.
 *
 * @package Holo-REA
 * @since   2021-09-15
 */
use hdk::prelude::*;
use hdk_type_serialization_macros::{
    RevisionHash,
    DnaAddressable, DnaIdentifiable,
};

use crate::{
    RecordAPIResult, DataIntegrityError,
    record_interface::{
        Identified, Identifiable, UniquelyIdentifiable,
        Updateable, UpdateableIdentifier,
    },
    link_helpers::get_linked_addresses,
    identity_helpers::calculate_identity_address,
    records::{
        create_record,
        read_record_entry_by_identity,
        // read_record_entry_by_header,
        get_latest_header_hash,
    },
    entries::{
        try_entry_from_element,
        try_decode_entry,
        get_entry_by_header,
        update_entry,
        delete_entry,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Calculate the identity path for a String-based ID
///
/// :TODO: :DUPE: could be genericised to fit `crate::identity_helpers::identity_path_for` signature?
///
fn identity_path_for<A, S>(
    entry_type_root_path: S,
    base_address: A,
) -> temp_path::path::Path
    where S: AsRef<str>,
        A: AsRef<str>,
{
    let type_root = entry_type_root_path.as_ref().as_bytes().to_vec();
    let string_id = base_address.as_ref().as_bytes().to_vec();

    temp_path::path::Path::from(vec![type_root.into(), string_id.into()])
}

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
fn calculate_anchor_address<I, S>(
    entry_type_root_path: S,
    base_address: I,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        I: AsRef<str>,
{
    Ok(identity_path_for(entry_type_root_path, base_address).hash()?)
}



/// Given an identity `EntryHash` (ie. the result of `create_entry_identity`),
/// query the underlying string identifier used to uniquely identify it.
///
fn read_entry_anchor_id(
    identity_path_address: &EntryHash,
) -> RecordAPIResult<String>
{
    let mut addrs = get_linked_addresses(identity_path_address, LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))?;
    let entry_hash = addrs.pop().ok_or(DataIntegrityError::IndexNotFound((*identity_path_address).clone()))?;

    let path_element = get(entry_hash, GetOptions::default())?;
    let entry = try_entry_from_element(path_element.as_ref())?;
    let path: temp_path::path::Path = try_decode_entry(entry.to_owned())?;
    let components: &Vec<temp_path::path::Component> = path.as_ref();
    let last_component = components.last().unwrap();

    Ok(last_component.try_into()?)
}

/// Given the `EntryHash` of an anchor `Path`, query the identity of the associated entry
///
fn read_anchor_identity(
    anchor_path_address: &EntryHash,
) -> RecordAPIResult<EntryHash>
{
    let mut addrs = get_linked_addresses(anchor_path_address, LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))?;
    Ok(addrs.pop().ok_or(DataIntegrityError::IndexNotFound((*anchor_path_address).clone()))?)
}

/// Reads an entry via its `anchor index`.
///
/// Follows an anchor identified by `id_entry_type`, `id_link_type` and
/// its well-known `id_string` to retrieve whichever entry of type `T` resides
/// at the anchored address.
///
/// @see anchor_helpers.rs
///
pub fn read_anchored_record_entry<T, R, B, A, S, I>(
    entry_type_root_path: &S,
    id_string: I,
) -> RecordAPIResult<(RevisionHash, A, T)>
    where S: AsRef<str>,
        I: AsRef<str>,
        T: std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        A: DnaIdentifiable<String>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, B>,
{
    let anchor_address = calculate_anchor_address(entry_type_root_path, &id_string)?;
    let identity_address = read_anchor_identity(&anchor_address)?;
    let (revision_id, _entry_addr, entry_data) = read_record_entry_by_identity::<T, R, B>(&identity_address)?;
    Ok((revision_id, A::new(dna_info()?.hash, id_string.as_ref().to_string()), entry_data))
}

/// Creates a new record in the DHT and assigns it a manually specified `anchor index`
/// that can be used like a primary key. The `create_payload` must also implement
/// `UniquelyIdentifiable` in order to derive the unique `anchor index` value.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_anchored_record<I, B, A, C, R, E, S>(
    entry_def_id: &S,
    create_payload: C,
) -> RecordAPIResult<(RevisionHash, A, I)>
    where S: AsRef<str>,
        B: DnaAddressable<EntryHash>,
        A: DnaIdentifiable<String>,
        C: Into<I> + UniquelyIdentifiable,
        I: Identifiable<R>,
        WasmError: From<E>,
        Entry: TryFrom<R, Error = E>,
        R: Clone + Identified<I, B>,
{
    // determine unique anchor index key
    // :TODO: deal with collisions
    let entry_id = create_payload.get_anchor_key()?;

    // write base record and identity index path
    let (revision_id, entry_internal_id, entry_data) = create_record::<I, R, _,_,_,_>(&entry_def_id, create_payload)?;

    // create manually assigned identifier
    let path = identity_path_for(&entry_def_id, &entry_id);
    path.ensure()?;

    // link the hash identifier to the manually assigned identifier so we can determine it when reading & updating
    let identifier_hash = calculate_identity_address(entry_def_id, &entry_internal_id)?;
    create_link(identifier_hash.clone(), path.hash()?, LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))?;
    create_link(path.hash()?, identifier_hash.clone(), LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))?;

    Ok((revision_id, A::new(dna_info()?.hash, entry_id), entry_data))
}

/// Updates a record via references to its `anchor index`.
///
/// The `update_payload` must contain all data necessary to determine both the existing
/// `anchor index` ID of the record, and the new `anchor index` that it has been moved to (if any).
///
/// @see hdk_records::record_interface::UpdateableIdentifier
///
pub fn update_anchored_record<I, R: Clone, A, B, U, E, S>(
    entry_def_id: &S,
    revision_id: &RevisionHash,
    update_payload: U,
) -> RecordAPIResult<(RevisionHash, B, I, I)>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaIdentifiable<String>,
        I: std::fmt::Debug + Identifiable<R> + Updateable<U>,
        U: UpdateableIdentifier,
        WasmError: From<E>,
        Entry: TryFrom<R, Error = E>,
        R: Clone + std::fmt::Debug + Identified<I, A>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
{
    // get referenced entry and identifiers for the given header
    let previous: R = get_entry_by_header(revision_id)?;
    let prev_entry = previous.entry();
    let identity = previous.identity()?;
    let identity_hash: &EntryHash = identity.as_ref();
    let maybe_current_id = read_entry_anchor_id(identity_hash);

    // ensure the referenced entry exists and has an anchored identifier path
    match maybe_current_id {
        Ok(current_id) => {
            let maybe_new_id = update_payload.get_new_anchor_key();
            let mut final_id = current_id.clone();

            // apply update payload
            let new_entry = prev_entry.update_with(update_payload);
            let storage: R = new_entry.with_identity(Some(identity_hash.clone()));

            // perform regular entry update using internal address
            let (header_addr, _new_entry_addr) = update_entry(&entry_def_id, revision_id, storage)?;

            // check if ID has changed
            match maybe_new_id {
                Some(new_id) => {
                    if new_id != final_id {
                        // clear any old identity path, ensuring the link structure is as expected
                        let mut addrs = get_linked_addresses(identity_hash, LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))?;
                        if addrs.len() != 1 {
                            return Err(DataIntegrityError::IndexNotFound(identity_hash.to_owned()));
                        }
                        let old_link = addrs.pop().unwrap();
                        let old_link_id = get_latest_header_hash(old_link)?;
                        let old_link_hash: &HeaderHash = old_link_id.as_ref();
                        delete_link(old_link_hash.to_owned())?;

                        // create the new identifier and link to it
                        let path = identity_path_for(&entry_def_id, &new_id);
                        path.ensure()?;
                        create_link(identity_hash.to_owned(), path.hash()?, LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))?;

                        // reference final ID in record updates to new identifier path
                        final_id = new_id.into();
                    }
                },
                None => (),
            }

            // return updated record details to caller
            Ok((header_addr, DnaIdentifiable::new(dna_info()?.hash, final_id), new_entry, prev_entry))
        },
        Err(_e) => Err(DataIntegrityError::EntryNotFound),
    }
}

/// Removes a record via references to its `anchor index`.
///
/// The index as well as the record's entry data will both be deleted; any failures
/// are considered an error.
///
/// :TODO: This is a stub- include any logic necessary to handle cleanup of associated links.
///        Not clearing old anchors may cause issues upon subsequent reinsert, which is not yet tested.
///
pub fn delete_anchored_record<T, A>(address: &A) -> RecordAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
        A: AsRef<HeaderHash>,
{
    delete_entry::<T, A>(address)?;
    Ok(true)
}
