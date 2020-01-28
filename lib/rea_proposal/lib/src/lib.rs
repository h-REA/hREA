/**
 * Holo-REA proposal zome library API
 *
 * Contains helper methods that can be used to manipulate `Proposal` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::error::{ ZomeApiResult, ZomeApiError };

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        // get_linked_addresses_as_type,
        get_linked_addresses_with_foreign_key_as_type,
    },
    local_indexes::{
        query_direct_index_with_foreign_key,
        // query_direct_remote_index_with_foreign_key,
    },
    // remote_indexes::{
        // RemoteEntryLinkResponse,
        // handle_sync_direct_remote_index_destination,
    // },
};

use vf_core::type_aliases::ProposedIntentAddress;

use hc_zome_rea_proposal_storage_consts::*;
use hc_zome_rea_proposal_storage::*;
use hc_zome_rea_proposal_rpc::*;

// use hc_zome_TODO_storage_consts::{
//     TODO_PARENT_OF_LINK_TYPE, TODO_PARENT_OF_LINK_TAG,
// };

pub fn receive_create_proposal(proposal: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_proposal(&proposal)
}

pub fn receive_get_proposal(address: ProposalAddress) -> ZomeApiResult<ResponseData> {
    handle_get_proposal(&address)
}

pub fn receive_update_proposal(proposal: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_proposal(&proposal)
}

pub fn receive_delete_proposal(address: ProposalAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_proposals(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_proposals(&params)
}

fn handle_get_proposal(address: &ProposalAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?, get_link_fields(address)))
}

fn handle_create_proposal(proposal: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProposalAddress, Entry) = create_record(
        PROPOSAL_BASE_ENTRY_TYPE, PROPOSAL_ENTRY_TYPE,
        PROPOSAL_INITIAL_ENTRY_LINK_TYPE,
        proposal.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_proposal(proposal: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = proposal.get_id();
    let new_entry = update_record(PROPOSAL_ENTRY_TYPE, base_address, proposal)?;
    Ok(construct_response(&base_address, &new_entry, get_link_fields(base_address)))
}

fn handle_query_proposals(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(ProposalAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: replace with real query filter logic
    match &params.publishes {
        Some(publishes) => {
            entries_result = query_direct_index_with_foreign_key(publishes, PROPOSAL_PUBLISHES_LINK_TYPE, PROPOSAL_PUBLISHES_LINK_TAG);
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                            get_link_fields(entry_base_address),
                        )),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ProposalAddress,
    e: &Entry,
    publishes: Option<Cow<'a, Vec<ProposedIntentAddress>>>
) -> ResponseData {
    ResponseData {
        proposal: Response {
            // entry fields
            id: address.to_owned(),
            name: e.name.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            unit_based: e.unit_based.to_owned(),
            created: e.created.to_owned(),
            note: e.note.to_owned(),
            // link fields
            publishes: publishes.map(Cow::into_owned),
        }
    }
}

pub fn get_link_fields <'a> ( proposal: &ProposalAddress ) -> Option<Cow<'a, Vec<ProposedIntentAddress>>> {
    Some(get_published_ids(proposal))
}


fn get_published_ids <'a> (p_to: &ProposalAddress) -> Cow<'a, Vec<ProposedIntentAddress>> {
    get_linked_addresses_with_foreign_key_as_type(p_to, PROPOSAL_PUBLISHES_LINK_TYPE, PROPOSAL_PUBLISHES_LINK_TAG)
}
