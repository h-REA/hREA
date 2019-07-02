/**
 * Handling for `Intent`-related requests
 */

use hdk::{
    get_links,
    holochain_core_types::{
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
    ResponseData,
    construct_response,
};
use super::satisfaction_requests::{
    INTENT_SATISFIEDBY_LINK_TYPE,
    INTENT_SATISFIEDBY_LINK_TAG,
};

// Entry types

pub const INTENT_BASE_ENTRY_TYPE: &str = "vf_intent_base";
pub const INTENT_ENTRY_TYPE: &str = "vf_intent";

// :TODO: move to hdk_graph_helpers module

pub fn handle_get_intent(address: Address) -> ZomeApiResult<ResponseData> {
    let entry = read_record_entry(&address)?;

    // :TODO: handle link fields
    let satisfaction_links = get_links(&address, Some(INTENT_SATISFIEDBY_LINK_TYPE.to_string()), Some(INTENT_SATISFIEDBY_LINK_TAG.to_string()))?;

    // construct output response
    Ok(construct_response(&address, entry, Some(satisfaction_links.addresses())))
}

pub fn handle_create_intent(intent: CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (Address, Entry) = create_record(INTENT_BASE_ENTRY_TYPE, INTENT_ENTRY_TYPE, intent)?;

    // :TODO: handle link fields

    // return entire record structure
    Ok(construct_response(&base_address, entry_resp, None))
}

pub fn handle_update_intent(intent: UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = intent.get_id();
    let new_entry = update_record(INTENT_ENTRY_TYPE, &base_address, &intent)?;

    // :TODO: link field handling

    Ok(construct_response(base_address, new_entry, None))
}

pub fn handle_delete_intent(address: Address) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}
