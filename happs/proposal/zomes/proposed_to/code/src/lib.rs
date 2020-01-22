/**
 * Holo-REA proposed intents zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate hdk;
extern crate serde;

extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

// use hdk_graph_helpers::remote_indexes::RemoteEntryLinkResponse; // :TODO: wire up remote indexing API if necessary

use hc_zome_rea_proposed_to_defs::{ entry_def, base_entry_def };
use hc_zome_rea_proposed_to_rpc::*;

extern crate vf_proposal;

use hc_zome_rea_proposed_to_defs::{ entry_def, base_entry_def };
use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_lib::*;

mod proposed_to_requests;

use vf_proposal::type_aliases::{
    ProposedToAddress,
};
use vf_proposal::proposed_to::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use proposed_to_requests::{
    receive_create_proposed_to,
    receive_get_proposed_to,
    receive_update_proposed_to,
    receive_delete_proposed_to,
};

use vf_proposal::identifiers::{
    PROPOSED_TO_ENTRY_TYPE,
    PROPOSED_TO_BASE_ENTRY_TYPE,
    PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE,
};

#[zome]
mod rea_proposed_to_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn proposed_to_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn proposed_to_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_proposed_to(proposed_to: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_proposed_to(proposed_to)
    }

    #[zome_fn("hc_public")]
    fn get_proposed_to(address: ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
        receive_get_proposed_to(address)
    }

    #[zome_fn("hc_public")]
    fn update_proposed_to(proposed_to: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_proposed_to(proposed_to)
    }

    #[zome_fn("hc_public")]
    fn delete_proposed_to(address: ProposedIntentAddress) -> ZomeApiResult<bool> {
        receive_delete_proposed_to(address)
    }

    #[zome_fn("hc_public")]
    fn query_proposed_tos(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_proposed_tos(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
