/**
 * Helpers related to uniquely idenfying entry data, such that different entries
 * can be referenced by consistent identities that do not change over time.
 * Implemented with combined `(DnaHash, AnyDhtHash)` pairs for addressing information
 * such that the identities of records from other cells can be encoded natively.
 *
 * This also implicitly manages an unordered sparse index to all publicly created
 * records across the shared DHT.
 *
 * :TODO: Paths should maybe be determined by initial `HeaderHash` to ensure uniqueness,
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
use hdk::prelude::*;
use hdk_uuid_types::DnaAddressable;

use crate::{
    RecordAPIResult, DataIntegrityError,
    entry_helpers::get_entry_by_address,
};

/// Determine root `Path` for an entry type, can be used to anchor type-specific indexes & queries.
///
pub (crate) fn entry_type_root_path<S>(
    entry_type_path: S,
) -> Path
    where S: AsRef<str>,
{
    Path::from(vec![entry_type_path.as_ref().as_bytes().to_vec().into()])
}

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
    let identifier = get_entry_by_address(identity_path_address);

    // throw meaningful error if reference is invalid
    match identifier {
        Err(_) => Err(DataIntegrityError::CorruptIndexError(identity_path_address.clone(), None)),
        Ok(identity) => Ok(identity),
    }
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a pointer to initialise a universally-unique ID for a new entry, and returns
/// the `EntryHash` of the stored identifier.
///
/// This identifier is intended to be used as an anchor to base links to/from the
/// entry onto.
///
/// Also links the identifier to a global index for all entries of the given `entry_type`.
/// :TODO: replace this linkage with date-ordered sparse index based on record creation time
/// @see query_root_index()
///
pub fn create_entry_identity<A, S, E>(
    entry_type: S,
    initial_address: &A,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash> + EntryDefRegistration,
        Entry: TryFrom<A, Error = E>,
        WasmError: From<E>,
{
    create_entry(initial_address.to_owned())?;

    let id_hash = calculate_identity_address(&entry_type, initial_address)?;

    let index_path = entry_type_root_path(&entry_type);
    index_path.ensure()?;
    create_link(
        index_path.path_entry_hash()?,
        id_hash.to_owned(),
        HdkLinkType::Any,
        LinkTag::new(crate::identifiers::RECORD_GLOBAL_INDEX_LINK_TAG),
    )?;

    Ok(id_hash)
}
