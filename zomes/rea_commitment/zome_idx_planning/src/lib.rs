/**
 * Commitment query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-28
 */
use hdk::prelude::*;
use hdk_semantic_indexes_zome_lib::{
    ByAddress, RecordAPIResult, DataIntegrityError,
    IndexingZomeConfig,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    read_index,
    query_index,
    sync_index,
};

use hc_zome_rea_commitment_rpc::*;
use hc_zome_rea_commitment_storage_consts::*;
use hc_zome_rea_fulfillment_storage_consts::{FULFILLMENT_ENTRY_TYPE, FULFILLMENT_FULFILLS_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_ENTRY_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_process_storage_consts::{ PROCESS_ENTRY_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG, PROCESS_COMMITMENT_INPUTS_LINK_TAG };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_ENTRY_TYPE, AGREEMENT_COMMITMENTS_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub commitment_index: IndexingZomeConfig,
}

const READ_FN_NAME: &str = "get_commitment";

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
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.fulfilled_by {
        Some(fulfilled_by) => {
            entries_result = query_index::<ResponseData, CommitmentAddress, _,_,_,_,_,_>(
                &FULFILLMENT_ENTRY_TYPE,
                fulfilled_by, FULFILLMENT_FULFILLS_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME
            );
        },
        _ => (),
    };
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = query_index::<ResponseData, CommitmentAddress, _,_,_,_,_,_>(
                &SATISFACTION_ENTRY_TYPE,
                satisfies, SATISFACTION_SATISFIEDBY_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = query_index::<ResponseData, CommitmentAddress, _,_,_,_,_,_>(
                &PROCESS_ENTRY_TYPE,
                input_of, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = query_index::<ResponseData, CommitmentAddress, _,_,_,_,_,_>(
                &PROCESS_ENTRY_TYPE,
                output_of, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME
            );
        },
        _ => (),
    };
    match &params.clause_of {
        Some(clause_of) => {
            entries_result = query_index::<ResponseData, CommitmentAddress, _,_,_,_,_,_>(
                &AGREEMENT_ENTRY_TYPE,
                clause_of, AGREEMENT_COMMITMENTS_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME
            );
        },
        _ => (),
    };

    // :TODO: return errors for UI, rather than filtering
    Ok(entries_result?.iter()
        .cloned()
        .filter_map(Result::ok)
        .collect())
}

#[hdk_extern]
fn _internal_read_commitment_process_inputs(ByAddress { address }: ByAddress<CommitmentAddress>) -> ExternResult<Vec<ProcessAddress>> {
    Ok(read_index(&COMMITMENT_ENTRY_TYPE, &address, &COMMITMENT_INPUT_OF_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_process_inputs(indexes: RemoteEntryLinkRequest<ProcessAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROCESS_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        PROCESS_COMMITMENT_INPUTS_LINK_TAG, COMMITMENT_INPUT_OF_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_commitment_process_outputs(ByAddress { address }: ByAddress<CommitmentAddress>) -> ExternResult<Vec<ProcessAddress>> {
    Ok(read_index(&COMMITMENT_ENTRY_TYPE, &address, &COMMITMENT_OUTPUT_OF_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_process_outputs(indexes: RemoteEntryLinkRequest<ProcessAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROCESS_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        PROCESS_COMMITMENT_OUTPUTS_LINK_TAG, COMMITMENT_OUTPUT_OF_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_commitment_fulfillments(ByAddress { address }: ByAddress<CommitmentAddress>) -> ExternResult<Vec<FulfillmentAddress>> {
    Ok(read_index(&COMMITMENT_ENTRY_TYPE, &address, &COMMITMENT_FULFILLEDBY_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_fulfillments(indexes: RemoteEntryLinkRequest<FulfillmentAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &FULFILLMENT_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        FULFILLMENT_FULFILLS_LINK_TAG, COMMITMENT_FULFILLEDBY_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_commitment_satisfactions(ByAddress { address }: ByAddress<CommitmentAddress>) -> ExternResult<Vec<SatisfactionAddress>> {
    Ok(read_index(&COMMITMENT_ENTRY_TYPE, &address, &COMMITMENT_SATISFIES_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_satisfactions(indexes: RemoteEntryLinkRequest<SatisfactionAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &SATISFACTION_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        SATISFACTION_SATISFIEDBY_LINK_TAG, COMMITMENT_SATISFIES_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_commitment_agreements(ByAddress { address }: ByAddress<CommitmentAddress>) -> ExternResult<Vec<AgreementAddress>> {
    Ok(read_index(&COMMITMENT_ENTRY_TYPE, &address, &COMMITMENT_CLAUSE_OF_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_agreement_clauses(indexes: RemoteEntryLinkRequest<AgreementAddress, CommitmentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &AGREEMENT_ENTRY_TYPE, &remote_entry,
        &COMMITMENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        AGREEMENT_COMMITMENTS_LINK_TAG, COMMITMENT_CLAUSE_OF_LINK_TAG,
    )?)
}
