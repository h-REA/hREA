// :TODO: documentation
#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_planning;
extern crate vf_observation;

mod intent_requests;

use hdk::prelude::*;
use hdk_proc_macros::zome;

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
};

#[zome]
mod rea_planning_intent_zome {
    
    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data : EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn intent_entry_def() -> ValidatingEntryType {
        entry!(
            name: INTENT_ENTRY_TYPE,
            description: "A planned economic flow, which can lead to economic events (sometimes through commitments).",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |validation_data: hdk::EntryValidationData<Entry>| {
                // CREATE
                if let EntryValidationData::Create{ entry, validation_data: _ } = validation_data {
                    let record: Entry = entry;
                    let result = record.validate_or_fields();
                    if result.is_ok() {
                        return record.validate_action();
                    }
                    return result;
                }
    
                // UPDATE
                if let EntryValidationData::Modify{ new_entry, old_entry: _, old_entry_header: _, validation_data: _ } = validation_data {
                    let record: Entry = new_entry;
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

    #[zome_fn("hc_public")]
    fn create_intent(intent: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_intent(intent)
    }

    #[zome_fn("hc_public")]
    fn get_intent(address: IntentAddress) -> ZomeApiResult<ResponseData> {
        receive_get_intent(address)
    }

    #[zome_fn("hc_public")]
    fn update_intent(intent: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_intent(intent)
    }

    #[zome_fn("hc_public")]
    fn delete_intent(address: IntentAddress) -> ZomeApiResult<bool> {
        receive_delete_intent(address)
    }

    #[zome_fn("hc_public")]
    fn query_intents(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
        receive_query_intents(params)
    }

    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }

}





// :TODO: there should be a process entry type def here, but it crashes the DNA
// to have conflicting entry types stored across zomes in the same DNA.
