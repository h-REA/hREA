/**
 * Holo-REA proposed intents zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_proposed_to_lib::*;
use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_storage_consts::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        ProposedToAddress::entry_def(),
        EntryDef {
            id: PROPOSED_TO_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[hdk_extern]
fn create_proposed_to(CreateParams { proposed_to }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_proposed_to(PROPOSED_TO_ENTRY_TYPE, proposed_to)?)
}

#[hdk_extern]
fn get_proposed_to(ByAddress { address }: ByAddress<ProposedToAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_proposed_to(PROPOSED_TO_ENTRY_TYPE, address)?)
}

#[hdk_extern]
fn delete_proposed_to(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_proposed_to(&revision_id)?)
}
