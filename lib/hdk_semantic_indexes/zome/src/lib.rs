/**
 * Helpers for index host zomes (the zome modules which manage & expose the
 * index data for querying)
 *
 * @package hdk_semantic_indexes
 * @since   2021-09-30
 */
use chrono::{DateTime, Utc};
use hdk::prelude::*;
use holo_hash::{DnaHash, HOLO_HASH_FULL_LEN};
use hdk_records::{
    identities::calculate_identity_address,
    rpc::call_local_zome_method,
};
use hdk_time_indexing::{ index_entry };
pub use hdk_time_indexing::{
    TimeIndex,
    read_all_entry_hashes,
    // get_latest_entry_hashes,
    // get_older_entry_hashes,
    sort_entries_by_time_index,
};
pub use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    DnaAddressable,
};
pub use hdk_semantic_indexes_zome_rpc::*;
pub use hdk_relay_pagination::PageInfo;
pub use hdk_semantic_indexes_integrity::LinkTypes;

// temporary: @see query_root_index()
pub const RECORD_GLOBAL_INDEX_LINK_TAG: &'static [u8] = b"all_entries";

pub const RECORD_IDENTITY_LINK_TAG: &'static [u8] = b"id|"; // :WARNING: byte length is important here. @see read_remote_entry_identity

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

/// Configuration attributes from indexing zomes which link to records in other zomes
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
pub struct IndexingZomeConfig {
    // Index zome will call to the specified zome to retrieve records by identity hash.
    pub record_storage_zome: String,
}

//--------------------------------[ READ ]--------------------------------------

/// Reads and returns all entry identities referenced by the given index from
/// (`base_entry_type.base_address` via `link_tag`.
///
/// The returned identities are sorted in reverse creation order.
///
/// Use this method to query associated IDs for a query edge, without retrieving
/// the records themselves.
///
pub fn read_index<'a, O, A, S, I>(
    base_address: &A,
    link_tag: &S,
    order_by_time_index: &I,
) -> RecordAPIResult<Vec<O>>
    where S: 'a + AsRef<[u8]> + ?Sized + std::fmt::Debug,
        I: AsRef<str> + std::fmt::Debug,
        A: DnaAddressable<EntryHash>,
        O: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<O, Error = SerializedBytesError>,
{
    let index_address = calculate_identity_address(base_address)?;
    let mut refd_index_addresses = get_linked_addresses(&index_address, LinkTag::new(link_tag.as_ref()))?;
    refd_index_addresses.sort_by(sort_entries_by_time_index(order_by_time_index));

    let (existing_link_results, read_errors): (Vec<RecordAPIResult<O>>, Vec<RecordAPIResult<O>>) = refd_index_addresses.iter()
        .map(read_remote_entry_identity)
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
    base_address: &A,
    link_tag: &S,
    order_by_time_index: &I,
    foreign_zome_name_from_config: &F,
    foreign_read_method_name: &J,
) -> RecordAPIResult<Vec<RecordAPIResult<T>>>
    where I: AsRef<str> + std::fmt::Debug,
        J: AsRef<str>,
        S: 'a + AsRef<[u8]> + ?Sized + std::fmt::Debug,
        A: DnaAddressable<EntryHash>,
        O: DnaAddressable<EntryHash>,
        T: serde::de::DeserializeOwned + std::fmt::Debug,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError> + TryInto<O, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    let index_address = calculate_identity_address(base_address)?;
    let mut addrs_result = get_linked_addresses(&index_address, LinkTag::new(link_tag.as_ref()))?;
    addrs_result.sort_by(sort_entries_by_time_index(order_by_time_index));

    let entries = retrieve_foreign_records::<T, O, C, F, J>(
        foreign_zome_name_from_config,
        foreign_read_method_name,
        &addrs_result,
    );
    Ok(entries)
}

