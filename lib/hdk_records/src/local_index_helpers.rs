/**
 * Helpers related to `local indexes`.
 *
 * A `local index` is a simple set of links between Holochain entries. These are
 * appropriate for linking directly between entries within the same DNA or in
 * remote DNAs, as identities are treated as tuples of `(DnaHash, EntryHash)`.
 *
 * @see     ../README.md
 * @package HDK Graph Helpers
 * @since   2019-05-16
 */
use hdk::prelude::*;

use crate::{
    HeaderHash, DnaAddressable,
    RecordAPIResult,
    record_interface::Identified,
    identity_helpers::{
        entry_type_root_path,
    },
    records::{
        read_record_entry_by_identity,
    },
};

/// Given a type of entry, returns a Vec of *all* records of that entry registered
/// internally with the DHT.
///
/// :TODO: sharding strategy for 2-nth order link destinations
///
pub fn query_root_index<'a, T, R, O, I: AsRef<str>>(
    base_entry_type: &I,
) -> RecordAPIResult<Vec<RecordAPIResult<(HeaderHash, O, T)>>>
    where T: std::fmt::Debug,
        O: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError> + TryInto<O, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, O>,
{
    let index_path = entry_type_root_path(base_entry_type);
    let linked_records: Vec<Link> = index_path.children()?.into();

    Ok(linked_records.iter()
        .map(|link| { read_record_entry_by_identity(&link.target) })
        .collect())
}
