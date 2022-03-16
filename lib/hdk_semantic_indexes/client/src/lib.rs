/**
 * Client library for driving semantic indexing zomes
 *
 * This module exposes some helper methods for use in Holochain zome modules which
 * wish to communicate with collaborating "semantic indexing" zomes for managing
 * record relationship data.
 *
 * In most relationships, there will be two sets of indexing zomes targeted per
 * operation: one for each of the records involved.
 *
 * There are two versions of most methods: `_local_` and `_remote_`. The only difference
 * between the two is the lookup mechanism used to locate the indexing zomes:
 *
 * - `_local_` zomes are both hosted in the same DNA as the calling zome, and
 *   are located via configuration attributes.
 * - `_remote_` zomes link to foreign Holochain app Cells- one of the indexes is
 *   present locally in the same DNA as the caller, and the other is hosted in a
 *   remote DNA which must be accessed via capability token.
 *
 * The exception to this pattern is `read_local_index`, for which there is no
 * `_remote_` version. This is because index zomes are always co-located with the
 * records to which they relate, and so it would not make sense to fetch relationship
 * data from other app Cells.
 *
 * @see     hdk_semantic_indexes_zome_lib
 * @package hdk_semantic_indexes_client_lib
 * @since   2020-08-07
 */
use std::collections::HashMap;
use hdk::prelude::*;
use holo_hash::DnaHash;
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

//-------------------------------[ MACRO LAYER ]-------------------------------------

/// Create indexes by defining record types, relationships and associated IDs.
///
#[macro_export]
macro_rules! create_index {
    // local bidirectional index
    (
        Local(
            $lrecord_type:ident.$lrel:ident($ldest_record_id:expr),
            $ldest_record_type:ident.$linv_rel:ident($lrecord_id:expr)
        )
    ) => {
        paste! {
            create_local_index(
                [<read_ $lrecord_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $lrecord_type:lower:snake _ $lrel:lower:snake>]),
                $lrecord_id,
                [<read_ $ldest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $ldest_record_type:lower:snake _ $linv_rel:lower:snake>]),
                $ldest_record_id,
            )
        }
    };
    // remote bidirectional index
    (
        Remote(
            $rrecord_type:ident.$rrel:ident($rdest_record_id:expr),
            $rdest_record_type:ident.$rinv_rel:ident($rrecord_id:expr)
        )
    ) => {
        paste! {
            create_remote_index(
                [<read_ $rrecord_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $rrecord_type:lower:snake _ $rrel:lower:snake>]),
                $rrecord_id,
                &stringify!([<index_ $rdest_record_type:lower:snake _ $rinv_rel:lower:snake>]),
                vec![$rdest_record_id.to_owned()].as_slice(),
            )
        }
    };
    // special case for self-referential or local-only indexes
    (
        Self(
            $record_type:ident($record_id:expr).$rel:ident($dest_record_id:expr)
        )
    ) => {
        paste! {
            create_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                $dest_record_id,
            )
        }
    };
    // automatic index:
    // local/remote determination is managed by DnaHash of target addresses
    (
        $record_type:ident.$rel:ident($dest_record_id:expr),
        $dest_record_type:ident.$inv_rel:ident($record_id:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                vec![$dest_record_id.to_owned()].as_slice(),
                vec![].as_slice(),
            )
        }
    };
    // self-referential, local-only indexes
    (
        $record_type:ident($record_id:expr).$rel:ident($dest_record_id:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                vec![$dest_record_id.to_owned()].as_slice(),
                vec![].as_slice(),
            )
        }
    };
}

/// Fetch the identifiers stored for a referenced relationship
///
#[macro_export]
macro_rules! read_index {
    (
        $record_type:ident($record_id:expr).$rel:ident
    ) => {
        paste! {
            read_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_read_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
            )
        }
    };
}

