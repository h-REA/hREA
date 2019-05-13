
/**
 * Handling for `Fulfillment` related behaviours as they relate to `EconomicEvent`s
 */

use hdk::{
    CAPABILITY_REQ,
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::Entry,
    },
    error::ZomeApiResult,
    call,
    link_entries,
    commit_entry,
};
use holochain_core_types_derive::{ DefaultJson };

use vf_observation::type_aliases::{
    CommitmentAddress_Required,
};
use vf_observation::{
    BRIDGED_PLANNING_DHT,
};

// Entry types

pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_baseurl";

// Link tags / link field names

pub const LINK_TAG_EVENT_FULFILLS: &str = "fulfills";
pub const LINK_TAG_COMMITMENT_FULFILLEDBY: &str = "fulfilledBy";

/**
 * Payload to send to linked DHT for updating links there
 */
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct TargetDNALinks {
    economic_event: Address,
    commitments: Vec<CommitmentAddress_Required>,
}

/**
 * Response data for linking operations
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct LinkResponseStatus {
    foreign_link_result: ZomeApiResult<JsonString>,
    own_link_result: Option<Vec<LinkBaseStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct LinkBaseStatus {
    entry_base: ZomeApiResult<Address>,
    entry_link: Option<Vec<ZomeApiResult<Address>>>,
}

pub fn link_fulfillments(address: &Address, targets: &Vec<Address>) -> LinkResponseStatus {
    // link `Fulfillment`s in the associated Planning DHT from this `EconomicEvent.fulfills`.
    // Planning will contain a base entry for this `EconomicEvent`'s hash; linked to the given target `Commitment`s.
    let foreign_link_result = call(BRIDGED_PLANNING_DHT, "main", Address::from(CAPABILITY_REQ.cap_token.to_string()), "link_fulfillments", TargetDNALinks{
        economic_event: address.clone().into(),
        commitments: targets.clone().into(),
    }.into());

    let internal_reqs: Vec<LinkBaseStatus> = targets.iter()
        .map(|addr| {
            // create a base entry pointer for the referenced commitment
            let base_entry = Entry::App(COMMITMENT_BASE_ENTRY_TYPE.into(), addr.into());
            let base_address = commit_entry(&base_entry);

            let entry_link = match base_address.clone() {
                Ok(base) => Some(vec![
                    // link to the indexes of the external `Commitment` via `EconomicEvent.fulfills`.
                    // Used for querying target entry addresses as a sub-field of the main `EconomicEvent` record.
                    // The actual query to dereference fulfillments is handled in the bridged Planning DHT, @see fulfillment_requests.rs
                    link_entries(&address, &base, LINK_TAG_EVENT_FULFILLS),
                    // link from the index of the external `Commitment` via `Commitment.fulfilledBy`.
                    // Used by the GraphQL layer to query event entries as relationships under a `Commitment`,
                    // and to otherwise filter events by the commitment they fulfill.
                    link_entries(&base, &address, LINK_TAG_COMMITMENT_FULFILLEDBY)
                ]),
                _ => None,
            };

            LinkBaseStatus{
                entry_base: base_address,
                entry_link
            }
        }).collect();

    LinkResponseStatus {
        foreign_link_result,
        own_link_result: Some(internal_reqs),
    }
}
