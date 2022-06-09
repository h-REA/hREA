/**
 * Helpers for index host zomes (the zome modules which manage & expose the
 * index data for querying)
 *
 * @package hdk_semantic_indexes
 * @since   2021-09-30
 */
use chrono::{DateTime, NaiveDateTime, Utc};
use hdk::prelude::*;
use hdk_records::{
    DnaAddressable,
    identities::{
        calculate_identity_address,
        read_entry_identity,
    },
    links::{get_linked_addresses, walk_links_matching_entry},
    rpc::call_local_zome_method,
};
pub use hdk_records::{ RecordAPIResult, DataIntegrityError };
pub use hdk_semantic_indexes_zome_rpc::*;
pub use hdk_relay_pagination::PageInfo;
pub use hc_time_index::{
    IndexableEntry, SearchStrategy,
    index_entry,
    get_links_and_load_for_time_span,
};

// temporary: @see query_root_index()
pub const RECORD_GLOBAL_INDEX_LINK_TAG: &'static [u8] = b"all_entries";

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

/// Configuration attributes from indexing zomes which link to records in other zomes
#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
pub struct IndexingZomeConfig {
    // Index zome will call to the specified zome to retrieve records by identity hash.
    pub record_storage_zome: String,
}

//--------------------------------[ READ ]--------------------------------------

