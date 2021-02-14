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
        sync_remote_index,
    },
};

use vf_core::type_aliases::{ AgreementAddress, CommitmentAddress };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_ENTRY_TYPE, AGREEMENT_COMMITMENTS_LINK_TAG };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG };

entry_defs![ Path::entry_def() ];

#[hdk_extern]
fn index_commitments(base_entry: CommitmentAddress, target_entries: Vec<AgreementAddress>, removed_entries: Vec<AgreementAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    Ok(sync_remote_index(
        &AGREEMENT_ENTRY_TYPE, base_entry.as_ref(),
        &COMMITMENT_ENTRY_TYPE,
        target_entries.iter().map(|e| *e.as_ref()).collect::<Vec<EntryHash>>().as_slice(),
        removed_entries.iter().map(|e| *e.as_ref()).collect::<Vec<EntryHash>>().as_slice(),
        &AGREEMENT_COMMITMENTS_LINK_TAG, &COMMITMENT_CLAUSE_OF_LINK_TAG,
    )?)
}
