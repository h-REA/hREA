/**
 * Agreement query indexes for agreement DNA
 *
 * :TODO:
 *
 * @package Holo-REA
 * @since   2021-09-06
 */
use hdk::prelude::*;
use hdk_semantic_indexes_zome_lib::{
    ByAddress,
    IndexingZomeConfig,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    read_index,
    sync_index,
};

use hc_zome_rea_agreement_rpc::*;
// use hc_zome_rea_agreement_lib::generate_query_handler;
use hc_zome_rea_agreement_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_REALIZATION_OF_LINK_TAG };
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_CLAUSE_OF_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub agreement_index: IndexingZomeConfig,
}

/*
fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.agreement_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_agreements(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
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
*/

#[hdk_extern]
fn _internal_read_agreement_realizations(ByAddress { address }: ByAddress<AgreementAddress>) -> ExternResult<Vec<EventAddress>> {
    Ok(read_index(&AGREEMENT_ENTRY_TYPE, &address, &AGREEMENT_EVENTS_LINK_TAG)?)
}

#[hdk_extern]
fn index_realized_events(indexes: RemoteEntryLinkRequest<EventAddress, AgreementAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &AGREEMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        EVENT_REALIZATION_OF_LINK_TAG, AGREEMENT_EVENTS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_agreement_clauses(ByAddress { address }: ByAddress<AgreementAddress>) -> ExternResult<Vec<CommitmentAddress>> {
    Ok(read_index(&AGREEMENT_ENTRY_TYPE, &address, &AGREEMENT_COMMITMENTS_LINK_TAG)?)
}

#[hdk_extern]
fn index_agreement_clauses(indexes: RemoteEntryLinkRequest<CommitmentAddress, AgreementAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &AGREEMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        COMMITMENT_CLAUSE_OF_LINK_TAG, AGREEMENT_COMMITMENTS_LINK_TAG,
    )?)
}
