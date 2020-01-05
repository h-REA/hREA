#![feature(proc_macro_hygiene)]
// :TODO: documentation

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate hdk_graph_helpers;
extern crate vf_observation;
extern crate vf_planning;

mod process_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address,
    holochain_core_types::dna::entry_types::Sharing,
    // holochain_json_api::{ json::JsonString, error::JsonError },
};
use hdk_graph_helpers::remote_indexes::RemoteEntryLinkResponse;
use hdk_proc_macros::zome;

use vf_observation::type_aliases::{
    ProcessAddress,
    CommitmentAddress,
    IntentAddress,
};
use vf_observation::process::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData,
};
use process_requests::{
    QueryParams,
    receive_get_process,
    receive_create_process,
    receive_update_process,
    receive_delete_process,
    receive_query_processes,
    receive_link_committed_inputs,
    receive_link_committed_outputs,
    receive_link_intended_inputs,
    receive_link_intended_outputs,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_INITIAL_ENTRY_LINK_TYPE,
    PROCESS_ENTRY_TYPE,
    EVENT_BASE_ENTRY_TYPE,
    PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TYPE,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TYPE,
};
use vf_planning::identifiers::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TYPE,
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TYPE,
};




// Zome entry type wrappers
#[zome]
mod rea_process_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn process_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROCESS_ENTRY_TYPE,
            description: "An activity that changes inputs into outputs.  It could transform or transport economic resource(s).",
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
    fn process_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: PROCESS_BASE_ENTRY_TYPE,
            description: "Base anchor for initial process addresses to provide lookup functionality",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    PROCESS_ENTRY_TYPE,
                    link_type: PROCESS_INITIAL_ENTRY_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    EVENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_EVENT_INPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    EVENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_EVENT_OUTPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_COMMITMENT_INPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    COMMITMENT_BASE_ENTRY_TYPE,
                    link_type: PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
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

    #[entry_def]
    fn commitment_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: COMMITMENT_BASE_ENTRY_TYPE,
            description: "Base anchor for commitments linking from external networks",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
                to!(
                    PROCESS_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_INPUT_OF_LINK_TYPE,
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    PROCESS_BASE_ENTRY_TYPE,
                    link_type: COMMITMENT_OUTPUT_OF_LINK_TYPE,
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

    #[entry_def]
    fn intent_base_entry_def() -> ValidatingEntryType {
        entry!(
            name: INTENT_BASE_ENTRY_TYPE,
            description: "Base anchor for intents linking from external networks",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<Address>| {
                Ok(())
            },
            links: [
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
    fn create_process(process: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_process(process)
    }

    #[zome_fn("hc_public")]
    fn get_process(address: ProcessAddress) -> ZomeApiResult<ResponseData> {
        receive_get_process(address)
    }

    #[zome_fn("hc_public")]
    fn update_process(process: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_process(process)
    }

    #[zome_fn("hc_public")]
    fn delete_process(address: ProcessAddress) -> ZomeApiResult<bool> {
        receive_delete_process(address)
    }

    #[zome_fn("hc_public")]
    fn query_processes(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_processes(params)
    }

    #[zome_fn("hc_public")]
    fn index_committed_inputs(base_entry: CommitmentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ZomeApiResult<RemoteEntryLinkResponse> {
        receive_link_committed_inputs(base_entry, target_entries, removed_entries)
    }

    #[zome_fn("hc_public")]
    fn index_committed_outputs(base_entry: CommitmentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ZomeApiResult<RemoteEntryLinkResponse> {
        receive_link_committed_outputs(base_entry, target_entries, removed_entries)
    }

    #[zome_fn("hc_public")]
    fn index_intended_inputs(base_entry: IntentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ZomeApiResult<RemoteEntryLinkResponse>{
        receive_link_intended_inputs(base_entry, target_entries, removed_entries)
    }

    #[zome_fn("hc_public")]
    fn index_intended_outputs(base_entry: IntentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ZomeApiResult<RemoteEntryLinkResponse>{
        receive_link_intended_outputs(base_entry, target_entries, removed_entries)
    }


    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    //   }

}
