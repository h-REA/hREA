/**
 * Holo-REA agent zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hc_zome_rea_agent_rpc::*;
use hc_zome_rea_agent_lib::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(EntryDefsCallbackResult::from(vec![
        PathEntry::entry_def(),
        AgentAddress::entry_def(),
        EntryDef {
            id: AGENT_ENTRY_TYPE.into(),
            visibility: EntryVisibility::Public,
            required_validations: 2.into(),
            required_validation_type: RequiredValidationType::default(),
        }
    ]))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub agent: CreateRequest,
}

#[hdk_extern]
fn create_agent(CreateParams { agent }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_agent(AGENT_ENTRY_TYPE, agent)?)
}

#[derive(Debug, Serialize, Deserialize)]
struct ReadParams {
    pub address: AgentAddress,
}

#[hdk_extern]
fn get_my_agent(_: ()) -> ExternResult<ResponseData> {
    Ok(handle_get_my_agent()?)
}
#[hdk_extern]
fn get_agent(ReadParams { address }: ReadParams) -> ExternResult<ResponseData> {
    Ok(handle_get_agent(AGENT_ENTRY_TYPE, address)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub agent: UpdateRequest,
}

#[hdk_extern]
fn update_agent(UpdateParams { agent }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_agent(AGENT_ENTRY_TYPE, agent)?)
}

#[hdk_extern]
fn delete_agent(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_agent(revision_id)?)
}
