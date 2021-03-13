#![feature(proc_macro_hygiene)]
/**
 * REA `Process` zome API definition
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

use hdk_records::remote_indexes::RemoteEntryLinkResponse;

use vf_core::type_aliases::{
    CommitmentAddress,
    IntentAddress,
};

use hc_zome_rea_process_defs::{ entry_def, base_entry_def };
use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_process_rpc::*;
use hc_zome_rea_process_lib::*;


// Zome entry type wrappers
#[zome]
mod rea_process_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn process_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn process_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_process(process: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_process(process)
    }

    #[zome_fn("hc_public")]
    fn get_process(address: ProcessAddress) -> ZomeApiResult<ResponseData> {
        receive_get_process(address)
    }

    #[zome_fn("hc_public")]
    fn update_process(process: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_process(process)
    }

    #[zome_fn("hc_public")]
    fn delete_process(address: ProcessAddress) -> ZomeApiResult<bool> {
        receive_delete_process(address)
    }

    #[zome_fn("hc_public")]
    fn query_processes(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_processes(params)
    }


    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    //   }

}
