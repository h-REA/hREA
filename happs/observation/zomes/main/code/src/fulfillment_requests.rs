
/**
 * Handling for `Fulfillment` related behaviours as they relate to `EconomicEvent`s
 */

use hdk::{
    CAPABILITY_REQ,
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::{
            AppEntryValue,
        },
    },
};
use holochain_core_types_derive::{ DefaultJson };
use hdk_graph_helpers::{
    link_entries_bidir,
    create_base_entry,
    link_remote_entries,
};

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

pub fn link_fulfillments(source_entry: &Address, targets: &Vec<Address>) -> AppEntryValue {
    // Build local index first (for reading entries of the `source_entry`)
    // Panics on any errors
    let _ = targets.iter()
        .inspect(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_address = create_base_entry(COMMITMENT_BASE_ENTRY_TYPE.into(), &base_entry_addr);
            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(&source_entry, &base_address, LINK_TAG_EVENT_FULFILLS, LINK_TAG_COMMITMENT_FULFILLEDBY);
        });

    // :TODO: implement bridge genesis callbacks & private chain entry to wire up cross-DNA link calls

    // Build query index in remote DNA (for retrieving linked `target` entries)
    // -> links to `Commitment`s in the associated Planning DNA from this `EconomicEvent.fulfills`,
    //    and back to this `EconomicEvent` via `Commitment.fulfilledBy`.
    link_remote_entries(
        BRIDGED_PLANNING_DHT,
        "main",
        Address::from(CAPABILITY_REQ.cap_token.to_string()),
        "link_fulfillments",
        &source_entry,
        &targets,
    )
}