/// Query foreign entries pointers from a time-ordered index, in order from most recent to oldest.
///
/// If `start_from` is provided, the given `EntryHash` is used to determine the starting location
/// for reading results. Otherwise the newest entries (as determined by their ordering in the time
/// index) are returned.
///
/// Full entry data is returned by querying from the associated record storage zome determined by
/// `zome_name_from_config` and `read_method_name`.
///
pub fn query_time_index<'a, T, B, C, F, I>(
    zome_name_from_config: &'a F,
    read_method_name: &I,
    index_name: &I,
    _start_from: Option<EntryHash>,
    _limit: usize,
) -> RecordAPIResult<Vec<RecordAPIResult<T>>>
    where T: serde::de::DeserializeOwned + std::fmt::Debug,
        B: DnaAddressable<EntryHash> + TryFrom<SerializedBytes, Error = SerializedBytesError>,
        I: AsRef<str> + std::fmt::Display + std::fmt::Debug,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    // this algorithm is the 'make it work' current pass, pending the full implementation mentioned
    // in the TODO below, regarding efficiency and completeness
    let linked_records = read_all_entry_hashes(index_name)
        .map_err(|e| { DataIntegrityError::BadTimeIndexError(e.to_string()) })?;

    // :TODO: efficient paginated retrieval
    // let linked_records = match start_from {
    //     None => get_latest_entry_hashes(index_name, limit),
    //     Some(cursor) => get_older_entry_hashes(index_name, cursor, limit),
    // }.map_err(|e| { DataIntegrityError::BadTimeIndexError(e.to_string()) })?;

    let read_single_record = retrieve_foreign_record::<T, B, _,_,_>(zome_name_from_config, read_method_name);

    Ok(linked_records.iter()
        .map(|addr| {
            // query full record from the associated CRUD zome
            read_single_record(addr)
        })
        .collect())
}

/// Fetches all referenced record entries found corresponding to the input
/// identity addresses.
///
/// Useful in loading the results of indexed data, where indexes link identity `Path`s for different records.
///
fn retrieve_foreign_records<'a, T, B, C, F, S>(
    zome_name_from_config: &'a F,
    method_name: &S,
    addresses: &'a Vec<EntryHash>,
) -> Vec<RecordAPIResult<T>>
    where S: AsRef<str>,
        T: serde::de::DeserializeOwned + std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    let read_single_record = retrieve_foreign_record::<T, B, _,_,_>(zome_name_from_config, &method_name);

    addresses.iter()
        .map(read_single_record)
        .collect()
}

fn retrieve_foreign_record<'a, T, B, C, F, S>(
    zome_name_from_config: &'a F,
    method_name: &'a S,
) -> impl Fn(&EntryHash) -> RecordAPIResult<T> + 'a
    where S: AsRef<str>,
        T: serde::de::DeserializeOwned + std::fmt::Debug,
        B: DnaAddressable<EntryHash>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |addr| {
        let address: B = read_remote_entry_identity(addr)?;
        let entry_res: T = call_local_zome_method(zome_name_from_config.to_owned(), method_name, ByAddress { address })?;
        Ok(entry_res)
    }
}

//--------------------------------[ UPDATE ]--------------------------------------

/// Respond to a request from a remote source to build a 'destination' link index for some externally linking content.
///
/// This essentially ensures an identity `Path` for the remote `source` and then links it to every
/// `dest_addresses` found locally within this DNA before removing any links to `removed_addresses`.
///
/// The returned `RemoteEntryLinkResponse` provides an appropriate format for responding to indexing
/// requests that originate from calls to `create/update/delete_remote_index` in a foreign DNA.
///
pub fn sync_index<A, B, S, I>(
    source: &A,
    dest_addresses: &[B],
    removed_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
    order_by_time_index: &I,
) -> OtherCellResult<RemoteEntryLinkResponse>
    where S: AsRef<[u8]> + ?Sized + std::fmt::Debug,
        I: AsRef<str> + std::fmt::Display + std::fmt::Debug,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // create any new indexes
    let indexes_created = create_remote_index_destination(
        source, dest_addresses, link_tag, link_tag_reciprocal,
    ).map_err(CrossCellError::from)?.iter()
        .map(convert_errors)
        .collect();

    // :SHONK: add remote source address to its own time series for retrieval.
    // This should be updated to be determined and passed with the request, so that the
    // query time is based on (externally determined) record creation time, rather
    // then "indexed" time, which isn't really useful as it doesn't even correlate with
    // record updates. (Indexes only change if the indexed field is updated.)
    let timestamp: DateTime<Utc> = sys_time()?.try_into()
        .map_err(|e: TimestampError| DataIntegrityError::BadTimeIndexError(e.to_string()))?;
    let time_index_created = append_to_time_index(order_by_time_index, source, timestamp);
    // :TODO: handle errors
    debug!("created {:?} time indexes in {:?} index zome for remote {:?} index target {:?}", order_by_time_index, zome_info()?.name, link_tag, time_index_created);

    // remove passed stale indexes
    let indexes_removed = remove_remote_index_links(
        source, removed_addresses, link_tag, link_tag_reciprocal,
    ).map_err(CrossCellError::from)?.iter()
        .map(convert_errors)
        .collect();

    Ok(RemoteEntryLinkResponse { indexes_created, indexes_removed })
}

