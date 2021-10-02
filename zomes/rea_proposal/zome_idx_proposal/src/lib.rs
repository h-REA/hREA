/**
 * Proposal query indexes for proposal DNA
 *
 * @package Holo-REA
 * @since   2021-09-21
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

use hc_zome_rea_proposal_rpc::*;
use hc_zome_rea_proposal_storage_consts::*;
use hc_zome_rea_proposed_intent_storage_consts::{PROPOSED_INTENT_ENTRY_TYPE, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG};
use hc_zome_rea_proposed_to_storage_consts::{PROPOSED_TO_ENTRY_TYPE, PROPOSED_TO_PROPOSED_LINK_TAG};

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub proposal_index: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposal_index.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

const READ_FN_NAME: &str = "get_proposal";

#[hdk_extern]
fn query_proposals(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        match &params.publishes {
            Some(publishes) => {
                entries_result = query_index::<ResponseData, ProposalAddress, _,_,_,_,_,_>(
                    &PROPOSED_INTENT_ENTRY_TYPE,
                    publishes, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
                    &read_index_target_zome, &READ_FN_NAME
                );
            }
            _ => (),
        };

        match &params.published_to {
            Some(published_to) => {
                entries_result = query_index::<ResponseData, ProposalAddress, _,_,_,_,_,_>(
                    &PROPOSED_TO_ENTRY_TYPE,
                    published_to, PROPOSED_TO_PROPOSED_LINK_TAG,
                    &read_index_target_zome, &READ_FN_NAME
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
fn _internal_read_proposal_proposed_intents(ByAddress { address }: ByAddress<ProposalAddress>) -> ExternResult<Vec<ProposedIntentAddress>> {
    Ok(read_index(&PROPOSAL_ENTRY_TYPE, &address, &PROPOSAL_PUBLISHES_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_proposed_intents(indexes: RemoteEntryLinkRequest<ProposedIntentAddress, ProposalAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROPOSED_INTENT_ENTRY_TYPE, &remote_entry,
        &PROPOSAL_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG, PROPOSAL_PUBLISHES_LINK_TAG,
    )?)
}

#[hdk_extern]
fn _internal_read_proposal_participants(ByAddress { address }: ByAddress<ProposalAddress>) -> ExternResult<Vec<ProposedToAddress>> {
    Ok(read_index(&PROPOSAL_ENTRY_TYPE, &address, &PROPOSAL_PUBLISHED_TO_LINK_TAG)?)
}

#[hdk_extern]
fn _internal_reindex_proposed_to(indexes: RemoteEntryLinkRequest<ProposedToAddress, ProposalAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROPOSED_TO_ENTRY_TYPE, &remote_entry,
        &PROPOSAL_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        PROPOSED_TO_PROPOSED_LINK_TAG, PROPOSAL_PUBLISHED_TO_LINK_TAG,
    )?)
}