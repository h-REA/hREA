#![feature(proc_macro_hygiene)]
// :TODO: documentation

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_observation;

mod economic_resource_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::dna::entry_types::Sharing,
    
};
use hdk_proc_macros::zome;

use vf_observation::type_aliases::{
    ResourceAddress,
};
use vf_observation::economic_resource::{
    Entry,
    UpdateRequest,
    ResponseData,
};
use economic_resource_requests::{
    QueryParams,
    receive_get_economic_resource,
    receive_update_economic_resource,
    receive_query_economic_resources,
};
use vf_observation::identifiers::{
    RESOURCE_BASE_ENTRY_TYPE,
    RESOURCE_INITIAL_ENTRY_LINK_TYPE,
    RESOURCE_ENTRY_TYPE,
    RESOURCE_CONTAINS_LINK_TYPE,
    RESOURCE_CONTAINED_IN_LINK_TYPE,
    RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE,
    EVENT_BASE_ENTRY_TYPE,
};

// Zome entry type wrappers

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
        entry!(
            name: RESOURCE_ENTRY_TYPE,
            description: "A resource which is useful to people or the ecosystem.",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Entry>| {
                Ok(())
            }
        )

    }

    #[entry_def]
    fn resource_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: RESOURCE_BASE_ENTRY_TYPE,
            description: "Base anchor for initial resource addresses to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    RESOURCE_ENTRY_TYPE,
                    link_type: RESOURCE_INITIAL_ENTRY_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    RESOURCE_BASE_ENTRY_TYPE,
                    link_type: RESOURCE_CONTAINS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    RESOURCE_BASE_ENTRY_TYPE,
                    link_type: RESOURCE_CONTAINED_IN_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    EVENT_BASE_ENTRY_TYPE,
                    link_type: RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE,
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
    fn get_resource(address: ResourceAddress) -> ZomeApiResult<ResponseData> {
        receive_get_economic_resource(address)
    }

    #[zome_fn("hc_public")]
    fn update_resource(resource: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_economic_resource(resource)
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