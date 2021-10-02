/**
 * satisfaction query indexes for observation DNA
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

const READ_FN_NAME: &str = "get_satisfaction";

#[hdk_extern]
fn query_satisfactions(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        match &params.satisfied_by {
            Some(satisfied_by) => {
                entries_result = query_index::<ResponseData, SatisfactionAddress, _,_,_,_,_,_>(
                    &EVENT_ENTRY_TYPE,
                    satisfied_by, EVENT_SATISFIES_LINK_TAG,
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
fn _internal_reindex_satisfiedby(indexes: RemoteEntryLinkRequest<EventAddress, SatisfactionAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &EVENT_ENTRY_TYPE, &remote_entry,
        &SATISFACTION_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &EVENT_SATISFIES_LINK_TAG, &SATISFACTION_SATISFIEDBY_LINK_TAG,
    )?)
}
