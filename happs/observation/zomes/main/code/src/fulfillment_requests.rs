
/**
 * Handling for `Fulfillment` related behaviours as they relate to `EconomicEvent`s
 */

use hdk::{
    PUBLIC_TOKEN,
    holochain_core_types::{
        cas::content::Address,
        error::HolochainError,
        json::JsonString,
        entry::{
            AppEntryValue,
        },
    },
    error::ZomeApiResult,
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

pub const EVENT_FULFILLS_LINK_TYPE: &str = "vf_economic_event_fulfills";
pub const COMMITMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_commitment_fulfilled_by";

pub const LINK_TAG_EVENT_FULFILLS: &str = "fulfills";
pub const LINK_TAG_COMMITMENT_FULFILLEDBY: &str = "fulfilled_by";

pub fn link_fulfillments(source_entry: &Address, targets: &Vec<Address>) -> Vec<Address> {
    // Build local index first (for reading entries of the `source_entry`)
    let mut commitment_results: Vec<Address> = targets.iter()
        .map(|base_entry_addr| {
            // create a base entry pointer for the referenced commitment
            let base_address = create_base_entry(COMMITMENT_BASE_ENTRY_TYPE.into(), base_entry_addr);
            // link event to commitment by `fulfilled`/`fulfilledBy` edge
            link_entries_bidir(
                &source_entry, &base_address,
                EVENT_FULFILLS_LINK_TYPE, LINK_TAG_EVENT_FULFILLS,
                COMMITMENT_FULFILLEDBY_LINK_TYPE, LINK_TAG_COMMITMENT_FULFILLEDBY
            );
            base_address
        })
        .collect();

    // :TODO: implement bridge genesis callbacks & private chain entry to wire up cross-DNA link calls

    // Build query index in remote DNA (for retrieving linked `target` entries)
    // -> links to `Commitment`s in the associated Planning DNA from this `EconomicEvent.fulfills`,
    //    and back to this `EconomicEvent` via `Commitment.fulfilledBy`.
    let mut result: Vec<Address> = link_remote_entries(
        BRIDGED_PLANNING_DHT,
        "main",
        Address::from(PUBLIC_TOKEN.to_string()),
        "link_fulfillments",
        &source_entry,
        targets,
    );

    result.append(&mut commitment_results);
    result
}

pub fn handle_link_fulfillments(economic_event: Address, commitments: Vec<Address>) -> ZomeApiResult<Vec<Address>> {
    Ok(link_fulfillments(&economic_event, &commitments))
}
