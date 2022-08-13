/**
 * RPC-handling abstractions for Holochain apps
 *
 * Handles common behaviours for cross-DNA and cross-zome communication, facilitating
 * behaviours where zomes must communicate between one another.
 *
 * @package Holo-REA
 * @since   2021-01-31
 */

use hdk::prelude::*;
use holo_hash::DnaHash;
use hc_zome_dna_auth_resolver_lib::{DNAConnectionAuth, ensure_authed, AvailableCapability};

use crate::{
    OtherCellResult,
    CrossCellError,
};

/**
 * Wrapper for `hdk::call` which handles decoding of the response and coercion of error types.
 */
pub fn call_zome_method<EN, H, R, I, S, LT, E, E2>(
    to_registered_dna: &H,
    remote_permission_id: &S,
    payload: I,
    capability_link_type: LT,
) -> OtherCellResult<R>
    where S: AsRef<str>,
        H: AsRef<DnaHash>,
        I: serde::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
        // links
        ScopedLinkType: TryFrom<LT, Error = E>, // associated with create_link
        LT: Clone + LinkTypeFilterExt, // LinkTypeFilterExt associated with get_links
        // entries
        EN: TryFrom<AvailableCapability, Error = E>,
        ScopedEntryDefIndex: for<'a> TryFrom<&'a EN, Error = E2>,
        EntryVisibility: for<'a> From<&'a EN>,
        Entry: TryFrom<EN, Error = E>,
        // links and entries
        WasmError: From<E> + From<E2>,
{
    let to_dna: &DnaHash = to_registered_dna.as_ref();
    let auth_data = ensure_authed(to_dna, remote_permission_id, capability_link_type)?;

    let DNAConnectionAuth { claim, method } = auth_data;

    let to_cell = CallTargetCell::OtherCell(CellId::new(to_dna.clone(), claim.grantor().to_owned()));
    let resp = call(to_cell, method.0, method.1, Some(claim.secret().to_owned()), payload)
        .map_err(CrossCellError::from)?;

    handle_resp(resp)
}

/**
 * Helper for making local-zome calls, which implicitly require no authentication and operate under a different security model.
 *
 * :TODO: Should this be using call_zome_method and similar config zome as https://github.com/holochain-open-dev/dna-auth-resolver/ ?
 *        Or will there be a system-level means of defining inter-zome permissions on the same DNA elsewhere?
 */
pub fn call_local_zome_method<C, F, R, I, S>(
    zome_name_from_config: F,
    method_name: S,
    payload: I,
) -> OtherCellResult<R>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: FnOnce(C) -> Option<String>,
        I: serde::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let this_zome = zome_info()?.name;
    let remote_local_zome_method = FunctionName(method_name.as_ref().to_string());

    let zome_props: C = dna_info()?.properties
        .try_into()
        .map_err(|_| { CrossCellError::NotConfigured(this_zome.to_owned(), remote_local_zome_method.to_owned()) })?;

    match zome_name_from_config(zome_props) {
        None => Err(CrossCellError::NotConfigured(this_zome, remote_local_zome_method)),
        Some(local_zome_id) => {
            let resp = call(CallTargetCell::Local, ZomeName::new(local_zome_id), remote_local_zome_method, None, payload)
                .map_err(CrossCellError::from)?;

            handle_resp(resp)
        },
    }

}

fn handle_resp<R>(
    resp: ZomeCallResponse,
) -> OtherCellResult<R>
    // :TODO: data.decode() requires Debug to be implemented. Is this expected behaviour?
    where R: serde::de::DeserializeOwned + std::fmt::Debug,
{
    match resp {
        ZomeCallResponse::Ok(data) =>
            Ok(data.decode()?),
        ZomeCallResponse::Unauthorized(cell, zome, fname, agent) =>
            Err(CrossCellError::Unauthorized(cell, zome, fname, agent)),
        ZomeCallResponse::NetworkError(msg) =>
            Err(CrossCellError::NetworkError(msg)),
        ZomeCallResponse::CountersigningSession(msg) =>
            Err(CrossCellError::NetworkError(msg)),
    }
}
