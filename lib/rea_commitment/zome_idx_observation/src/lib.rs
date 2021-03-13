/**
 * Holo-REA commitment index zome API definition
 *
 * Provides remote indexing capability for the commitments of process records.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use vf_attributes_hdk::{ ProcessAddress, CommitmentAddress };
use hdk_records::{
    remote_indexes::{
        handle_sync_direct_remote_index_destination,
    },
};
use hc_zome_rea_commitment_storage_consts::{COMMITMENT_ENTRY_TYPE, COMMITMENT_INPUT_OF_LINK_TAG, COMMITMENT_OUTPUT_OF_LINK_TAG};
use hc_zome_rea_process_storage_consts::{PROCESS_COMMITMENT_INPUTS_LINK_TAG, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG};

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
fn index_committed_inputs(base_entry: CommitmentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    handle_sync_direct_remote_index_destination(
        COMMITMENT_ENTRY_TYPE,
        COMMITMENT_INPUT_OF_LINK_TAG,
        PROCESS_COMMITMENT_INPUTS_LINK_TAG,
        &base_entry, target_entries, removed_entries
    )
}

#[hdk_extern]
fn index_committed_outputs(base_entry: CommitmentAddress, target_entries: Vec<ProcessAddress>, removed_entries: Vec<ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    handle_sync_direct_remote_index_destination(
        COMMITMENT_ENTRY_TYPE,
        COMMITMENT_OUTPUT_OF_LINK_TAG,
        PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
        &base_entry, target_entries, removed_entries
    )
}
