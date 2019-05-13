#![feature(try_from)]
/**
 * Planning zome API definition
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
extern crate vf_planning;

mod commitment_requests;
mod fulfillment_requests;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
};

use vf_planning::{
    commitment::Entry as Commitment,
};

use commitment_requests::{
    COMMITMENT_ENTRY_TYPE,
    CommitmentCreateFields,
    handle_create_commitment,
};

use fulfillment_requests::{
    EVENT_BASE_ENTRY_TYPE,
    LinkResponseStatus,
    handle_link_fulfillments,
    handle_get_fulfillments,
};
    // handle_update_economic_event,

// Zome entry type wrappers

fn commitment_entry_def() -> ValidatingEntryType {
    entry!(
        name: COMMITMENT_ENTRY_TYPE,
        description: "A planned economic flow that has been promised by an agent to another agent.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Commitment>| {
            Ok(())
        }
    )
}

fn event_base_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_BASE_ENTRY_TYPE,
        description: "Pointer to an economic event from a separate but related system.",
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
        commitment_entry_def(),
        event_base_def()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_commitment: {
            inputs: |commitment: CommitmentCreateFields|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_commitment
        }

        link_fulfillments: {
            inputs: |economic_event: Address, commitments: Vec<Address>|,
            outputs: |result: ZomeApiResult<LinkResponseStatus>|,
            handler: handle_link_fulfillments
        }
        get_fulfillments: {
            inputs: |economic_event: Address|,
            outputs: |result: ZomeApiResult<Vec<Commitment>>|,
            handler: handle_get_fulfillments
        }
    ]

    traits: {
        hc_public [
            create_commitment,
            link_fulfillments,
            get_fulfillments
        ]
    }
}
