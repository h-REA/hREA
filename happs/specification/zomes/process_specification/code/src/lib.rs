// :TODO: documentation
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_specification;

mod process_specification_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::dna::entry_types::Sharing,
    holochain_json_api::{ json::JsonString, error::JsonError },
};

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
    // QueryParams,
    receive_create_process_specification,
    receive_get_process_specification,
    receive_update_process_specification,
    receive_delete_process_specification,
};
use vf_specification::identifiers::{
    PROCESS_SPECIFICATION,
};

// Zome entry type wrappers

fn resource_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_SPECIFICATION,
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
        create_process_specification: {
            inputs: |address: CreateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_create_process_specification
        }
        get_process_specification: {
            inputs: |address: ProcessSpecificationAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_get_process_specification
        }
        update_process_specification: {
            inputs: |resource: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_update_process_specification
        }
        delete_process_specification: {
            inputs: |address: ProcessSpecificationAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_process_specification
        }
    ]
    traits: {
        hc_public [
            create_process_specification,
            get_process_specification,
            update_process_specification,
            delete_process_specification
        ]
    }
}
