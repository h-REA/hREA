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
pub fn call_zome_method<H, R, I, S>(
    to_registered_dna: &H,
    remote_permission_id: &S,
    payload: I,
) -> OtherCellResult<R>
    where S: AsRef<str>,
        H: AsRef<DnaHash>,
        I: serde::Serialize + std::fmt::Debug,
        R: serde::de::DeserializeOwned + std::fmt::Debug,
{
    let to_dna: &DnaHash = to_registered_dna.as_ref();

    // check for previous inter-DNA authentication using Auth Resolver lib
    let mut cell_auth = get_auth_data(to_dna, remote_permission_id);
    match &cell_auth {
        Ok(_) => {},
        // transparently request indicated permission if not granted
        Err(_) => {
            let _ = make_auth_request(to_dna, remote_permission_id)?;

            // re-check for permissions after request, bail if failed
            cell_auth = get_auth_data(to_dna, remote_permission_id);
            match cell_auth {
                Ok(_) => {},
                Err(e) => {
                    return Err(CrossCellError::CellAuthFailed(to_dna.to_owned(), e.to_string()));
                },
            }
        },
    }

    let auth_data = cell_auth.unwrap();
    let DNAConnectionAuth { claim, method } = auth_data;

    let to_cell = Some(CellId::new(to_dna.clone(), claim.grantor().to_owned()));
    let resp = call(to_cell, method.0, method.1, Some(claim.secret().to_owned()), payload)
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
