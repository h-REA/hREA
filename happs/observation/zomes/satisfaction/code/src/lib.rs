#![feature(proc_macro_hygiene)]
/**
 * Holo-REA satisfaction remote index zome API definition
 *
 * Manages indexes for querying `EconomicEvents` against remote `Satisfactions`.
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate serde_json;
extern crate hdk;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hc_zome_rea_satisfaction_defs::{ entry_def, remote_entry_def };
use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib_destination::*;

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
        remote_entry_def()
    }

    #[zome_fn("hc_public")]
    fn satisfaction_created(satisfaction: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_satisfaction(satisfaction)
    }

    #[zome_fn("hc_public")]
    fn satisfaction_updated(satisfaction: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_satisfaction(satisfaction)
    }

    #[zome_fn("hc_public")]
    fn satisfaction_deleted(address: SatisfactionAddress) -> ZomeApiResult<bool> {
        receive_delete_satisfaction(address)
    }

    #[zome_fn("hc_public")]
    fn get_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<ResponseData> {
        receive_get_satisfaction(address)
    }

    #[zome_fn("hc_public")]
    fn query_satisfactions(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_satisfactions(params)
    }

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }

}
