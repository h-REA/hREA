/**
 * Handling for external request structure for economic event records
 */

use hdk::{
    PUBLIC_TOKEN,
    call,
    commit_entry,
    get_entry,
    link_entries,
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::Entry,
    },
    error::ZomeApiResult,
    error::ZomeApiError,
};
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
    BRIDGED_PLANNING_DHT,
    economic_event::{
        Entry as EconomicEventEntry,
    },
    measurement::QuantityValue,
};

// Entry types

pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";
pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment_baseurl";

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

// field names

pub const LINK_TAG_COMMITMENT_FULFILLEDBY: &str = "fulfilledBy";

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
 * Payload to send to linked DHT for updating links there
 */
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct TargetDNALinks {
    source: Address,
    targets: Vec<CommitmentAddress_Required>,
}

// :TODO: abstract behaviours for picking link-based fields out into link management structure

pub fn handle_get_economic_event(address: Address) -> ZomeApiResult<Option<Entry>> {
    let entry = get_entry(&address);

    let fulfillments: ZomeApiResult<JsonString> = call(
        BRIDGED_PLANNING_DHT, "main", Address::from(PUBLIC_TOKEN.to_string()), "get_fulfillments", address.into()
    );

    // :TODO:

    entry
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkResponseStatus {
    foreign_link_result: ZomeApiResult<JsonString>,
    own_link_result: Option<Vec<LinkBaseStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkBaseStatus {
    entry_base: ZomeApiResult<Address>,
    entry_link: Option<ZomeApiResult<Address>>,
}

pub fn handle_create_economic_event(event: EconomicEventRequest) -> ZomeApiResult<Address> {
    // copy necessary fields for link processing first, since `event.into()` will borrow the fields into the target Entry
    let fulfills = event.fulfills.clone();

    // handle core entry fields
    let entry_struct: EconomicEventEntry = event.into();
    let entry = Entry::App(EVENT_ENTRY_TYPE.into(), entry_struct.into());
    let address = commit_entry(&entry)?;

    // handle cross-DHT link fields
    let fulfillments: Option<LinkResponseStatus> = match fulfills {
        Some(targets) => {
            // link `Fulfillment`s in the associated Planning DHT from this `EconomicEvent.fulfilled`.
            // Planning will contain a base entry for this `EconomicEvent`'s hash; linked to the given target `Commitment`s.
            let foreign_link_result = call(BRIDGED_PLANNING_DHT, "main", Address::from(PUBLIC_TOKEN.to_string()), "link_fulfillments", TargetDNALinks{
                source: address.clone().into(),
                targets: targets.clone().into(),
            }.into());

            // link `Fulfillment`s in our own DHT reciprocally from the target `Commitment.fulfilledBy`.
            // Observation will contain a base entry for each target `Commitment`'s hash; linked to this `EconomicEvent`.
            let internal_reqs: Vec<LinkBaseStatus> = targets.iter()
                .map(|addr| {
                    let base_entry = Entry::App(COMMITMENT_ENTRY_TYPE.into(), addr.into());
                    let base_address = commit_entry(&base_entry);
                    let entry_link = match base_address.clone() {
                        Ok(base) => Some(link_entries(&base, &address, LINK_TAG_COMMITMENT_FULFILLEDBY)),
                        _ => None,
                    };

                    LinkBaseStatus{
                        entry_base: base_address,
                        entry_link
                    }
                }).collect();

            Some(LinkResponseStatus {
                foreign_link_result,
                own_link_result: Some(internal_reqs),
            })
        }
        _ => None
    };

    // :TODO: disable debug
    println!("{:?}", fulfillments);

    // return address
    // :TODO: return entire record structure
    Ok(address)
}
