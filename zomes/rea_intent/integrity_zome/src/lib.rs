/**
 * hREA fulfillment integrity zome for API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdi::prelude::*;
use hc_zome_rea_intent_storage::{
    EntryStorage, EntryTypes, EntryTypesUnit, Identified, LinkTypes,
};

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

#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op {
        Op::StoreRecord { .. } => Ok(ValidateCallbackResult::Valid),
        Op::StoreEntry(StoreEntry { entry, .. }) => validate_entry(entry),
        Op::RegisterCreateLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }
}

fn validate_entry(entry: Entry) -> ExternResult<ValidateCallbackResult> {
    match EntryStorage::try_from(&entry) {
        Ok(event_storage) => {
            let record = event_storage.entry();
            record
                .validate_or_fields()
                .and_then(|()| record.validate_action())
                .and_then(|()| Ok(ValidateCallbackResult::Valid))
                .or_else(|e| Ok(ValidateCallbackResult::Invalid(e)))
        }
        _ => Ok(ValidateCallbackResult::Valid),
    }
}
