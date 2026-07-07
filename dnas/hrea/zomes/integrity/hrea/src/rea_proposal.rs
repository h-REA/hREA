use hdi::prelude::*;

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct ReaProposal {
    pub id: Option<ActionHash>,
    pub name: Option<String>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub unit_based: Option<bool>,
    pub created: Option<Timestamp>,
    pub note: Option<String>,
    pub in_scope_of: Option<Vec<ActionHash>>,
    pub publishes: Option<Vec<ActionHash>>,
    pub reciprocal: Option<Vec<ActionHash>>,
    pub proposed_to: Option<Vec<ActionHash>>,
    // vf:Proposal.purpose -> vf:ProposalPurpose ("offer" | "request"), VF 1.0.
    // Modelled as Option<String> rather than a Rust enum to avoid HDI
    // deserialization friction as the value set evolves across DNA versions;
    // the allowed values are enforced by validation below instead.
    pub purpose: Option<String>,
}

/// Allowed values for `ReaProposal.purpose` per vf:ProposalPurpose (VF 1.0).
const VALID_PROPOSAL_PURPOSES: [&str; 2] = ["offer", "request"];

/// Reject any `purpose` value other than "offer" or "request". `None` is valid
/// (the field is optional and legacy proposals carry no purpose).
fn validate_proposal_purpose(rea_proposal: &ReaProposal) -> ValidateCallbackResult {
    match &rea_proposal.purpose {
        None => ValidateCallbackResult::Valid,
        Some(purpose) if VALID_PROPOSAL_PURPOSES.contains(&purpose.as_str()) => {
            ValidateCallbackResult::Valid
        }
        Some(purpose) => ValidateCallbackResult::Invalid(format!(
            "Invalid Proposal purpose '{purpose}': must be 'offer' or 'request'"
        )),
    }
}

/// Validate the intrinsic fields of a `ReaProposal`: the `purpose` value set
/// (vf:ProposalPurpose) and the temporal fields (VF temporal semantics).
fn validate_proposal_fields(e: &ReaProposal) -> ValidateCallbackResult {
    // Preserve the existing purpose check exactly.
    if let ValidateCallbackResult::Invalid(reason) = validate_proposal_purpose(e) {
        return ValidateCallbackResult::Invalid(reason);
    }
    // ReaProposal has no has_point_in_time field; pass None.
    crate::vf_check!(crate::vf_validate_temporal(
        e.has_beginning,
        e.has_end,
        None,
        "Proposal"
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.in_scope_of,
        crate::MAX_COLLECTION_LEN,
        "inScopeOf",
        "Proposal",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.publishes,
        crate::MAX_COLLECTION_LEN,
        "publishes",
        "Proposal",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.reciprocal,
        crate::MAX_COLLECTION_LEN,
        "reciprocal",
        "Proposal",
    ));
    crate::vf_check!(crate::vf_validate_collection_bound(
        &e.proposed_to,
        crate::MAX_COLLECTION_LEN,
        "proposedTo",
        "Proposal",
    ));
    ValidateCallbackResult::Valid
}

pub fn validate_create_rea_proposal(
    _action: EntryCreationAction,
    rea_proposal: ReaProposal,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_proposal_fields(&rea_proposal))
}

/// On update, `purpose` is immutable ("an offer does not become a request"):
/// the coordinator indexes proposals under a purpose path at creation and
/// deliberately performs no link cleanup on update, so a purpose change would
/// silently corrupt the offers/requests indexes.
fn validate_proposal_update(new: &ReaProposal, old: &ReaProposal) -> ValidateCallbackResult {
    crate::vf_check!(crate::vf_validate_unchanged(
        &old.purpose,
        &new.purpose,
        "purpose",
        "Proposal",
    ));
    validate_proposal_fields(new)
}

pub fn validate_update_rea_proposal(
    _action: Update,
    rea_proposal: ReaProposal,
    _original_action: EntryCreationAction,
    original_rea_proposal: ReaProposal,
) -> ExternResult<ValidateCallbackResult> {
    Ok(validate_proposal_update(&rea_proposal, &original_rea_proposal))
}

pub fn validate_delete_rea_proposal(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_rea_proposal: ReaProposal,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_rea_proposal_updates(
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
    let _rea_proposal: crate::ReaProposal = record
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
    let _rea_proposal: crate::ReaProposal = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_rea_proposal_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "ReaProposalUpdates links cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_proposals(
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
    let _rea_proposal: crate::ReaProposal = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_proposals(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
