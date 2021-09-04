/**
 * Helpers relating to `foreign indexes`.
 *
 * A `foreign index` is similar to a `remote index`, except that it binds records
 * between *zomes in the same DNA* rather than *zomes in remote DNAs*. One set
 * of records and queries is managed in a "source" zome, another in a "destination".
 *
 * :WARNING: :TODO:
 *
 * If this seems confusing, it is. This module will likely be deprecated at some
 * point in favour of purpose-specific configuration attributes provided by Holochain
 * Core that enable more robust contracts between zomes at a DNA-local level. This
 * should also obviate some of the security concerns surfaced by the current logic
 * in this module; namely an ability afforded to the end-user to interfere with internal
 * consistency of the conductor database state.
 * In hREA's codebase, these WASM externs are exposed with the prefix `_internal_`.
 *
 * @see also
 * https://github.com/holochain/holochain/issues/563
 * https://github.com/holochain/holochain/issues/743
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2020-08-07
 */
use hdk::prelude::*;

use crate::{
    OtherCellResult, RecordAPIResult, DataIntegrityError, CrossCellError,
    DnaAddressable,
    remote_indexes::{
        RemoteEntryLinkRequest, RemoteEntryLinkResponse,
    },
    rpc::{
        call_local_zome_method,
    },
};

//-------------------------------[ CREATE ]-------------------------------------

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

    // let mut or = create_remote_index_origin(source_entry_type, source, dest_entry_type, &dests, link_tag, link_tag_reciprocal);
    let or = request_sync_foreign_index_destination(origin_zome_name_from_config, origin_fn_name, dest, &sources, &vec![])?;
    let dr = request_sync_foreign_index_destination(dest_zome_name_from_config, dest_fn_name, source, &dests, &vec![])?;

    let indexes_created = vec! [
        dr.indexes_created
            .first().ok_or(CrossCellError::Internal("cross-zome index creation failed".to_string()))?
            .clone()
            .map_err(|e| { DataIntegrityError::RemoteRequestError(e.to_string()) }),
        or.indexes_created
            .first().ok_or(CrossCellError::Internal("cross-zome index creation failed".to_string()))?
            .clone()
            .map_err(|e| { DataIntegrityError::RemoteRequestError(e.to_string()) }),
    ];

    Ok(indexes_created)
}

//-------------------------------[ UPDATE ]-------------------------------------

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

    let resp = request_sync_foreign_index_destination(
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

/// Request for another cell to sync its indexes for a record updated within this cell
///
fn request_sync_foreign_index_destination<C, F, A, B, S>(
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
