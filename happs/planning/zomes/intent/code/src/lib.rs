// :TODO: documentation

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;
extern crate vf_observation;

mod intent_requests;

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

use vf_planning::type_aliases::{ IntentAddress };
use vf_planning::intent::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use intent_requests::{
    QueryParams,
    receive_get_intent,
    receive_create_intent,
    receive_update_intent,
    receive_delete_intent,
    receive_query_intents,
};
use vf_planning::identifiers::{
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INITIAL_ENTRY_LINK_TYPE,
    INTENT_ENTRY_TYPE,
    INTENT_SATISFIEDBY_LINK_TYPE,
    SATISFACTION_BASE_ENTRY_TYPE,
    INTENT_INPUT_OF_LINK_TYPE,
    INTENT_OUTPUT_OF_LINK_TYPE,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TYPE,
};

// Zome entry type wrappers

fn intent_entry_def() -> ValidatingEntryType {
    entry!(
        name: INTENT_ENTRY_TYPE,
        description: "A planned economic flow, which can lead to economic events (sometimes through commitments).",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Entry>| {
            Ok(())
        }
    )
}

fn intent_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: INTENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial intent addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                INTENT_ENTRY_TYPE,
                link_type: INTENT_INITIAL_ENTRY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                SATISFACTION_BASE_ENTRY_TYPE,
                link_type: INTENT_SATISFIEDBY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: INTENT_INPUT_OF_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: INTENT_OUTPUT_OF_LINK_TYPE,
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

fn process_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: PROCESS_BASE_ENTRY_TYPE,
        description: "Base anchor for processes being linked to in external networks",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                INTENT_BASE_ENTRY_TYPE,
                link_type: PROCESS_INTENT_INPUTS_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                INTENT_BASE_ENTRY_TYPE,
                link_type: PROCESS_INTENT_OUTPUTS_LINK_TYPE,
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
        intent_entry_def(),
        intent_base_entry_def(),
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
        create_intent: {
            inputs: |intent: CreateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_create_intent
        }
        get_intent: {
            inputs: |address: IntentAddress|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_get_intent
        }
        update_intent: {
            inputs: |intent: UpdateRequest|,
            outputs: |result: ZomeApiResult<ResponseData>|,
            handler: receive_update_intent
        }
        delete_intent: {
            inputs: |address: IntentAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_intent
        }

        query_intents: {
            inputs: |params: QueryParams|,
            outputs: |result: ZomeApiResult<Vec<ResponseData>>|,
            handler: receive_query_intents
        }
    ]

    traits: {
        hc_public [
            create_intent,
            get_intent,
            update_intent,
            delete_intent,
            query_intents
        ]
    }
}
