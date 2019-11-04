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
use vf_core::type_aliases::ProcessAddress;
use resource_specification_requests::{
    // QueryParams,
    receive_create_resource_specification,
    receive_get_resource_specification,
    receive_update_resource_specification,
    receive_delete_resource_specification,
};
use vf_specification::identifiers::{
    ECONOMIC_RESOURCE,
    ECONOMIC_RESOURCE_CONFORMING,
};
use vf_observation::identifiers::{
    RESOURCE_ENTRY_TYPE
};

// Zome entry type wrappers

fn resource_entry_def() -> ValidatingEntryType {
    entry!(
        name: ECONOMIC_RESOURCE,
        description: "A resource which is useful to people or the ecosystem.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        },
        links: [
            to!(
                RESOURCE_ENTRY_TYPE,
                link_type: ECONOMIC_RESOURCE_CONFORMING,
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
        create_resource_specification: {
            inputs: |address: ResourceAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_create_resource_specification
        }
        get_resource_specification: {
            inputs: |address: ResourceAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_get_resource_specification
        }
        update_resource_specification: {
            inputs: |resource: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_update_resource_specification
        }
        delete_resource_specification: {
            inputs: |address: ProcessAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_resource_specification
        }
    ]
    traits: {
        hc_public [
            create_resource_specification,
            get_resource_specification,
            update_resource_specification,
            delete_resource_specification
        ]
    }
}
