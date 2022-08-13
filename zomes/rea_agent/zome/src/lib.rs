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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateParams {
    pub agent: CreateRequest,
}

#[hdk_extern]
fn create_agent(CreateParams { agent }: CreateParams) -> ExternResult<ResponseData> {
    Ok(handle_create_agent(AGENT_ENTRY_TYPE, agent)?)
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssociateAgentParams {
    pub agent_address: AgentAddress,
}

#[hdk_extern]
fn associate_my_agent(AssociateAgentParams { agent_address }: AssociateAgentParams) -> ExternResult<bool> {
    match handle_associate_my_agent(agent_address) {
        Ok(()) => Ok(true),
        Err(e) => Err(e.into())
    }
}

#[hdk_extern]
fn get_my_agent(_: ()) -> ExternResult<ResponseData> {
    Ok(handle_get_my_agent()?)
}

#[hdk_extern]
fn get_agent(ByAddress { address }: ByAddress<AgentAddress>) -> ExternResult<ResponseData> {
    Ok(handle_get_agent(address)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WhoisParams {
    pub agent_pub_key: AgentPubKey,
}

#[hdk_extern]
fn whois(WhoisParams { agent_pub_key }: WhoisParams) -> ExternResult<ResponseData> {
    Ok(handle_whois_query(agent_pub_key)?)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateParams {
    pub agent: UpdateRequest,
}

#[hdk_extern]
fn update_agent(UpdateParams { agent }: UpdateParams) -> ExternResult<ResponseData> {
    Ok(handle_update_agent(agent)?)
}

#[hdk_extern]
fn delete_agent(ByRevision { revision_id }: ByRevision) -> ExternResult<bool> {
    Ok(handle_delete_agent(revision_id)?)
}
