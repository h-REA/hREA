/**
 * ProposedTo query indexes for proposal DNA
 *
 * @package Holo-REA
 * @since   2021-09-26
 */
use hdk::prelude::*;
use hdk_semantic_indexes_zome_lib::{
    IndexingZomeConfig,
    RemoteEntryLinkRequest,
    RemoteEntryLinkResponse,
    sync_index,
};

use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_lib::generate_query_handler;
use hc_zome_rea_proposed_to_storage_consts::*;
use hc_zome_rea_proposal_storage_consts::{ PROPOSAL_ENTRY_TYPE, PROPOSAL_PUBLISHED_TO_LINK_TAG };

entry_defs![Path::entry_def()];

// :TODO: obviate this with zome-specific configs
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub proposed_to: IndexingZomeConfig,
}

fn read_index_target_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.record_storage_zome)
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchInputs {
    pub params: QueryParams,
}

#[hdk_extern]
fn query_proposed_tos(SearchInputs { params }: SearchInputs) -> ExternResult<Vec<ResponseData>>
{
    let handler = generate_query_handler(
        read_index_target_zome,
        PROPOSAL_ENTRY_TYPE,
    );

    Ok(handler(&params)?)
}

#[hdk_extern]
fn _internal_reindex_proposals(indexes: RemoteEntryLinkRequest<ProposalAddress, ProposedToAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_index(
        &PROPOSAL_ENTRY_TYPE, &remote_entry,
        &PROPOSED_TO_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &PROPOSAL_PUBLISHED_TO_LINK_TAG, &PROPOSED_TO_PROPOSED_TO_LINK_TAG,
    )?)
}
