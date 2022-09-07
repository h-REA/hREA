/**
 * hREA economic resource integrity zome for API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package hREA
 */
use hdi::prelude::*;
use hc_zome_rea_economic_resource_storage::Identified;
use hc_zome_rea_economic_resource_storage::{EntryStorage, EntryTypes, EntryTypesUnit, LinkTypes};

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

/// Macro to generate a default (permissable) validation function for EconomicResource
/// records in the local zome (local field checks only).
/// This is the minimum validation required by any zome, and regardless of other
/// validation rules being implemented it is critical that `record.validate()`
/// be peformed upon `EntryStorage` creation.
///
/// Use this method as a reference, and always call the below logic before any
/// application-specific validation.
// #[macro_export]
macro_rules! declare_economic_resource_zome_validation_defaults {
    ( /*$zome_api:ty*/ ) => {
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
                Ok(resource_storage) => {
                    let record = resource_storage.entry();
                    record
                        .validate()
                        .and_then(|()| Ok(ValidateCallbackResult::Valid))
                        .or_else(|e| Ok(ValidateCallbackResult::Invalid(e)))
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    };
}

declare_economic_resource_zome_validation_defaults!();
