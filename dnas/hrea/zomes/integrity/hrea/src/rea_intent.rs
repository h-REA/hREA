pub use crate::QuantityValue;
use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaIntent {
    pub id: Option<ActionHash>,
    pub rea_action: String,
    // vf:name — advertised by the GraphQL schema; was previously absent here,
    // so the adapter's `name` was silently dropped at the zome boundary.
    pub name: Option<String>,
    pub note: Option<String>,
    pub image: Option<String>,
    pub input_of: Option<ActionHash>,
    pub output_of: Option<ActionHash>,
    pub provider: Option<ActionHash>,
    pub receiver: Option<ActionHash>,
    pub resource_classified_as: Option<Vec<String>>,
    pub resource_conforms_to: Option<ActionHash>,
    pub resource_quantity: Option<QuantityValue>,
    pub effort_quantity: Option<QuantityValue>,
    pub available_quantity: Option<QuantityValue>,
    pub minimum_quantity: Option<QuantityValue>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub has_point_in_time: Option<Timestamp>,
    pub due: Option<Timestamp>,
    pub at_location: Option<String>,
    pub agreed_in: Option<String>,
    pub finished: Option<bool>,
    pub in_scope_of: Option<Vec<ActionHash>>,
}

pub fn validate_create_rea_intent(
    _action: TypedAction<EntryCreationData>,
    rea_intent: ReaIntent,
) -> ExternResult<ValidateCallbackResult> {
    if let Some(action_hash) = rea_intent.input_of.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_process: crate::ReaProcess = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_intent.provider.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_agent: crate::ReaAgent = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    Ok(validate_intent_fields(&rea_intent))
}

/// VF 1.0 field validation for Intent: temporal consistency, quantity positivity,
/// required action, and minimum/available quantity ordering.
fn validate_intent_fields(e: &ReaIntent) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_temporal(e.has_beginning, e.has_end, e.has_point_in_time, "Intent"));
    crate::vf_check!(crate::vf_validate_quantity(&e.resource_quantity, "resourceQuantity", "Intent"));
    crate::vf_check!(crate::vf_validate_quantity(&e.effort_quantity, "effortQuantity", "Intent"));
    crate::vf_check!(crate::vf_validate_quantity(&e.available_quantity, "availableQuantity", "Intent"));
    crate::vf_check!(crate::vf_validate_quantity(&e.minimum_quantity, "minimumQuantity", "Intent"));
    crate::vf_check!(crate::vf_validate_required_string(&e.rea_action, "action", "Intent"));
    crate::vf_check!(crate::vf_validate_action(&e.rea_action, "Intent"));
    if let (Some(min), Some(avail)) = (&e.minimum_quantity, &e.available_quantity) {
        if min.has_numerical_value > avail.has_numerical_value {
            return ValidateCallbackResult::Invalid(
                "Intent minimumQuantity must not exceed availableQuantity".to_string(),
            );
        }
    }
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.resource_classified_as,
        crate::MAX_COLLECTION_LEN,
        "resourceClassifiedAs",
        "Intent",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.in_scope_of,
        crate::MAX_COLLECTION_LEN,
        "inScopeOf",
        "Intent",
    ));
    ValidateCallbackResult::Valid
}

/// On update, the parties and action of an Intent are immutable.
fn validate_intent_update(new: &ReaIntent, old: &ReaIntent) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_unchanged(&old.provider, &new.provider, "provider", "Intent"));
    crate::vf_check!(crate::vf_validate_unchanged(&old.receiver, &new.receiver, "receiver", "Intent"));
    crate::vf_check!(crate::vf_validate_unchanged(&old.rea_action, &new.rea_action, "action", "Intent"));
    validate_intent_fields(new)
}

pub fn validate_update_rea_intent(
    _action: TypedAction<UpdateData>,
    rea_intent: ReaIntent,
    _original_action: TypedAction<EntryCreationData>,
    original_rea_intent: ReaIntent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_intent_update(&rea_intent, &original_rea_intent))
}

pub fn validate_delete_rea_intent(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_rea_intent: ReaIntent,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_intent_to_satisfying_commitments(
    _action: TypedAction<CreateLinkData>,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // satisfaction is rea_intent
    let action_hash = base_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_intent: crate::ReaIntent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // target is the satisfying ReaCommitment
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_commitment: crate::ReaCommitment = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_intent_to_satisfying_commitments(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_intent_to_satisfying_economic_events(
    _action: TypedAction<CreateLinkData>,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // satisfaction is rea_intent
    let action_hash = base_address
        .into_action_hash()
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "No action hash associated with link".to_string()
        )))?;
    let record = must_get_valid_record(action_hash)?;
    let _rea_intent: crate::ReaIntent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // target is the satisfying ReaEconomicEvent
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
pub fn validate_delete_link_intent_to_satisfying_economic_events(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_process_to_rea_intents(
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
    let _rea_process: crate::ReaProcess = record
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
    let _rea_intent: crate::ReaIntent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_process_to_rea_intents(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_agent_to_rea_intents(
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
    let _rea_intent: crate::ReaIntent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_agent_to_rea_intents(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_intent_updates(
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
    let _rea_intent: crate::ReaIntent = record
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
    let _rea_intent: crate::ReaIntent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_intent_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaIntentUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_intents(
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
    let _rea_intent: crate::ReaIntent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_intents(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
