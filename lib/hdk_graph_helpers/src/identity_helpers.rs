/**
 * Helpers related to uniquely idenfying entry data, such that different entries
 * can be referenced by consistent identities that do not change over time.
 *
 * :TODO: paths should be determined by initial `HeaderHash` to ensure uniqueness,
 *        rather than relying on consumer to inject random bytes or timestamps.
 *
 * @see     crate::record_interface::Identified::identity()
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use hdk3::prelude::*;
use hdk3::hash_path::path::Component;

use crate::{
    GraphAPIResult,
};

/// Represent `key index` record identities using native Holochain `Path` construct
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

//--------------------------------[ READ ]--------------------------------------

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
pub (crate) fn calculate_identity_address<S>(
    entry_type_root_path: S,
    base_address: &EntryHash,
) -> GraphAPIResult<EntryHash>
    where S: AsRef<str>,
{
    Ok(identity_path_for(entry_type_root_path, base_address).hash()?)
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
) -> GraphAPIResult<EntryHash>
    where S: AsRef<str>,
{
    let path = identity_path_for(entry_type_root_path, initial_address);
    path.ensure()?;
    Ok(path.hash()?)
}
