#![feature(proc_macro_hygiene)]
/**
 * REA `EconomicResource` zome API definition
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

use hc_zome_rea_economic_resource_defs::*;
use hc_zome_rea_economic_resource_lib::*;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_event_rpc::ResourceResponseData as ResponseData;

#[zome]
mod rea_economic_resource_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn resource_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn resource_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[entry_def]
    fn resource_root_entry_def() -> ValidatingEntryType {
        root_entry_def()
    }

    #[zome_fn("hc_public")]
    fn get_resource(address: ResourceAddress) -> ZomeApiResult<ResponseData> {
        receive_get_economic_resource(address)
    }

    #[zome_fn("hc_public")]
    fn update_resource(resource: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_economic_resource(resource)
    }

    #[zome_fn("hc_public")]
    fn get_all_resources() -> ZomeApiResult<Vec<ResponseData>> {
        receive_get_all_economic_resources()
    }


    #[zome_fn("hc_public")]
    fn query_resources(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_economic_resources(params)
    }


    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }

}
