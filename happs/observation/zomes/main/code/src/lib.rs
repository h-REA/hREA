#![feature(try_from)]
/**
 * Observations zome API definition
 *
 * # Remarks
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the `entry!` type macros
 * and `define_zome!` definition to the standard Rust code in the rest of this
 * module.
 */

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate vf_observation;
mod economic_event_requests;
mod fulfillment_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
};

use vf_observation::economic_event::{
    Entry as EconomicEventEntry,
    Request as EconomicEventRequest,
    ResponseData as EconomicEventResponse,
};

use economic_event_requests::{
    EVENT_ENTRY_TYPE,
    handle_get_economic_event,
    handle_create_economic_event,
};
    // handle_update_economic_event,
use fulfillment_requests::{
    COMMITMENT_BASE_ENTRY_TYPE,
    handle_link_fulfillments,
};

// Zome entry type wrappers

fn event_entry_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_ENTRY_TYPE,
        description: "An observed economic flow, as opposed to a flow planned to happen in the future. This could reflect a change in the quantity of an economic resource. It is also defined by its behavior (action) in relation to the economic resource.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<EconomicEventEntry>| {
            Ok(())
        }
    )
}

fn commitment_base_def() -> ValidatingEntryType {
    entry!(
        name: COMMITMENT_BASE_ENTRY_TYPE,
        description: "Pointer to a commitment from a separate but related system.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        }
    )
}

// Zome definition

define_zome! {
    entries: [
       event_entry_def(),
       commitment_base_def()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_event: {
            inputs: |event: EconomicEventRequest|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: handle_create_economic_event
        }
        get_event: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<EconomicEventResponse>|,
            handler: handle_get_economic_event
        }
        // update_event: {
        //     inputs: |prevHash: Address, event: EconomicEventRequest|,
        //     outputs: |result: ZomeApiResult<Address>|,
        //     handler: handle_update_economic_event
        // }

        link_fulfillments: {
            inputs: |economic_event: Address, commitments: Vec<Address>|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: handle_link_fulfillments
        }
    ]

    traits: {
        hc_public [
            create_event,
            get_event
        ]
    }
}
