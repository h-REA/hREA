#![feature(proc_macro_hygiene)]
/**
 * Holo-REA event actions zome API definition
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

use hc_zome_rea_action_defs::{ entry_def, base_entry_def };
use hc_zome_rea_action_storage_consts::*;
use hc_zome_rea_action_structs_rpc::*;
use hc_zome_rea_action_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_action_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn action_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn action_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_action(action: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_action(action)
    }

    #[zome_fn("hc_public")]
    fn get_action(address: ActionAddress) -> ZomeApiResult<ResponseData> {
        receive_get_action(address)
    }

    #[zome_fn("hc_public")]
    fn update_action(action: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_action(action)
    }

    #[zome_fn("hc_public")]
    fn delete_action(address: ActionAddress) -> ZomeApiResult<bool> {
        receive_delete_action(address)
    }

    #[zome_fn("hc_public")]
    fn query_actions(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_actions(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
