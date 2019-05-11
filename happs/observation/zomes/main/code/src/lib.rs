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
 *
 * @package: HoloREA
 * @author:  pospi <pospi@spadgos.com>
 * @since:   2019-02-06
 */

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate vf_observation;
mod economic_event_requests;

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

use vf_observation::economic_event::Entry as EconomicEvent;

use economic_event_requests::{
    EVENT_ENTRY_TYPE,
    EconomicEventRequest,
    handle_get_economic_event,
    handle_create_economic_event,
};
    // handle_update_economic_event,

// Zome entry type wrappers

fn event_entry_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_ENTRY_TYPE,
        description: "An observed economic flow, as opposed to a flow planned to happen in the future. This could reflect a change in the quantity of an economic resource. It is also defined by its behavior (action) in relation to the economic resource.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<EconomicEvent>| {
            Ok(())
        }
    )
}

// Zome definition

define_zome! {
    entries: [
       event_entry_def()
    ]

    genesis: || { Ok(()) }

    functions: [
        get_event: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_economic_event
        }
        create_event: {
            inputs: |event: EconomicEventRequest|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_economic_event
        }
        // update_event: {
        //     inputs: |prevHash: Address, event: EconomicEventRequest|,
        //     outputs: |result: ZomeApiResult<Address>|,
        //     handler: handle_update_economic_event
        // }
    ]

    traits: {
        hc_public [
            create_event,
            get_event
        ]
    }
}
