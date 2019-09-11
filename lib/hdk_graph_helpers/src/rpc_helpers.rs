/**
 * RPC-handling abstractions for Holochain apps
 *
 * Handles common behaviours for cross-DNA and cross-zome communication, facilitating
 * behaviours where zomes must communicate between one another.
 *
 * @package HoloREA
 * @since   2019-09-09
 */

use std::convert::{ TryInto, TryFrom };
use std::fmt::{Debug,Display};
use serde::{de::DeserializeOwned};

use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiError, ZomeApiResult },
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::entry::entry_type::AppEntryType,
    call,
};
use holochain_json_derive::{ DefaultJson };

use super::MaybeUndefined;
use super::link_helpers::{
    create_local_query_index,
    create_remote_query_index,
    replace_remote_entry_link_set,
    remove_remote_entry_link_set,
};

// Common request format (zome trait) for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct RemoteEntryLinkRequest {
    base_entry: Address,
    target_entries: Vec<Address>,
    removed_entries: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct RemoteEntryLinkResponse {
    indexes_created: Vec<ZomeApiResult<Address>>,
    indexes_removed: Vec<ZomeApiResult<()>>,
}





// CREATE

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
    target_base_addresses: Vec<Address>,
) -> Vec<ZomeApiResult<Address>> {
    let mut local_results = create_local_query_index(
        remote_base_entry_type,
        origin_relationship_link_type,
        origin_relationship_link_tag,
        destination_relationship_link_type,
        destination_relationship_link_tag,
        source_base_address,
        target_base_addresses.clone(),
    );

    let mut remote_results = request_sync_remote_query_index(
        remote_dna_id,
        remote_zome_id,
        remote_zome_method,
        remote_request_cap_token,
        source_base_address,
        target_base_addresses,
        vec![],
    ).unwrap_or_else(|e| { vec![Err(e)] });

    local_results.append(&mut remote_results);
    local_results
}

/// Ask another bridged DNA or zome to build a 'remote query index' to match the
/// one we have just created locally.
/// When calling zomes within the same DNA, use `hdk::THIS_INSTANCE` as `remote_dna_id`.
///
/// :TODO: implement bridge genesis callbacks & private chain entry to wire up cross-DNA link calls
/// :TODO: propagate errors from callee in error context, rather than masking them
/// :TODO: return indexes_removed to the caller
///
fn request_sync_remote_query_index(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    source_base_address: &Address,
    target_base_addresses: Vec<Address>,
    removed_base_addresses: Vec<Address>,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>> {
    // Call into remote DNA to enable target entries to setup data structures
    // for querying the associated remote entry records back out.
    let response: ZomeApiResult<RemoteEntryLinkResponse> = read_from_zome(
        remote_dna_id, remote_zome_id, remote_request_cap_token, remote_zome_method, RemoteEntryLinkRequest {
            base_entry: source_base_address.clone().into(),
            target_entries: target_base_addresses,
            removed_entries: removed_base_addresses,
        }.into()
    );

    match response {
        Ok(RemoteEntryLinkResponse { indexes_created, .. }) => {
            Ok(indexes_created) // :TODO: how to treat deletion errors?
        },
        Err(_) => Err(ZomeApiError::Internal("indexing error in remote DNA".to_string())),
    }
}

/// Respond to a request from an external source to build a link index for some externally linking content.
///
/// This essentially creates a base link for the `source_base_address` and then links it to every
/// `target_base_addresses` found locally within this DNA.
///
/// The returned `RemoteEntryLinkResponse` provides an appropriate format for responding to indexing
/// requests that originate with a call to `create_remote_index_pair` in a foreign DNA.
///
pub fn handle_remote_index_sync_request<'a, A, B>(
    remote_base_entry_type: &'a str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &A,
    target_base_addresses: Vec<B>,
    removed_base_addresses: Vec<B>,
) -> ZomeApiResult<RemoteEntryLinkResponse>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq + Debug,
        Address: From<B>,
{
    // remove passed stale indexes
    let remove_resp = remove_remote_entry_link_set(
        source_base_address,
        removed_base_addresses,
        remote_base_entry_type,
        origin_relationship_link_type, origin_relationship_link_tag,
        destination_relationship_link_type, destination_relationship_link_tag,
    );

    // create any new indexes
    let create_resp = create_remote_query_index(
        remote_base_entry_type,
        origin_relationship_link_type, origin_relationship_link_tag,
        destination_relationship_link_type, destination_relationship_link_tag,
        source_base_address,
        target_base_addresses,
    );
    if let Err(index_creation_failure) = create_resp {
        return Err(index_creation_failure);
    }

    Ok(RemoteEntryLinkResponse { indexes_created: create_resp.unwrap(), indexes_removed: remove_resp })
}




