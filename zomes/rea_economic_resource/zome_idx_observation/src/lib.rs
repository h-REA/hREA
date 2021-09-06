/**
 * Resource query indexes for observation DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk::prelude::*;
use hdk_records::{
    index_retrieval::{
        ByAddress,
        IndexingZomeConfig,
    },
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
    local_indexes::{
        read_index,
    },
};

use hc_zome_rea_economic_resource_rpc::QueryParams;
use hc_zome_rea_economic_event_rpc::{ EventAddress, ResourceAddress, ResourceResponseData as ResponseData };
use hc_zome_rea_economic_resource_lib::{PROCESS_ENTRY_TYPE, generate_query_handler};
use hc_zome_rea_economic_resource_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_AFFECTS_RESOURCE_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub economic_resource_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.economic_resource_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_resources(params: QueryParams) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        EVENT_ENTRY_TYPE,
        PROCESS_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_reindex_affecting_events(indexes: RemoteEntryLinkRequest<EventAddress, ResourceAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &RESOURCE_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_AFFECTS_RESOURCE_LINK_TAG, &RESOURCE_AFFECTED_BY_EVENT_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_container_resource(ByAddress { address }: ByAddress<ResourceAddress>) -> ExternResult<Vec<ResourceAddress>> {
    Ok(read_index(&RESOURCE_ENTRY_TYPE, &address, &RESOURCE_CONTAINED_IN_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_container_resources(indexes: RemoteEntryLinkRequest<ResourceAddress, ResourceAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &RESOURCE_ENTRY_TYPE, &remote_entry,
        &RESOURCE_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &RESOURCE_CONTAINS_LINK_TAG, &RESOURCE_CONTAINED_IN_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_contained_resources(ByAddress { address }: ByAddress<ResourceAddress>) -> ExternResult<Vec<ResourceAddress>> {
    Ok(read_index(&RESOURCE_ENTRY_TYPE, &address, &RESOURCE_CONTAINS_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_contained_resources(indexes: RemoteEntryLinkRequest<ResourceAddress, ResourceAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &RESOURCE_ENTRY_TYPE, &remote_entry,
        &RESOURCE_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &RESOURCE_CONTAINED_IN_LINK_TAG, &RESOURCE_CONTAINS_LINK_TAG,
    )?)
}
