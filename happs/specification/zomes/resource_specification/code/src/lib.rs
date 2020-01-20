#![feature(proc_macro_hygiene)]
/**
 * Holo-REA resource specification zome API definition
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

use hc_zome_rea_resource_specification_defs::{ entry_def, base_entry_def };
use hc_zome_rea_resource_specification_storage_consts::*;
use hc_zome_rea_resource_specification_rpc::*;
use hc_zome_rea_resource_specification_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_resource_specification_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn resource_specification_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn resource_specification_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_resource_specification(resource_specification: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_resource_specification(resource_specification)
    }

    #[zome_fn("hc_public")]
    fn get_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<ResponseData> {
        receive_get_resource_specification(address)
    }

    #[zome_fn("hc_public")]
    fn update_resource_specification(resource_specification: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_resource_specification(resource_specification)
    }

    #[zome_fn("hc_public")]
    fn delete_resource_specification(address: ResourceSpecificationAddress) -> ZomeApiResult<bool> {
        receive_delete_resource_specification(address)
    }

    #[zome_fn("hc_public")]
    fn query_resource_specifications(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_resource_specifications(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
