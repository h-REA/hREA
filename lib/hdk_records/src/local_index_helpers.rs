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
    internals::*,
    identity_helpers::{
        calculate_identity_address,
        read_entry_identity_full,
        entry_type_root_path,
    },
    links::{
        get_linked_headers,
        get_linked_addresses,
    },
    records::{
        read_record_entry_by_identity,
    },
    index_retrieval_helpers::retrieve_foreign_records,
};

//--------------------------------[ READ ]--------------------------------------

/// Reads and returns all entry identities referenced by the given index from
/// (`base_entry_type.base_address` via `link_tag`.
///
/// Use this method to query associated IDs for a query edge, without retrieving
/// the records themselves.
///
pub fn read_index<'a, O, A, S, I>(
    base_entry_type: &I,
    base_address: &A,
    link_tag: &S,
) -> RecordAPIResult<Vec<O>>
    where S: 'a + AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        O: DnaAddressable<EntryHash>,
{
    let index_address = calculate_identity_address(base_entry_type, base_address)?;
    let refd_index_addresses = get_linked_addresses(&index_address, LinkTag::new(link_tag.as_ref()))?;

    let (existing_link_results, read_errors): (Vec<RecordAPIResult<O>>, Vec<RecordAPIResult<O>>) = refd_index_addresses.iter()
        .map(read_entry_identity_full)
        .partition(Result::is_ok);

    // :TODO: this might have some issues as it presumes integrity of the DHT; needs investigating
    throw_any_error(read_errors)?;

    Ok(existing_link_results.iter().cloned()
        .map(Result::unwrap)
        .collect())
}

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

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the `HeaderHash`es of the (respectively) forward & reciprocal links created.
pub fn create_index<A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest: &B,
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where I: AsRef<str>,
        S: AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let source_hash = calculate_identity_address(source_entry_type, source)?;
    let dest_hash = calculate_identity_address(dest_entry_type, dest)?;

    Ok(vec! [
        Ok(create_link(source_hash.clone(), dest_hash.clone(), LinkTag::new(link_tag.as_ref()))?),
        Ok(create_link(dest_hash, source_hash, LinkTag::new(link_tag_reciprocal.as_ref()))?),
    ])
}

//-------------------------------[ UPDATE ]-------------------------------------

/// Updates an index set from some originating entry located at the Path `source_entry_type`.`source`.
///
/// The destination entry is stored at the Path prefix `dest_entry_type`. For this prefix, any entry
/// identifiers in `add_dest_addresses` which are not already linked will have indexes created.
///
/// Any indexes which are already present between the source and addresses in `remove_dest_addresses`
/// will be removed.
///
/// An update for a single entry is thus performed by specifiying the previous entry ID in
/// `remove_dest_addresses`, and the new entry ID in `add_dest_addresses`.
///
pub fn update_index<'a, A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    link_tag: &S,
    link_tag_reciprocal: &S,
    add_dest_addresses: &[B],
    remove_dest_addresses: &[B],
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // load any existing linked entries from the originating address
    let existing_links: Vec<B> = read_index(source_entry_type, source, link_tag)?;

    // determine links to erase (those being removed but not also added)
    let to_erase: Vec<B> = existing_links
        .iter()
        .filter(link_pair_matches(&vect_difference(&remove_dest_addresses.to_vec(), &add_dest_addresses.to_vec())))
        .cloned()
        .collect();

    // wipe any indexes flagged for removal
    let delete_index_results: Vec<RecordAPIResult<HeaderHash>> = to_erase
        .iter()
        .flat_map(delete_dest_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal))
        .collect();

    // check which inserts are needed (those being added not already present)
    let already_present: Vec<B> = existing_links
        .iter()
        .filter(link_pair_matches(add_dest_addresses))
        .cloned()
        .collect();

    let to_add = vect_difference(&add_dest_addresses.to_vec(), &already_present);

    // add any new links not already present
    let create_index_results: Vec<RecordAPIResult<HeaderHash>> = to_add
        .iter()
        .flat_map(create_dest_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal))
        .collect();

    Ok(delete_index_results
        .iter().cloned().chain(
            create_index_results.iter().cloned()
        ).collect()
    )
}

//-------------------------------[ DELETE ]-------------------------------------

/// Deletes a bidirectional link between two entry addresses. Any active links between
/// the given addresses using the given tags will be deleted.
///
/// :TODO: this should probably only delete the referenced IDs, at the moment it clears anything matching tags.
///
pub fn delete_index<'a, A, B, S, I>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest: &B,
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let tag_source = LinkTag::new(link_tag.as_ref());
    let tag_dest = LinkTag::new(link_tag_reciprocal.as_ref());
    let address_source = calculate_identity_address(source_entry_type, source)?;
    let address_dest = calculate_identity_address(dest_entry_type, dest)?;

    let mut links = get_linked_headers(&address_source, tag_source)?;
    links.append(& mut get_linked_headers(&address_dest, tag_dest)?);

    Ok(links
        .iter().cloned()
        .map(|l| { Ok(delete_link(l)?) })
        .collect()
    )
}
