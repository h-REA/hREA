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
// use std::convert::TryFrom;
// use std::fmt::Debug;
use hdk::prelude::*;
use holochain_serialized_bytes::{
    SerializedBytes, SerializedBytesError,
    // UnsafeBytes,
    // /*decode,*/ encode,
};
use hdk_semantic_indexes_zome_rpc::{
    RemoteEntryLinkRequest, RemoteEntryLinkResponse,
};

use crate::{
    DnaAddressable,
    // CrossCellError,
    OtherCellResult, RecordAPIResult,
    foreign_index_helpers::{
        request_sync_foreign_index_destination,
        merge_indexing_results,
    },
    rpc_helpers::call_zome_method,
};

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
    let resp = request_sync_index_destination(
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
    let resp = request_sync_index_destination(
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
fn request_sync_index_destination<A, B, I>(
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
