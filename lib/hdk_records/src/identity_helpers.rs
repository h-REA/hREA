/**
 * Helpers related to uniquely idenfying entry data, such that different entries
 * can be referenced by consistent identities that do not change over time.
 *
 * This also implicitly manages an unordered sparse index to all publicly created
 * records across the shared DHT.
 *
 * :TODO: paths should be determined by initial `HeaderHash` to ensure uniqueness,
 *        rather than relying on consumer to inject random bytes or timestamps.
 *
 * @see     crate::record_interface::Identified::identity()
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use hdk::prelude::*;
use hdk::hash_path::path::Component;

use crate::{
    RecordAPIResult,
    DataIntegrityError,
    link_helpers::get_linked_addresses,
};

/// Represent `key index` record identities using native Holochain `Path` construct
///
/// :TODO: sharding strategy for `c2`
///
fn identity_path_for<S>(
    entry_type_root_path: S,
    base_address: &EntryHash,
) -> Path
    where S: AsRef<str>,
{
    let c1: Component = entry_type_root_path.as_ref().as_bytes().to_vec().into();
    let c2: Component = base_address.as_ref().to_vec().into();
    Path::from(vec![c1, c2])
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
pub (crate) fn calculate_identity_address<S>(
    entry_type_root_path: S,
    base_address: &EntryHash,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
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

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a `Path` to initialise a unique index for a new entry, and returns
/// the `EntryHash` of the new `Path`.
///
/// This `Path` is intended to be used as an anchor to base links to/from the
/// entry onto.
///
pub (crate) fn create_entry_identity<S>(
    entry_type_root_path: S,
    initial_address: &EntryHash,
) -> RecordAPIResult<EntryHash>
    where S: AsRef<str>,
{
    let path = identity_path_for(entry_type_root_path, initial_address);
    path.ensure()?;
    Ok(path.hash()?)
}
