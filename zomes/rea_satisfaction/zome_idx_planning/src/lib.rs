/**
 * satisfaction query indexes for planning DNA
 *
 * @package Holo-REA
 * @since   2021-08-29
 */
use hdk::prelude::*;

use hdk_semantic_indexes_zome_lib::{
    IndexingZomeConfig, RecordAPIResult, DataIntegrityError,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    query_index,
    sync_index,
};

use hc_zome_rea_satisfaction_rpc::*;
use hc_zome_rea_satisfaction_storage_consts::*;
use hc_zome_rea_commitment_storage_consts::{ COMMITMENT_ENTRY_TYPE, COMMITMENT_SATISFIES_LINK_TAG };
use hc_zome_rea_intent_storage_consts::{ INTENT_ENTRY_TYPE, INTENT_SATISFIEDBY_LINK_TAG };

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

const READ_FN_NAME: &str = "get_satisfaction";

#[hdk_extern]
fn query_satisfactions(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = query_index::<ResponseData, SatisfactionAddress, _,_,_,_,_,_>(
                &INTENT_ENTRY_TYPE,
                satisfies, INTENT_SATISFIEDBY_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME,
            );
        },
        _ => (),
    };
    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = query_index::<ResponseData, SatisfactionAddress, _,_,_,_,_,_>(
                &COMMITMENT_ENTRY_TYPE,
                satisfied_by, COMMITMENT_SATISFIES_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME,
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
fn _internal_reindex_satisfiedby(indexes: RemoteEntryLinkRequest<CommitmentAddress, SatisfactionAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &SATISFACTION_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &COMMITMENT_SATISFIES_LINK_TAG, &SATISFACTION_SATISFIEDBY_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_reindex_intents(indexes: RemoteEntryLinkRequest<IntentAddress, SatisfactionAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &INTENT_ENTRY_TYPE, &remote_entry,
        &SATISFACTION_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &INTENT_SATISFIEDBY_LINK_TAG, &SATISFACTION_SATISFIES_LINK_TAG,
    )?)
}
