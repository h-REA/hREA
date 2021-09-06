/**
 * satisfaction query indexes for observation DNA
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

use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_lib_destination::generate_query_handler;
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_economic_event_storage_consts::{ EVENT_ENTRY_TYPE, EVENT_SATISFIES_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub satisfaction_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.satisfaction_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_satisfactions(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        EVENT_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_reindex_satisfiedby(indexes: RemoteEntryLinkRequest<EventAddress, SatisfactionAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &SATISFACTION_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_SATISFIES_LINK_TAG, &SATISFACTION_SATISFIEDBY_LINK_TAG,
    )?)
}
