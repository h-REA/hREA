/**
 * Holo-REA commitment index zome API definition
 *
 * Provides remote indexing capability for the commitments of process records.
 *
 * @package Holo-REA
 */
use hdk3::prelude::*;

use vf_core::type_aliases::{ ProcessAddress, IntentAddress };
use hdk_graph_helpers::{
    remote_indexes::{
        handle_sync_direct_remote_index_destination,
    },
};
use hc_zome_rea_intent_storage_consts::{INTENT_ENTRY_TYPE, INTENT_INPUT_OF_LINK_TAG, INTENT_OUTPUT_OF_LINK_TAG};
use hc_zome_rea_process_storage_consts::{PROCESS_INTENT_INPUTS_LINK_TAG, PROCESS_INTENT_OUTPUTS_LINK_TAG};

entry_defs![
    Path::entry_def(),
];

#[hdk_extern]
fn init() {
    Ok(())
}

#[hdk_extern]
pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
    Ok(())
}

#[hdk_extern]
fn index_intended_inputs(base_entry: IntentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse>{
    handle_sync_direct_remote_index_destination(
        INTENT_ENTRY_TYPE,
        INTENT_INPUT_OF_LINK_TAG,
        PROCESS_INTENT_INPUTS_LINK_TAG,
        &base_entry, target_entries, removed_entries
    )
}

#[hdk_extern]
fn index_intended_outputs(base_entry: IntentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse>{
    handle_sync_direct_remote_index_destination(
        INTENT_ENTRY_TYPE,
        INTENT_OUTPUT_OF_LINK_TAG,
        PROCESS_INTENT_OUTPUTS_LINK_TAG,
        &base_entry, target_entries, removed_entries
    )
}