/// Indexes an entry pointer (which may reference the local DNA, or a remote one)
/// into the time-ordered index `index_name` at the given `timestamp` for subsequent
/// ordered retrieval.
///
/// Multiple indexes may be created per entry, where multiple orderings are appropriate.
///
pub fn append_to_time_index<'a, A, I>(
    index_name: &I,
    entry_address: &A,
    timestamp: DateTime<Utc>,
) -> RecordAPIResult<()>
    where A: DnaAddressable<EntryHash>,
        I: AsRef<str> + std::fmt::Display,
{
    // determine hash for index pointer
    let entry_hash: &EntryHash = entry_address.as_ref();

    // store fully-qualified target identifier in a loopback link
    create_id_tag(entry_address)?;

    // populate a date-based index for the entry
    index_entry(index_name, entry_hash.to_owned(), timestamp)
        .map_err(|e| { DataIntegrityError::BadTimeIndexError(e.to_string()) })?;

    Ok(())
}

/// Creates a 'destination' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of an identity `Path` for the remote content and bidirectional
/// links between it and its `dest_addresses`.
///
fn create_remote_index_destination<A, B, S>(
    source: &A,
    dest_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<ActionHash>>>
    where S: AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    // link all referenced records to this pointer to the remote origin record
    Ok(dest_addresses.iter()
        .flat_map(create_dest_indexes(source, link_tag, link_tag_reciprocal))
        .collect()
    )
}

/// Helper for index update to add multiple destination links from some source.
fn create_dest_indexes<'a, A, B, S>(
    source: &'a A,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&B) -> Vec<RecordAPIResult<ActionHash>> + 'a>
    where S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    Box::new(move |dest| {
        match create_index(source, dest, link_tag, link_tag_reciprocal) {
            Ok(created) => created,
            Err(_) => {
                let h: &EntryHash = dest.as_ref();
                vec![Err(DataIntegrityError::IndexNotFound(h.clone()))]
            },
        }
    })
}

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the `ActionHash`es of the (respectively) forward & reciprocal links created.
fn create_index<A, B, S>(
    source: &A,
    dest: &B,
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<ActionHash>>>
    where S: AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let source_hash = calculate_identity_address(source)?;
    let dest_hash = calculate_identity_address(dest)?;

    Ok(vec! [
        // :TODO: prevent duplicates- is there an efficient way to ensure a link of a given tag exists?
        Ok(create_link(source_hash.clone(), dest_hash.clone(), LinkTypes::SemanticIndex, LinkTag::new(link_tag.as_ref()))?),
        Ok(create_link(dest_hash, source_hash, LinkTypes::SemanticIndex, LinkTag::new(link_tag_reciprocal.as_ref()))?),
    ])
}

//-------------------------------[ DELETE ]-------------------------------------

/// Deletes a set of links between a remote record reference and some set
/// of local target EntryHashes.
///
/// The `Path` representing the remote target is not
/// affected in the removal, and is simply left dangling in the
/// DHT space as an indicator of previously linked items.
///
fn remove_remote_index_links<A, B, S>(
    source: &A,
    remove_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<ActionHash>>>
    where S: AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    Ok(remove_addresses.iter()
        .flat_map(delete_dest_indexes(
            source, link_tag, link_tag_reciprocal,
        ))
        .collect()
    )
}

/// Helper for index update to remove multiple destination links from some source.
fn delete_dest_indexes<'a, A, B, S>(
    source: &'a A,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&B) -> Vec<RecordAPIResult<ActionHash>> + 'a>
    where S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    Box::new(move |dest_addr| {
        match delete_index(source, dest_addr, link_tag, link_tag_reciprocal) {
            Ok(deleted) => deleted,
            Err(_) => {
                let dest_hash: &EntryHash = dest_addr.as_ref();
                vec![Err(DataIntegrityError::IndexNotFound(dest_hash.clone()))]
            },
        }
    })
}

/// Deletes a bidirectional link between two entry addresses. Any active links between
/// the given addresses using the given tags will be deleted.
///
fn delete_index<'a, A, B, S>(
    source: &A,
    dest: &B,
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<ActionHash>>>
    where S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
{
    let tag_source = LinkTag::new(link_tag.as_ref());
    let tag_dest = LinkTag::new(link_tag_reciprocal.as_ref());
    let address_source = calculate_identity_address(source)?;
    let address_dest = calculate_identity_address(dest)?;

    let mut links = walk_links_matching_entry(
        &address_source,
        &address_dest,
        tag_source,
        delete_link_target_action,
    )?;
    links.append(& mut walk_links_matching_entry(
        &address_dest,
        &address_source,
        tag_dest,
        delete_link_target_action,
    )?);

    Ok(links)
}

//--------------------------[ UTILITIES  / INTERNALS ]---------------------

