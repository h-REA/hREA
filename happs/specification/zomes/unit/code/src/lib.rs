// :TODO: documentation
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;

extern crate vf_specification;

mod unit_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_core_types::dna::entry_types::Sharing,
    holochain_json_api::{ json::JsonString, error::JsonError },
};

use vf_specification::type_aliases::{
    UnitAddress,
};
use vf_specification::unit::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
 use unit_requests::{
    receive_create_unit,
    receive_get_unit,
    receive_update_unit,
    receive_delete_unit,
};

use vf_specification::identifiers::{
    UNIT_ENTRY_TYPE,
    UNIT_BASE_ENTRY_TYPE,
};

fn unit_entry_def() -> ValidatingEntryType {
    entry!(
        name: UNIT_ENTRY_TYPE,
        description: "Unit",
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
fn unit_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: UNIT_BASE_ENTRY_TYPE,
        description: "Unit entry base",
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
        unit_entry_def(),
        unit_base_entry_def()
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
        create_unit_specification: {
            inputs: |address: CreateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_create_unit
        }
        get_unit_specification: {
            inputs: |address: UnitAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_get_unit
        }
        update_unit_specification: {
            inputs: |resource: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_update_unit
        }
        delete_unit_specification: {
            inputs: |address: UnitAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_unit
        }
    ]
    traits: {
        hc_public [
            create_unit_specification,
            get_unit_specification,
            update_unit_specification,
            delete_unit_specification
        ]
    }
}
