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
extern crate holorea_core;

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

use holorea_core::{
    vf_core::VfEntry
};

pub fn handle_create_entry_test(entry: VfEntry) -> ZomeApiResult<Address> {
    let entry = Entry::App("vf_entry".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_entry_test(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

// Zome entry type wrappers

fn vf_entry_def() -> ValidatingEntryType {
    entry!(
        name: "vf_entry",
        description: "Describes any VF record entry",
        sharing: Sharing::Public,
        native_type: VfEntry,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: VfEntry, _validation_data: hdk::ValidationData| {
            Ok(())
        }
    )
}

// Zome definition

define_zome! {
    entries: [
       vf_entry_def()
    ]

    genesis: || { Ok(()) }

    functions: [
        create_vf_entry: {
            inputs: |entry: VfEntry|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_entry_test
        }
        get_vf_entry: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_entry_test
        }
    ]

    capabilities: {
        public (Public) [create_vf_entry,get_vf_entry]
    }
}
