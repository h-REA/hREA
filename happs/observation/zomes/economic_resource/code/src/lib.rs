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
    holochain_json_api::{ json::JsonString, error::JsonError },
};

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

// Zome definition

define_zome! {
    entries: [
        resource_entry_def(),
        resource_base_entry_def()
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
        query_resources: {
            inputs: |params: QueryParams|,
            outputs: |result: ZomeApiResult<Vec<ResponseData>>|,
            handler: receive_query_economic_resources
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
