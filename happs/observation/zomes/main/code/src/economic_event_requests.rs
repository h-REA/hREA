/**
 * Handling for external request structure for economic event records
 */

use hdk::{
    commit_entry,
    update_entry,
    get_links,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
    },
    error::ZomeApiResult,
    utils::{
        get_as_type,
    },
};

use vf_observation::economic_event::{
    Entry as EconomicEventEntry,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
    ResponseData as EconomicEventResponse,
    construct_response,
};
use super::fulfillment_requests::{
    EVENT_FULFILLS_LINK_TYPE,
    LINK_TAG_EVENT_FULFILLS,
    link_fulfillments,
};

// Entry types

pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";

// :TODO: pull

pub fn handle_get_economic_event(address: Address) -> ZomeApiResult<EconomicEventResponse> {
    // It is important to note that there is no need to traverse the graph in any zome API read callbacks.
    // When querying links, we only need to read the target addresses from the links EAV in our DHT.
    // We leave it to the client GraphQL layer to handle fetching the details of associated fulfillments,
    // which would be performed externally as a call to the associated `planning` DHT for "get_fulfillments".
    let fulfillment_links = get_links(&address, Some(EVENT_FULFILLS_LINK_TYPE.to_string()), Some(LINK_TAG_EVENT_FULFILLS.to_string()))?;

    let result_address = address.clone();
    let entry: EconomicEventEntry = get_as_type(address)?;

    // :TODO: disable debug
    println!("{:?}", fulfillment_links);

    Ok(construct_response(result_address, entry.clone(), Some(fulfillment_links.addresses())))
}

pub fn handle_create_economic_event(event: EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    // copy necessary fields for link processing first, since `event.into()` will borrow the fields into the target Entry
    let fulfills = event.fulfills.clone();

    // handle core entry fields
    let entry_struct: EconomicEventEntry = event.into();
    let entry_resp = entry_struct.clone();
    let entry = Entry::App(EVENT_ENTRY_TYPE.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    // handle cross-DHT link fields
    let fulfillments = match fulfills.clone() {
        Some(f) => { link_fulfillments(&address, &f); },
        None => ()
    };

    // :TODO: disable debug
    println!("{:?}", fulfillments);

    // return entire record structure
    Ok(construct_response(address, entry_resp, fulfills))
}

pub fn handle_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let entry_id = event.get_id();

    // copy necessary fields for link processing first, since `event.into()` will borrow the fields into the target Entry
    let fulfills = event.fulfills.clone();

    // handle core entry fields
    let entry_struct: EconomicEventEntry = event.into();
    let entry_resp = entry_struct.clone();
    let entry = Entry::App(EVENT_ENTRY_TYPE.into(), entry_struct.into());
    update_entry(entry, &entry_id)?;

    // :TODO: link field handling

    Ok(construct_response(entry_id, entry_resp, fulfills))
}
