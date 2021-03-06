/**
 * Holo-REA commitment index zome API definition
 *
 * Provides remote indexing capability for the commitments of agreement records.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_graph_helpers::{
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_ENTRY_TYPE, AGREEMENT_COMMITMENTS_LINK_TAG };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG };

entry_defs![ Path::entry_def() ];

#[hdk_extern]
fn index_commitments(indexes: RemoteEntryLinkRequest) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &AGREEMENT_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &AGREEMENT_COMMITMENTS_LINK_TAG, &COMMITMENT_CLAUSE_OF_LINK_TAG,
    )?)
}
