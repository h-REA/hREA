/**
* Holo-REA proposal addresses zome library API
*
* Contains helper methods that can be used to manipulate `ProposedTo` data
* structures in either the local Holochain zome, or a separate DNA-local zome.
*
* @package Holo-REA
*/
use hdk::prelude::*;

use std::borrow::Cow;

use vf_core::type_aliases::{ProposalAddress, ProposedToAddress};

use hdk_records::{
    // links::get_linked_addresses_with_foreign_key_as_type,
    // remote_indexes::{
    // RemoteEntryLinkResponse,
    // handle_sync_direct_remote_index_destination,
    // },
    local_indexes::{create_index, delete_index, query_index},
    records::{create_record, delete_record, read_record_entry},
};

use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_storage::*;
use hc_zome_rea_proposed_to_storage_consts::*;

use hc_zome_rea_proposal_storage_consts::*;

pub fn receive_create_proposed_to(proposed_to: CreateRequest) -> GraphAPIResult<ResponseData> {
    handle_create_proposed_to(&proposed_to)
}

pub fn receive_get_proposed_to(address: ProposedToAddress) -> GraphAPIResult<ResponseData> {
    handle_get_proposed_to(&address)
}

pub fn receive_delete_proposed_to(address: ProposedToAddress) -> GraphAPIResult<bool> {
    let entry: Entry = read_record_entry(&address)?;
    delete_index(
        PROPOSED_TO_ENTRY_TYPE, address.as_ref(),
        PROPOSAL_ENTRY_TYPE, entry.proposed.as_ref(),
        PROPOSED_TO_PROPOSED_LINK_TAG,
        PROPOSAL_PUBLISHED_TO_LINK_TAG,
    )?;
    delete_record::<Entry>(&address)
}

pub fn receive_query_proposed_to(params: QueryParams) -> GraphAPIResult<Vec<ResponseData>> {
    handle_query_proposed_to(&params)
}

fn handle_get_proposed_to(address: &ProposedToAddress) -> GraphAPIResult<ResponseData> {
    let (revision, entry): (HeaderHash, Entry) = read_record_entry(PROPOSED_TO_ENTRY_TYPE, address)?;
    Ok(construct_response(address, &revision, &entry/*, get_link_fields(&address)*/))
}

fn handle_create_proposed_to(proposed_to: &CreateRequest) -> GraphAPIResult<ResponseData> {
    let (revision_id, base_address, entry_resp): (_, ProposedToAddress, Entry) = create_record(PROPOSED_TO_ENTRY_TYPE, proposed_to)?;
    create_index(
        PROPOSED_TO_ENTRY_TYPE, base_address.as_ref(),
        PROPOSAL_ENTRY_TYPE, proposed_to.proposed.as_ref(),
        PROPOSED_TO_PROPOSED_LINK_TAG,
        PROPOSAL_PUBLISHED_TO_LINK_TAG,
    )?;
    Ok(construct_response(&base_address, &revision_id, &entry_resp))
}

fn handle_query_proposed_to(params: &QueryParams) -> GraphAPIResult<Vec<ResponseData>> {
    let mut entries_result: GraphAPIResult<Vec<(ProposedToAddress, GraphAPIResult<Entry>)>> =
        Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.proposed {
        Some(proposed) => {
            entries_result = query_index(
                PROPOSAL_ENTRY_TYPE, proposed,
                PROPOSAL_PUBLISHED_TO_LINK_TAG,
            );
        }
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(entries
            .iter()
            .map(|(entry_base_address, maybe_entry)| match maybe_entry {
                Some(entry) => Ok(construct_response(entry_base_address, &entry)),
                None => Err(ZomeApiError::Internal(
                    "referenced entry not found".to_string(),
                )),
            })
            .filter_map(Result::ok)
            .collect()),
        _ => Err(ZomeApiError::Internal(
            "could not load linked addresses".to_string(),
        )),
    }
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(address: &ProposedToAddress, revision_id: &HeaderHash, e: &Entry) -> ResponseData {
    ResponseData {
        proposed_to: Response {
            id: address.to_owned(),
            revision_id: revisin_id.to_owned(),
            proposed_to: e.proposed_to.to_owned(),
            proposed: e.proposed.to_owned(),
        },
    }
}

pub fn get_link_fields<'a>(p_to: &ProposedToAddress) -> Option<Cow<'a, Vec<ProposalAddress>>> {
    Some(get_proposed_ids(p_to))
}

fn get_proposed_ids<'a>(p_to: &ProposedToAddress) -> Cow<'a, Vec<ProposalAddress>> {
    get_linked_addresses_with_foreign_key_as_type(
        p_to,
        PROPOSED_TO_PROPOSED_LINK_TYPE,
        PROPOSED_TO_PROPOSED_LINK_TAG,
    )
}
