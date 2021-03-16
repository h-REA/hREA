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
use hdk::hash_path::path::Component;
use holo_hash::{DnaHash, HOLO_HASH_UNTYPED_LEN};

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
) -> Path
    where S: AsRef<str>,
        A: AsRef<DnaHash> + AsRef<EntryHash>,
{
    let type_root = entry_type_root_path.as_ref().as_bytes().to_vec();

    // use single identifier to combine EntryHash + DnaHash
    // place EntryHash before DnaHash to aid in sharding strategies that look at header bytes
    let entry_address: &EntryHash = base_address.as_ref();
    let dna_hash: &DnaHash = base_address.as_ref();
    let mut combined_id = entry_address.as_ref().to_vec();
    combined_id.append(&mut dna_hash.as_ref().to_vec());

    Path::from(vec![type_root.into(), combined_id.into()])
}

/// Determine root `Path` for an entry type, can be used to
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
pub (crate) fn calculate_identity_address<A, S>(
    entry_type_root_path: S,
    base_address: &A,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        A: AsRef<DnaHash> + AsRef<EntryHash> + From<(DnaHash, EntryHash)>,
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
/// query the origin `DnaHash` by inspecting the associated `Path` entry.
///
pub (crate) fn read_entry_identity_full<A>(
    identity_path_address: &EntryHash,
) -> RecordAPIResult<A>
    where A: From<(DnaHash, EntryHash)>,
{
    let index_path: Path = get_entry_by_address(&identity_path_address)?;
    let components: &Vec<Component> = index_path.as_ref();
    let compound_key = components.last();

    // ensure that a path component exists
    if None == compound_key { return Err(DataIntegrityError::CorruptIndexError(identity_path_address.clone(), None)); }

    // ensure final addressing path component length
    // :WARNING: if sharding is introduced, this will cause runtime failures until changed
    let key_bytes = compound_key.unwrap().as_ref();
    if key_bytes.len() != HOLO_HASH_UNTYPED_LEN * 2 { return Err(DataIntegrityError::CorruptIndexError(identity_path_address.clone(), Some(key_bytes.to_vec()))) }

    // pull DnaHash from last 36 bytes; first 36 are for EntryHash/HeaderHash
    // @see holo_hash::hash
    Ok((
        DnaHash::from_raw_36(key_bytes[HOLO_HASH_UNTYPED_LEN..].to_vec()),
        EntryHash::from_raw_36(key_bytes[0..HOLO_HASH_UNTYPED_LEN].to_vec()),
    ).into())
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a `Path` to initialise a unique index for a new entry, and returns
/// the `EntryHash` of the new `Path`.
///
/// This `Path` is intended to be used as an anchor to base links to/from the
/// entry onto.
///
pub (crate) fn create_entry_identity<A, S>(
    entry_type_root_path: S,
    initial_address: &A,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
        A: AsRef<DnaHash> + AsRef<EntryHash>,
{
    let path = identity_path_for(entry_type_root_path, initial_address);
    path.ensure()?;
    Ok(path.hash()?)
}
