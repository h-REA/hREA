use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    PUBLIC_TOKEN,
};
/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome library API
 *
 * Contains helper methods that can be used to manipulate `ProposedIntent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;

use hdk_graph_helpers::{
    links::get_linked_addresses_with_foreign_key_as_type,
    local_indexes::query_direct_index_with_foreign_key,
    records::{create_record, delete_record, read_record_entry},
    remote_indexes::create_direct_remote_index,
    MaybeUndefined,
};

use hc_zome_rea_proposed_intent_rpc::*;
use hc_zome_rea_proposed_intent_storage::*;
use hc_zome_rea_proposed_intent_storage_consts::*;

use vf_core::type_aliases::{Address, ProposalAddress};

use hc_zome_rea_intent_storage_consts::{
    INTENT_BASE_ENTRY_TYPE, INTENT_PROPOSED_INTENT_PUBLISHES_LINK_TAG,
    INTENT_PROPOSED_INTENT_PUBLISHES_LINK_TYPE,
};

pub fn receive_create_proposed_intent(
    proposed_intent: CreateRequest,
) -> ZomeApiResult<ResponseData> {
    handle_create_proposed_intent(&proposed_intent)
}

pub fn receive_get_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
    handle_get_proposed_intent(&address)
}

pub fn receive_delete_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_proposed_intents(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_proposed_intents(&params)
}

fn handle_get_proposed_intent(address: &ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(
        address,
        &read_record_entry(address)?,
        get_link_fields(address),
    ))
}

fn handle_create_proposed_intent(proposed_intent: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProposedIntentAddress, Entry) = create_record(
        PROPOSED_INTENT_BASE_ENTRY_TYPE,
        PROPOSED_INTENT_ENTRY_TYPE,
        PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE,
        proposed_intent.to_owned(),
    )?;
    // handle link fields
    if let CreateRequest {
        publishes: MaybeUndefined::Some(publishes),
        ..
    } = proposed_intent
    {
        let _results = create_direct_remote_index(
            BRIDGED_PLANNING_DHT,
            "intent",
            "index_publishes",
            Address::from(PUBLIC_TOKEN.to_string()),
            INTENT_BASE_ENTRY_TYPE,
            PROPOSED_INTENT_PUBLISHES_LINK_TYPE,
            PROPOSED_INTENT_PUBLISHES_LINK_TAG,
            INTENT_PROPOSED_INTENT_PUBLISHES_LINK_TYPE,
            INTENT_PROPOSED_INTENT_PUBLISHES_LINK_TAG,
            base_address.as_ref(),
            vec![(publishes.as_ref()).clone()],
        );
    };
    Ok(construct_response(
        &base_address,
        &entry_resp,
        get_link_fields(&base_address),
    ))
}

fn handle_query_proposed_intents(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(ProposedIntentAddress, Option<Entry>)>> =
        Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: replace with real query filter logic
    match &params.published_in {
        Some(published_in) => {
            entries_result = query_direct_index_with_foreign_key(
                published_in,
                PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE,
                PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
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
                    get_link_fields(entry_base_address),
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
pub fn construct_response<'a>(
    address: &ProposedIntentAddress,
    e: &Entry,
    published_in: Option<Cow<'a, Vec<ProposalAddress>>>,
) -> ResponseData {
    ResponseData {
        proposed_intent: Response {
            // entry fields
            id: address.to_owned(),
            reciprocal: e.reciprocal,
            // link field
            published_in: published_in.map(Cow::into_owned),
            publishes: e.publishes.to_owned(),
        },
    }
}

pub fn get_link_fields<'a>(p_in: &ProposedIntentAddress) -> Option<Cow<'a, Vec<ProposalAddress>>> {
    Some(get_published_in_ids(p_in))
}

fn get_published_in_ids<'a>(p_to: &ProposedIntentAddress) -> Cow<'a, Vec<ProposalAddress>> {
    get_linked_addresses_with_foreign_key_as_type(
        p_to,
        PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE,
        PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
    )
}
