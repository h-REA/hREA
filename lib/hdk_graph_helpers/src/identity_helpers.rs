/**
 * Helpers related to uniquely idenfying entry data, such that different entries
 * can be referenced by consistent identities that do not change over time.
 *
 * @see     super::record_interface::Identified
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use hdk3::prelude::*;
use hdk3::hash_path::path::Component;

use crate::{
    record_interface::Identified,
    GraphAPIResult,
};

/// Represent `key index` record identities using native Holochain `Path` construct
///
/// :TODO: optimise to remove need for `Clone` trait in this method and dependants.
///
fn get_identity_path<S, A>(
    entry_type_root_path: &S,
    base_address: &A,
) -> Path
    where S: Clone + Into<String>,
        A: Clone + Into<EntryHash>,
{
    let c1: Component = (*entry_type_root_path).clone().into().as_bytes().to_vec().into();
    let c2: Component = (*base_address).clone().into().into_inner().into();
    Path::from(vec![c1, c2])
}

//--------------------------------[ READ ]--------------------------------------

/// Retrieve the identity entry address from a given `Identified` entry
///
pub (crate) fn get_identity_address<'a, T, A>(identified_entry: &'a A) -> GraphAPIResult<&'a EntryHash>
    where A: Identified<T>,
{
    identified_entry.identity()
}

/// Determine the underlying `EntryHash` for a given `base_address` identifier, without querying the DHT.
///
pub (crate) fn calculate_identity_address<S, A>(
    entry_type_root_path: &S,
    base_address: &A,
) -> GraphAPIResult<EntryHash>
    where S: Clone + Into<String>,
        A: Clone + Into<EntryHash>,
{
    Ok(get_identity_path(entry_type_root_path, base_address).hash()?)
}

//-------------------------------[ CREATE ]-------------------------------------

/// Creates a `Path` to initialise a "base anchor" for a new entry, and returns
/// the `EntryHash` of the new `Path`.
///
/// This `Path` is intended to be used as an anchor to base links to/from the
/// entry onto.
///
pub (crate) fn create_entry_identity<'a, S, A>(
    entry_type_root_path: &S,
    initial_address: &A,
) -> GraphAPIResult<EntryHash>
    where S: Clone + Into<String>,
        A: Clone + Into<EntryHash>,
{
    let path = get_identity_path(entry_type_root_path, initial_address);
    path.ensure()?;
    Ok(path.hash()?)
}
