/**
 * Holo-REA agreement zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_agreement_rpc::*;
use hc_zome_rea_agreement_lib::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        AgreementAddress::entry_def(),
        EntryDef {
            id: AGREEMENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateParams {
    pub agreement: CreateRequest,
}

#[hdk_extern]
fn create_agreement(CreateParams { agreement }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_agreement(AGREEMENT_ENTRY_TYPE, agreement)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadParams {
    pub address: AgreementAddress,
}

#[hdk_extern]
fn get_agreement(ReadParams { address }: ReadParams) -> ExternResult<ResponseData> {
    Ok(handle_get_agreement(AGREEMENT_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateParams {
    pub agreement: UpdateRequest,
}

#[hdk_extern]
fn update_agreement(UpdateParams { agreement }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_agreement(AGREEMENT_ENTRY_TYPE, agreement)?)
}

#[hdk_extern]
fn delete_agreement(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_agreement(revision_id)?)
}
