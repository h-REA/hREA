/**
 * Event query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
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

use hc_zome_rea_economic_event_rpc::*;
use hc_zome_rea_economic_event_storage_consts::*;
use hc_zome_rea_economic_resource_storage_consts::{RESOURCE_ENTRY_TYPE, RESOURCE_AFFECTED_BY_EVENT_LINK_TAG};
use hc_zome_rea_process_storage_consts::{PROCESS_ENTRY_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG, PROCESS_EVENT_OUTPUTS_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_ENTRY_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_agreement_storage_consts::{AGREEMENT_ENTRY_TYPE, AGREEMENT_EVENTS_LINK_TAG};
use hc_zome_rea_fulfillment_storage_consts::{ FULFILLMENT_ENTRY_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub economic_event_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.economic_event_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

const READ_FN_NAME: &str = "get_event";

#[hdk_extern]
fn query_events(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: implement proper AND search rather than exclusive operations

    match &params.satisfies {
        Some(satisfies) =>
            entries_result = query_index::<ResponseData, EventAddress, _,_,_,_,_,_>(&SATISFACTION_ENTRY_TYPE, satisfies, &SATISFACTION_SATISFIEDBY_LINK_TAG, &read_index_target_zome, &READ_FN_NAME),
        _ => (),
    };
    match &params.fulfills {
        Some(fulfills) =>
            entries_result = query_index::<ResponseData, EventAddress, _,_,_,_,_,_>(&FULFILLMENT_ENTRY_TYPE, fulfills, &FULFILLMENT_FULFILLEDBY_LINK_TAG, &read_index_target_zome, &READ_FN_NAME),
        _ => (),
    };
    match &params.input_of {
        Some(input_of) =>
            entries_result = query_index::<ResponseData, EventAddress, _,_,_,_,_,_>(&PROCESS_ENTRY_TYPE, input_of, &PROCESS_EVENT_INPUTS_LINK_TAG, &read_index_target_zome, &READ_FN_NAME),
        _ => (),
    };
    match &params.output_of {
        Some(output_of) =>
            entries_result = query_index::<ResponseData, EventAddress, _,_,_,_,_,_>(&PROCESS_ENTRY_TYPE, output_of, &PROCESS_EVENT_OUTPUTS_LINK_TAG, &read_index_target_zome, &READ_FN_NAME),
        _ => (),
    };
    match &params.realization_of {
        Some(realization_of) =>
            entries_result = query_index::<ResponseData, EventAddress, _,_,_,_,_,_>(&AGREEMENT_ENTRY_TYPE, realization_of, &AGREEMENT_EVENTS_LINK_TAG, &read_index_target_zome, &READ_FN_NAME),
        _ => (),
    };

    // :TODO: return errors for UI, rather than filtering
    Ok(entries_result?.iter()
        .cloned()
        .filter_map(Result::ok)
        .collect())
}

#[hdk_extern]
fn _internal_read_affected_resources(ByAddress { address }: ByAddress<EventAddress>) -> ExternResult<Vec<ResourceAddress>> {
    Ok(read_index(&EVENT_ENTRY_TYPE, &address, &EVENT_AFFECTS_RESOURCE_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_affected_resources(indexes: RemoteEntryLinkRequest<ResourceAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &RESOURCE_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &RESOURCE_AFFECTED_BY_EVENT_LINK_TAG, &EVENT_AFFECTS_RESOURCE_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_process_inputs(indexes: RemoteEntryLinkRequest<ProcessAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROCESS_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &PROCESS_EVENT_INPUTS_LINK_TAG, &EVENT_INPUT_OF_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_process_outputs(indexes: RemoteEntryLinkRequest<ProcessAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROCESS_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &PROCESS_EVENT_OUTPUTS_LINK_TAG, &EVENT_OUTPUT_OF_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_event_fulfillments(ByAddress { address }: ByAddress<EventAddress>) -> ExternResult<Vec<FulfillmentAddress>> {
    Ok(read_index(&EVENT_ENTRY_TYPE, &address, &EVENT_FULFILLS_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_fulfillments(indexes: RemoteEntryLinkRequest<FulfillmentAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &FULFILLMENT_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &FULFILLMENT_FULFILLEDBY_LINK_TAG, &EVENT_FULFILLS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_event_satisfactions(ByAddress { address }: ByAddress<EventAddress>) -> ExternResult<Vec<SatisfactionAddress>> {
    Ok(read_index(&EVENT_ENTRY_TYPE, &address, &EVENT_SATISFIES_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_satisfactions(indexes: RemoteEntryLinkRequest<SatisfactionAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &SATISFACTION_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &SATISFACTION_SATISFIEDBY_LINK_TAG, &EVENT_SATISFIES_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_realized_agreements(ByAddress { address }: ByAddress<EventAddress>) -> ExternResult<Vec<AgreementAddress>> {
    Ok(read_index(&EVENT_ENTRY_TYPE, &address, &EVENT_REALIZATION_OF_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_realized_agreements(indexes: RemoteEntryLinkRequest<AgreementAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &AGREEMENT_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &AGREEMENT_EVENTS_LINK_TAG, &EVENT_REALIZATION_OF_LINK_TAG,
    )?)
}
