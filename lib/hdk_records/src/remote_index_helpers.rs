/**
 * Helpers relating to `remote indexes`.
 *
 * A `remote index` is similar to a `local index`, except that it is composed of
 * two indexes which service queries on either side of the network boundary.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use std::convert::TryFrom;
use std::fmt::Debug;
use hdk::prelude::*;
use hdk_type_serialization_macros::{extern_id_to_bytes, bytes_to_extern_id};

use crate::{
    DnaAddressable,
    CrossCellError, OtherCellResult, RecordAPIResult,
    internals::*,
    identity_helpers::create_entry_identity,
    rpc_helpers::call_zome_method
};

/// Common request format (zome trait) for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteEntryLinkRequest<A, B>
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    pub remote_entry: A,
    pub target_entries: Vec<B>,
    pub removed_entries: Vec<B>,
}

// Custom serialization logic for genericising `RemoteEntryLinkRequest` data at WASM boundaries
//
// :TODO: revisit the necessity of this layer.
// It was mostly added because of difficulty in assigning trait bounds for serde::Serialize & Deserialize
// on the template types of `RemoteEntryLinkRequest`- Deserialize requires fiddly lifetime bounds which
// greatly complicate all bindings to `DnaAddressable`.
// Less intrusive option was to create the `RemoteEntryLinkPayload` intermediary and have a custom serialization
// implementation for `RemoteEntryLinkRequest` that uses conversions to the `*Payload` struct to change its
// internal fields into byte arrays rather than typed data prior to transmission.

/// intermediary format to pass link request data through
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
struct RemoteEntryLinkPayload {
    pub remote_entry: Vec<u8>,
    pub target_entries: Vec<Vec<u8>>,
    pub removed_entries: Vec<Vec<u8>>,
}

impl<A, B> From<RemoteEntryLinkRequest<A, B>> for RemoteEntryLinkPayload
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    fn from(r: RemoteEntryLinkRequest<A, B>) -> Self {
        Self {
            remote_entry: extern_id_to_bytes::<A, EntryHash>(&r.remote_entry),
            target_entries: r.target_entries.iter().map(|t| { extern_id_to_bytes::<B, EntryHash>(t) }).collect(),
            removed_entries: r.removed_entries.iter().map(|t| { extern_id_to_bytes::<B, EntryHash>(t) }).collect(),
        }
    }
}

impl<A, B> TryFrom<RemoteEntryLinkPayload> for RemoteEntryLinkRequest<A, B>
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(r: RemoteEntryLinkPayload) -> std::result::Result<Self, SerializedBytesError> {
        Ok(Self {
            remote_entry: bytes_to_extern_id::<A>(&r.remote_entry)?,
            target_entries: r.target_entries.iter()
                .map(|t| { bytes_to_extern_id::<B>(t) })
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect(),
            removed_entries: r.removed_entries.iter()
                .map(|t| { bytes_to_extern_id::<B>(t) })
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect(),
        })
    }
}

/*
:SHONK:
ideally we would be using these conversions instead of wrapping in `RemoteEntryLinkPayload::from()`
in `request_remote_index_destination`, but the missing serde trait bounds on A & B (discussed above) make
such a binding impossible for now.

impl<A, B> TryFrom<&RemoteEntryLinkRequest<A, B>> for SerializedBytes
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(t: &RemoteEntryLinkRequest<A, B>) -> std::result::Result<SerializedBytes, SerializedBytesError> {
        let p = RemoteEntryLinkPayload::from(t.clone());
        encode(&p).map(|v|
            SerializedBytes::from(UnsafeBytes::from(v))
        )
    }
}

impl<A, B> TryFrom<RemoteEntryLinkRequest<A, B>> for SerializedBytes
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(t: RemoteEntryLinkRequest<A, B>) -> std::result::Result<SerializedBytes, SerializedBytesError> {
        SerializedBytes::try_from(&t)
    }
}
*/

impl<A, B> TryFrom<SerializedBytes> for RemoteEntryLinkRequest<A, B>
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(sb: SerializedBytes) -> std::result::Result<RemoteEntryLinkRequest<A, B>, SerializedBytesError> {
        let payload: RemoteEntryLinkPayload = decode(sb.bytes())?;
        Ok(RemoteEntryLinkRequest::try_from(payload)?)
    }
}

// Factory / constructor method to assist with constructing responses

