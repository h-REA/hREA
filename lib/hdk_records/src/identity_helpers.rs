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
use hdk_type_serialization_macros::{extern_id_to_bytes, bytes_to_extern_id, DnaAddressable};

use crate::{
    RecordAPIResult, DataIntegrityError,
    link_helpers::get_linked_addresses,
    entry_helpers::get_entry_by_address,
};

/// Represent `key index` record identities using native Holochain `Path` construct
///
/// :TODO: sharding strategy for `c2`
///
fn identity_path_for<A, S>(
    entry_type_root_path: S,
    base_address: &A,
) -> temp_path::path::Path
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
{
    let type_root = entry_type_root_path.as_ref().as_bytes().to_vec();

    temp_path::path::Path::from(vec![type_root.into(), extern_id_to_bytes::<A, EntryHash>(base_address).into()])
}

/// Determine root `Path` for an entry type, can be used to anchor type-specific indexes & queries.
///
pub (crate) fn entry_type_root_path<S>(
    entry_type_path: S,
) -> temp_path::path::Path
    where S: AsRef<str>,
{
  temp_path::path::Path::from(vec![entry_type_path.as_ref().as_bytes().to_vec().into()])
}

//--------------------------------[ READ ]--------------------------------------

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
pub fn calculate_identity_address<A, S>(
    entry_type_root_path: S,
    base_address: &A,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
{
    Ok(identity_path_for(entry_type_root_path, base_address).hash()?)
}

/// Given an identity `EntryHash` (ie. the result of `create_entry_identity`),
/// query the underlying address for the progenitor Entry of the record.
///
pub (crate) fn read_entry_identity(
    identity_path_address: &EntryHash,
) -> RecordAPIResult<EntryHash>
{
    let mut addrs = get_linked_addresses(identity_path_address, LinkTag::new(crate::identifiers::RECORD_INITIAL_ENTRY_LINK_TAG))?;
    let entry_hash = addrs.pop().ok_or(DataIntegrityError::IndexNotFound((*identity_path_address).clone()))?;

    Ok(entry_hash)
}

/// Given an identity `EntryHash` (ie. the result of `create_entry_identity`),
/// query the `DnaHash` and `AnyDhtHash` of the record by inspecting the associated `Path` entry.
///
/// :WARNING: if sharding is introduced, this will cause runtime failures until changed
///
pub fn read_entry_identity_full<A>(
    identity_path_address: &EntryHash,
) -> RecordAPIResult<A>
    where A: DnaAddressable<EntryHash>,
{
    let index_path: temp_path::path::Path = get_entry_by_address(&identity_path_address)?;
    let components: &Vec<temp_path::path::Component> = index_path.as_ref();
    let compound_key = components.last();

    // ensure that a path component exists
    if None == compound_key { return Err(DataIntegrityError::CorruptIndexError(identity_path_address.clone(), None)); }

    // ensure final addressing path component length
    let key_bytes = compound_key.unwrap().as_ref();
    match bytes_to_extern_id(key_bytes) {
        Err(_) => Err(DataIntegrityError::CorruptIndexError(identity_path_address.clone(), Some(key_bytes.to_vec()))),
        Ok(identity) => Ok(identity),
    }
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a `Path` to initialise a unique index for a new entry, and returns
/// the `EntryHash` of the new `Path`.
///
/// This `Path` is intended to be used as an anchor to base links to/from the
/// entry onto.
///
pub fn create_entry_identity<A, S>(
    entry_type_root_path: S,
    initial_address: &A,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        A: DnaAddressable<EntryHash>,
{
    let path = identity_path_for(entry_type_root_path, initial_address);
    path.ensure()?;
    Ok(path.hash()?)
}
