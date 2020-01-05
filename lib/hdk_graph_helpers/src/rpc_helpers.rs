/**
 * RPC-handling abstractions for Holochain apps
 *
 * Handles common behaviours for cross-DNA and cross-zome communication, facilitating
 * behaviours where zomes must communicate between one another.
 *
 * @package HoloREA
 * @since   2019-09-09
 */

use std::ops::Deref;
use std::convert::{ TryInto, TryFrom };
use serde::{de::DeserializeOwned};
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiError, ZomeApiResult },
    holochain_persistence_api::cas::content::Address,
    call,
};

use super::{
    identifiers::{ ERR_MSG_REMOTE_REQUEST_ERR, ERR_MSG_REMOTE_RESPONSE_FORMAT_ERR },
};

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
    where S: Clone + Into<String> + Deref<Target=str>,
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
        Ok(Err(_response_err)) => Err(ZomeApiError::Internal(build_zome_req_error(ERR_MSG_REMOTE_REQUEST_ERR, &instance_handle[..], &zome_name[..], &fn_name[..]))),
        Err(_decoding_err) => Err(ZomeApiError::Internal(build_zome_req_error(ERR_MSG_REMOTE_RESPONSE_FORMAT_ERR, &instance_handle[..], &zome_name[..], &fn_name[..]))),
    }
}

/// Error reporting for inter-zome RPC calls
pub (crate) fn build_zome_req_error(
    prefix_string: &str,
    remote_dna_id: &str,
    remote_zome_id: &str,
    remote_zome_method: &str,
 ) -> String {
    let mut err_string = prefix_string.to_string();
    err_string.push_str(remote_dna_id);
    err_string.push_str("/");
    err_string.push_str(remote_zome_id);
    err_string.push_str("/");
    err_string.push_str(remote_zome_method);
    err_string
}
