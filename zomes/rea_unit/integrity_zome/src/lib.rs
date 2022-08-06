/**
 * Holo-REA unit integrity zome for API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */

pub use hc_zome_rea_unit_storage::EntryStorage;

#[hdk_entry_defs]
#[unit_enum(UnitEntryType)]
pub enum EntryTypes {
    Unit(EntryStorage),
}
impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::Unit(e)
    }
}

#[hdk_link_types]
pub enum LinkTypes {
    UnitIdentifier
}
