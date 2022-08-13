/**
 * Holo-REA proposahc_zome_rea_proposal_rpcl zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_proposal_lib::*;
use hc_zome_rea_proposal_rpc::*;
use hc_zome_rea_proposal_storage_consts::*;

#[hdk_extern]
fn create_proposal(CreateParams { proposal }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_proposal(PROPOSAL_ENTRY_TYPE, proposal)?)
}

#[hdk_extern]
fn get_proposal(ByAddress { address }: ByAddress<ProposalAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_proposal(address)?)
}

#[hdk_extern]
fn update_proposal(UpdateParams { proposal }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_proposal(proposal)?)
}

#[hdk_extern]
fn delete_proposal(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_proposal(revision_id)?)
}
