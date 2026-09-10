use hdi::prelude::*;
use crate::QuantityValue;

/// vf:Claim (VF 1.0 Planning layer): a claim for future economic event(s) in
/// reciprocity for an event that already occurred (referenced by `triggered_by`).
#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaClaim {
    pub id: Option<ActionHash>,
    pub rea_action: String,
    pub resource_classified_as: Option<Vec<String>>,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub triggered_by: Option<ActionHash>,
    pub due: Option<Timestamp>,
    pub created: Option<Timestamp>,
    pub finished: Option<bool>,
    pub note: Option<String>,
    pub agreed_in: Option<String>,
}

fn validate_claim_fields(e: &ReaClaim) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_quantity(&e.resource_quantity, "resourceQuantity", "Claim"));
    crate::vf_check!(crate::vf_validate_quantity(&e.effort_quantity, "effortQuantity", "Claim"));
    crate::vf_check!(crate::vf_validate_action(&e.rea_action, "Claim"));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.resource_classified_as,
        crate::MAX_COLLECTION_LEN,
        "resourceClassifiedAs",
        "Claim",
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_claim(
    _action: TypedAction<EntryCreationData>,
    rea_claim: ReaClaim,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_claim_fields(&rea_claim))
}

pub fn validate_update_rea_claim(
    _action: TypedAction<UpdateData>,
    rea_claim: ReaClaim,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_claim: ReaClaim,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_claim_fields(&rea_claim))
}

pub fn validate_delete_rea_claim(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_claim: ReaClaim,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_claim_updates(
    _action: TypedAction<CreateLinkData>,
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
    let _rea_claim: crate::ReaClaim = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    let action_hash = target_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_claim: crate::ReaClaim = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_claim_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaClaimUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_claims(
    _action: TypedAction<CreateLinkData>,
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
    let _rea_claim: crate::ReaClaim = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_claims(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_claim_to_settling_events(
    _action: TypedAction<CreateLinkData>,
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
    let _rea_claim: crate::ReaClaim = record
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
    let _rea_economic_event: crate::ReaEconomicEvent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_claim_to_settling_events(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
