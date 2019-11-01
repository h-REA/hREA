// :TODO: documentation

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_specification;

mod resource_specification_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::dna::entry_types::Sharing,
    holochain_json_api::{ json::JsonString, error::JsonError },
};

use vf_specification::type_aliases::{
    ResourceAddress,
};
use vf_specification::resource_specification::{
    Entry,
    UpdateRequest,
    ResponseData,
};
use resource_specification_requests::{
    QueryParams,
    receive_get_economic_resource,
    receive_update_economic_resource,
    receive_query_economic_resources,
};
// use vf_specification::identifiers::{
// };

// Zome entry type wrappers

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
            )
        ]
    )
}

// Zome definition

define_zome! {
    entries: [
        resource_entry_def()
    ]

    init: || {
        Ok(())
    }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    receive: |from, payload| {
      format!("Received: {} from {}", payload, from)
    }

    functions: [
        create_resource: {
            inputs: |address: ResourceAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_create_economic_resource
        }
        get_resource: {
            inputs: |address: ResourceAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_get_economic_resource
        }
        update_resource: {
            inputs: |resource: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_update_economic_resource
        }
        delete_process: {
            inputs: |address: ProcessAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_process
        }
    ]

    traits: {
        hc_public [
            get_resource,
            update_resource,
            query_resources
        ]
    }
}
