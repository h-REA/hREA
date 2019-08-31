/**
 * Handling for `Intent`-related requests
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    error::ZomeApiResult,
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
};
use vf_planning::intent::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};
use super::satisfaction_requests::{
    get_satisfied_by,
};
use super::{
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INITIAL_ENTRY_LINK_TYPE,
    INTENT_ENTRY_TYPE,
};

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

// pub fn receive_query_intents(: Address) -> ZomeApiResult<Vec<Response>> {
//     handle_query_intents(&)
// }

// :TODO: move to hdk_graph_helpers module

fn handle_get_intent(address: &Address) -> ZomeApiResult<Response> {
    let entry = read_record_entry(&address)?;

    // :TODO: handle link fields
    let satisfaction_links = get_satisfied_by(&address)?;

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
    let satisfaction_links = get_satisfied_by(&base_address)?;

    Ok(construct_response(base_address, &new_entry, &Some(satisfaction_links)))
}
