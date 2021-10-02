/**
 * ProposedIntent query indexes for proposal DNA
 *
 * @package Holo-REA
 * @since   2021-09-26
 */
use hdk::prelude::*;
use hdk_semantic_indexes_zome_lib::{
    IndexingZomeConfig, RecordAPIResult, DataIntegrityError,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    query_index,
    sync_index,
};

use hc_zome_rea_proposed_intent_rpc::*;
use hc_zome_rea_proposed_intent_storage_consts::*;
use hc_zome_rea_proposal_storage_consts::{ PROPOSAL_ENTRY_TYPE, PROPOSAL_PUBLISHES_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub proposed_intent: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_intent.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

const READ_FN_NAME: &str = "get_proposed_intent";

#[hdk_extern]
fn query_proposed_intents(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: replace with real query filter logic
    match &params.published_in {
        Some(published_in) => {
            entries_result = query_index::<ResponseData, ProposedIntentAddress, _,_,_,_,_,_>(
                &PROPOSAL_ENTRY_TYPE,
                published_in, PROPOSAL_PUBLISHES_LINK_TAG,
                &read_index_target_zome, &READ_FN_NAME,
            );
        }
        _ => (),
    };

    // :TODO: return errors for UI, rather than filtering
    Ok(entries_result?.iter()
        .cloned()
        .filter_map(Result::ok)
        .collect())
}

#[hdk_extern]
fn _internal_reindex_proposals(indexes: RemoteEntryLinkRequest<ProposalAddress, ProposedIntentAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROPOSAL_ENTRY_TYPE, &remote_entry,
        &PROPOSED_INTENT_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &PROPOSAL_PUBLISHES_LINK_TAG, &PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
    )?)
}
