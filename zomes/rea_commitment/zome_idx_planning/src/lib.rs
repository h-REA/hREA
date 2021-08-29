/**
 * Commitment query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-28
 */
use hdk::prelude::*;
use hdk_records::{
    index_retrieval::IndexingZomeConfig,
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

use hc_zome_rea_commitment_rpc::*;
use hc_zome_rea_commitment_lib::generate_query_handler;
use hc_zome_rea_commitment_storage_consts::*;
use hc_zome_rea_fulfillment_storage_consts::{FULFILLMENT_ENTRY_TYPE, FULFILLMENT_FULFILLS_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_ENTRY_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_process_storage_consts::{ PROCESS_ENTRY_TYPE };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_ENTRY_TYPE };

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub commitment_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.commitment_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_commitments(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        PROCESS_ENTRY_TYPE,
        FULFILLMENT_ENTRY_TYPE,
        SATISFACTION_ENTRY_TYPE,
        AGREEMENT_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_reindex_fulfillments(indexes: RemoteEntryLinkRequest<FulfillmentAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &FULFILLMENT_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        FULFILLMENT_FULFILLS_LINK_TAG, COMMITMENT_FULFILLEDBY_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_satisfactions(indexes: RemoteEntryLinkRequest<SatisfactionAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &SATISFACTION_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        SATISFACTION_SATISFIEDBY_LINK_TAG, COMMITMENT_SATISFIES_LINK_TAG,
    )?)
}
