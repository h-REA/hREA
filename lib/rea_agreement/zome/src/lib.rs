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
        Path::entry_def(),
        EntryDef {
            id: AGREEMENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
struct CreateParams {
    pub agreement: CreateRequest,
}

#[hdk_extern]
fn create_agreement(params: CreateParams) -> ExternResult<ResponseData> {
    let CreateParams { agreement } = params;
    debug!("!!!!!CREATING AGREEMENT {:?}", agreement);
    Ok(receive_create_agreement(AGREEMENT_ENTRY_TYPE, agreement)?)
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
struct ReadParams {
    pub address: AgreementAddress,
}

#[hdk_extern]
fn get_agreement(params: ReadParams) -> ExternResult<ResponseData> {
    let ReadParams { address } = params;
    debug!("!!!!!READING AGREEMENT {:?}", address);
    Ok(receive_get_agreement(AGREEMENT_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
struct UpdateParams {
    pub agreement: UpdateRequest,
}

#[hdk_extern]
fn update_agreement(params: UpdateParams) -> ExternResult<ResponseData> {
    let UpdateParams { agreement } = params;
    Ok(receive_update_agreement(AGREEMENT_ENTRY_TYPE, agreement)?)
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
struct DeleteParams {
    pub address: RevisionHash,
}

#[hdk_extern]
fn delete_agreement(params: DeleteParams) -> ExternResult<bool> {
    let DeleteParams { address } = params;
    Ok(receive_delete_agreement(address)?)
}
