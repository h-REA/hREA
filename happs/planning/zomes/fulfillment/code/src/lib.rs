#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;

mod fulfillment_requests;

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

use hdk_graph_helpers::{
    LINK_TYPE_INITIAL_ENTRY,
};
use vf_planning::fulfillment::{
    Entry as FulfillmentEntry,
    CreateRequest as FulfillmentCreateRequest,
    UpdateRequest as FulfillmentUpdateRequest,
    ResponseData as FulfillmentResponse,
};

use fulfillment_requests::{
    handle_create_fulfillment,
    handle_get_fulfillment,
    handle_delete_fulfillment,
    handle_query_fulfillments,
    handle_update_fulfillment,
};
use vf_planning::identifiers::{
    FULFILLMENT_ENTRY_TYPE,
    FULFILLMENT_BASE_ENTRY_TYPE,
    FULFILLMENT_FULFILLS_LINK_TYPE,
    COMMITMENT_BASE_ENTRY_TYPE,
};

// Entry type definitions

fn fulfillment_entry_def() -> ValidatingEntryType {
    entry!(
        name: FULFILLMENT_ENTRY_TYPE,
        description: "Represents many-to-many relationships between commitments and economic events that fully or partially satisfy one or more commitments.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<FulfillmentEntry>| {
            Ok(())
        }
    )
}

fn fulfillment_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: FULFILLMENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial fulfillment addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                FULFILLMENT_ENTRY_TYPE,
                link_type: LINK_TYPE_INITIAL_ENTRY,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                COMMITMENT_BASE_ENTRY_TYPE,
                link_type: FULFILLMENT_FULFILLS_LINK_TYPE,

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
        fulfillment_entry_def(),
        fulfillment_base_entry_def()
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
        create_fulfillment: {
            inputs: |fulfillment: FulfillmentCreateRequest|,
            outputs: |result: ZomeApiResult<FulfillmentResponse>|,
            handler: handle_create_fulfillment
        }
        get_fulfillment: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<FulfillmentResponse>|,
            handler: handle_get_fulfillment
        }
        update_fulfillment: {
            inputs: |fulfillment: FulfillmentUpdateRequest|,
            outputs: |result: ZomeApiResult<FulfillmentResponse>|,
            handler: handle_update_fulfillment
        }
        delete_fulfillment: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_delete_fulfillment
        }
        query_fulfillments: {
            inputs: |commitment: Address|,
            outputs: |result: ZomeApiResult<Vec<FulfillmentResponse>>|,
            handler: handle_query_fulfillments
        }
    ]

    traits: {
        hc_public [
            create_fulfillment,
            get_fulfillment,
            update_fulfillment,
            delete_fulfillment,
            query_fulfillments
        ]
    }
}
