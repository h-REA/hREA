/**
 * Handling for `Intent`-related requests
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    error::ZomeApiError,
    get_links,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_links_and_load_entry_data,
    },
};

use vf_planning::identifiers::{
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INITIAL_ENTRY_LINK_TYPE,
    INTENT_ENTRY_TYPE,
    INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
    SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
};
use vf_planning::intent::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    satisfied_by: Option<Address>,
}

pub fn receive_create_intent(intent: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_intent(&intent)
}

pub fn receive_get_intent(address: Address) -> ZomeApiResult<Response> {
    handle_get_intent(&address)
}

pub fn receive_update_intent(intent: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_intent(&intent)
}

pub fn receive_delete_intent(address: Address) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_intents(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_intents(&params)
}

// :TODO: move to hdk_graph_helpers module

fn handle_get_intent(address: &Address) -> ZomeApiResult<Response> {
    let entry = read_record_entry(&address)?;

    // :TODO: handle link fields
    let satisfaction_links = get_satisfaction_ids(&address)?;

    // construct output response
    Ok(construct_response(&address, &entry, &Some(satisfaction_links)))
}

fn handle_create_intent(intent: &CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (Address, Entry) = create_record(
        INTENT_BASE_ENTRY_TYPE, INTENT_ENTRY_TYPE,
        INTENT_INITIAL_ENTRY_LINK_TYPE,
        intent.to_owned(),
    )?;

    // return entire record structure
    Ok(construct_response(&base_address, &entry_resp, &None))
}

fn handle_update_intent(intent: &UpdateRequest) -> ZomeApiResult<Response> {
    let base_address = intent.get_id();
    let new_entry = update_record(INTENT_ENTRY_TYPE, &base_address, intent)?;

    // :TODO: link field handling
    let satisfaction_links = get_satisfaction_ids(&base_address)?;

    Ok(construct_response(base_address, &new_entry, &Some(satisfaction_links)))
}

fn handle_query_intents(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(Address, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = get_links_and_load_entry_data(
                &satisfied_by, SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
            );
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
                            &Some(get_satisfaction_ids(&entry_base_address)?)
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

/// Used to load the list of linked Fulfillment IDs
fn get_satisfaction_ids(intent: &Address) -> ZomeApiResult<Vec<Address>> {
    Ok(get_links(&intent, Exactly(INTENT_SATISFIEDBY_LINK_TYPE), Exactly(INTENT_SATISFIEDBY_LINK_TAG))?.addresses())
}
