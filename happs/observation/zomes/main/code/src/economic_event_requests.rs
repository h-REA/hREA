/**
 * Handling for external request structure for economic event records
 */

use hdk::{
    commit_entry,
    get_links,
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::Entry,
    },
    error::ZomeApiResult,
    utils::{
        get_as_type,
    },
};
use holochain_wasm_utils::api_serialization::get_links::{ GetLinksResult };
use holochain_core_types_derive::{ DefaultJson };

use vf_observation::type_aliases::{
    Timestamp,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessOrTransferAddress,
    ResourceSpecificationAddress,
    CommitmentAddress_Required,
};
use vf_observation::{
    economic_event::{
        Entry as EconomicEventEntry,
    },
    measurement::QuantityValue,
};
use super::fulfillment_requests::{
    LINK_TAG_EVENT_FULFILLS,
    link_fulfillments,
};

// Entry types

pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";

/**
 * I/O struct to describe the complete record, including all managed links
 */
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct EconomicEventRequest {
    // ENTRY FIELDS
    note: Option<String>,
    // action: Action, :TODO:
    input_of: ProcessOrTransferAddress,
    output_of: ProcessOrTransferAddress,
    provider: AgentAddress,
    receiver: AgentAddress,
    resource_inventoried_as: ResourceAddress,
    resource_classified_as: Option<Vec<ExternalURL>>,
    resource_conforms_to: ResourceSpecificationAddress,
    affected_quantity: Option<QuantityValue>,
    has_beginning: Timestamp,
    has_end: Timestamp,
    has_point_in_time: Timestamp,
    before: Timestamp,
    after: Timestamp,
    at_location: LocationAddress,
    in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    fulfills: Option<Vec<CommitmentAddress_Required>>,    // :TODO: I am glossing over the intermediary Fulfillment for now, just experimenting!
}

/**
 * Pick relevant fields out of I/O record into underlying DHT entry
 */
impl From<EconomicEventRequest> for EconomicEventEntry {
    fn from(e: EconomicEventRequest) -> EconomicEventEntry {
        EconomicEventEntry {
            note: e.note.into(),
            input_of: e.input_of.into(),
            output_of: e.output_of.into(),
            provider: e.provider.into(),
            receiver: e.receiver.into(),
            resource_inventoried_as: e.resource_inventoried_as.into(),
            resource_classified_as: e.resource_classified_as.into(),
            resource_conforms_to: e.resource_conforms_to.into(),
            affected_quantity: e.affected_quantity.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            has_point_in_time: e.has_point_in_time.into(),
            before: e.before.into(),
            after: e.after.into(),
            at_location: e.at_location.into(),
            in_scope_of: e.in_scope_of.into(),
        }
    }
}

/**
 * Create response from input DHT primitives
 */
fn construct_response(e: EconomicEventEntry, fulfillments: Option<Vec<Address>>) -> EconomicEventRequest {
    EconomicEventRequest{
        note: e.note.into(),
        input_of: e.input_of.into(),
        output_of: e.output_of.into(),
        provider: e.provider.into(),
        receiver: e.receiver.into(),
        resource_inventoried_as: e.resource_inventoried_as.into(),
        resource_classified_as: e.resource_classified_as.into(),
        resource_conforms_to: e.resource_conforms_to.into(),
        affected_quantity: e.affected_quantity.into(),
        has_beginning: e.has_beginning.into(),
        has_end: e.has_end.into(),
        has_point_in_time: e.has_point_in_time.into(),
        before: e.before.into(),
        after: e.after.into(),
        at_location: e.at_location.into(),
        in_scope_of: e.in_scope_of.into(),
        fulfills: fulfillments.into(),
    }
}

// :TODO: abstract behaviours for picking link-based fields out into link management structure
// :TODO: split input / output structure

pub fn handle_get_economic_event(address: Address) -> ZomeApiResult<EconomicEventRequest> {
    // It is important to note that there is no need to traverse the graph in any zome API read callbacks.
    // When querying links, we only need to read the target addresses from the links EAV in our DHT.
    // We leave it to the client GraphQL layer to handle fetching the details of associated fulfillments,
    // which would be performed externally as a call to the associated `planning` DHT for "get_fulfillments".
    let fulfillment_links = get_links(&address, LINK_TAG_EVENT_FULFILLS)?;

    let entry: EconomicEventEntry = get_as_type(address)?;

    // :TODO: disable debug
    println!("{:?}", fulfillment_links);

    Ok(construct_response(entry.into(), Some(fulfillment_links.addresses())))
}

pub fn handle_create_economic_event(event: EconomicEventRequest) -> ZomeApiResult<Address> {
    // copy necessary fields for link processing first, since `event.into()` will borrow the fields into the target Entry
    let fulfills = event.fulfills.clone();

    // handle core entry fields
    let entry_struct: EconomicEventEntry = event.into();
    let entry = Entry::App(EVENT_ENTRY_TYPE.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    // handle cross-DHT link fields
    let fulfillments = link_fulfillments(&address, &fulfills.unwrap());

    // :TODO: disable debug
    println!("{:?}", fulfillments);

    // return address
    // :TODO: return entire record structure
    Ok(address)
}
