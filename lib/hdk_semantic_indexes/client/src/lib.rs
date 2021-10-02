/**
 * Client library for driving semantic indexing zomes
 *
 * @see     hdk_semantic_indexes_zome_lib
 * @package hdk_semantic_indexes_client_lib
 * @since   2020-08-07
 */
use hdk::prelude::*;
use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    OtherCellResult, CrossCellError,
    DnaAddressable,
    rpc::{
        call_local_zome_method,
        call_zome_method,
    },
};
use hdk_semantic_indexes_zome_rpc::{
    ByAddress,
    RemoteEntryLinkRequest, RemoteEntryLinkResponse,
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
        request_sync_local_index(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &sources, &vec![],
        )
    }).collect();

    // request building of remote index in foreign cell
    let resp = request_sync_remote_index(
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

/// Creates a bidirectional link between a local entry and another from a foreign zome in the same DNA,
/// and returns a vector of the `HeaderHash`es of the (respectively) forward & reciprocal links created.
pub fn create_foreign_index<C, F, G, A, B, S>(
    origin_zome_name_from_config: F,
    origin_fn_name: &S,
    source: &A,
    dest_zome_name_from_config: G,
    dest_fn_name: &S,
    dest: &B,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: FnOnce(C) -> Option<String>,
        G: FnOnce(C) -> Option<String>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let dests = vec![(*dest).clone()];
    let sources = vec![source.clone()];

    let or = request_sync_local_index(origin_zome_name_from_config, origin_fn_name, dest, &sources, &vec![]);
    let dr = request_sync_local_index(dest_zome_name_from_config, dest_fn_name, source, &dests, &vec![]);

    let indexes_created = vec! [
        match dr {
            Ok(drr) => drr.indexes_created
                .first().ok_or(CrossCellError::Internal("cross-zome index creation failed".to_string()))?
                .clone()
                .map_err(|e| { DataIntegrityError::RemoteRequestError(e.to_string()) }),
            Err(e) => Err(e.into()),
        },
        match or {
            Ok(orr) => orr.indexes_created
                .first().ok_or(CrossCellError::Internal("cross-zome index creation failed".to_string()))?
                .clone()
                .map_err(|e| { DataIntegrityError::RemoteRequestError(e.to_string()) }),
            Err(e) => Err(e.into()),
        },
    ];

    Ok(indexes_created)
}

//--------------------------------[ READ ]--------------------------------------

/// Reads and returns all entry identities referenced by the given index from
/// (`base_entry_type.base_address` via `link_tag`.
///
/// Use this method to query associated IDs for a query edge, without retrieving
/// the records themselves.
///
pub fn read_foreign_index<'a, O, A, S, F, C>(
    zome_name_from_config: F,
    query_fn_name: &S,
    base_address: &A,
) -> RecordAPIResult<Vec<O>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: FnOnce(C) -> Option<String>,
        A: DnaAddressable<EntryHash>,
        O: serde::de::DeserializeOwned + DnaAddressable<EntryHash>,
{
    Ok(call_local_zome_method(
        zome_name_from_config, query_fn_name,
        ByAddress { address: base_address.to_owned() },
    )?)
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
        request_sync_local_index(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &sources, &vec![],
        )
    }).collect();

    let deleted: Vec<OtherCellResult<RemoteEntryLinkResponse>> = remove_addresses.iter().map(|dest| {
        request_sync_local_index(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &vec![], &sources,
        )
    }).collect();

    // forward request to remote cell to update destination indexes
    let resp = request_sync_remote_index(
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

/// Toplevel API for triggering an update flow between two indexes, where one is
/// in the local DNA and another is managed in a remote Cell.
///
pub fn update_foreign_index<C, F, G, A, B, S>(
    origin_zome_name_from_config: F,
    origin_fn_name: &S,
    source: &A,
    dest_zome_name_from_config: G,
    dest_fn_name: &S,
    dest_addresses: &[B],
    remove_addresses: &[B],
) -> RecordAPIResult<RemoteEntryLinkResponse>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Clone + FnOnce(C) -> Option<String>,
        G: FnOnce(C) -> Option<String>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let sources = vec![source.clone()];

    // :TODO: optimise to call once per target DNA
    let created: Vec<OtherCellResult<RemoteEntryLinkResponse>> = dest_addresses.iter().map(|dest| {
        request_sync_local_index(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &sources, &vec![],
        )
    }).collect();

    let deleted: Vec<OtherCellResult<RemoteEntryLinkResponse>> = remove_addresses.iter().map(|dest| {
        request_sync_local_index(
            origin_zome_name_from_config.to_owned(), origin_fn_name,
            dest, &vec![], &sources,
        )
    }).collect();

    let resp = request_sync_local_index(
        dest_zome_name_from_config, dest_fn_name,
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

fn merge_indexing_results(
    foreign_zome_results: &[OtherCellResult<RemoteEntryLinkResponse>],
    response_accessor: impl Fn(&RemoteEntryLinkResponse) -> Vec<OtherCellResult<HeaderHash>>,
) -> Vec<OtherCellResult<HeaderHash>>
{
    foreign_zome_results.iter()
        .flat_map(|r| {
            match r {
                Ok(resp) => response_accessor(resp),
                Err(e) => vec![Err(e.to_owned())],
            }
        })
        .collect()
}

/// Ask another bridged cell to build a 'destination query index' to match the
/// 'origin' one that we have just created locally.
/// When calling zomes within the same DNA, use `None` as `to_cell`.
///
fn request_sync_remote_index<A, B, I>(
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

/// Request for another cell to sync its indexes for a record updated within this cell
///
fn request_sync_local_index<C, F, A, B, S>(
    zome_name_from_config: F,
    foreign_fn_name: &S,
    source: &A,
    dest_addresses: &[B],
    removed_addresses: &[B],
) -> OtherCellResult<RemoteEntryLinkResponse>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: FnOnce(C) -> Option<String>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    Ok(call_local_zome_method(
        zome_name_from_config, foreign_fn_name,
        RemoteEntryLinkRequest::new(
            source,
            dest_addresses, removed_addresses,
        )
    )?)
}
