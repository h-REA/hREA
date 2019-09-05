// :TODO: documentation

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_observation;
extern crate vf_planning;

mod process_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
};

use vf_observation::type_aliases::{
    ProcessAddress,
};
use vf_observation::process::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use process_requests::{
    QueryParams,
    receive_get_process,
    receive_create_process,
    receive_update_process,
    receive_delete_process,
    receive_query_processes,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_INITIAL_ENTRY_LINK_TYPE,
    PROCESS_ENTRY_TYPE,
};

// Zome entry type wrappers

fn process_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_ENTRY_TYPE,
        description: "An activity that changes inputs into outputs.  It could transform or transport economic resource(s).",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        }
    )
}

fn process_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_BASE_ENTRY_TYPE,
        description: "Base anchor for initial process addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                PROCESS_ENTRY_TYPE,
                link_type: PROCESS_INITIAL_ENTRY_LINK_TYPE,
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
        process_entry_def(),
        process_base_entry_def()
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
        create_process: {
            inputs: |process: CreateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_create_process
        }
        get_process: {
            inputs: |address: ProcessAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_get_process
        }
        update_process: {
            inputs: |process: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_update_process
        }
        delete_process: {
            inputs: |address: ProcessAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_process
        }

        query_processes: {
            inputs: |params: QueryParams|,
            outputs: |result: ZomeApiResult<Vec<ResponseData>>|,
            handler: receive_query_processes
        }
    ]

    traits: {
        hc_public [
            create_process,
            get_process,
            update_process,
            delete_process,
            query_processes
        ]
    }
}
