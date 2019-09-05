/**
 * Link-handling abstractions for Holochain apps
 *
 * Handles common behaviours for linking between data in different hApps,
 * in a way that is predictable, semantically meaningful and easy to reason about.
 *
 * @package HoloREA
 * @since   2019-07-03
 */

use std::borrow::Cow;
use std::convert::{ TryInto, TryFrom };
use hdk::{
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::{
        entry::Entry::App as AppEntry,
        entry::AppEntryValue,
        entry::entry_type::AppEntryType,
        link::LinkMatch,
        link::LinkMatch::Exactly,
    },
    holochain_wasm_utils::api_serialization::get_links::GetLinksOptions,
    error::{ ZomeApiError, ZomeApiResult },
    link_entries,
    get_entry,
    call,
    get_links,
    get_links_with_options,
};
use holochain_json_derive::{ DefaultJson };

use super::{
    records::{
        create_base_entry,
    },
};

/// Toplevel method for triggering a link creation flow between two records in
/// different DNAs. The calling DNA will have a 'local query index' created for
/// fetching the referenced remote IDs; the destination DNA will have a 'remote
/// query index' created for querying the referenced records in full.
pub fn create_remote_index_pair(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    remote_base_entry_type: &str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>> {
    let mut local_results = create_local_query_index(
        remote_base_entry_type,
        origin_relationship_link_type,
        origin_relationship_link_tag,
        destination_relationship_link_type,
        destination_relationship_link_tag,
        source_base_address,
        target_base_addresses,
    )?;

    let mut remote_results = request_remote_query_index(
        remote_dna_id,
        remote_zome_id,
        remote_zome_method,
        remote_request_cap_token,
        source_base_address,
        target_base_addresses,
    )?;

    local_results.append(&mut remote_results);
    Ok(local_results)
}

/// Creates a 'remote' query index used for following a link from some external record
/// into records contained within the current DNA / zome.
///
/// This basically consists of a `base` address for the remote content and bidirectional
/// links between it and its target_base_addresses.
pub fn create_remote_query_index<T>(
    remote_base_entry_type: T,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>>
    where T: Into<AppEntryType>,
{
    // create a base entry pointer for the referenced origin record
    let base_address = create_base_entry(&(remote_base_entry_type.into()), &source_base_address).unwrap();

    // link all referenced records to our pointer to the remote origin record
    let results = target_base_addresses.iter()
        .map(|target_address| {
            // link origin record to local records by specified edge
            link_entries_bidir(
                &base_address, &target_address,
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            target_address.to_owned()
        })
        .collect();

    Ok(results)
}

/// Creates a 'local' query index used for fetching and querying pointers to other
/// records that are stored externally to this DNA / zome.
///
/// In the local DNA, this consists of `base` addresses for all referenced foreign
/// content, bidirectionally linked to the originating record for querying in either direction.
///
/// In the remote DNA, a corresponding remote query index is built via `create_remote_query_index`,
/// which is presumed to be linked to the other end of the specified `remote_zome_method`.
pub fn create_local_query_index(
    remote_base_entry_type: &str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>> {
    // abort if target_base_addresses are empty
    if target_base_addresses.len() == 0 { return Ok(vec![]) }

    // Build local index first (for reading linked record IDs from the `source_base_address`)
    let results: Vec<Address> = target_base_addresses.iter()
        .map(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_address = create_base_entry(&(remote_base_entry_type.to_string().into()), base_entry_addr).unwrap();
            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(
                &source_base_address, &base_address,
                origin_relationship_link_type, origin_relationship_link_tag,
                destination_relationship_link_type, destination_relationship_link_tag
            );

            base_entry_addr.to_owned()
        })
        .collect();

    Ok(results)
}

/// Ask another bridged DNA or zome to build a 'remote query index' to match the
/// one we have just created locally.
/// When calling zomes within the same DNA, use `hdk::THIS_INSTANCE` as `remote_dna_id`.
pub fn request_remote_query_index(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<Address>> {
    // :TODO: implement bridge genesis callbacks & private chain entry to wire up cross-DNA link calls

    // Build query index in remote DNA (for retrieving linked `target` entries)
    // -> links to `Commitment`s in the associated Planning DNA from this `EconomicEvent.fulfills`,
    //    and back to this `EconomicEvent` via `Commitment.fulfilledBy`.
    // :TODO: resolve typecasting issue and propagate any errors in the response
    let mut _result: JsonString = link_remote_entries(
        remote_dna_id,
        remote_zome_id,
        &remote_request_cap_token,
        remote_zome_method,
        &source_base_address,
        target_base_addresses,
    )?;

    // :TODO: decode and return status codes from link requests
    Ok(vec![])
}

/// Creates a bidirectional link between two entry addresses, and returns a vector
/// of the addresses of the (respectively) forward & reciprocal links created.
pub fn link_entries_bidir<S: Into<String>>(
    source: &Address,
    dest: &Address,
    link_type: S,
    link_name: S,
    link_type_reciprocal: S,
    link_name_reciprocal: S,
) -> Vec<Address> {
    vec! [
        link_entries(source, dest, link_type, link_name).unwrap(),
        link_entries(dest, source, link_type_reciprocal, link_name_reciprocal).unwrap(),
    ]
}

/// Load any set of records of type `R` that are linked from the
/// `base_address` entry via `link_type` and `link_name`.
///
/// Results are automatically deserialized into `R` as they are retrieved from the DHT.
/// Any entries that either fail to load or cannot be converted to the type will be dropped.
///
/// :TODO: return errors, improve error handling
///
pub fn get_links_and_load_entry_data<R, F, A>(
    base_address: &F,
    link_type: &str,
    link_name: &str,
) -> ZomeApiResult<Vec<(A, Option<R>)>>
    where R: Clone + TryFrom<AppEntryValue>,
        A: From<Address>,
        F: AsRef<Address>,
{
    let get_links_result = get_links_with_options(
        base_address.as_ref(),
        LinkMatch::Exactly(link_type),
        LinkMatch::Exactly(link_name),
        GetLinksOptions::default(),
    )?;

    let addresses = get_links_result.addresses();

    let entries: Vec<Option<R>> = addresses.iter()
        .map(|address| {
            let entry_address = get_entry(&address)?;
            let entry = match entry_address {
                Some(AppEntry(_, entry_address_value)) => {
                    get_entry(&Address::try_from(entry_address_value)?)
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            };
            match entry {
                Ok(Some(AppEntry(_, entry_value))) => {
                    match R::try_from(entry_value.to_owned()) {
                        Ok(val) => Ok(Some(val)),
                        Err(_) => Err(ZomeApiError::Internal("Could not convert get_links result to requested type".to_string())),
                    }
                },
                _ => Err(ZomeApiError::Internal("Could not locate entry".to_string())),
            }
        })
        .filter_map(Result::ok)
        .collect();

    Ok(addresses.iter()
        .map(|address| {
            address.to_owned().into()
        })
        .zip(entries)
        .collect()
    )
}

/// Common request format for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct RemoteEntryLinkRequest {
    base_entry: Address,
    target_entries: Vec<Address>,
}

/// Calls into a neighbouring DNA to link a "base entry" for the given entry ID
/// to multiple target entries, via a zome API request conforming to `RemoteEntryLinkRequest`.
/// This enables the DNA holding the target entries to setup data structures
/// for querying the associated remote entry records back out.
fn link_remote_entries<R>(
    target_dna_id: &str,
    zome_name: &str,
    cap_token: &Address,
    fn_name: &str,
    base_entry: &Address,
    target_entries: &Vec<Address>,
) -> ZomeApiResult<R>
  where R: TryFrom<AppEntryValue>,
{
    let result = call(target_dna_id, zome_name, cap_token.to_owned(), fn_name, RemoteEntryLinkRequest {
        base_entry: base_entry.clone().into(),
        target_entries: target_entries.clone().into(),
    }.into())?;

    result.try_into().map_err(|_| {
        ZomeApiError::Internal("Could not convert link_remote_entries result to requested type".to_string())
    })
}