/// Reads and returns all entry identities referenced by the given index from
/// (`base_entry_type.base_address` via `link_tag`.
///
/// Use this method to query associated IDs for a query edge, without retrieving
/// the records themselves.
///
pub fn read_index<'a, O, A, S, I, E>(
    base_entry_type: &I,
    base_address: &A,
    link_tag: &S,
) -> RecordAPIResult<Vec<O>>
    where S: 'a + AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        O: DnaAddressable<EntryHash>,
        Entry: TryFrom<A, Error = E> + TryFrom<O, Error = E>,
        SerializedBytes: TryInto<O, Error = SerializedBytesError>,
        WasmError: From<E>,
{
    let index_address = calculate_identity_address(base_entry_type, base_address)?;
    let refd_index_addresses = get_linked_addresses(&index_address, LinkTag::new(link_tag.as_ref()))?;

    let (existing_link_results, read_errors): (Vec<RecordAPIResult<O>>, Vec<RecordAPIResult<O>>) = refd_index_addresses.iter()
        .map(read_entry_identity)
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
pub fn query_index<'a, T, O, C, F, A, S, I, J, E>(
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
        SerializedBytes: TryInto<C, Error = SerializedBytesError> + TryInto<O, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
        Entry: TryFrom<A, Error = E>,
        WasmError: From<E>,
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
pub fn query_root_index<'a, T, B, C, F, I>(
    zome_name_from_config: &'a F,
    method_name: &I,
    base_entry_type: &I,
    from_date: Option<DateTime<Utc>>,
    to_date: Option<DateTime<Utc>>,
    limit: Option<usize>,
) -> RecordAPIResult<Vec<RecordAPIResult<T>>>
    where T: serde::de::DeserializeOwned + std::fmt::Debug,
        B: DnaAddressable<EntryHash> + IndexableEntry,
        I: AsRef<str> + std::fmt::Display,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError> + TryInto<B, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    let linked_records: Vec<B> = get_links_and_load_for_time_span(
        base_entry_type.to_string(),
        if from_date.is_none() {
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc) // if none, start @ epoch
            } else { from_date.unwrap() },
        if to_date.is_none() {
                let now = sys_time()?.as_seconds_and_nanos();   // if none, end @ current time
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(now.0, now.1), Utc)
            } else { to_date.unwrap() },
        Some(LinkTag::new(RECORD_GLOBAL_INDEX_LINK_TAG)),
        if from_date.is_some() && to_date.is_some() {  SearchStrategy::Dfs } else {  SearchStrategy::Bfs },
        limit,
    )?;

    let read_single_record = retrieve_foreign_record::<T, B, _,_,_>(zome_name_from_config, method_name);

    Ok(linked_records.iter()
        .map(|link| {
            read_single_record(&link.target.to_owned().into())
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
        let address: B = read_entry_identity(addr)?;
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
pub fn sync_index<A, B, S, I, E>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    removed_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> OtherCellResult<RemoteEntryLinkResponse>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str> + std::fmt::Display + std::fmt::Display,
        A: DnaAddressable<EntryHash> + EntryDefRegistration + IndexableEntry,
        B: DnaAddressable<EntryHash> + EntryDefRegistration + IndexableEntry,
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    // create any new indexes
    let indexes_created = create_remote_index_destination(
        source_entry_type, source,
        dest_entry_type, dest_addresses,
        link_tag, link_tag_reciprocal,
    ).map_err(CrossCellError::from)?.iter()
        .map(convert_errors)
        .collect();

    // remove passed stale indexes
    let indexes_removed = remove_remote_index_links(
        source_entry_type, source,
        dest_entry_type, removed_addresses,
        link_tag, link_tag_reciprocal,
    ).map_err(CrossCellError::from)?.iter()
        .map(convert_errors)
        .collect();

    Ok(RemoteEntryLinkResponse { indexes_created, indexes_removed })
}

/// Given a type of entry, append the new entry to a sparsely-populated index
/// of all available entries. Ensures all DHT entries and link structures are present.
///
/// Returns the `HeaderHash` of the created `Entry`.
/// Note that there are other `Link`s between the returned `Link` and root
/// index entry for the record type, in the form of a date-based tree-like DHT structure.
///
pub fn append_to_root_index<'a, A, I, E>(
    base_entry_type: &I,
    initial_address: &A,
) -> RecordAPIResult<HeaderHash>
    where A: Clone + EntryDefRegistration + IndexableEntry,
        Entry: TryFrom<A, Error = E>,
        WasmError: From<E>,
        I: AsRef<str> + std::fmt::Display,
{
    // ensure the index pointer exists as its own node in the graph
    // :TODO: optimise to prevent duplicate writes
    let entry_header = create_entry(initial_address.to_owned())?;

    // populate a date-based index for the entry
    index_entry(base_entry_type.to_string(), initial_address.to_owned(), RECORD_GLOBAL_INDEX_LINK_TAG.to_vec())?;

    Ok(entry_header)
}

/// Creates a 'destination' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of an identity `Path` for the remote content and bidirectional
/// links between it and its `dest_addresses`.
///
fn create_remote_index_destination<A, B, S, I, E>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    dest_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str> + std::fmt::Display + std::fmt::Display,
        A: DnaAddressable<EntryHash> + EntryDefRegistration + IndexableEntry,
        B: DnaAddressable<EntryHash> + EntryDefRegistration + IndexableEntry,
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    // create a base entry pointer for the referenced origin record
    let _identity_hash = append_to_root_index::<A, I,_>(source_entry_type, source)
        .map_err(|e| { DataIntegrityError::LocalIndexNotConfigured(source_entry_type.to_string(), e.to_string()) })?;

    // link all referenced records to this pointer to the remote origin record
    Ok(dest_addresses.iter()
        .flat_map(create_dest_identities_and_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal))
        .collect()
    )
}

fn create_dest_identities_and_indexes<'a, A, B, S, I, E>(
    source_entry_type: &'a I,
    source: &'a A,
    dest_entry_type: &'a I,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&B) -> Vec<RecordAPIResult<HeaderHash>> + 'a>
    where I: AsRef<str> + std::fmt::Display,
        S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash> + EntryDefRegistration,
        B: 'a + DnaAddressable<EntryHash> + EntryDefRegistration + IndexableEntry,
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    let base_method = create_dest_indexes(source_entry_type, source, dest_entry_type, link_tag, link_tag_reciprocal);

    Box::new(move |dest| {
        match append_to_root_index(dest_entry_type, dest) {
            Ok(_link_hash) => {
                base_method(dest)
            },
            Err(e) => vec![Err(e)],
        }
    })
}

