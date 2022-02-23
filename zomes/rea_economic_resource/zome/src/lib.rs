/**
 * REA `EconomicResource` zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_economic_resource_zome_api::*;
use hc_zome_rea_economic_resource_lib::*;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_resource_storage::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        Path::entry_def(),
        EntryDef {
            id: CAP_STORAGE_ENTRY_DEF_ID.into(),
            visibility: EntryVisibility::Private,
            crdt_type: CrdtType,
            required_validations: 1.into(),
            required_validation_type: RequiredValidationType::default(),
        },
        EntryDef {
            id: RESOURCE_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

declare_economic_resource_zome_validation_defaults!();
declare_economic_resource_zome_api!(EconomicResourceZomePermissableDefault);
