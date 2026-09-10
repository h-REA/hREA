use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaProcessSpecification {
    pub id: Option<ActionHash>,
    pub name: String,
    pub note: Option<String>,
    pub image: Option<String>,
}

/// Intrinsic field checks for a `ReaProcessSpecification`: required name.
fn validate_process_specification_fields(e: &ReaProcessSpecification) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_required_string(
        &e.name,
        "name",
        "ProcessSpecification"
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_process_specification(
    _action: TypedAction<EntryCreationData>,
    rea_process_specification: ReaProcessSpecification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_process_specification_fields(&rea_process_specification))
}

pub fn validate_update_rea_process_specification(
    _action: TypedAction<UpdateData>,
    rea_process_specification: ReaProcessSpecification,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_process_specification: ReaProcessSpecification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_process_specification_fields(&rea_process_specification))
}

pub fn validate_delete_rea_process_specification(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_process_specification: ReaProcessSpecification,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_process_specification_updates(
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
    let _rea_process_specification: crate::ReaProcessSpecification = record
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
    let _rea_process_specification: crate::ReaProcessSpecification = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_process_specification_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaProcessSpecificationUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_process_specifications(
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
    let _rea_process_specification: crate::ReaProcessSpecification = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_process_specifications(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
