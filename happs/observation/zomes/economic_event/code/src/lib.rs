#![feature(proc_macro_hygiene)]
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
extern crate vf_planning;
extern crate vf_specification;
mod economic_event_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

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

#[zome]
mod rea_economic_event_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
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
                    let result = record.validate_or_fields();
                    if result.is_ok() {
                        return record.validate_action();
                    }
                    return result;
                }

                // UPDATE
                if let EntryValidationData::Modify{ new_entry, old_entry: _, old_entry_header: _, validation_data: _ } = validation_data {
                    let record: EconomicEventEntry = new_entry;
                    let result = record.validate_or_fields();
                    if result.is_ok() {
                        return record.validate_action();
                    }
                    return result;
                }

                // DELETE
                // if let EntryValidationData::Delete{ old_entry, old_entry_header: _, validation_data: _ } = validation_data {

                // }

                Ok(())
            }
        )
    }

    #[entry_def]
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

    #[zome_fn("hc_public")]
    fn create_event(event: EconomicEventCreateRequest, new_inventoried_resource: Option<EconomicResourceCreateRequest>) -> ZomeApiResult<EconomicEventResponse> {
        receive_create_economic_event(event, new_inventoried_resource)
    }

    #[zome_fn("hc_public")]
    fn get_event(address: EventAddress) -> ZomeApiResult<EconomicEventResponse> {
        receive_get_economic_event(address)
    }

    #[zome_fn("hc_public")]
    fn update_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
        receive_update_economic_event(event)

    }

    #[zome_fn("hc_public")]
    fn delete_event(address: EventAddress) -> ZomeApiResult<bool> {
        receive_delete_economic_event(address)
    }

    #[zome_fn("hc_public")]
    fn query_events(params: QueryParams) -> ZomeApiResult<Vec<EconomicEventResponse>> {
        receive_query_events(params)
    }



    // :TODO:
    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }



}
