use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaUnit {
    pub id: Option<ActionHash>,
    pub label: String,
    pub symbol: String,
    pub om_unit_identifier: String,
    pub classified_as: Option<Vec<String>>,
}

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct QuantityValue {
    // :TODO: https://users.rust-lang.org/t/currency-in-rust/890/9 ?
    pub has_numerical_value: f64, // :NOTE: uses https://en.wikipedia.org/wiki/IEEE_754 for math
    pub has_unit: Option<ActionHash>,
}

fn validate_unit_fields(e: &ReaUnit) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_required_string(
        &e.label, "label", "Unit"
    ));
    crate::vf_check!(crate::vf_validate_required_string(
        &e.symbol, "symbol", "Unit"
    ));
    crate::vf_check!(crate::vf_validate_required_string(
        &e.om_unit_identifier,
        "om_unit_identifier",
        "Unit"
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.classified_as,
        crate::MAX_COLLECTION_LEN,
        "classifiedAs",
        "Unit",
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_unit(
    _action: TypedAction<EntryCreationData>,
    rea_unit: ReaUnit,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_unit_fields(&rea_unit))
}

pub fn validate_update_rea_unit(
    _action: TypedAction<UpdateData>,
    rea_unit: ReaUnit,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_unit: ReaUnit,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_unit_fields(&rea_unit))
}

pub fn validate_delete_rea_unit(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_unit: ReaUnit,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_unit_updates(
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
    let _rea_unit: crate::ReaUnit = record
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
    let _rea_unit: crate::ReaUnit = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_unit_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaUnitUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_units(
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
    let _rea_unit: crate::ReaUnit = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_units(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
