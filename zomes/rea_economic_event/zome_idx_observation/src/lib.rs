/**
 * Event query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
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

use hc_zome_rea_economic_event_rpc::*;
use hc_zome_rea_economic_event_lib::generate_query_handler;
use hc_zome_rea_economic_event_storage_consts::*;
use hc_zome_rea_process_storage_consts::{PROCESS_ENTRY_TYPE};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_ENTRY_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_agreement_storage_consts::{AGREEMENT_ENTRY_TYPE};
use hc_zome_rea_fulfillment_storage_consts::{ FULFILLMENT_ENTRY_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG };

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

#[hdk_extern]
fn query_events(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
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
fn _internal_reindex_fulfillments(indexes: RemoteEntryLinkRequest<FulfillmentAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &FULFILLMENT_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &FULFILLMENT_FULFILLEDBY_LINK_TAG, &EVENT_FULFILLS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_satisfactions(indexes: RemoteEntryLinkRequest<SatisfactionAddress, EventAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &SATISFACTION_ENTRY_TYPE, &remote_entry,
        &EVENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &SATISFACTION_SATISFIEDBY_LINK_TAG, &EVENT_SATISFIES_LINK_TAG,
    )?)
}
