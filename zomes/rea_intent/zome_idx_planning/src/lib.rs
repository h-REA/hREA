/**
 * Intent query indexes for planning DNA
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

use hc_zome_rea_intent_rpc::*;
use hc_zome_rea_intent_lib::generate_query_handler;
use hc_zome_rea_intent_storage_consts::*;
use hc_zome_rea_satisfaction_storage_consts::{ SATISFACTION_ENTRY_TYPE, SATISFACTION_SATISFIES_LINK_TAG };
use hc_zome_rea_process_storage_consts::{ PROCESS_ENTRY_TYPE };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub intent_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.intent_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_intents(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        SATISFACTION_ENTRY_TYPE,
        PROCESS_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_reindex_satisfactions(indexes: RemoteEntryLinkRequest<SatisfactionAddress, IntentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &SATISFACTION_ENTRY_TYPE, &remote_entry,
        &INTENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &SATISFACTION_SATISFIES_LINK_TAG, &INTENT_SATISFIEDBY_LINK_TAG,
    )?)
}
