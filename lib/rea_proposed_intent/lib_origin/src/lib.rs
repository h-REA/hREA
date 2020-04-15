/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome library API
 *
 * Contains helper methods that can be used to manipulate `ProposedIntent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk::{
    call,
    error::{ZomeApiError, ZomeApiResult},
    PUBLIC_TOKEN,
};

use hdk_graph_helpers::{
    local_indexes::{create_direct_index, query_direct_index_with_foreign_key},
    records::{create_record, delete_record, read_record_entry},
};

use hc_zome_rea_proposed_intent_rpc::*;
use hc_zome_rea_proposed_intent_storage::*;
use hc_zome_rea_proposed_intent_storage_consts::*;

use vf_core::type_aliases::Address;

use hc_zome_rea_proposal_storage_consts::*;

pub fn receive_create_proposed_intent(
    proposed_intent: CreateRequest,
) -> ZomeApiResult<ResponseData> {
    handle_create_proposed_intent(&proposed_intent)
}

pub fn receive_get_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
    handle_get_proposed_intent(&address)
}

pub fn receive_delete_proposed_intent(address: ProposedIntentAddress) -> ZomeApiResult<bool> {
    // update in the associated foreign DNA as well
    let res = delete_record::<Entry>(&address);
    let _pingback = call(
        BRIDGED_PLANNING_DHT,
        "proposed_intent",
        Address::from(PUBLIC_TOKEN.to_string()),
        "deleted_proposed_intent",
        address.into(),
    );
    res
}

pub fn receive_query_proposed_intents(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_proposed_intents(&params)
}

fn handle_get_proposed_intent(address: &ProposedIntentAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?))
}

fn handle_create_proposed_intent(proposed_intent: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (ProposedIntentAddress, Entry) = create_record(
        PROPOSED_INTENT_BASE_ENTRY_TYPE,
        PROPOSED_INTENT_ENTRY_TYPE,
        PROPOSED_INTENT_INITIAL_ENTRY_LINK_TYPE,
        proposed_intent.to_owned(),
    )?;
    // handle link fields
    let _ = create_direct_index(
        base_address.as_ref(),
        proposed_intent.published_in.as_ref(),
        PROPOSED_INTENT_PUBLISHED_IN_LINK_TYPE,
        PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
        PROPOSAL_PUBLISHES_LINK_TYPE,
        PROPOSAL_PUBLISHES_LINK_TAG,
    );
    // update in the associated foreign DNA as well
    let pingback = call(
        BRIDGED_PLANNING_DHT,
        "proposed_intent",
        Address::from(PUBLIC_TOKEN.to_string()),
        "created_proposed_intent",
        FwdCreateRequest {
            proposed_intent: proposed_intent.to_owned(),
        }
        .into(),
    );
    let _ = hdk::debug(format!("grepme {:?}", pingback));
    Ok(construct_response(&base_address, &entry_resp))
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
pub fn construct_response<'a>(address: &ProposedIntentAddress, e: &Entry) -> ResponseData {
    ResponseData {
        proposed_intent: Response {
            // entry fields
            id: address.to_owned(),
            reciprocal: e.reciprocal,
            // link field
            published_in: e.published_in.to_owned(),
            publishes: e.publishes.to_owned(),
        },
    }
}
