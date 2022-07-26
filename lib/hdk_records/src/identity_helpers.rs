/**
 * Helpers related to uniquely idenfying entry data, such that different entries
 * can be referenced by consistent identities that do not change over time.
 * Implemented with combined `(DnaHash, AnyDhtHash)` pairs for addressing information
 * such that the identities of records from other cells can be encoded natively.
 *
 * This also implicitly manages an unordered sparse index to all publicly created
 * records across the shared DHT.
 *
 * :TODO: Paths should maybe be determined by initial `ActionHash` to ensure uniqueness,
 *        rather than relying on consumer to inject random bytes or timestamps.
 *        Though the random bytes thing is good, because it allows apps to decide
 *        whether data they write should be universally idempotent or not.
 *
 * :TODO: sharding of record path keyspace
 *
 * @see     crate::record_interface::Identified::identity()
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use chrono::{DateTime, NaiveDateTime, Utc};
use hdk::prelude::*;
use hdk_uuid_types::DnaAddressable;

use crate::{
    RecordAPIResult, DataIntegrityError,
    entry_helpers::get_entry_by_address,
    rpc_helpers::call_local_zome_method,
};
use hdk_semantic_indexes_zome_rpc::{
    AppendAddress,
};

//--------------------------------[ READ ]--------------------------------------

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
pub fn calculate_identity_address<A, S, E>(
    _entry_type_root_path: S,
    base_address: &A,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        Entry: TryFrom<A, Error = E>,
        WasmError: From<E>,
{
    Ok(hash_entry(base_address.to_owned())?)
}

/// Given an identity `EntryHash` (ie. the result of `create_entry_identity`),
/// query the `DnaHash` and `AnyDhtHash` of the record.
///
pub fn read_entry_identity<A>(
    identity_path_address: &EntryHash,
) -> RecordAPIResult<A>
    where A: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<A, Error = SerializedBytesError>,
{
    let (_meta, identifier) = get_entry_by_address(identity_path_address)
        .map_err(|_e| DataIntegrityError::CorruptIndexError(identity_path_address.clone(), None))?;

    Ok(identifier)
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a pointer to initialise a universally-unique ID for a new entry.
///
/// This identifier is intended to be used as an anchor to base links to/from the
/// entry onto.
///
/// Also links the identifier to a global index for all entries of the given `entry_type`.
///
pub fn create_entry_identity<A, S, F, C>(
    zome_name_from_config: F,
    entry_def_id: S,
    initial_address: &A,
) -> RecordAPIResult<()>
    where S: AsRef<str> + std::fmt::Display,
        A: DnaAddressable<EntryHash> + EntryDefRegistration,
        F: FnOnce(C) -> Option<String>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
{
    // @see hdk_semantic_indexes_zome_derive::index_zome
    let append_fn_name = format!("record_new_{}", entry_def_id);

    // :TODO: use timestamp from written Record action rather than system time at time of RPC call
    let now = sys_time()?.as_seconds_and_nanos();
    let now_stamp = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(now.0, now.1), Utc);

    // request addition to index in companion zome
    // :TODO: move this to postcommit hook of coordinator zome, @see #264
    Ok(call_local_zome_method(
        zome_name_from_config, append_fn_name,
        AppendAddress {
            address: initial_address.to_owned(),
            timestamp: now_stamp,
        },
    ).map_err(|e| { DataIntegrityError::LocalIndexNotConfigured(entry_def_id.to_string(), e.to_string()) })?)
}