impl<A, B> RemoteEntryLinkRequest<A, B>
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    pub fn new(local_cell_entry: &A, add_remote_entries: &[B], remove_remote_entries: &[B]) -> Self {
        RemoteEntryLinkRequest {
            remote_entry: (*local_cell_entry).clone(),
            target_entries: add_remote_entries.to_vec(),
            removed_entries: remove_remote_entries.to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct RemoteEntryLinkResponse {
    indexes_created: Vec<OtherCellResult<HeaderHash>>,
    indexes_removed: Vec<OtherCellResult<HeaderHash>>,
}

//-------------------------------[ CREATE ]-------------------------------------

/// Toplevel method for triggering a link creation flow between two records in
/// different DNA cells. The calling cell will have an 'origin query index' created for
/// fetching the referenced remote IDs; the destination cell will have a
/// 'destination query index' created for querying the referenced records in full.
///
pub fn create_remote_index<A, B, S, I, J>(
    remote_permission_id: &J,
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<OtherCellResult<HeaderHash>>>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        J: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // Build local index first (for reading linked record IDs from the `source`)
    let mut indexes_created: Vec<OtherCellResult<HeaderHash>> = create_remote_index_origin(
        source_entry_type, source,
        dest_entry_type, dest_addresses,
        link_tag, link_tag_reciprocal,
    ).iter()
        .map(convert_errors)
        .collect();

    // request building of remote index in foreign cell
    let resp = request_sync_remote_index_destination(
        remote_permission_id,
        source, dest_addresses, &vec![],
    );

    match resp {
        Ok(mut remote_results) => {
            indexes_created.append(&mut remote_results.indexes_created)
        },
        Err(e) => {
            indexes_created.push(Err(e.into()))
        },
    };

    Ok(indexes_created)
}

/// Creates an 'origin' query index used for fetching and querying pointers to other
/// records that are stored externally to this DNA / zome.
///
/// In the local DNA, this consists of indexes for all referenced foreign
/// content, bidirectionally linked to the source record for querying in either direction.
///
/// In the remote DNA, a corresponding 'destination' query index is built
/// @see create_remote_index_destination
///
fn create_remote_index_origin<A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> Vec<RecordAPIResult<HeaderHash>>
    where I: AsRef<str>,
        S: AsRef<[u8]> + ?Sized,
        A: Debug + DnaAddressable<EntryHash>,
        B: Debug + DnaAddressable<EntryHash>,
{
    dest_addresses.iter()
        .flat_map(create_dest_identities_and_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal))
        .collect()
}

/// Creates a 'destination' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of an identity `Path` for the remote content and bidirectional
/// links between it and its `dest_addresses`.
///
fn create_remote_index_destination<A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // create a base entry pointer for the referenced origin record
    let _identity_hash = create_entry_identity(source_entry_type, source)?;

    // link all referenced records to this pointer to the remote origin record
    Ok(dest_addresses.iter()
        .flat_map(create_dest_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal))
        .collect()
    )
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Toplevel method for triggering a link update flow between two records in
/// different DNAs. Indexes on both sides of the network boundary will be updated.
///
/// :NOTE: All remote index deletion logic should use the update/sync API, as IDs
/// must be explicitly provided in order to guard against indexes from unrelated
/// cells being wiped by this cell.
///
pub fn update_remote_index<A, B, S, I, J>(
    remote_permission_id: &J,
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    remove_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<RemoteEntryLinkResponse>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        J: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // handle local 'origin' index first
    let mut indexes_created: Vec<OtherCellResult<HeaderHash>> = create_remote_index_origin(
        source_entry_type, source,
        dest_entry_type, dest_addresses,
        link_tag, link_tag_reciprocal,
    ).iter()
        .map(convert_errors)
        .collect();

    let mut indexes_removed: Vec<OtherCellResult<HeaderHash>> = remove_remote_index_links(
        source_entry_type, source,
        dest_entry_type, remove_addresses,
        link_tag, link_tag_reciprocal,
    )?.iter()
        .map(convert_errors)
        .collect();

    // forward request to remote cell to update destination indexes
    let resp = request_sync_remote_index_destination(
        remote_permission_id,
        source, dest_addresses, remove_addresses,
    );

    match resp {
        Ok(mut remote_results) => {
            indexes_created.append(&mut remote_results.indexes_created);
            indexes_removed.append(&mut remote_results.indexes_removed);
        },
        Err(e) => {
            indexes_created.push(Err(e));
        },
    };

    Ok(RemoteEntryLinkResponse { indexes_created, indexes_removed })
}

/// Ask another bridged cell to build a 'destination query index' to match the
/// 'origin' one that we have just created locally.
/// When calling zomes within the same DNA, use `None` as `to_cell`.
///
/// :TODO: implement bridge genesis callbacks & private chain entry to wire up cross-DNA link calls
///
fn request_sync_remote_index_destination<A, B, I>(
    remote_permission_id: &I,
    source: &A,
    dest_addresses: &[B],
    removed_addresses: &[B],
) -> OtherCellResult<RemoteEntryLinkResponse>
    where I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // Call into remote DNA to enable target entries to setup data structures
    // for querying the associated remote entry records back out.
    Ok(call_zome_method(
        source, remote_permission_id,
        &RemoteEntryLinkPayload::from(RemoteEntryLinkRequest::new(
            source,
            dest_addresses, removed_addresses,
        ))
    )?)
}

/// Respond to a request from a remote source to build a 'destination' link index for some externally linking content.
///
/// This essentially ensures an identity `Path` for the remote `source` and then links it to every
/// `dest_addresses` found locally within this DNA before removing any links to `removed_addresses`.
///
/// The returned `RemoteEntryLinkResponse` provides an appropriate format for responding to indexing
/// requests that originate from calls to `create/update/delete_remote_index` in a foreign DNA.
///
pub fn sync_remote_index<A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    removed_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> OtherCellResult<RemoteEntryLinkResponse>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // create any new indexes
    let indexes_created = create_remote_index_destination(
        source_entry_type, source,
        dest_entry_type, dest_addresses,
        link_tag, link_tag_reciprocal,
    ).map_err(CrossCellError::from)?.iter()
        .map(convert_errors)
        .collect();

    // remove passed stale indexes
    let indexes_removed = remove_remote_index_links(
        source_entry_type, source,
        dest_entry_type, removed_addresses,
        link_tag, link_tag_reciprocal,
    ).map_err(CrossCellError::from)?.iter()
        .map(convert_errors)
        .collect();

    Ok(RemoteEntryLinkResponse { indexes_created, indexes_removed })
}

//-------------------------------[ DELETE ]-------------------------------------

/// Deletes a set of links between a remote record reference and some set
/// of local target EntryHashes.
///
/// The `Path` representing the remote target is not
/// affected in the removal, and is simply left dangling in the
/// DHT space as an indicator of previously linked items.
///
fn remove_remote_index_links<A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    remove_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    Ok(remove_addresses.iter()
        .flat_map(delete_dest_indexes(
            source_entry_type, source,
            dest_entry_type,
            link_tag, link_tag_reciprocal,
        ))
        .collect()
    )
}