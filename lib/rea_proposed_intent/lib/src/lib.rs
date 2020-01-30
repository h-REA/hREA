/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome library API
 *
 * Contains helper methods that can be used to manipulate `ProposedIntent` data
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
    //     query_direct_remote_index_with_foreign_key,
    },
    // remote_indexes::{
    //     RemoteEntryLinkResponse,
    //     handle_sync_direct_remote_index_destination,
    // },
};

use hc_zome_rea_proposed_intent_storage_consts::*;
use hc_zome_rea_proposed_intent_storage::*;
use hc_zome_rea_proposed_intent_rpc::*;

use vf_core::type_aliases::{
    ProposalAddress,
};

pub fn receive_create_proposed_intent(proposed_intent: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_proposed_intent(&proposed_intent)
}

pub fn receive_get_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
    handle_get_proposed_intent(&address)
}

pub fn receive_update_proposed_intent(proposed_intent: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_proposed_intent(&proposed_intent)
}

pub fn receive_delete_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_proposed_intents(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_proposed_intents(&params)
}

fn handle_get_proposed_intent(address: &ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?, get_link_fields(address)))
}

fn handle_create_proposed_intent(proposed_intent: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProposedIntentAddress, Entry) = create_record(
        PROPOSED_INTENT_BASE_ENTRY_TYPE, PROPOSED_INTENT_ENTRY_TYPE,
        PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE,
        proposed_intent.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_proposed_intent(proposed_intent: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = proposed_intent.get_id();
    let new_entry = update_record(PROPOSED_INTENT_ENTRY_TYPE, base_address, proposed_intent)?;
    Ok(construct_response(&base_address, &new_entry, get_link_fields(base_address)))
}

fn handle_query_proposed_intents(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(ProposedIntentAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: replace with real query filter logic
    match &params.published_in {
        Some(published_in) => {
            entries_result = query_direct_index_with_foreign_key(published_in, PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG);
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
    address: &ProposedIntentAddress, e: &Entry,
    published_in: Option<Cow<'a, Vec<ProposalAddress>>>
) -> ResponseData {
    ResponseData {
        proposed_intent: Response {
            // entry fields
            id: address.to_owned(),
            reciprocal: e.reciprocal,
            // link field
            published_in: published_in.map(Cow::into_owned),
            publishes: e.publishes.to_owned(),
        }
    }
}

pub fn get_link_fields <'a> ( p_in: &ProposedIntentAddress ) -> Option<Cow<'a, Vec<ProposalAddress>>> {
    Some(get_published_in_ids(p_in))
}


fn get_published_in_ids <'a> (p_to: &ProposedIntentAddress) -> Cow<'a, Vec<ProposalAddress>> {
    get_linked_addresses_with_foreign_key_as_type(p_to, PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG)
}
