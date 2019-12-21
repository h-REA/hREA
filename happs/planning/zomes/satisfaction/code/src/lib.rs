#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;

mod satisfaction_requests;
use hdk_proc_macros::zome;

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

use vf_planning::type_aliases::{ SatisfactionAddress };
use vf_planning::satisfaction::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
};

use satisfaction_requests::{
    QueryParams,
    receive_create_satisfaction,
    receive_get_satisfaction,
    receive_delete_satisfaction,
    receive_query_satisfactions,
    receive_update_satisfaction,
};
use vf_planning::identifiers::{
    SATISFACTION_BASE_ENTRY_TYPE,
    SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
    SATISFACTION_ENTRY_TYPE,
    SATISFACTION_SATISFIES_LINK_TYPE,
    INTENT_BASE_ENTRY_TYPE,
    SATISFACTION_SATISFIEDBY_LINK_TYPE,
    COMMITMENT_BASE_ENTRY_TYPE,
};

#[zome]
mod rea_satisfaction_zome {

    #[init]
    fn init() {
        Ok(())
    }
    
    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn satisfaction_entry_def() -> ValidatingEntryType {
        entry!(
            name: SATISFACTION_ENTRY_TYPE,
            description: "Represents many-to-many relationships between intents and commitments or economic events that fully or partially satisfy one or more intents.",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Entry>| {
                Ok(())
            }
        )
    }

    #[entry_def]
    fn satisfaction_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: SATISFACTION_BASE_ENTRY_TYPE,
            description: "Base anchor for initial satisfaction addresses to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    SATISFACTION_ENTRY_TYPE,
                    link_type: SATISFACTION_INITIAL_ENTRY_LINK_TYPE,
    
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
    
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: SATISFACTION_SATISFIEDBY_LINK_TYPE,
    
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
    
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    INTENT_BASE_ENTRY_TYPE,
                    link_type: SATISFACTION_SATISFIES_LINK_TYPE,
    
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
    fn create_satisfaction(satisfaction: CreateRequest) -> ZomeApiResult<Response> {
        receive_create_satisfaction(satisfaction)
    }

    #[zome_fn("hc_public")]
    fn get_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<Response> {
        receive_get_satisfaction(address)
    }

    #[zome_fn("hc_public")]
    fn update_satisfaction(satisfaction: UpdateRequest) -> ZomeApiResult<Response> {
        receive_update_satisfaction(satisfaction)
    }

    #[zome_fn("hc_public")]
    fn delete_satisfaction(address: SatisfactionAddress) -> ZomeApiResult<bool> {
        receive_delete_satisfaction(address)
    }

    #[zome_fn("hc_public")]
    fn query_satisfactions(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
        receive_query_satisfactions(params)
    }

    //:TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // } 

}