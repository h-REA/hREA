use hdi::prelude::*;

/// vf:SpatialThing (VF 1.0): a physical mappable location.
#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaSpatialThing {
    pub id: Option<ActionHash>,
    pub name: String,
    pub mappable_address: Option<String>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub alt: Option<f64>,
    pub note: Option<String>,
}

fn validate_spatial_thing_fields(e: &ReaSpatialThing) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_required_string(
        &e.name,
        "name",
        "SpatialThing"
    ));
    if let Some(lat) = e.lat {
        if !(-90.0..=90.0).contains(&lat) {
            return ValidateCallbackResult::Invalid(
                "SpatialThing lat must be between -90 and 90".to_string(),
            );
        }
    }
    if let Some(long) = e.long {
        if !(-180.0..=180.0).contains(&long) {
            return ValidateCallbackResult::Invalid(
                "SpatialThing long must be between -180 and 180".to_string(),
            );
        }
    }
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_spatial_thing(
    _action: EntryCreationAction,
    rea_spatial_thing: ReaSpatialThing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_spatial_thing_fields(&rea_spatial_thing))
}

pub fn validate_update_rea_spatial_thing(
    _action: Update,
    rea_spatial_thing: ReaSpatialThing,
    _original_action: EntryCreationAction,
    _original_rea_spatial_thing: ReaSpatialThing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_spatial_thing_fields(&rea_spatial_thing))
}

pub fn validate_delete_rea_spatial_thing(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_rea_spatial_thing: ReaSpatialThing,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_spatial_thing_updates(
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
    let _rea_spatial_thing: crate::ReaSpatialThing = record
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
    let _rea_spatial_thing: crate::ReaSpatialThing = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_spatial_thing_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaSpatialThingUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_spatial_things(
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
    let _rea_spatial_thing: crate::ReaSpatialThing = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_spatial_things(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
