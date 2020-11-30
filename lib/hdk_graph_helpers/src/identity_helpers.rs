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
fn identity_path_for<S, A>(
    entry_type_root_path: S,
    base_address: A,
) -> Path
    where S: AsRef<str>,
        A: AsRef<EntryHash>,
{
    let addr = base_address.as_ref().clone();
    let c1: Component = entry_type_root_path.as_ref().as_bytes().to_vec().into();
    let c2: Component = addr.into_inner().into();
    Path::from(vec![c1, c2])
}

//--------------------------------[ READ ]--------------------------------------

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
pub (crate) fn calculate_identity_address<S, A>(
    entry_type_root_path: S,
    base_address: A,
) -> GraphAPIResult<EntryHash>
    where S: AsRef<str>,
        A: AsRef<EntryHash>,
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
pub (crate) fn create_entry_identity<'a, S, A>(
    entry_type_root_path: S,
    initial_address: A,
) -> GraphAPIResult<EntryHash>
    where S: AsRef<str>,
        A: AsRef<EntryHash>,
{
    let path = identity_path_for(entry_type_root_path, initial_address);
    path.ensure()?;
    Ok(path.hash()?)
}
