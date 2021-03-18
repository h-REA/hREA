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
use hc_zome_dna_auth_resolver_lib::*;

use crate::{
    OtherCellResult,
    CrossCellError,
};

/**
 * Wrapper for `hdk::call` which handles decoding of the response and coercion of error types.
 */
pub fn call_zome_method<H, R, I>(
    to_registered_dna: &H,
    zome_name: ZomeName,
    zome_method: FunctionName,
    payload: I,
) -> OtherCellResult<R>
    where H: AsRef<DnaHash>,
        I: serde::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let to_dna: &DnaHash = to_registered_dna.as_ref();
    let cell_auth = get_auth_data(to_registered_dna.as_ref())?;
    let to_cell = Some(CellId::new(to_dna.clone(), cell_auth.agent_pubkey));
    let resp = call(to_cell, zome_name, zome_method, cell_auth.cap_secret, payload)
        .map_err(CrossCellError::from)?;

    match resp {
        ZomeCallResponse::Ok(data) =>
            Ok(data.decode()?),
        ZomeCallResponse::Unauthorized(cell, zome, fname, agent) =>
            Err(CrossCellError::Unauthorized(cell, zome, fname, agent)),
        ZomeCallResponse::NetworkError(msg) =>
            Err(CrossCellError::NetworkError(msg)),
    }
}
