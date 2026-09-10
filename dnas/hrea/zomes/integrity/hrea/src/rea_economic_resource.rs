pub use crate::QuantityValue;
use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaEconomicResource {
    pub id: Option<ActionHash>,
    pub name: Option<String>,
    pub conforms_to: Option<ActionHash>,
    pub tracking_identifier: Option<String>,
    pub lot: Option<String>,
    pub image: Option<String>,
    pub current_location: Option<String>,
    pub note: Option<String>,
    pub stage: Option<ActionHash>,
    pub state: Option<String>,
    pub contained_in: Option<ActionHash>,
    pub accounting_quantity: Option<QuantityValue>,
    pub onhand_quantity: Option<QuantityValue>,
    pub primary_accountable: Option<ActionHash>,
    pub unit_of_effort: Option<ActionHash>,
}

fn validate_economic_resource_fields(e: &ReaEconomicResource) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_quantity(
        &e.accounting_quantity,
        "accountingQuantity",
        "EconomicResource"
    ));
    crate::vf_check!(crate::vf_validate_quantity(
        &e.onhand_quantity,
        "onhandQuantity",
        "EconomicResource"
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_economic_resource(
    _action: TypedAction<EntryCreationData>,
    rea_economic_resource: ReaEconomicResource,
) -> ExternResult<ValidateCallbackResult> {
    if let Some(action_hash) = rea_economic_resource.contained_in.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_economic_resource: crate::ReaEconomicResource = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    Ok(validate_economic_resource_fields(&rea_economic_resource))
}

pub fn validate_update_rea_economic_resource(
    _action: TypedAction<UpdateData>,
    rea_economic_resource: ReaEconomicResource,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_economic_resource: ReaEconomicResource,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_economic_resource_fields(&rea_economic_resource))
}

pub fn validate_delete_rea_economic_resource(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_economic_resource: ReaEconomicResource,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_economic_resource_to_rea_economic_resources(
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
    let _rea_economic_resource: crate::ReaEconomicResource = record
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
    let _rea_economic_resource: crate::ReaEconomicResource = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_economic_resource_to_rea_economic_resources(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_economic_resource_updates(
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
    let _rea_economic_resource: crate::ReaEconomicResource = record
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
    let _rea_economic_resource: crate::ReaEconomicResource = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_economic_resource_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaEconomicResourceUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_economic_resources(
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
    let _rea_economic_resource: crate::ReaEconomicResource = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_economic_resources(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
