/**
 * RPC-handling abstractions for Holochain apps
 *
 * Handles common behaviours for cross-DNA and cross-zome communication, facilitating
 * behaviours where zomes must communicate between one another.
 *
 * @package Holo-REA
 * @since   2021-01-31
 */

use hdk3::prelude::*;

use crate::{
    OtherCellResult,
    CrossCellError,
};

/**
 * Wrapper for `hdk::call` which handles decoding of the response and coercion of error types.
 */
pub fn call_zome_method<R, I>(
    to_cell: Option<CellId>,
    zome_name: ZomeName,
    zome_method: FunctionName,
    cap_secret: Option<CapSecret>,
    payload: I,
) -> OtherCellResult<R>
    where I: serde::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let resp = call(to_cell, zome_name, zome_method, cap_secret, payload)
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