// READ

/// Helper for reading data from other zomes or DNAs. Abstracts away the details of dealing with
/// response decoding and type conversion.
/// Simply use `ZomeApiResult<X>` as the return type, where X is the response struct format you wish to decode.
///
/// :TODO: propagate errors from callee in error context, rather than masking them
///
pub fn read_from_zome<R, S>(
    instance_handle: S,
    zome_name: S,
    cap_token: Address,
    fn_name: S,
    fn_args: JsonString,
) -> ZomeApiResult<R>
    where S: Display + Clone + Into<String>,
        R: TryFrom<JsonString> + Into<JsonString> + DeserializeOwned,
{
    let rpc_response = call(instance_handle.clone(), zome_name.clone(), cap_token, fn_name.clone(), fn_args);
    if let Err(bad_call) = rpc_response {
        return Err(bad_call);
    }

    let strng = rpc_response.unwrap();
    let decoded: Result<Result<R, ZomeApiError>, JsonError> = strng.try_into();

    match decoded {
        Ok(Ok(response_data)) => Ok(response_data),
        Ok(Err(_response_err)) => Err(ZomeApiError::Internal("error in zome RPC request handler".to_string())),
        Err(_decoding_err) => Err(ZomeApiError::Internal("zome RPC response not in expected format".to_string())),
    }
}





// UPDATE

/// Toplevel method for triggering a link update flow between two records in
/// different DNAs. Indexes on both sides of the network boundary will be updated.
///
/// :TODO: update to accept multiple targets for the replacement links
///
pub fn update_remote_index_pair<A, B, S>(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    remote_base_entry_type: S,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &A,
    target_base_address: &MaybeUndefined<B>,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq + Debug,
        S: Into<AppEntryType>,
{
    // no change, bail early
    if let MaybeUndefined::Undefined = target_base_address {
        return Ok(vec![]);
    }

    // process local index first and collect all removed link target address
    // :TODO: error handling
    let removed_links = replace_remote_entry_link_set(
        source_base_address,
        target_base_address,
        remote_base_entry_type,
        origin_relationship_link_type,
        origin_relationship_link_tag,
        destination_relationship_link_type,
        destination_relationship_link_tag,
    )?.iter()
        .filter_map(|r| { r.clone().ok() })
        .collect();

    // pass removed IDs and new IDs to remote DNA for re-indexing
    request_sync_remote_query_index(
        remote_dna_id,
        remote_zome_id,
        remote_zome_method,
        remote_request_cap_token,
        source_base_address.as_ref(),
        match &target_base_address {
            &MaybeUndefined::Some(target) => vec![target.as_ref().clone()],
            _ => vec![],
        },
        removed_links,
    )
}





// DELETE

/// Toplevel method for triggering a link update flow between two records in
/// different DNAs. Indexes on both sides of the network boundary will be updated.
///
/// :TODO: update to accept multiple targets for the replacement links
///
pub fn remove_remote_index_pair<'a, A, B>(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    remote_base_entry_type: &'a str,
    origin_relationship_link_type: &str,
    origin_relationship_link_tag: &str,
    destination_relationship_link_type: &str,
    destination_relationship_link_tag: &str,
    source_base_address: &A,
    remove_base_address: &B,
) -> Vec<ZomeApiResult<()>>
    where A: AsRef<Address> + From<Address> + Clone,
        B: AsRef<Address> + From<Address> + Clone + PartialEq + Debug,
        Address: From<B>,
{
    // process local index first and collect any errors
    let mut local_results = remove_remote_entry_link_set(
        source_base_address,
        vec![remove_base_address.clone().into()],
        remote_base_entry_type,
        origin_relationship_link_type,
        origin_relationship_link_tag,
        destination_relationship_link_type,
        destination_relationship_link_tag,
    );

    // pass removed IDs and new IDs to remote DNA for re-indexing
    let remote_results = request_sync_remote_query_index(
        remote_dna_id,
        remote_zome_id,
        remote_zome_method,
        remote_request_cap_token,
        source_base_address.as_ref(),
        vec![],
        vec![remove_base_address.as_ref().clone()],
    );

    match remote_results {
        Ok(results) => {
            let mut remote_errors: Vec<ZomeApiResult<()>> = results.iter()
                .filter(|r| { r.is_err() })
                .map(|e| { Err(e.clone().err().unwrap()) })
                .collect();

            local_results.append(&mut remote_errors);
        },
        Err(e) => local_results.push(Err(e)),
    }

    local_results
}
