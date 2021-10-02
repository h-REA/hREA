/**
 * Resource query indexes for observation DNA
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
    // query_index,
    sync_index,
};

use hc_zome_rea_economic_resource_rpc::QueryParams;
use hc_zome_rea_economic_event_rpc::{ EventAddress, ResourceAddress, ResourceSpecificationAddress, ResourceResponseData as ResponseData };
use hc_zome_rea_economic_resource_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_AFFECTS_RESOURCE_LINK_TAG };
use hc_zome_rea_resource_specification_storage_consts::{ ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub economic_resource_index: IndexingZomeConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

// const READ_FN_NAME: &str = "get_resource";

// fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
//     Some(conf.economic_resource_index.record_storage_zome)
// }

#[hdk_extern]
fn query_resources(_params: QueryParams) -> ExternResult<Vec<ResponseData>>
{
    let entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    /* :TODO:
    match &params.contains {
        Some(contains) => {
            entries_result = query_direct_index_with_foreign_key(
                &contains, RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.contained_in {
        Some(contained_in) => {
            entries_result = query_direct_index_with_foreign_key(
                contained_in, RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.conforms_to {
        Some(conforms_to) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                conforms_to, ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG,
            );
        },
        _ => (),
    };
    */

    // :TODO: return errors for UI, rather than filtering
    Ok(entries_result?.iter()
        .cloned()
        .filter_map(Result::ok)
        .collect())
}

#[hdk_extern]
fn _internal_read_affecting_events(ByAddress { address }: ByAddress<ResourceAddress>) -> ExternResult<Vec<EventAddress>> {
    Ok(read_index(&RESOURCE_ENTRY_TYPE, &address, &RESOURCE_AFFECTED_BY_EVENT_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_affecting_events(indexes: RemoteEntryLinkRequest<EventAddress, ResourceAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
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

    Ok(sync_index(
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

    Ok(sync_index(
        &RESOURCE_ENTRY_TYPE, &remote_entry,
        &RESOURCE_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &RESOURCE_CONTAINED_IN_LINK_TAG, &RESOURCE_CONTAINS_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_resource_specifications(indexes: RemoteEntryLinkRequest<ResourceSpecificationAddress, ResourceAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE, &remote_entry,
        &RESOURCE_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG, &RESOURCE_CONFORMS_TO_LINK_TAG,
    )?)
}
