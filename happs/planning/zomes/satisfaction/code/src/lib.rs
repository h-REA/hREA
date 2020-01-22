#![feature(proc_macro_hygiene)]
/**
 * Holo-REA satisfaction zome API definition
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

use hc_zome_rea_satisfaction_defs::{ entry_def, base_entry_def };
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib_origin::*;

// Zome entry type wrappers
#[zome]
mod rea_satisfaction_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn satisfaction_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn satisfaction_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_satisfaction(satisfaction: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_satisfaction(satisfaction)
    }

    #[zome_fn("hc_public")]
    fn get_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<ResponseData> {
        receive_get_satisfaction(address)
    }

    #[zome_fn("hc_public")]
    fn update_satisfaction(satisfaction: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_satisfaction(satisfaction)
    }

    #[zome_fn("hc_public")]
    fn delete_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<bool> {
        receive_delete_satisfaction(address)
    }

    #[zome_fn("hc_public")]
    fn query_satisfactions(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_satisfactions(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
