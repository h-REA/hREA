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
use hdk::{prelude::*, hash_path::path::TypedPath};
use holo_hash::DnaHash;
use hdk_records::{
    RecordAPIResult, OtherCellResult, DataIntegrityError,
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
use hdk_semantic_indexes_integrity::LinkTypes;

//-------------------------------[ MACRO LAYER ]-------------------------------------

/// Create indexes by defining record types, relationships and associated IDs.
/// Local / remote determination is managed by DnaHash of target addresses.
///
#[macro_export]
macro_rules! create_index {
    // bidirectional 1:1 indexes
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

/// Manage arbitrary string-based indexes.
/// $addressable_type must be declared as the inner `DnaAddressable` type to use for the internal hash-based reference
#[macro_export]
macro_rules! update_string_index {
    // self-referential, local-only string indexes; add only
    (
        $record_type:ident($record_id:expr).$rel:ident($dest_string_ids:expr)<$addressable_type:ident>
    ) => { {
        let string_hashes: Vec<$addressable_type> = string_index_hashes($dest_string_ids)?;
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                string_hashes.as_slice(),
                vec![].as_slice(),
            )
        }
    } };
    // self-referential or local-only string indexes, remove only
    (
        $record_type:ident($record_id:expr).$rel:ident.not($remove_string_ids:expr)<$addressable_type:ident>
    ) => { {
        let string_hashes: Vec<$addressable_type> = string_index_hashes($dest_string_ids)?;
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                &vec![].as_slice(),
                string_hashes.as_slice(),
            )
        }
    } };
    // self-referential or local-only string indexes, add & remove
    (
        $record_type:ident($record_id:expr).$rel:ident($dest_string_ids:expr).not($remove_string_ids:expr)<$addressable_type:ident>
    ) => { {
        let dest_string_hashes: Vec<$addressable_type> = string_index_hashes($dest_string_ids)?;
        let remove_string_hashes: Vec<$addressable_type> = string_index_hashes($remove_string_ids)?;
        paste! {
            manage_index(
                [<read_ $record_type:lower:snake _index_zome>],
                &stringify!([<_internal_index_ $record_type:lower:snake _ $rel:lower:snake>]),
                $record_id,
                |_| { None }, // specify none for destination index
                &"", // ignored, since no index zome name is returned
                &"", // ignored, since no index zome name is returned
                dest_string_hashes.as_slice(),
                remove_string_hashes.as_slice(),
            )
        }
    } };
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
/// Local / remote determination is managed by DnaHash of target addresses, and
/// you can freely mix identifiers from disparate DNAs in the same input.
///
#[macro_export]
macro_rules! update_index {
    // add only
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
    // add and remove
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
    // remove only
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

    let empty = vec![];

    let local_forward_add = (
        if targets.local_dests.0.len() > 0 { targets.local_dests.0.iter() }
        else { empty.iter() }
    ).map(|dest| {
        request_sync_local_index(
            origin_zome_name_from_config, origin_fn_name,
            dest, &sources, &vec![],
        )
    });

    let local_forward_remove = (
        if targets.local_dests.1.len() > 0 { targets.local_dests.1.iter() }
        else { empty.iter() }
    ).map(|dest| {
        request_sync_local_index(
            origin_zome_name_from_config, origin_fn_name,
            dest, &vec![], &sources,
        )
    });

    let mut local_updates = vec![];
    let local_reciprocal_update =
        if targets.local_dests.0.len() > 0 || targets.local_dests.1.len() > 0 {
            let mut others = vec![request_sync_local_index(
                dest_zome_name_from_config, dest_fn_name,
                source, targets.local_dests.0.as_slice(), targets.local_dests.1.as_slice(),
            )];
            local_updates.append(&mut others);
            local_updates.to_owned()
        } else { vec![] };

    // Manage remote index creation / removal & append to resultset

    // :TODO: improve error handling by asserting that successful RPC
    // calls fired for local targets + remote targets add up to equal
    // the number of input `dest_addresses` & `remove_addresses`

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

pub fn string_index_hashes<T>(dest_string_ids: Vec<String>) -> RecordAPIResult<Vec<T>>
    where T: DnaAddressable<EntryHash>,
{
    let (maybe_string_hashes, errors): (
        Vec<RecordAPIResult<T>>,
        Vec<RecordAPIResult<T>>,
    ) = dest_string_ids.iter().map(string_index_hash).partition(Result::is_ok);
    if errors.first().is_some() {
        return Err(DataIntegrityError::BadStringIndexError(
            dest_string_ids.concat().as_bytes().to_vec()
        ));
    }
    Ok(maybe_string_hashes.iter().cloned().map(Result::unwrap).collect())
}

fn string_index_hash<T>(index_value: &String) -> RecordAPIResult<T>
    where T: DnaAddressable<EntryHash>,
{
    let type_path: Path = index_value.try_into()?;
    let typed_type_path: TypedPath = type_path.typed(LinkTypes::SemanticIndex)?;
    // :TODO: this can be done entirely in-memory after HDI upgrade, does not have to be written to the DHT
    typed_type_path.ensure()?;
    Ok(T::new(dna_info()?.hash, typed_type_path.path_entry_hash()?))
}
