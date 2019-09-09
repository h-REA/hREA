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
use std::fmt::Display;
use serde::{de::DeserializeOwned};

use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiError, ZomeApiResult },
    holochain_persistence_api::cas::content::Address,
    call,
};
use holochain_json_derive::{ DefaultJson };

use super::link_helpers::create_local_query_index;

/// Common request format (zome trait) for linking remote entries in cooperating DNAs
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct RemoteEntryLinkRequest {
    base_entry: Address,
    target_entries: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct RemoteEntryLinkResponse {
    indexes_processed: Vec<ZomeApiResult<Address>>,
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
    target_base_addresses: &Vec<Address>,
) -> Vec<ZomeApiResult<Address>> {
    let mut local_results = create_local_query_index(
        remote_base_entry_type,
        origin_relationship_link_type,
        origin_relationship_link_tag,
        destination_relationship_link_type,
        destination_relationship_link_tag,
        source_base_address,
        target_base_addresses,
    );

    let mut remote_results = request_remote_query_index(
        remote_dna_id,
        remote_zome_id,
        remote_zome_method,
        remote_request_cap_token,
        source_base_address,
        target_base_addresses,
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
///
fn request_remote_query_index(
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
    remote_request_cap_token: Address,
    source_base_address: &Address,
    target_base_addresses: &Vec<Address>,
) -> ZomeApiResult<Vec<ZomeApiResult<Address>>> {
    // Call into remote DNA to enable target entries to setup data structures
    // for querying the associated remote entry records back out.
    let response: ZomeApiResult<RemoteEntryLinkResponse> = read_from_zome(
        remote_dna_id, remote_zome_id, remote_request_cap_token, remote_zome_method, RemoteEntryLinkRequest {
            base_entry: source_base_address.clone().into(),
            target_entries: target_base_addresses.clone().into(),
        }.into()
    );

    match response {
        Ok(RemoteEntryLinkResponse { indexes_processed, .. }) => {
            Ok(indexes_processed)
        },
        Err(_) => Err(ZomeApiError::Internal("indexing error in remote DNA".to_string())),
    }
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
