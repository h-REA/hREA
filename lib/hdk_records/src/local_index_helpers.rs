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
    RevisionHash, DnaAddressable,
    RecordAPIResult,
    record_interface::Identified,
    identity_helpers::{
        calculate_identity_address,
        // read_entry_identity_full,
        entry_type_root_path,
    },
    links::{
        // get_linked_headers,
        get_linked_addresses,
    },
    records::{
        read_record_entry_by_identity,
    },
    index_retrieval_helpers::retrieve_foreign_records,
};

/// Given a base address to query from, returns a Vec of tuples of all target
/// `EntryHash`es referenced via the given link tag, bound to the result of
/// attempting to decode each referenced entry into the requested type `R`.
///
/// Use this method to query associated records for a query edge in full.
///
pub fn query_index<'a, T, O, C, F, A, S, I, J>(
    base_entry_type: &I,
    base_address: &A,
    link_tag: &S,
    foreign_zome_name_from_config: &F,
    foreign_read_method_name: &J,
) -> RecordAPIResult<Vec<RecordAPIResult<T>>>
    where I: AsRef<str>,
        J: AsRef<str>,
        S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        O: DnaAddressable<EntryHash>,
        T: serde::de::DeserializeOwned + std::fmt::Debug,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    let index_address = calculate_identity_address(base_entry_type, base_address)?;
    let addrs_result = get_linked_addresses(&index_address, LinkTag::new(link_tag.as_ref()))?;
    let entries = retrieve_foreign_records::<T, O, C, F, J>(
        foreign_zome_name_from_config,
        foreign_read_method_name,
        &addrs_result,
    );
    Ok(entries)
}

/// Given a type of entry, returns a Vec of *all* records of that entry registered
/// internally with the DHT.
///
/// :TODO: sharding strategy for 2-nth order link destinations
///
pub fn query_root_index<'a, T, R, O, I: AsRef<str>>(
    base_entry_type: &I,
) -> RecordAPIResult<Vec<RecordAPIResult<(RevisionHash, O, T)>>>
    where T: std::fmt::Debug,
        O: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<R, Error = SerializedBytesError>,
        Entry: TryFrom<R>,
        R: std::fmt::Debug + Identified<T, O>,
{
    let index_path = entry_type_root_path(base_entry_type);
    let linked_records: Vec<Link> = index_path.children()?.into();

    Ok(linked_records.iter()
        .map(|link| { read_record_entry_by_identity(&link.target) })
        .collect())
}

//-------------------------------[ CREATE ]-------------------------------------

//-------------------------------[ DELETE ]-------------------------------------
