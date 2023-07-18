/**
 * Helpers for managing records which are associated with manually assigned
 * string-based identifiers, similar to UNIQUE keys in relational databases.
 *
 * :TODO: this code is pretty rough around the edges, needs a major review
 * and thought given to efficiency. Probably a lot of duplicated logic that
 * could be cleaned up too.
 *
 * @package hREA
 * @since   2021-09-15
 */
use hdk::prelude::*;
use hdk_uuid_types::{
    DnaAddressable, DnaIdentifiable,
};

use crate::{
    RecordAPIResult, DataIntegrityError, SemanticIndexError,
    record_interface::{
        Identified, Identifiable, UniquelyIdentifiable,
        Updateable, UpdateableIdentifier,
    },
    identity_helpers::calculate_identity_address,
    records::{
        create_record,
        read_record_entry_by_identity,
    },
    entries::{
        get_entry_by_action,
        update_entry,
        delete_entry,
    },
};

//--------------------------------[ READ ]--------------------------------------

/// Calculate the identity path for a String-based ID
///
/// :TODO: :DUPE: could be genericised to fit `crate::identity_helpers::identity_path_for` signature?
///
fn identity_path_for<A>(
    base_address: A,
) -> RecordAPIResult<Path>
    where A: AsRef<str>,
{
    Ok(Path::from(base_address.as_ref()))
}

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
fn calculate_anchor_address<I>(
    base_address: I,
) -> RecordAPIResult<EntryHash>
    where I: AsRef<str>,
{
    Ok(identity_path_for(base_address)?.path_entry_hash()?)
}

/// Given an identity `EntryHash` (ie. the result of `create_entry_identity`),
/// query the underlying string identifier used to uniquely identify it.
///
fn read_entry_anchor_id(
    link_type: impl LinkTypeFilterExt,
    identity_path_address: &EntryHash,
) -> RecordAPIResult<String> {
    get_links(
        identity_path_address.to_owned(),
        link_type,
        Some(LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))
    )?
    .first()
    .map(|link| {
        let bytes = &link.tag.to_owned().into_inner()[3..];
        Ok(String::from_utf8(bytes.to_vec())
            .map_err(|e| SemanticIndexError::from(e) )?)
    })
    .ok_or(SemanticIndexError::IndexNotFound((*identity_path_address).clone()))?
}

/// Given the `EntryHash` of an anchor `Path`, query the identity of the associated entry
///
fn read_anchor_identity(
    link_type: impl LinkTypeFilterExt,
    anchor_path_address: &EntryHash,
) -> RecordAPIResult<EntryHash> {
    get_links(
        anchor_path_address.to_owned(),
        link_type,
        Some(LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))
    )?
    .first()
    .map(|l| Ok(l.target.to_owned().into()))
    .ok_or(SemanticIndexError::IndexNotFound((*anchor_path_address).clone()))?
}

/// Reads an entry via its `anchor index`.
///
/// Follows an anchor identified by `id_entry_type`, `id_link_type` and
/// its well-known `id_string` to retrieve whichever entry of type `T` resides
/// at the anchored address.
///
/// @see anchor_helpers.rs
///
pub fn read_anchored_record_entry<LT, T, R, B, A, I>(
    link_type: LT,
    id_string: I,
) -> RecordAPIResult<(SignedActionHashed, A, T)>
    where LT: LinkTypeFilterExt,
        I: AsRef<str>,
        T: std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        A: DnaIdentifiable<String>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, B>,
{
    let anchor_address = calculate_anchor_address(&id_string)?;
    let identity_address = read_anchor_identity(link_type, &anchor_address)?;
    let (meta, _entry_addr, entry_data) = read_record_entry_by_identity::<T, R, B>(&identity_address)?;
    Ok((meta, A::new(dna_info()?.hash, id_string.as_ref().to_string()), entry_data))
}

/// Creates a new record in the DHT and assigns it a manually specified `anchor index`
/// that can be used like a primary key. The `create_payload` must also implement
/// `UniquelyIdentifiable` in order to derive the unique `anchor index` value.
///
/// It is recommended that you include a creation timestamp in newly created records, to avoid
/// them conflicting with previously entered entries that may be of the same content.
///
pub fn create_anchored_record<LT, I, B, A, C, R, T, E, S, F, G>(
    link_type: LT,
    indexing_zome_name_from_config: F,
    entry_def_id: &S,
    create_payload: C,
) -> RecordAPIResult<(SignedActionHashed, A, I)>
    where S: AsRef<str> + std::fmt::Display,
        B: DnaAddressable<EntryHash>,
        A: DnaIdentifiable<String>,
        C: TryInto<I, Error = DataIntegrityError> + UniquelyIdentifiable,
        I: Identifiable<R>,
        WasmError: From<E>,
        Entry: TryFrom<R, Error = E> + TryFrom<T, Error = E>,
        R: Clone + Identified<I, B>,
        T: From<R>,
        F: FnOnce(G) -> Option<String>,
        G: std::fmt::Debug,
        SerializedBytes: TryInto<G, Error = SerializedBytesError>,
        ScopedEntryDefIndex: for<'a> TryFrom<&'a T, Error = E>,
        EntryVisibility: for<'a> From<&'a T>,
        LT: Clone + LinkTypeFilterExt,
        ScopedLinkType: TryFrom<LT, Error = E>,
{
    // determine unique anchor index key
    // :TODO: deal with collisions
    let entry_id = create_payload.get_anchor_key()?;

    // write base record and identity index path
    let (meta, entry_internal_id, entry_data) = create_record::<T, I, R,_,_,_,_,_,_>(
        indexing_zome_name_from_config,
        &entry_def_id, create_payload,
    )?;

    // link the hash identifier to a new manually assigned identifier so we can determine the anchor when reading & updating
    let identifier_hash = calculate_identity_address(&entry_internal_id)?;
    link_identities(link_type, &identifier_hash, &entry_id)?;

    Ok((meta, A::new(dna_info()?.hash, entry_id), entry_data))
}

