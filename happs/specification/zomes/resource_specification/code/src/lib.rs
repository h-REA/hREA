#![feature(proc_macro_hygiene)]
// :TODO: documentation
extern crate hdk;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_specification;

mod resource_specification_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_specification::type_aliases::{
    ResourceSpecificationAddress,
};
use vf_specification::resource_specification::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use resource_specification_requests::{
    // QueryParams,
    receive_create_resource_specification,
    receive_get_resource_specification,
    receive_update_resource_specification,
    receive_delete_resource_specification,
};
use vf_specification::identifiers::{
    ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
    ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
    ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
};

#[zome]
mod rea_specification_resourcepecification_zome {

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
        entry!(
            name: ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
            description: "Specification of a resource which is useful to people or the ecosystem.",
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
    fn resource_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
            description: "Specification base",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<ResourceSpecificationAddress>| {
                Ok(())
            },
            links: [
                to!(
                    ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE,
                    link_type: ECONOMIC_RESOURCE_SPECIFICATION_INITIAL_ENTRY_LINK_TYPE,
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

    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }

    #[zome_fn("hc_public")]
    fn create_resource_specification(resource_specification: CreateRequest) -> ZomeApiResult<ResponseData>{
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
}
