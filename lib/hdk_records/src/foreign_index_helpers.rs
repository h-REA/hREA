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
        create_remote_index_origin,
        remove_remote_index_links,
    },
    rpc::{
        call_local_zome_method,
    },
    internals::{
        convert_errors,
    },
};

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a bidirectional link between a local entry and another from a foreign zome in the same DNA,
/// and returns a vector of the `HeaderHash`es of the (respectively) forward & reciprocal links created.
pub fn create_foreign_index<C, F, A, B, S, I>(
    zome_name_from_config: F,
    foreign_fn_name: &FunctionName,
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest: &B,
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where I: AsRef<str>,
        S: AsRef<[u8]> + ?Sized,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: FnOnce(C) -> Option<String>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let dests = vec![(*dest).clone()];
    let mut or = create_remote_index_origin(source_entry_type, source, dest_entry_type, &dests, link_tag, link_tag_reciprocal);
    let dr = request_sync_foreign_index_destination(zome_name_from_config, foreign_fn_name, source, &dests, &vec![])?;

    let mut indexes_created = vec! [
        dr.indexes_created
            .first().ok_or(CrossCellError::Internal("cross-zome index creation failed".to_string()))?
            .clone()
            .map_err(|e| { DataIntegrityError::RemoteRequestError(e.to_string()) }),
    ];
    indexes_created.append(&mut or);

    Ok(indexes_created)
}

//-------------------------------[ UPDATE ]-------------------------------------

pub fn update_foreign_index<C, F, A, B, S, I>(
    zome_name_from_config: F,
    foreign_fn_name: &FunctionName,
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
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: FnOnce(C) -> Option<String>,
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
    let resp = request_sync_foreign_index_destination(
        zome_name_from_config, foreign_fn_name,
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

/// Request for another cell to sync its indexes for a record updated within this cell
///
fn request_sync_foreign_index_destination<C, F, A, B>(
    zome_name_from_config: F,
    foreign_fn_name: &FunctionName,
    source: &A,
    dest_addresses: &[B],
    removed_addresses: &[B],
) -> OtherCellResult<RemoteEntryLinkResponse>
    where C: std::fmt::Debug,
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
