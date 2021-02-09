/**
 * Holo-REA commitment index zome API definition
 *
 * Provides remote indexing capability for the commitments of agreement records.
 *
 * @package Holo-REA
 */
use hdk3::prelude::*;

use hdk_graph_helpers::{
    remote_indexes::{
        RemoteEntryLinkResponse,
        handle_sync_direct_remote_index_destination,
    },
};

use vf_core::type_aliases::{ AgreementAddress, CommitmentAddress };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_COMMITMENTS_LINK_TAG };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG };

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
fn index_commitments(base_entry: CommitmentAddress, target_entries: Vec<AgreementAddress>, removed_entries: Vec<AgreementAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    handle_sync_direct_remote_index_destination(
        COMMITMENT_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG, AGREEMENT_COMMITMENTS_LINK_TAG,
        &base_entry, target_entries, removed_entries
    )
}