/// Helper for index update to add multiple destination links from some source.
fn create_dest_indexes<'a, A, B, S, I, E>(
    source_entry_type: &'a I,
    source: &'a A,
    dest_entry_type: &'a I,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&B) -> Vec<RecordAPIResult<HeaderHash>> + 'a>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    Box::new(move |dest| {
        match create_index(source_entry_type, source, dest_entry_type, dest, link_tag, link_tag_reciprocal) {
            Ok(created) => created,
            Err(_) => {
                let h: &EntryHash = dest.as_ref();
                vec![Err(DataIntegrityError::IndexNotFound(h.clone()))]
            },
        }
    })
}

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the `HeaderHash`es of the (respectively) forward & reciprocal links created.
fn create_index<A, B, S, I, E>(
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
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    let source_hash = calculate_identity_address(source_entry_type, source)?;
    let dest_hash = calculate_identity_address(dest_entry_type, dest)?;

    Ok(vec! [
        // :TODO: prevent duplicates- is there an efficient way to ensure a link of a given tag exists?
        Ok(create_link(source_hash.clone(), dest_hash.clone(), HdkLinkType::Any, LinkTag::new(link_tag.as_ref()))?),
        Ok(create_link(dest_hash, source_hash, HdkLinkType::Any, LinkTag::new(link_tag_reciprocal.as_ref()))?),
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
fn remove_remote_index_links<A, B, S, I, E>(
    source_entry_type: &I,
    source: &A,
    dest_entry_type: &I,
    remove_addresses: &[B],
    link_tag: &S,
    link_tag_reciprocal: &S,
) -> RecordAPIResult<Vec<RecordAPIResult<HeaderHash>>>
    where S: AsRef<[u8]> + ?Sized,
        I: AsRef<str>,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    Ok(remove_addresses.iter()
        .flat_map(delete_dest_indexes(
            source_entry_type, source,
            dest_entry_type,
            link_tag, link_tag_reciprocal,
        ))
        .collect()
    )
}

/// Helper for index update to remove multiple destination links from some source.
fn delete_dest_indexes<'a, A, B, S, I, E>(
    source_entry_type: &'a I,
    source: &'a A,
    dest_entry_type: &'a I,
    link_tag: &'a S,
    link_tag_reciprocal: &'a S,
) -> Box<dyn for<'r> Fn(&B) -> Vec<RecordAPIResult<HeaderHash>> + 'a>
    where I: AsRef<str>,
        S: 'a + AsRef<[u8]> + ?Sized,
        A: DnaAddressable<EntryHash>,
        B: DnaAddressable<EntryHash>,
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    Box::new(move |dest_addr| {
        match delete_index(source_entry_type, source, dest_entry_type, dest_addr, link_tag, link_tag_reciprocal) {
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
fn delete_index<'a, A, B, S, I, E>(
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
        Entry: TryFrom<A, Error = E> + TryFrom<B, Error = E>,
        WasmError: From<E>,
{
    let tag_source = LinkTag::new(link_tag.as_ref());
    let tag_dest = LinkTag::new(link_tag_reciprocal.as_ref());
    let address_source = calculate_identity_address(source_entry_type, source)?;
    let address_dest = calculate_identity_address(dest_entry_type, dest)?;

    let mut links = walk_links_matching_entry(
        &address_source,
        &address_dest,
        tag_source,
        delete_link_target_header,
    )?;
    links.append(& mut walk_links_matching_entry(
        &address_dest,
        &address_source,
        tag_dest,
        delete_link_target_header,
    )?);

    Ok(links)
}

/// Determine root `Path` for an entry type, can be used to anchor type-specific indexes & queries.
///
/// :TODO: upgrade to use date-ordered indexing #220
///
fn entry_type_root_path<S>(
    entry_type_path: S,
) -> Path
    where S: AsRef<str>,
{
    Path::from(vec![entry_type_path.as_ref().as_bytes().to_vec().into()])
}

//--------------------------[ UTILITIES  / INTERNALS ]---------------------

fn delete_link_target_header(l: &Link) -> RecordAPIResult<HeaderHash> {
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
fn convert_errors<E: Clone, F>(r: &Result<HeaderHash, E>) -> Result<HeaderHash, F>
    where F: From<E>,
{
    match r {
        Ok(header) => Ok(header.clone()),
        Err(e) => Err(F::from((*e).clone())),
    }
}
