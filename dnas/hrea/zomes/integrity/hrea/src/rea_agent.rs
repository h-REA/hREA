use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaAgent {
    pub id: Option<ActionHash>,
    pub name: String,
    pub agent_type: String,
    pub image: Option<String>,
    pub classified_as: Option<Vec<String>>,
    pub note: Option<String>,
}

/// Allowed values for `ReaAgent.agent_type`. Mirrors the values the coordinator zome
/// round-trips (dnas/hrea/zomes/coordinator/hrea/src/rea_agent.rs): "Person" | "Organization".
const VALID_AGENT_TYPES: [&str; 2] = ["Person", "Organization"];

/// Intrinsic field checks for a `ReaAgent`: required name and agent_type, agent_type
/// vocabulary, and bounded classification list. All single-entry and deterministic.
fn validate_agent_fields(e: &ReaAgent) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_required_string(&e.name, "name", "Agent"));
    crate::vf_check!(crate::vf_validate_required_string(
        &e.agent_type,
        "agentType",
        "Agent"
    ));
    if !VALID_AGENT_TYPES.contains(&e.agent_type.as_str()) {
        return ValidateCallbackResult::Invalid(format!(
            "Agent agentType '{}' must be 'Person' or 'Organization'",
            e.agent_type
        ));
    }
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.classified_as,
        crate::MAX_COLLECTION_LEN,
        "classifiedAs",
        "Agent",
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_agent(
    _action: EntryCreationAction,
    rea_agent: ReaAgent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_agent_fields(&rea_agent))
}

pub fn validate_update_rea_agent(
    _action: Update,
    rea_agent: ReaAgent,
    _original_action: EntryCreationAction,
    _original_rea_agent: ReaAgent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_agent_fields(&rea_agent))
}

pub fn validate_delete_rea_agent(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_rea_agent: ReaAgent,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_agent_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = base_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_agent: crate::ReaAgent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_agent: crate::ReaAgent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_agent_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaAgentUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_agents(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_agent: crate::ReaAgent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_agents(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