/// Update indexes by defining added and removed identifiers.
///
#[macro_export]
macro_rules! update_index {
    // local index, add only
    (
        Local(
            $record_type:ident.$rel:ident($dest_record_ids:expr),
            $dest_record_type:ident.$inv_rel:ident($record_id:expr)
        )
    ) => {
        paste! {
            update_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                $dest_record_ids,
                &vec![].as_slice(),
            )
        }
    };
    // local index, remove only
    (
        Local(
            $record_type:ident.$rel:ident.not($remove_record_ids:expr),
            $dest_record_type:ident.$inv_rel:ident($record_id:expr)
        )
    ) => {
        paste! {
            update_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                &vec![].as_slice(),
                $remove_record_ids,
            )
        }
    };
    // local index, add and remove
    (
        Local(
            $record_type:ident.$rel:ident($dest_record_ids:expr).not($remove_record_ids:expr),
            $dest_record_type:ident.$inv_rel:ident($record_id:expr)
        )
    ) => {
        paste! {
            update_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                $dest_record_ids,
                $remove_record_ids,
            )
        }
    };

    // remote index, add only
    (
        Remote(
            $record_type:ident.$rel:ident($dest_record_ids:expr),
            $dest_record_type:ident.$inv_rel:ident($record_id:expr)
        )
    ) => {
        paste! {
            update_remote_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                $dest_record_ids,
                &vec![].as_slice(),
            )
        }
    };
    // remote index, remove only
    (
        Remote(
            $record_type:ident.$rel:ident.not($remove_record_ids:expr),
            $dest_record_type:ident.$inv_rel:ident($record_id:expr)
        )
    ) => {
        paste! {
            update_remote_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                &vec![].as_slice(),
                $remove_record_ids,
            )
        }
    };
    // remote index, add and remove
    (
        Remote(
            $record_type:ident.$rel:ident($dest_record_ids:expr).not($remove_record_ids:expr),
            $dest_record_type:ident.$inv_rel:ident($record_id:expr)
        )
    ) => {
        paste! {
            update_remote_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                $dest_record_ids,
                $remove_record_ids,
            )
        }
    };

    // self-referential or local-only indexes, add only
    (
        Self(
            $record_type:ident($record_id:expr).$rel:ident($dest_record_ids:expr)
        )
    ) => {
        paste! {
            update_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                $dest_record_ids,
                &vec![].as_slice(),
            )
        }
    };
    // self-referential or local-only indexes, remove only
    (
        Self(
            $record_type:ident($record_id:expr).$rel:ident.not($remove_record_ids:expr)
        )
    ) => {
        paste! {
            update_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &vec![].as_slice(),
                $remove_record_ids,
            )
        }
    };
    // self-referential or local-only indexes, add & remove
    (
        Self(
            $record_type:ident($record_id:expr).$rel:ident($dest_record_ids:expr).not($remove_record_ids:expr)
        )
    ) => {
        paste! {
            update_local_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                $dest_record_ids,
                $remove_record_ids,
            )
        }
    };

    // automatic index, add only
    (
        $record_type:ident.$rel:ident($dest_record_ids:expr),
        $dest_record_type:ident.$inv_rel:ident($record_id:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                $dest_record_ids,
                vec![].as_slice(),
            )
        }
    };
    // automatic index, add and remove
    (
        $record_type:ident.$rel:ident($dest_record_ids:expr).not($remove_record_ids:expr),
        $dest_record_type:ident.$inv_rel:ident($record_id:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                $dest_record_ids,
                $remove_record_ids,
            )
        }
    };
    // automatic index, remove only
    (
        $record_type:ident.$rel:ident.not($remove_record_ids:expr),
        $dest_record_type:ident.$inv_rel:ident($record_id:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                [<read_ $dest_record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                &stringify!([<index_ $dest_record_type:lower:snake _ $inv_rel:lower:snake>]),
                vec![].as_slice(),
                $remove_record_ids,
            )
        }
    };

    // self-referential or local-only indexes, add only
    (
        $record_type:ident($record_id:expr).$rel:ident($dest_record_ids:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                $dest_record_ids,
                &vec![].as_slice(),
            )
        }
    };
    // self-referential or local-only indexes, remove only
    (
        $record_type:ident($record_id:expr).$rel:ident.not($remove_record_ids:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                &vec![].as_slice(),
                $remove_record_ids,
            )
        }
    };
    // self-referential or local-only indexes, add & remove
    (
        $record_type:ident($record_id:expr).$rel:ident($dest_record_ids:expr).not($remove_record_ids:expr)
    ) => {
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                $dest_record_ids,
                $remove_record_ids,
            )
        }
    };
}

//-------------------------------[ CREATE ]-------------------------------------

