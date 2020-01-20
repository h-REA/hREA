#![feature(proc_macro_hygiene)]
/**
 * Holo-REA process specification zome API definition
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

use hc_zome_rea_process_specification_defs::{ entry_def, base_entry_def };
use hc_zome_rea_process_specification_storage_consts::*;
use hc_zome_rea_process_specification_rpc::*;
use hc_zome_rea_process_specification_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_process_specification_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn process_specification_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn process_specification_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_process_specification(process_specification: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_process_specification(process_specification)
    }

    #[zome_fn("hc_public")]
    fn get_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<ResponseData> {
        receive_get_process_specification(address)
    }

    #[zome_fn("hc_public")]
    fn update_process_specification(process_specification: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_process_specification(process_specification)
    }

    #[zome_fn("hc_public")]
    fn delete_process_specification(address: ProcessSpecificationAddress) -> ZomeApiResult<bool> {
        receive_delete_process_specification(address)
    }

    #[zome_fn("hc_public")]
    fn query_process_specifications(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_process_specifications(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
