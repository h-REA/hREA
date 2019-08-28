/**
 * Observations zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 */

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_observation;
mod economic_event_requests;

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
use vf_observation::economic_event::{
    Entry as EconomicEventEntry,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
    ResponseData as EconomicEventResponse,
};
use economic_event_requests::{
    handle_get_economic_event,
    handle_create_economic_event,
    handle_update_economic_event,
    handle_delete_economic_event,
};
use vf_observation::identifiers::{
    EVENT_BASE_ENTRY_TYPE,
    EVENT_ENTRY_TYPE,
    EVENT_FULFILLS_LINK_TYPE,
    EVENT_FULFILLS_LINK_TAG,
};
use vf_planning::identifiers::{
    FULFILLMENT_BASE_ENTRY_TYPE,
};

// Zome entry type wrappers

fn event_entry_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_ENTRY_TYPE,
        description: "An observed economic flow, as opposed to a flow planned to happen in the future. This could reflect a change in the quantity of an economic resource. It is also defined by its behavior (action) in relation to the economic resource.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<EconomicEventEntry>| {
            Ok(())
        }
    )
}

fn event_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial event addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                EVENT_ENTRY_TYPE,
                link_type: LINK_TYPE_INITIAL_ENTRY,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                FULFILLMENT_BASE_ENTRY_TYPE,
                link_type: EVENT_FULFILLS_LINK_TYPE,

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
       event_entry_def(),
       event_base_entry_def()
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
        create_event: {
            inputs: |event: EconomicEventCreateRequest|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: handle_create_economic_event
        }
        get_event: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: handle_get_economic_event
        }
        update_event: {
            inputs: |event: EconomicEventUpdateRequest|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: handle_update_economic_event
        }
        delete_event: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_delete_economic_event
        }
    ]

    traits: {
        hc_public [
            create_event,
            get_event,
            update_event,
            delete_event
        ]
    }
}