/// Outer method for creating indexes.
///
/// :TODO: documentation
///
/// @see create_index!
///
pub fn manage_index<C, F, G, A, B, S>(
    origin_zome_name_from_config: F,
    origin_fn_name: &S,
    source: &A,
    dest_zome_name_from_config: G,
    dest_fn_name: &S,
    remote_permission_id: &S,
    dest_addresses: &[B],
    remove_addresses: &[B],
) -> RecordAPIResult<Vec<OtherCellResult<RemoteEntryLinkResponse>>>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Copy + Fn(C) -> Option<String>,
        G: Copy + Fn(C) -> Option<String>,
{
    // altering an index with no targets is a no-op
    if dest_addresses.len() == 0 && remove_addresses.len() == 0 {
        return Ok(vec![])
    }

    let sources = vec![source.clone()];
    let targets = prefilter_target_dnas(dest_addresses, remove_addresses)?;

    // Manage local index creation / removal

    let local_forward_add = targets.local_dests.0.iter()
        .map(|dest| {
            request_sync_local_index(
                origin_zome_name_from_config, origin_fn_name,
                dest, &sources, &vec![],
            )
        });
    let local_forward_remove = targets.local_dests.1.iter()
        .map(|dest| {
            request_sync_local_index(
                origin_zome_name_from_config, origin_fn_name,
                dest, &vec![], &sources,
            )
        });
    let local_reciprocal_update = std::iter::once(request_sync_local_index(
        dest_zome_name_from_config, dest_fn_name,
        source, targets.local_dests.0.as_slice(), targets.local_dests.1.as_slice(),
    ));

    // Manage remote index creation / removal & append to resultset

    Ok(std::iter::empty()
        .chain(local_forward_add)
        .chain(local_forward_remove)
        .chain(local_reciprocal_update)
        .chain(targets.remote_dests.iter()
            .flat_map(|(_dna, (add_dests, remove_dests))| {
                let remote_forward_add = add_dests.iter()
                    .map(|dest| {
                        request_sync_local_index(
                            origin_zome_name_from_config, origin_fn_name,
                            dest, &sources, &vec![],
                        )
                    });
                let remote_forward_remove = remove_dests.iter()
                    .map(|dest| {
                        request_sync_local_index(
                            origin_zome_name_from_config, origin_fn_name,
                            dest, &vec![], &sources,
                        )
                    });
                let remote_reciprocal_update = std::iter::once(request_sync_remote_index(
                    remote_permission_id,
                    source, add_dests, remove_dests,
                ));

                std::iter::empty()
                    .chain(remote_forward_add)
                    .chain(remote_forward_remove)
                    .chain(remote_reciprocal_update)
            }))
        .collect())
}

/// Toplevel method for triggering a link creation flow between two records in
/// different DNA cells. The calling cell will have an 'origin query index' created for
/// fetching the referenced remote IDs; the destination cell will have a
/// 'destination query index' created for querying the referenced records in full.
///
/// :IMPORTANT: in handling errors from this method, one should take care to test
/// ALL `OtherCellResult`s in the returned Vector if the success of updating both
/// sides of the index is important. By default, an index failure on either side
/// may occur without the outer result failing.
///
/// :TODO: consider a robust method for handling updates of data in remote DNAs &
/// foreign zomes where the transactionality guarantees of single-zome execution
/// are foregone.
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
///
/// :IMPORTANT: unlike remote indexes, it can be considered that DNA-local indexes are executing
/// "in good faith", since their inclusion in a DNA means they become part of the (hashed & shared)
/// computation space. The expectation is placed onto the DNA configurator to ensure that all
/// identifiers align and options correctly validate. As such, we treat local index failures more
/// severely than remote ones and WILL return a toplevel `DataIntegrityError` if *either* of the
/// paired index zomes fails to update; possibly causing a rollback in any other client logic
/// already attempted.
///
/// :TODO: as above for remote indexes, so with local ones.
///
pub fn create_local_index<C, F, G, A, B, S>(
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
pub fn read_local_index<'a, O, A, S, F, C>(
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
pub fn update_local_index<C, F, G, A, B, S>(
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


/// internal struct for pre-arranging lists of IDs for transmission to remote
/// DNA-relative API endpoints
#[derive(Debug)]
struct TargetsByDna<B>
    where B: DnaAddressable<EntryHash>,
{
    pub local_dests: (Vec<B>, Vec<B>),
    pub remote_dests: HashMap<DnaHash, (Vec<B>, Vec<B>)>,
}

// pre-arrange input IDs for transmission to target DNAs
fn prefilter_target_dnas<'a, B>(
    dest_addresses: &'a [B],
    remove_addresses: &'a [B],
) -> RecordAPIResult<TargetsByDna<B>>
    where B: DnaAddressable<EntryHash>,
{
    let local_dna = dna_info()?.hash;

    let results = dest_addresses.iter()
        .fold(TargetsByDna {
            local_dests: (vec![], vec![]),
            remote_dests: HashMap::new(),
        }, |mut targets: TargetsByDna<B>, val| {
            let target_dna: &DnaHash = val.as_ref();
            if local_dna == target_dna.to_owned() {
                targets.local_dests.0.push(val.to_owned());
            } else {
                match targets.remote_dests.insert(target_dna.to_owned(), (vec![val.to_owned()], vec![])) {
                    Some(mut prev_val) => {
                        let vals = targets.remote_dests.get_mut(target_dna).unwrap();
                        vals.0.append(&mut prev_val.0);
                    },
                    None => (),
                }
            }
            targets
        });

    Ok(remove_addresses.iter()
        .fold(results, |mut targets: TargetsByDna<B>, val| {
            let target_dna: &DnaHash = val.as_ref();
            if local_dna == target_dna.to_owned() {
                targets.local_dests.1.push(val.to_owned());
            } else {
                match targets.remote_dests.insert(target_dna.to_owned(), (vec![], vec![val.to_owned()])) {
                    Some(mut prev_val) => {
                        let vals = targets.remote_dests.get_mut(target_dna).unwrap();
                        vals.0.append(&mut prev_val.0);
                        vals.1.append(&mut prev_val.1);
                    },
                    None => (),
                }
            }
            targets
        })
    )
}
