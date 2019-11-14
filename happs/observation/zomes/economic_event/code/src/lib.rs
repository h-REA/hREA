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

extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_observation;
mod economic_event_requests;

use hdk::prelude::*;

use vf_observation::type_aliases::EventAddress;
use vf_observation::economic_event::{
    Entry as EconomicEventEntry,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
    ResponseData as EconomicEventResponse,
};
use vf_observation::economic_resource::{
    CreateRequest as EconomicResourceCreateRequest,
};
use economic_event_requests::{
    QueryParams,
    receive_get_economic_event,
    receive_create_economic_event,
    receive_update_economic_event,
    receive_delete_economic_event,
    receive_query_events,
};
use vf_observation::identifiers::{
    EVENT_BASE_ENTRY_TYPE,
    EVENT_INITIAL_ENTRY_LINK_TYPE,
    EVENT_ENTRY_TYPE,
    EVENT_FULFILLS_LINK_TYPE,
    EVENT_SATISFIES_LINK_TYPE,
    PROCESS_BASE_ENTRY_TYPE,
    EVENT_INPUT_OF_LINK_TYPE,
    EVENT_OUTPUT_OF_LINK_TYPE,
};
use vf_planning::identifiers::{
    FULFILLMENT_BASE_ENTRY_TYPE,
    SATISFACTION_BASE_ENTRY_TYPE,
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
        validation: |validation_data: hdk::EntryValidationData<EconomicEventEntry>| {
            // CREATE
            if let EntryValidationData::Create{ entry, validation_data: _ } = validation_data {
                let record: EconomicEventEntry = entry;
                if !(record.resource_inventoried_as.is_some() || record.resource_classified_as.is_some() || record.resource_conforms_to.is_some()) {
                    return Err("Event must reference an inventoried resource, resource specification or resource classification".into());
                }
                if !(record.resource_quantity.is_some() || record.effort_quantity.is_some()) {
                    return Err("Event must include either a resource quantity or an effort quantity".into());
                }
                if !(record.has_beginning.is_some() || record.has_end.is_some() || record.has_point_in_time.is_some()) {
                    return Err("Event must have a beginning, end or exact time".into());
                }
            }

            // UPDATE
            // if let EntryValidationData::Modify{ new_entry, old_entry, old_entry_header: _, validation_data: _ } = validation_data {

            // }

            // DELETE
            // if let EntryValidationData::Delete{ old_entry, old_entry_header: _, validation_data: _ } = validation_data {

            // }

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
                link_type: EVENT_INITIAL_ENTRY_LINK_TYPE,

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
            ),
            to!(
                SATISFACTION_BASE_ENTRY_TYPE,
                link_type: EVENT_SATISFIES_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: EVENT_INPUT_OF_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                PROCESS_BASE_ENTRY_TYPE,
                link_type: EVENT_OUTPUT_OF_LINK_TYPE,
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
            inputs: |event: EconomicEventCreateRequest, new_inventoried_resource: Option<EconomicResourceCreateRequest>|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: receive_create_economic_event
        }
        get_event: {
            inputs: |address: EventAddress|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: receive_get_economic_event
        }
        update_event: {
            inputs: |event: EconomicEventUpdateRequest|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: receive_update_economic_event
        }
        delete_event: {
            inputs: |address: EventAddress|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: receive_delete_economic_event
        }
        query_events: {
            inputs: |params: QueryParams|,
            outputs: |result: ZomeApiResult<Vec<EconomicEventResponse>>|,
            handler: receive_query_events
        }
    ]

    traits: {
        hc_public [
            create_event,
            get_event,
            update_event,
            delete_event,
            query_events
        ]
    }
}
