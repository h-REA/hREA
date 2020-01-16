#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_observation;
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
};

use hdk_proc_macros::zome;

use vf_planning::type_aliases::{FulfillmentAddress};
use vf_planning::fulfillment::{
    Entry as FulfillmentEntry,
    CreateRequest as FulfillmentCreateRequest,
    UpdateRequest as FulfillmentUpdateRequest,
    ResponseData as FulfillmentResponse,
};
use fulfillment_requests::{
    QueryParams,
    receive_create_fulfillment,
    receive_get_fulfillment,
    receive_update_fulfillment,
    receive_delete_fulfillment,
    receive_query_fulfillments,
};
use hc_zome_rea_economic_event_storage_consts::{
    EVENT_BASE_ENTRY_TYPE,
};
use vf_planning::identifiers::{
    FULFILLMENT_BASE_ENTRY_TYPE,
    FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,
    FULFILLMENT_ENTRY_TYPE,
    FULFILLMENT_FULFILLEDBY_LINK_TYPE,
};

// Entry type definitions





#[zome]
mod rea_fulfillment_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
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

    #[entry_def]
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
                    link_type: FULFILLMENT_INITIAL_ENTRY_LINK_TYPE,

                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },

                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    EVENT_BASE_ENTRY_TYPE,
                    link_type: FULFILLMENT_FULFILLEDBY_LINK_TYPE,

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
    fn fulfillment_created(fulfillment: FulfillmentCreateRequest) -> ZomeApiResult<FulfillmentResponse> {
        receive_create_fulfillment(fulfillment)
    }

    #[zome_fn("hc_public")]
    fn fulfillment_updated(fulfillment: FulfillmentUpdateRequest) -> ZomeApiResult<FulfillmentResponse> {
        receive_update_fulfillment(fulfillment)
    }

    #[zome_fn("hc_public")]
    fn fulfillment_deleted(address: FulfillmentAddress) -> ZomeApiResult<bool> {
        receive_delete_fulfillment(address)
    }

    #[zome_fn("hc_public")]
    fn get_fulfillment(address: FulfillmentAddress) -> ZomeApiResult<FulfillmentResponse> {
        receive_get_fulfillment(address)
    }

    #[zome_fn("hc_public")]
    fn query_fulfillments(params: QueryParams) -> ZomeApiResult<Vec<FulfillmentResponse>> {
        receive_query_fulfillments(params)
    }

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    //   }


}