/// Updates a record via references to its `anchor index`.
///
/// The `update_payload` must contain all data necessary to determine both the existing
/// `anchor index` ID of the record, and the new `anchor index` that it has been moved to (if any).
///
/// @see hdk_records::record_interface::UpdateableIdentifier
///
pub fn update_anchored_record<LT, I, R, A, B, U, E>(
    link_type: LT,
    revision_id: &ActionHash,
    update_payload: U,
) -> RecordAPIResult<(SignedActionHashed, B, I, I)>
    where A: DnaAddressable<EntryHash>,
        B: DnaIdentifiable<String>,
        I: std::fmt::Debug + Identifiable<R> + Updateable<U>,
        U: UpdateableIdentifier,
        WasmError: From<E>,
        Entry: TryFrom<R, Error = E>,
        R: Clone + std::fmt::Debug + Identified<I, A>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        LT: Clone + LinkTypeFilterExt,
        ScopedLinkType: TryFrom<LT, Error = E>,
{
    // get referenced entry and identifiers for the given action
    let (_meta, previous): (_, R) = get_entry_by_action(revision_id)?;

    let prev_entry = previous.entry();
    let identity = previous.identity()?;

    let identity_hash = calculate_identity_address(&identity)?;
    let maybe_current_id = read_entry_anchor_id([link_type.to_owned()], &identity_hash);

    // ensure the referenced entry exists and has an anchored identifier path
    match maybe_current_id {
        Ok(current_id) => {
            let maybe_new_id = update_payload.get_new_anchor_key();
            let mut final_id = current_id.clone();

            // apply update payload
            let new_entry = prev_entry.update_with(update_payload)?;
            let storage: R = new_entry.with_identity(Some(identity_hash.clone()));

            // perform regular entry update using internal address
            let (meta, _new_entry_addr) = update_entry(revision_id, storage)?;

            // check if ID has changed
            match maybe_new_id {
                Some(new_id) => {
                    if new_id != final_id {
                        // clear any old identity path, ensuring the link structure is as expected
                        let mut addrs = get_links(
                            identity_hash.to_owned(),
                            [link_type.to_owned()],
                            Some(LinkTag::new(crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG))
                        )?;
                        if addrs.len() != 1 {
                            return Err(DataIntegrityError::IndexNotFound(identity_hash.to_owned()));
                        }
                        let old_link = addrs.pop().map(|l| l.create_link_hash.to_owned().into()).unwrap();

                        delete_link(old_link)?;

                        // create the new identifier and link to it
                        link_identities(link_type, &identity_hash, &new_id)?;

                        // reference final ID in record updates to new identifier path
                        final_id = new_id.into();
                    }
                },
                None => (),
            }

            // return updated record details to caller
            Ok((meta, DnaIdentifiable::new(dna_info()?.hash, final_id), new_entry, prev_entry))
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
pub fn delete_anchored_record<T>(address: &ActionHash) -> RecordAPIResult<bool>
    where SerializedBytes: TryInto<T, Error = SerializedBytesError>,
{
    delete_entry::<T>(address)?;
    Ok(true)
}

//--------------------------[ UTILITIES  / INTERNALS ]---------------------

/// Writes a bidirectional set of anchoring entries for a record so that the string-based identifier
/// can be looked up from the content-addressable `EntryHash`-based identifier
///
fn link_identities<LT, A, E>(
    link_type: LT,
    identifier_hash: &EntryHash,
    id_string: A,
) -> RecordAPIResult<()>
    where A: Clone + AsRef<str>,
        LT: Clone + LinkTypeFilterExt,
        ScopedLinkType: TryFrom<LT, Error = E>,
        WasmError: From<E>,
{
    // create manually assigned identifier
    let path = identity_path_for(&id_string)?;
    let identifier_tag = create_id_tag(id_string.to_owned());

    link_if_not_linked::<LT, E>(identifier_hash.clone(), path.path_entry_hash()?, link_type.to_owned(), identifier_tag.to_owned())?;
    link_if_not_linked::<LT, E>(path.path_entry_hash()?, identifier_hash.clone(), link_type, identifier_tag)?;

    Ok(())
}

/// Generate a link tag for the identity anchor of a record by encoding the ID string into the tag
/// so that it can be retreived by querying the DHT later.
///
fn create_id_tag<S>(id_str: S) -> LinkTag
    where S: AsRef<str>,
{
    LinkTag::new([crate::identifiers::RECORD_IDENTITY_ANCHOR_LINK_TAG, id_str.as_ref().as_bytes()].concat())
}

// :DUPE: link_if_not_linked but unlike others, this one is generic over `link_type`.
fn link_if_not_linked<LT, E>(
    origin_hash: EntryHash,
    dest_hash: EntryHash,
    link_type: LT,
    link_tag: LinkTag,
) -> RecordAPIResult<Option<ActionHash>>
    where LT: Clone + LinkTypeFilterExt,
        ScopedLinkType: TryFrom<LT, Error = E>,
        WasmError: From<E>,
{
    if false == get_links(origin_hash.to_owned(), link_type.to_owned(), Some(link_tag.to_owned()))?
        .iter().any(|l| { EntryHash::from(l.target.to_owned()) == dest_hash })
    {
        Ok(Some(create_link(
            origin_hash.to_owned(),
            dest_hash.to_owned(),
            link_type,
            link_tag,
        )?))
    } else {
        Ok(None)
    }
}
