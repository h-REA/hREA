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
extern crate hdk_graph_helpers;
extern crate vf_planning;

mod commitment_requests;
mod fulfillment_requests;
mod satisfaction_requests;

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

use hdk_graph_helpers::{
    LINK_TYPE_INITIAL_ENTRY,
};
use vf_planning::commitment::{
    Entry as CommitmentEntry,
    CreateRequest as CommitmentCreateRequest,
    UpdateRequest as CommitmentUpdateRequest,
    ResponseData as CommitmentResponse,
};

use commitment_requests::{
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_BASE_ENTRY_TYPE,
    handle_get_commitment,
    handle_create_commitment,
    handle_update_commitment,
    handle_delete_commitment,
};
use fulfillment_requests::{
    EVENT_BASE_ENTRY_TYPE,
    EVENT_FULFILLS_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    handle_link_fulfillments,
    handle_get_fulfillments,
};
use satisfaction_requests::{
    INTENT_BASE_ENTRY_TYPE,
    COMMITMENT_SATISFIES_LINK_TYPE,
    INTENT_SATISFIEDBY_LINK_TYPE,
};

// Zome entry type wrappers

fn commitment_entry_def() -> ValidatingEntryType {
    entry!(
        name: COMMITMENT_ENTRY_TYPE,
        description: "A planned economic flow that has been promised by an agent to another agent.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<CommitmentEntry>| {
            Ok(())
        }
    )
}

fn commitment_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: COMMITMENT_BASE_ENTRY_TYPE,
        description: "Base anchor for initial commitment addresses to provide lookup functionality",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                COMMITMENT_ENTRY_TYPE,
                link_type: LINK_TYPE_INITIAL_ENTRY,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                EVENT_BASE_ENTRY_TYPE,
                link_type: COMMITMENT_FULFILLEDBY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                INTENT_BASE_ENTRY_TYPE,
                link_type: COMMITMENT_SATISFIES_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                INTENT_BASE_ENTRY_TYPE,
                link_type: INTENT_SATISFIEDBY_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

fn event_base_entry_def() -> ValidatingEntryType {
    entry!(
        name: EVENT_BASE_ENTRY_TYPE,
        description: "Pointer to an economic event from a separate but related system.",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<Address>| {
            Ok(())
        },
        links: [
            to!(
                COMMITMENT_BASE_ENTRY_TYPE,
                link_type: EVENT_FULFILLS_LINK_TYPE,

                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },

                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

// Zome definition

define_zome! {
    entries: [
        commitment_entry_def(),
        commitment_base_entry_def(),
        event_base_entry_def()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_commitment: {
            inputs: |commitment: CommitmentCreateRequest|,
            outputs: |result: ZomeApiResult<CommitmentResponse>|,
            handler: handle_create_commitment
        }
        get_commitment: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<CommitmentResponse>|,
            handler: handle_get_commitment
        }
        update_commitment: {
            inputs: |commitment: CommitmentUpdateRequest|,
            outputs: |result: ZomeApiResult<CommitmentResponse>|,
            handler: handle_update_commitment
        }
        delete_commitment: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<bool>|,
            handler: handle_delete_commitment
        }

        link_fulfillments: {
            inputs: |base_entry: Address, target_entries: Vec<Address>|,
            outputs: |result: ZomeApiResult<Vec<Address>>|,
            handler: handle_link_fulfillments
        }
        get_fulfillments: {
            inputs: |economic_event: Address|,
            outputs: |result: ZomeApiResult<Vec<CommitmentResponse>>|,
            handler: handle_get_fulfillments
        }
    ]

    traits: {
        hc_public [
            create_commitment,
            get_commitment,
            update_commitment,
            delete_commitment,
            link_fulfillments,
            get_fulfillments
        ]
    }
}
