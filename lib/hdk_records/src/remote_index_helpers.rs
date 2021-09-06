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
use holochain_serialized_bytes::{
    SerializedBytes, SerializedBytesError, UnsafeBytes,
    /*decode,*/ encode,
};

use crate::{
    DnaAddressable,
    CrossCellError, OtherCellResult, RecordAPIResult,
    internals::*,
    identity_helpers::create_entry_identity,
    foreign_index_helpers::{
        request_sync_foreign_index_destination,
        merge_indexing_results,
    },
    rpc_helpers::call_zome_method,
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

impl<A, B> TryFrom<&RemoteEntryLinkRequest<A, B>> for SerializedBytes
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(t: &RemoteEntryLinkRequest<A, B>) -> Result<SerializedBytes, SerializedBytesError> {
        encode(t).map(|v|
            SerializedBytes::from(UnsafeBytes::from(v))
        )
    }
}

impl<A, B> TryFrom<RemoteEntryLinkRequest<A, B>> for SerializedBytes
    where A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    type Error = SerializedBytesError;
    fn try_from(t: RemoteEntryLinkRequest<A, B>) -> Result<SerializedBytes, SerializedBytesError> {
        SerializedBytes::try_from(&t)
    }
}

// :TODO: is this needed?
// impl<'de, A: Deserialize<'de>, B: Deserialize<'de>> TryFrom<SerializedBytes> for RemoteEntryLinkRequest<A, B>
//     where A: DnaAddressable<EntryHash>,
//         B: DnaAddressable<EntryHash>,
// {
//     type Error = SerializedBytesError;
//     fn try_from(sb: SerializedBytes) -> Result<RemoteEntryLinkRequest<A, B>, SerializedBytesError> {
//         decode(sb.bytes())
//     }
// }

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
    pub indexes_created: Vec<OtherCellResult<HeaderHash>>,
    pub indexes_removed: Vec<OtherCellResult<HeaderHash>>,
}

//-------------------------------[ CREATE ]-------------------------------------

/// Toplevel method for triggering a link creation flow between two records in
/// different DNA cells. The calling cell will have an 'origin query index' created for
/// fetching the referenced remote IDs; the destination cell will have a
/// 'destination query index' created for querying the referenced records in full.
///
pub fn create_remote_index<C, F, A, B, S>(
    origin_zome_name_from_config: F,
    origin_fn_name: &S,
    source: &A,
    remote_permission_id: &S,
    dest_addresses: &[B],
) -> RecordAPIResult<Vec<OtherCellResult<HeaderHash>>>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Clone + FnOnce(C) -> Option<String>,
{
    let sources = vec![source.clone()];

    // Build local index first (for reading linked record IDs from the `source`)
    // :TODO: optimise to call once per target DNA
    let created: Vec<OtherCellResult<RemoteEntryLinkResponse>> = dest_addresses.iter().map(|dest| {
        request_sync_foreign_index_destination(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &sources, &vec![],
        )
    }).collect();

    // request building of remote index in foreign cell
    let resp = request_sync_remote_index_destination(
        remote_permission_id,
        source, dest_addresses, &vec![],
    );

    let mut indexes_created = merge_indexing_results(&created, |r| { r.indexes_created.to_owned() });

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

/// Creates a 'destination' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of an identity `Path` for the remote content and bidirectional
/// links between it and its `dest_addresses`.
///
pub (crate) fn create_remote_index_destination<A, B, S, I>(
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
        .flat_map(create_dest_identities_and_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal))
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
pub fn update_remote_index<C, F, A, B, S>(
    origin_zome_name_from_config: F,
    origin_fn_name: &S,
    source: &A,
    remote_permission_id: &S,
    dest_addresses: &[B],
    remove_addresses: &[B],
) -> RecordAPIResult<RemoteEntryLinkResponse>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Clone + FnOnce(C) -> Option<String>,
{
    // handle local 'origin' index first
    let sources = vec![source.clone()];

    // :TODO: optimise to call once per target DNA
    let created: Vec<OtherCellResult<RemoteEntryLinkResponse>> = dest_addresses.iter().map(|dest| {
        request_sync_foreign_index_destination(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &sources, &vec![],
        )
    }).collect();

    let deleted: Vec<OtherCellResult<RemoteEntryLinkResponse>> = remove_addresses.iter().map(|dest| {
        request_sync_foreign_index_destination(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &vec![], &sources,
        )
    }).collect();

    // forward request to remote cell to update destination indexes
    let resp = request_sync_remote_index_destination(
        remote_permission_id,
        source, dest_addresses, remove_addresses,
    );

    let mut indexes_created = merge_indexing_results(&created, |r| { r.indexes_created.to_owned() });
    let mut indexes_removed = merge_indexing_results(&deleted, |r| { r.indexes_removed.to_owned() });
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
    // :TODO: :SHONK: currently, all destination/removal addresses are assumed to
    //        be of the same DNA. We should partition inputs into sets keyed by
    //        destination DNA before firing off these operations.
    let context_dna = dest_addresses.iter().cloned().nth(0)
        .unwrap_or_else(|| {
            removed_addresses.iter().cloned().nth(0)
                .unwrap()
        });

    // Call into remote DNA to enable target entries to setup data structures
    // for querying the associated remote entry records back out.
    Ok(call_zome_method(
        &context_dna, remote_permission_id,
        RemoteEntryLinkRequest::new(
            source,
            dest_addresses, removed_addresses,
        )
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
pub (crate) fn remove_remote_index_links<A, B, S, I>(
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
