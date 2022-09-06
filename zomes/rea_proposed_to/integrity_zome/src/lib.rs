/**
 * hREA proposed to integrity zome for API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdi::prelude::*;
pub use hc_zome_rea_proposed_to_storage::{EntryTypes, EntryTypesUnit, LinkTypes};

#[hdk_extern]
pub fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    let defs: Vec<EntryDef> = EntryTypes::ENTRY_DEFS
        .iter()
        .map(|a| EntryDef::from(a.clone()))
        .collect();
    Ok(EntryDefsCallbackResult::from(defs))
}

#[no_mangle]
pub fn __num_entry_types() -> u8 {
    EntryTypesUnit::len()
}

#[no_mangle]
pub fn __num_link_types() -> u8 {
    LinkTypes::len()
}