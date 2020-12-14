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
use hdk3::prelude::*;

use crate::{
    GraphAPIResult,
    OtherCellResult, CrossCellError,
    internals::*,
    identity_helpers::{
        create_entry_identity,
    },
};

// Common request format (zome trait) for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
struct RemoteEntryLinkRequest {
    base_entry: EntryHash,
    target_entries: Vec<EntryHash>,
    removed_entries: Vec<EntryHash>,
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
pub fn create_remote_index<S: AsRef<[u8]>, I: AsRef<str>>(
    to_cell: Option<CellId>,
    zome_name: ZomeName,
    zome_method: FunctionName,
    cap_secret: Option<CapSecret>,
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest_addresses: &[EntryHash],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> GraphAPIResult<Vec<OtherCellResult<HeaderHash>>> {
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
        to_cell, zome_name, zome_method, cap_secret,
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
fn create_remote_index_origin<S: AsRef<[u8]>, I: AsRef<str>>(
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest_addresses: &[EntryHash],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> Vec<GraphAPIResult<HeaderHash>> {
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
fn create_remote_index_destination<S: AsRef<[u8]>, I: AsRef<str>>(
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest_addresses: &[EntryHash],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> GraphAPIResult<Vec<GraphAPIResult<HeaderHash>>> {
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
pub fn update_remote_index<S: AsRef<[u8]>, I: AsRef<str>>(
    to_cell: Option<CellId>,
    zome_name: ZomeName,
    zome_method: FunctionName,
    cap_secret: Option<CapSecret>,
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest_addresses: &[EntryHash],
    remove_addresses: &[EntryHash],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> GraphAPIResult<RemoteEntryLinkResponse>
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
        to_cell, zome_name, zome_method, cap_secret,
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
fn request_sync_remote_index_destination(
    to_cell: Option<CellId>,
    zome_name: ZomeName,
    zome_method: FunctionName,
    cap_secret: Option<CapSecret>,
    source: &EntryHash,
    dest_addresses: &[EntryHash],
    removed_addresses: &[EntryHash],
) -> OtherCellResult<RemoteEntryLinkResponse> {
    // Call into remote DNA to enable target entries to setup data structures
    // for querying the associated remote entry records back out.
    Ok(call(
        to_cell, zome_name, zome_method, cap_secret, &RemoteEntryLinkRequest {
            base_entry: source.clone(),
            target_entries: dest_addresses.to_vec(),
            removed_entries: removed_addresses.to_vec(),
        }
    ).map_err(CrossCellError::from)?)
}

/// Respond to a request from a remote source to build a 'destination' link index for some externally linking content.
///
/// This essentially ensures an identity `Path` for the remote `source` and then links it to every
/// `dest_addresses` found locally within this DNA before removing any links to `removed_addresses`.
///
/// The returned `RemoteEntryLinkResponse` provides an appropriate format for responding to indexing
/// requests that originate from calls to `create/update/delete_remote_index` in a foreign DNA.
///
pub fn sync_remote_index<S: AsRef<[u8]>, I: AsRef<str>>(
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    dest_addresses: &[EntryHash],
    removed_addresses: &[EntryHash],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> OtherCellResult<RemoteEntryLinkResponse> {
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
fn remove_remote_index_links<S: AsRef<[u8]>, I: AsRef<str>>(
    source_entry_type: &I,
    source: &EntryHash,
    dest_entry_type: &I,
    remove_addresses: &[EntryHash],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> GraphAPIResult<Vec<GraphAPIResult<HeaderHash>>> {
    Ok(remove_addresses.iter()
        .flat_map(delete_dest_indexes(
            source_entry_type, source,
            dest_entry_type,
            link_tag, link_tag_reciprocal,
        ))
        .collect()
    )
}
