use hdk::error::{ZomeApiError, ZomeApiResult};
/**
 * Holo-REA proposal addresses zome library API
 *
 * Contains helper methods that can be used to manipulate `ProposedTo` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;

use vf_core::type_aliases::{ProposalAddress, ProposedToAddress};

use hdk_graph_helpers::{
    links::get_linked_addresses_with_foreign_key_as_type,
    // remote_indexes::{
    // RemoteEntryLinkResponse,
    // handle_sync_direct_remote_index_destination,
    // },
    local_indexes::create_direct_index,
    local_indexes::query_direct_index_with_foreign_key,
    records::{create_record, delete_record, read_record_entry},
};

use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_storage::*;
use hc_zome_rea_proposed_to_storage_consts::*;

use hc_zome_rea_proposal_storage_consts::*;

pub fn receive_create_proposed_to(proposed_to: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_proposed_to(&proposed_to)
}

pub fn receive_get_proposed_to(address: ProposedToAddress) -> ZomeApiResult<ResponseData> {
    handle_get_proposed_to(&address)
}

pub fn receive_delete_proposed_to(address: ProposedToAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_proposed_to(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_proposed_to(&params)
}

fn handle_get_proposed_to(address: &ProposedToAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?))
}

fn handle_create_proposed_to(proposed_to: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProposedToAddress, Entry) = create_record(
        PROPOSED_TO_BASE_ENTRY_TYPE,
        PROPOSED_TO_ENTRY_TYPE,
        PROPOSED_TO_INITIAL_ENTRY_LINK_TYPE,
        proposed_to.to_owned(),
    )?;
    let _ = create_direct_index(
        base_address.as_ref(),
        proposed_to.proposed.as_ref(),
        PROPOSED_TO_PROPOSED_LINK_TYPE,
        PROPOSED_TO_PROPOSED_LINK_TAG,
        PROPOSAL_PUBLISHED_TO_LINK_TYPE,
        PROPOSAL_PUBLISHED_TO_LINK_TAG,
    );
    Ok(construct_response(
        &base_address,
        &entry_resp,
    ))
}

fn handle_query_proposed_to(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(ProposedToAddress, Option<Entry>)>> =
        Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.proposed {
        Some(proposed) => {
            entries_result = query_direct_index_with_foreign_key(
                proposed,
                PROPOSED_TO_PROPOSED_LINK_TYPE,
                PROPOSED_TO_PROPOSED_LINK_TAG,
            );
        }
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(entries
            .iter()
            .map(|(entry_base_address, maybe_entry)| match maybe_entry {
                Some(entry) => Ok(construct_response(
                    entry_base_address,
                    &entry,
                )),
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
pub fn construct_response<'a>(address: &ProposedToAddress, e: &Entry) -> ResponseData {
    ResponseData {
        proposed_to: Response {
            id: address.to_owned(),
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
