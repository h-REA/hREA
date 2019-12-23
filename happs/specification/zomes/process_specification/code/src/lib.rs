#![feature(proc_macro_hygiene)]
// :TODO: documentation
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_specification;

mod process_specification_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_specification::type_aliases::{
    ProcessSpecificationAddress,
};
use vf_specification::process_specification::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use process_specification_requests::{
    receive_create_process_specification,
    receive_get_process_specification,
    receive_update_process_specification,
    receive_delete_process_specification,
};
use vf_specification::identifiers::{
    PROCESS_SPECIFICATION_ENTRY_TYPE,
    PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
    PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
};

#[zome]
mod rea_specification_processspecification_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }

    #[entry_def]
    fn entry_def() -> ValidatingEntryType {
        entry!(
            name: PROCESS_SPECIFICATION_ENTRY_TYPE,
            description: "Process specification",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Entry>| {
                Ok(())
            },
            links: [
            ]
        )
    }

    #[entry_def]
    fn base_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROCESS_SPECIFICATION_BASE_ENTRY_TYPE,
            description: "Process specification",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<ProcessSpecificationAddress>| {
                Ok(())
            },
            links: [
                to!(
                    PROCESS_SPECIFICATION_ENTRY_TYPE,
                    link_type: PROCESS_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[zome_fn("hc_public")]
    fn create_process_specification(process_specification: CreateRequest) -> ZomeApiResult<ResponseData>{
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
}
