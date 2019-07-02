/**
 * Handling for `Intent`-related requests
 */

use hdk::{
    commit_entry,
    update_entry,
    remove_entry,
    link_entries,
    get_links,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry::App as AppEntry,
    },
    error::ZomeApiResult,
    utils::{
        get_as_type,
    },
};

use hdk_graph_helpers::{
    records::{
        create_record,
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
    let base_address = address.clone();

    // read base entry to determine dereferenced entry address
    let entry_address: Address = get_as_type(address)?;

    // read reference fields
    let satisfaction_links = get_links(&base_address, Some(INTENT_SATISFIEDBY_LINK_TYPE.to_string()), Some(INTENT_SATISFIEDBY_LINK_TAG.to_string()))?;

    // read core entry data
    let entry: Entry = get_as_type(entry_address)?;  // :NOTE: automatically retrieves latest entry by following metadata

    // construct output response
    Ok(construct_response(base_address, entry, Some(satisfaction_links.addresses())))
}

pub fn handle_create_intent(intent: CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (Address, Entry) = create_record(INTENT_BASE_ENTRY_TYPE, INTENT_ENTRY_TYPE, intent)?;

    // :TODO: handle link fields

    // return entire record structure
    Ok(construct_response(base_address, entry_resp, None))
}

pub fn handle_update_intent(intent: UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = intent.get_id();
    let entry_address: Address = get_as_type(base_address.to_owned())?;
    let update_address = entry_address.clone();

    // handle core entry fields
    let prev_entry: Entry = get_as_type(entry_address)?;
    let entry_struct: Entry = prev_entry.update_with(&intent);
    let entry_resp = entry_struct.clone();
    let entry = AppEntry(INTENT_ENTRY_TYPE.into(), entry_struct.into());
    update_entry(entry, &update_address)?;

    // :TODO: link field handling

    Ok(construct_response(base_address.to_owned(), entry_resp, None))
}

pub fn handle_delete_intent(address: Address) -> ZomeApiResult<bool> {
    delete_record::<Entry>(address)  // :TODO: validate correct type is being deleted in validation CB
}
