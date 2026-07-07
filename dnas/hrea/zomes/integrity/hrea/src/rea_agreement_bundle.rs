use hdi::prelude::*;

/// vf:AgreementBundle (VF 1.0): a grouping of agreements.
#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaAgreementBundle {
    pub id: Option<ActionHash>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub agreements: Option<Vec<ActionHash>>,
}

/// Intrinsic field checks for a `ReaAgreementBundle`: bounded agreements list.
fn validate_agreement_bundle_fields(e: &ReaAgreementBundle) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.agreements,
        crate::MAX_COLLECTION_LEN,
        "agreements",
        "AgreementBundle",
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_agreement_bundle(
    _action: EntryCreationAction,
    rea_agreement_bundle: ReaAgreementBundle,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_agreement_bundle_fields(&rea_agreement_bundle))
}

pub fn validate_update_rea_agreement_bundle(
    _action: Update,
    rea_agreement_bundle: ReaAgreementBundle,
    _original_action: EntryCreationAction,
    _original_rea_agreement_bundle: ReaAgreementBundle,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_agreement_bundle_fields(&rea_agreement_bundle))
}

pub fn validate_delete_rea_agreement_bundle(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_rea_agreement_bundle: ReaAgreementBundle,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_agreement_bundle_updates(
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
    let _rea_agreement_bundle: crate::ReaAgreementBundle = record
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
    let _rea_agreement_bundle: crate::ReaAgreementBundle = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_agreement_bundle_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaAgreementBundleUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_agreement_bundles(
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
    let _rea_agreement_bundle: crate::ReaAgreementBundle = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_agreement_bundles(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
