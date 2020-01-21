#![feature(proc_macro_hygiene)]
/**
 * REA `Process` zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk_graph_helpers::remote_indexes::RemoteEntryLinkResponse;

use hc_zome_rea_commitment_storage_consts::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TYPE,
};
use hc_zome_rea_intent_storage_consts::{
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TYPE,
};

use vf_core::type_aliases::{
    CommitmentAddress,
    IntentAddress,
};

use hc_zome_rea_process_defs::{ entry_def, base_entry_def };
use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_process_rpc::*;
use hc_zome_rea_process_lib::*;


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
        entry_def()
    }

    #[entry_def]
    fn process_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
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
