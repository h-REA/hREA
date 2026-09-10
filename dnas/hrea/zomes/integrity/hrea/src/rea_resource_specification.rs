use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaResourceSpecification {
    pub id: Option<ActionHash>,
    pub name: String,
    pub image: Option<String>,
    pub note: Option<String>,
    pub default_unit_of_effort: Option<ActionHash>,
    pub default_unit_of_resource: Option<ActionHash>,
    // VF 1.0 boolean datatypes on vf:ResourceSpecification.
    // substitutable: whether instances of this specification are interchangeable.
    pub substitutable: Option<bool>,
    // mediumOfExchange: whether this specification is used as a medium of exchange.
    pub medium_of_exchange: Option<bool>,
}

/// Intrinsic field checks for a `ReaResourceSpecification`: required name.
fn validate_resource_specification_fields(e: &ReaResourceSpecification) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_required_string(
        &e.name,
        "name",
        "ResourceSpecification"
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_resource_specification(
    _action: TypedAction<EntryCreationData>,
    rea_resource_specification: ReaResourceSpecification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_resource_specification_fields(&rea_resource_specification))
}

pub fn validate_update_rea_resource_specification(
    _action: TypedAction<UpdateData>,
    rea_resource_specification: ReaResourceSpecification,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_resource_specification: ReaResourceSpecification,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_resource_specification_fields(&rea_resource_specification))
}

pub fn validate_delete_rea_resource_specification(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_resource_specification: ReaResourceSpecification,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_resource_specification_updates(
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
    let _rea_resource_specification: crate::ReaResourceSpecification = record
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
    let _rea_resource_specification: crate::ReaResourceSpecification = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_resource_specification_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaResourceSpecificationUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_resource_specifications(
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
    let _rea_resource_specification: crate::ReaResourceSpecification = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_resource_specifications(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
