#![feature(proc_macro_hygiene)]
/**
 * Holo-REA proposal zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk_graph_helpers::remote_indexes::RemoteEntryLinkResponse; // :TODO: wire up remote indexing API if necessary

use hc_zome_rea_proposal_defs::{ entry_def, base_entry_def };
use hc_zome_rea_proposal_rpc::*;
use hc_zome_rea_proposal_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_proposal_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn proposal_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn proposal_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_proposal(proposal: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_proposal(proposal)
    }

    #[zome_fn("hc_public")]
    fn get_proposal(address: ProposalAddress) -> ZomeApiResult<ResponseData> {
        receive_get_proposal(address)
    }

    #[zome_fn("hc_public")]
    fn update_proposal(proposal: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_proposal(proposal)
    }

    #[zome_fn("hc_public")]
    fn delete_proposal(address: ProposalAddress) -> ZomeApiResult<bool> {
        receive_delete_proposal(address)
    }

    #[zome_fn("hc_public")]
    fn query_proposals(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_proposals(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