/// Generate a link tag for the identity anchor of a record by encoding the ID string into the tag
/// so that it can be retreived by querying the DHT later.
///
fn ensure_id_tag<A>(ident: &A) -> RecordAPIResult<Option<ActionHash>>
    where A: DnaAddressable<EntryHash>,
{
    let dna: &DnaHash = ident.as_ref();
    let hash: &EntryHash = ident.as_ref();
    let id_tag = LinkTag::new([crate::RECORD_IDENTITY_LINK_TAG, dna.as_ref(), hash.as_ref()].concat());

    link_if_not_linked(hash.to_owned(), hash.to_owned(), LinkTypes::IdentityAnchor, id_tag)
}

fn link_if_not_linked(
    origin_hash: EntryHash,
    dest_hash: EntryHash,
    link_type: LinkTypes,
    link_tag: LinkTag,
) -> RecordAPIResult<Option<ActionHash>> {
    if false == get_links(origin_hash.to_owned(), link_type, Some(link_tag.to_owned()))?
        .iter().any(|l| { EntryHash::from(l.target.to_owned()) == dest_hash })
    {
        Ok(Some(create_link(
            origin_hash.to_owned(),
            dest_hash.to_owned(),
            LinkTypes::IdentityAnchor,
            link_tag,
        )?))
    } else {
        Ok(None)
    }
}

/// Given an identity `EntryHash` (ie. the result of `calculate_identity_address`),
/// query the `DnaHash` and `AnyDhtHash` of the record.
///
fn read_remote_entry_identity<A>(
    identity_address: &EntryHash,
) -> RecordAPIResult<A>
    where A: DnaAddressable<EntryHash>,
        SerializedBytes: TryInto<A, Error = SerializedBytesError>,
{
    get_links(
        identity_address.to_owned(),
        LinkTypes::IdentityAnchor,
        Some(LinkTag::new(crate::RECORD_IDENTITY_LINK_TAG))
    )?
    .first()
    .map(|link| {
        let bytes = &link.tag.to_owned().into_inner()[3..];
        Ok(A::new(
            DnaHash::from_raw_39(bytes[0..HOLO_HASH_FULL_LEN].to_vec())
                .map_err(|e| DataIntegrityError::CorruptIndexError(identity_address.to_owned(), e.to_string()))?,
            EntryHash::from_raw_39(bytes[HOLO_HASH_FULL_LEN..].to_vec())
                .map_err(|e| DataIntegrityError::CorruptIndexError(identity_address.to_owned(), e.to_string()))?,
        ))
    })
    .ok_or(DataIntegrityError::IndexNotFound((*identity_address).clone()))?
}

/// Load any set of linked `EntryHash`es being referenced from the
/// provided `base_address` with the given `link_tag`.
///
pub fn get_linked_addresses(
    base_address: &EntryHash,
    link_tag: LinkTag,
) -> RecordAPIResult<Vec<EntryHash>> {
    Ok(
        get_links((*base_address).clone(), LinkTypes::SemanticIndex, Some(link_tag))?
            .iter()
            .map(|l| l.target.to_owned().into())
            .collect()
    )
}

/// Execute the provided `link_map` function against the set of links
/// between a `base_address` and `target_address` via the given `link_tag`.
///
/// If you have a bidirectional link between two `EntryHash`es, you must
/// run this method twice (once to remove each direction of the paired link).
///
fn walk_links_matching_entry<T, F>(
    base_address: &EntryHash,
    target_address: &EntryHash,
    link_tag: LinkTag,
    link_map: F,
) -> RecordAPIResult<Vec<T>>
    where F: Fn(&Link) -> T,
{
    let links_result = get_links(base_address.to_owned(), LinkTypes::SemanticIndex, Some(link_tag))?;

    Ok(links_result
        .iter()
        .filter(|l| { EntryHash::from(l.target.clone()) == *target_address })
        .map(link_map)
        .collect()
    )
}

fn delete_link_target_action(l: &Link) -> RecordAPIResult<ActionHash> {
    Ok(delete_link(l.create_link_hash.to_owned())?)
}

/// Returns the first error encountered (if any). Best used with the `?` operator.
fn throw_any_error<T>(mut errors: Vec<RecordAPIResult<T>>) -> RecordAPIResult<()> {
    if errors.len() == 0 {
        return Ok(());
    }
    let first_err = errors.pop().unwrap();
    Err(first_err.err().unwrap())
}

/// Convert internal zome errors into externally encodable type for response
fn convert_errors<E: Clone, F>(r: &Result<ActionHash, E>) -> Result<ActionHash, F>
    where F: From<E>,
{
    match r {
        Ok(action) => Ok(action.clone()),
        Err(e) => Err(F::from((*e).clone())),
    }
}
