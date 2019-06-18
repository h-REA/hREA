/**
 * Handling for external request structure for economic event records
 */

use hdk::{
    commit_entry,
    update_entry,
    link_entries,
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

use hdk_graph_helpers::{
    create_base_entry,
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

pub const EVENT_BASE_ENTRY_TYPE: &str = "vf_economic_event_base";
pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";

pub const LINK_TYPE_INITIAL_ENTRY: &str = "record_initial_entry";
pub const LINK_TAG_INITIAL_ENTRY: &str = LINK_TYPE_INITIAL_ENTRY;

// :TODO: pull

pub fn handle_get_economic_event(address: Address) -> ZomeApiResult<EconomicEventResponse> {
    let base_address = address.clone();

    // read base entry to determine dereferenced entry address
    let entry_address: Address = get_as_type(address)?;

    // It is important to note that there is no need to traverse the graph in any zome API read callbacks.
    // When querying links, we only need to read the target addresses from the links EAV in our DHT.
    // We leave it to the client GraphQL layer to handle fetching the details of associated fulfillments,
    // which would be performed externally as a call to the associated `planning` DHT for "get_fulfillments".
    let fulfillment_links = get_links(&base_address, Some(EVENT_FULFILLS_LINK_TYPE.to_string()), Some(LINK_TAG_EVENT_FULFILLS.to_string()))?;

    let entry: EconomicEventEntry = get_as_type(entry_address)?;  // :NOTE: automatically retrieves latest entry by following metadata

    // :TODO: disable debug
    println!("{:?}", fulfillment_links);

    Ok(construct_response(base_address, entry.clone(), Some(fulfillment_links.addresses())))
}

pub fn handle_create_economic_event(event: EconomicEventCreateRequest) -> ZomeApiResult<EconomicEventResponse> {
    // copy necessary fields for link processing first, since `event.into()` will borrow the fields into the target Entry
    let fulfills = event.get_fulfills();

    // handle core entry fields
    let entry_struct: EconomicEventEntry = event.into();
    let entry_resp = entry_struct.clone();
    let entry = Entry::App(EVENT_ENTRY_TYPE.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    // create a base entry pointer
    let base_address = create_base_entry(EVENT_BASE_ENTRY_TYPE.into(), &address);
    // :NOTE: link is just for inference by external tools, it's not actually needed to query
    link_entries(&base_address, &address, LINK_TYPE_INITIAL_ENTRY, LINK_TAG_INITIAL_ENTRY)?;

    // handle cross-DHT link fields
    let fulfillments = match fulfills.clone() {
        Some(f) => { link_fulfillments(&address, &f); },
        None => ()
    };

    // :TODO: disable debug
    println!("{:?}", fulfillments);

    // return entire record structure
    Ok(construct_response(base_address, entry_resp, fulfills))
}

pub fn handle_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let base_address = event.get_id();
    let entry_address: Address = get_as_type(base_address.to_owned())?;
    let update_address = entry_address.clone();

    // copy necessary fields for link processing first, since `event.into()` will borrow the fields into the target Entry
    let fulfills = event.get_fulfills();

    // handle core entry fields
    let entry_struct: EconomicEventEntry = event.into();
    let entry_resp = entry_struct.clone();
    let entry = Entry::App(EVENT_ENTRY_TYPE.into(), entry_struct.into());
    update_entry(entry, &update_address)?;

    // :TODO: link field handling

    Ok(construct_response(base_address.to_owned(), entry_resp, fulfills))
}
