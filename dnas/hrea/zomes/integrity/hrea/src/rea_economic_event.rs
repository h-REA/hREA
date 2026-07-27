pub use crate::QuantityValue;
use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaEconomicEvent {
    pub id: Option<ActionHash>,
    pub rea_action: String,
    pub note: Option<String>,
    pub input_of: Option<ActionHash>,
    pub output_of: Option<ActionHash>,
    pub provider: Option<ActionHash>,
    pub receiver: Option<ActionHash>,
    pub resource_inventoried_as: Option<ActionHash>,
    pub to_resource_inventoried_as: Option<ActionHash>,
    pub resource_classified_as: Option<Vec<String>>,
    pub resource_conforms_to: Option<ActionHash>,
    pub resource_quantity: Option<QuantityValue>,
    // VF 1.0: vf:EconomicEvent.effortQuantity (vf:Measure). Required for work/cite/use actions.
    pub effort_quantity: Option<QuantityValue>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub has_point_in_time: Option<Timestamp>,
    pub at_location: Option<String>,
    pub agreed_in: Option<String>,
    pub realization_of: Option<ActionHash>,
    // VF 1.0: vf:EconomicEvent.reciprocalRealizationOf -> vf:Agreement (reciprocal counterpart of realizationOf).
    pub reciprocal_realization_of: Option<ActionHash>,
    // VF 1.0: vf:EconomicEvent.settles -> vf:Claim (the claim this event settles).
    pub settles: Option<ActionHash>,
    pub in_scope_of: Option<Vec<ActionHash>>,
    pub triggered_by: Option<ActionHash>,
    pub fulfills: Option<Vec<ActionHash>>,
    pub satisfies: Option<Vec<ActionHash>>,
    pub corrects: Option<ActionHash>,
}

pub fn validate_create_rea_economic_event(
    _action: EntryCreationAction,
    rea_economic_event: ReaEconomicEvent,
) -> ExternResult<ValidateCallbackResult> {
    if let Some(action_hash) = rea_economic_event.input_of.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_process: crate::ReaProcess = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_economic_event.provider.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_agent: crate::ReaAgent = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_economic_event.realization_of.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_agreement: crate::ReaAgreement = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_economic_event.triggered_by.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_economic_event: crate::ReaEconomicEvent = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    // Symmetric referential checks for the remaining ActionHash relations, so a
    // malformed event referencing a non-existent entry is rejected at the DHT
    // gate rather than producing dangling links. Mirrors the provider/realization_of
    // checks above; the target type is asserted on decode (a wrong-type hash errors).
    if let Some(action_hash) = rea_economic_event.receiver.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_agent: crate::ReaAgent = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_economic_event.resource_inventoried_as.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_economic_resource: crate::ReaEconomicResource = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_economic_event.to_resource_inventoried_as.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_economic_resource: crate::ReaEconomicResource = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    if let Some(action_hash) = rea_economic_event.resource_conforms_to.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_resource_specification: crate::ReaResourceSpecification = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    // VF 1.0: EconomicEvent.settles -> Claim (reverse: Claim.settledBy).
    if let Some(action_hash) = rea_economic_event.settles.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_claim: crate::ReaClaim = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    // VF 1.0: EconomicEvent.reciprocalRealizationOf -> Agreement.
    if let Some(action_hash) = rea_economic_event.reciprocal_realization_of.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_agreement: crate::ReaAgreement = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Dependant action must be accompanied by an entry"
            ))))?;
    }
    Ok(validate_economic_event_fields(&rea_economic_event))
}

/// VF 1.0 DHT validation (#396): temporal consistency and quantity positivity.
/// Real on-DHT validation that replaces the prior always-Valid stub, so malformed
/// events are rejected before they ever land on the DHT.
fn validate_economic_event_fields(e: &ReaEconomicEvent) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_action(&e.rea_action, "EconomicEvent"));
    crate::vf_check!(crate::vf_validate_temporal(
        e.has_beginning,
        e.has_end,
        e.has_point_in_time,
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_quantity(
        &e.resource_quantity,
        "resourceQuantity",
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_quantity(
        &e.effort_quantity,
        "effortQuantity",
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_transfer_agents(
        &e.rea_action,
        &e.provider,
        &e.receiver,
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.resource_classified_as,
        crate::MAX_COLLECTION_LEN,
        "resourceClassifiedAs",
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.in_scope_of,
        crate::MAX_COLLECTION_LEN,
        "inScopeOf",
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.fulfills,
        crate::MAX_COLLECTION_LEN,
        "fulfills",
        "EconomicEvent",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.satisfies,
        crate::MAX_COLLECTION_LEN,
        "satisfies",
        "EconomicEvent",
    ));
    ValidateCallbackResult::Valid
}

/// On update, the core economic-event facts (who, what action) are immutable;
/// corrections are modelled as new events (vf:EconomicEvent.corrects), not edits.
fn validate_economic_event_update(
    new: &ReaEconomicEvent,
    old: &ReaEconomicEvent,
) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_unchanged(&old.provider, &new.provider, "provider", "EconomicEvent"));
    crate::vf_check!(crate::vf_validate_unchanged(&old.receiver, &new.receiver, "receiver", "EconomicEvent"));
    crate::vf_check!(crate::vf_validate_unchanged(&old.rea_action, &new.rea_action, "action", "EconomicEvent"));
    validate_economic_event_fields(new)
}

pub fn validate_update_rea_economic_event(
    _action: Update,
    rea_economic_event: ReaEconomicEvent,
    _original_action: EntryCreationAction,
    original_rea_economic_event: ReaEconomicEvent,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_economic_event_update(
        &rea_economic_event,
        &original_rea_economic_event,
    ))
}

pub fn validate_delete_rea_economic_event(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_rea_economic_event: ReaEconomicEvent,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_process_to_rea_economic_events(
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
    let _rea_economic_event: crate::ReaEconomicEvent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_process_to_rea_economic_events(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_agent_to_rea_economic_events(
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
    let _rea_economic_event: crate::ReaEconomicEvent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_agent_to_rea_economic_events(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_agreement_to_rea_economic_events(
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
    let _rea_agreement: crate::ReaAgreement = record
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
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_agreement_to_rea_economic_events(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_economic_event_to_rea_economic_events(
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
    let _rea_economic_event: crate::ReaEconomicEvent = record
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
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_economic_event_to_rea_economic_events(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_economic_event_updates(
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
    let _rea_economic_event: crate::ReaEconomicEvent = record
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
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_economic_event_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaEconomicEventUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_economic_events(
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
    let _rea_economic_event: crate::ReaEconomicEvent = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_economic_events(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
