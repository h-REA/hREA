/**
 * Holo-REA satisfaction integrity zome for API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdi::prelude::*;
pub use hc_zome_rea_satisfaction_storage::EntryStorage;

#[hdk_entry_defs]
#[unit_enum(UnitEntryType)]
pub enum EntryTypes {
    Satisfaction(EntryStorage),
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::Satisfaction(e)
    }
}
