use crate::helpers::*;
use hdk::prelude::*;
use hrea_integrity::*;
use vf_actions::{get_builtin_action, ActionEffect};

#[derive(Clone, PartialEq, Debug, Deserialize)]
pub struct EconomicEventWithResource {
    pub event: ReaEconomicEvent,
    pub new_inventoried_resource: Option<ReaEconomicResource>,
}

/// Create response carrying the event record plus, when a
/// `new_inventoried_resource` was requested, the created resource record —
/// so the GraphQL EconomicEventResponse can populate `economicResource`
/// instead of silently returning null for a resource that was in fact created.
#[derive(Serialize, Deserialize, Debug)]
pub struct EconomicEventCreateResponse {
    pub event: Record,
    pub resource: Option<Record>,
}

#[hdk_extern]
pub fn create_rea_economic_event(
    event_with_resource: EconomicEventWithResource,
) -> ExternResult<EconomicEventCreateResponse> {
    let mut rea_economic_event = ReaEconomicEvent {
        ..event_with_resource.event.clone()
    };
    let mut created_resource_hash: Option<ActionHash> = None;

    if event_with_resource.new_inventoried_resource.is_some() {
        let mut event_quantity = Some(QuantityValue {
            has_numerical_value: 0.0,
            has_unit: None,
        });

        if let Some(reference_quantity) = event_with_resource.event.resource_quantity.clone() {
            event_quantity = Some(reference_quantity.clone());
        }

        let mut rea_economic_resource = ReaEconomicResource {
            primary_accountable: event_with_resource.event.receiver.clone(),
            accounting_quantity: event_quantity.clone(),
            onhand_quantity: event_quantity.clone(),
            ..event_with_resource.new_inventoried_resource.unwrap()
        };
        // EconomicResource.conformsTo is non-nullable in the GraphQL schema:
        // when the resource params don't name a specification, inherit the
        // event's resourceConformsTo (the vf semantics of creating a resource
        // through a produce/raise event).
        if rea_economic_resource.conforms_to.is_none() {
            rea_economic_resource.conforms_to =
                event_with_resource.event.resource_conforms_to.clone();
        }

        // Create the economic resource
        let rea_economic_resource_hash = create_entry(&EntryTypes::ReaEconomicResource(
            rea_economic_resource.clone(),
        ))?;

        let resource_tag_prefix = LinkTag(rea_economic_resource_hash.get_raw_39().to_vec());
        if let Some(base) = rea_economic_resource.contained_in.clone() {
            create_link(
                base,
                rea_economic_resource_hash.clone(),
                LinkTypes::ReaEconomicResourceToReaEconomicResources,
                resource_tag_prefix.clone(),
            )?;
        }

        let path = Path::from("all_economic_resources");
        create_link(
            path.path_entry_hash()?,
            rea_economic_resource_hash.clone(),
            LinkTypes::AllEconomicResources,
            resource_tag_prefix,
        )?;

        // Add the economic resource to the economic event
        rea_economic_event.resource_inventoried_as = Some(rea_economic_resource_hash.clone());
        created_resource_hash = Some(rea_economic_resource_hash);
    } else if let Some(resource_id) = rea_economic_event.resource_inventoried_as.clone() {
        // Affect existing resource
        let links_query =
            LinkQuery::try_new(resource_id.clone(), LinkTypes::ReaEconomicResourceUpdates)?;
        let links = get_links(links_query, GetStrategy::Local)?;
        let latest_link = links
            .into_iter()
            .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));

        let latest_rea_economic_resource_hash =
            match latest_link {
                Some(link) => link.target.clone().into_action_hash().ok_or(wasm_error!(
                    WasmErrorInner::Guest("No action hash associated with link".to_string())
                ))?,
                _ => resource_id.clone(),
            };

        let resource_bytes = get(
            latest_rea_economic_resource_hash.clone(),
            GetOptions::default(),
        )?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Could not find the resource".to_string()
        )))?;

        // decode the resource
        let resource: ReaEconomicResource = resource_bytes
            .entry()
            .to_app_option()
            .map_err(|err| wasm_error!(err))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "Could not deserialize record to Resource.".into(),
            )))?;

        let rea_action = Some(rea_economic_event.clone().rea_action);

        let action_info = if let Some(ref action) = rea_action {
            get_builtin_action(action)
        } else {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "rea_action is None".to_string()
            )));
        };

        fn apply_effect(effect: ActionEffect, resource_quantity: f64, event_quantity: f64) -> f64 {
            match effect {
                ActionEffect::Increment => resource_quantity + event_quantity,
                ActionEffect::Decrement => resource_quantity - event_quantity,
                ActionEffect::NoEffect => resource_quantity,
                ActionEffect::DecrementIncrement => resource_quantity,
            }
        }

        // If onhand_effect is decrement, subtract event resource_quantity from primary_accountable
        let action_info = action_info.ok_or(wasm_error!(WasmErrorInner::Guest(
            "Action info is None".to_string()
        )))?;

        let resource_onhand_quantity =
            resource
                .clone()
                .onhand_quantity
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "Resource onhand_quantity is None".to_string()
                )))?;

        let resource_accounting_quantity =
            resource
                .clone()
                .accounting_quantity
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "Resource accounting_quantity is None".to_string()
                )))?;

        let event_resource_quantity =
            rea_economic_event
                .clone()
                .resource_quantity
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "Event resource_quantity is None".to_string()
                )))?;

        let resource_update_params = ReaEconomicResource {
            onhand_quantity: Some(QuantityValue {
                has_numerical_value: apply_effect(
                    action_info.onhand_effect.clone(),
                    resource_onhand_quantity.has_numerical_value,
                    event_resource_quantity.has_numerical_value,
                ),
                has_unit: resource_onhand_quantity.has_unit.clone(),
            }),
            accounting_quantity: Some(QuantityValue {
                has_numerical_value: apply_effect(
                    action_info.accounting_effect.clone(),
                    resource_accounting_quantity.has_numerical_value,
                    event_resource_quantity.has_numerical_value,
                ),
                has_unit: resource_accounting_quantity.has_unit.clone(),
            }),
            // Revisions written here must carry the original id like every update_* extern does,
            // otherwise clients derive the id from the revision hash and fork the revision chain.
            id: Some(resource.id.clone().unwrap_or(resource_id.clone())),
            ..resource.clone()
        };

        let updated_resource_action_hash = update_entry(
            latest_rea_economic_resource_hash.clone(),
            &EntryTypes::ReaEconomicResource(resource_update_params),
        )?;

        let resource_tag_prefix: LinkTag = LinkTag(resource_id.get_raw_39().to_vec());

        create_link(
            resource_id.clone(),
            updated_resource_action_hash.clone(),
            LinkTypes::ReaEconomicResourceUpdates,
            resource_tag_prefix.clone(),
        )?;

        if let Some(base) = resource.contained_in.clone() {
            update_link(
                AnyLinkableHash::from(base),
                updated_resource_action_hash.clone(),
                LinkTypes::ReaEconomicResourceToReaEconomicResources,
                resource_id.clone().into(),
            )?;
        }

        update_link(
            AnyLinkableHash::from(Path::from("all_economic_resources").path_entry_hash()?),
            updated_resource_action_hash.clone(),
            LinkTypes::AllEconomicResources,
            resource_id.clone().into(),
        )?;

        // The event keeps the original resource id (already in `resource_inventoried_as`);
        // the latest revision is reachable through the ReaEconomicResourceUpdates links.
    }

    // Create the economic event
    let rea_economic_event_hash =
        create_entry(&EntryTypes::ReaEconomicEvent(rea_economic_event.clone()))?;
    let econ_tag_prefix: LinkTag = LinkTag(rea_economic_event_hash.get_raw_39().to_vec());
    if let Some(base) = rea_economic_event.input_of.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaProcessToReaEconomicEventInputs,
            econ_tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_economic_event.output_of.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaProcessToReaEconomicEventOutputs,
            econ_tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_economic_event.provider.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ProviderToReaEconomicEvents,
            econ_tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_economic_event.receiver.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReceiverToReaEconomicEvents,
            econ_tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_economic_event.realization_of.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaAgreementToReaEconomicEvents,
            econ_tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_economic_event.triggered_by.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaEconomicEventToReaEconomicEvents,
            econ_tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_economic_event.fulfills.clone() {
        for b in base {
            create_link(
                b,
                rea_economic_event_hash.clone(),
                LinkTypes::CommitmentToFulfillingEconomicEvents,
                econ_tag_prefix.clone(),
            )?;
        }
    }
    if let Some(base) = rea_economic_event.satisfies.clone() {
        for b in base {
            create_link(
                b,
                rea_economic_event_hash.clone(),
                // events satisfying an intent index under their own link type;
                // IntentToSatisfyingCommitments is reserved for commitments
                // (Intent.observedBy reads this link type back).
                LinkTypes::IntentToSatisfyingEconomicEvents,
                econ_tag_prefix.clone(),
            )?;
        }
    }
    // VF 1.0: link the settled Claim -> this settling EconomicEvent, so a Claim can
    // resolve its settledBy (reverse of EconomicEvent.settles).
    if let Some(base) = rea_economic_event.settles.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ClaimToSettlingEvents,
            econ_tag_prefix.clone(),
        )?;
    }

    if let Some(rea_economic_resource_hash) = rea_economic_event.resource_inventoried_as.clone() {
        // Create a link from the resource to the economic event
        create_link(
            rea_economic_resource_hash.clone(),
            rea_economic_event_hash.clone(),
            LinkTypes::ReaEconomicResourceToReaEconomicEvents,
            econ_tag_prefix.clone(),
        )?;
    }

    let path = Path::from("all_economic_events");
    create_link(
        path.path_entry_hash()?,
        rea_economic_event_hash.clone(),
        LinkTypes::AllEconomicEvents,
        econ_tag_prefix.clone(),
    )?;

    let record =
        get(rea_economic_event_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the newly created ReaEconomicEvent".to_string())
        ))?;
    let resource = match created_resource_hash {
        Some(hash) => Some(get(hash, GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(
                "Could not find the newly created ReaEconomicResource".to_string()
            )
        ))?),
        None => None,
    };
    Ok(EconomicEventCreateResponse {
        event: record,
        resource,
    })
}

#[hdk_extern]
pub fn get_latest_rea_economic_event(
    original_rea_economic_event_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links_query = LinkQuery::try_new(
        original_rea_economic_event_hash.clone(),
        LinkTypes::ReaEconomicEventUpdates,
    )?;
    let links = get_links(links_query, GetStrategy::Local)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_rea_economic_event_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_rea_economic_event_hash.clone(),
    };
    get(latest_rea_economic_event_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_rea_economic_event(
    original_rea_economic_event_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_rea_economic_event_hash, GetOptions::default())?
    else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}

#[hdk_extern]
pub fn get_all_revisions_for_rea_economic_event(
    original_rea_economic_event_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) =
        get_original_rea_economic_event(original_rea_economic_event_hash.clone())?
    else {
        return Ok(vec![]);
    };
    let links_query = LinkQuery::try_new(
        original_rea_economic_event_hash.clone(),
        LinkTypes::ReaEconomicEventUpdates,
    )?;
    let links = get_links(links_query, GetStrategy::Local)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| {
            Ok(GetInput::new(
                link.target
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "No action hash associated with link".to_string()
                    )))?
                    .into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let mut records: Vec<Record> = records.into_iter().flatten().collect();
    records.insert(0, original_record);
    Ok(records)
}

/// Sparse update payload for `update_rea_economic_event`: an absent (`None`)
/// field means "leave unchanged", a present one replaces the stored value. There
/// is no way to clear a field through an update, which is the same contract as
/// every other `*UpdateParams` merged with `merge_partial`.
///
/// What an update may change follows ValueFlows: an EconomicEvent is an
/// observed fact, so the fact itself (action, who, which process, what it
/// corrects) is fixed at creation and a mistake there is fixed with a new event
/// that `corrects` this one. What may change is the context recorded around the
/// fact: the note, the agreement it realizes, what triggered it, what it
/// fulfills, satisfies or settles, and its scope.
///
/// - Applied when present: `note`, `agreed_in`, `realization_of`,
///   `reciprocal_realization_of`, `settles`, `in_scope_of`, `triggered_by`,
///   `fulfills`, `satisfies`.
/// - Accepted only when unchanged: `provider` and `receiver` (rejected by the
///   integrity zome's `vf_validate_unchanged`), and `input_of`, `output_of` and
///   `corrects` (rejected here by `reject_immutable_change`). Resending the
///   stored value is a no-op, so a client that echoes the whole entity back
///   still succeeds.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReaEconomicEventUpdateParams {
    pub note: Option<String>,
    pub input_of: Option<ActionHash>,
    pub output_of: Option<ActionHash>,
    pub provider: Option<ActionHash>,
    pub receiver: Option<ActionHash>,
    pub agreed_in: Option<String>,
    pub realization_of: Option<ActionHash>,
    pub reciprocal_realization_of: Option<ActionHash>,
    pub settles: Option<ActionHash>,
    pub in_scope_of: Option<Vec<ActionHash>>,
    pub triggered_by: Option<ActionHash>,
    pub fulfills: Option<Vec<ActionHash>>,
    pub satisfies: Option<Vec<ActionHash>>,
    pub corrects: Option<ActionHash>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateReaEconomicEventInput {
    pub revision_id: ActionHash,
    pub entry: ReaEconomicEventUpdateParams,
}

/// Refuse an update that would change a field fixed at creation. `None` in the
/// params means "not sent" and always passes; sending the stored value passes too.
fn reject_immutable_change(
    current: &Option<ActionHash>,
    requested: &Option<ActionHash>,
    field: &str,
) -> ExternResult<()> {
    match requested {
        Some(value) if current.as_ref() != Some(value) => Err(wasm_error!(WasmErrorInner::Guest(
            format!(
                "EconomicEvent {field} cannot be changed after creation; record a new event that corrects this one instead"
            )
        ))),
        _ => Ok(()),
    }
}

/// Refuse a changed `reciprocal_realization_of` that does not point at an
/// Agreement. Create checks this in the integrity zome, but the update rule
/// does not, and no link index validates the base, so the coordinator checks it
/// here (an integrity rule would change the DNA hash). The entry type is read
/// from the action, since decoding the entry alone would accept any entry whose
/// fields happen to fit `ReaAgreement`.
fn ensure_reciprocal_agreement(
    current: &Option<ActionHash>,
    requested: &Option<ActionHash>,
) -> ExternResult<()> {
    let Some(hash) = requested else { return Ok(()) };
    if current.as_ref() == Some(hash) {
        return Ok(());
    }
    match crate::get_entry_for_action(hash)? {
        Some(EntryTypes::ReaAgreement(_)) => Ok(()),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "EconomicEvent reciprocalRealizationOf must reference an Agreement".to_string()
        ))),
    }
}

/// Keep a single-valued relationship index in step with an update: drop the
/// link under the old base when the base changed, then point the link under the
/// current base at the new revision.
fn reindex_single(
    old_base: &Option<ActionHash>,
    new_base: &Option<ActionHash>,
    link_type: LinkTypes,
    revision: &ActionHash,
    id: &ActionHash,
) -> ExternResult<()> {
    if let Some(old) = old_base {
        if new_base.as_ref() != Some(old) {
            delete_links(old.clone().into(), id.clone().into(), link_type.clone())?;
        }
    }
    if let Some(base) = new_base {
        update_link(base.clone().into(), revision.clone(), link_type, id.clone().into())?;
    }
    Ok(())
}

/// The multi-valued counterpart of `reindex_single`, for `fulfills` and `satisfies`.
fn reindex_many(
    old_bases: &Option<Vec<ActionHash>>,
    new_bases: &Option<Vec<ActionHash>>,
    link_type: LinkTypes,
    revision: &ActionHash,
    id: &ActionHash,
) -> ExternResult<()> {
    let new_bases: &[ActionHash] = new_bases.as_deref().unwrap_or(&[]);
    for old in old_bases.as_deref().unwrap_or(&[]) {
        if !new_bases.contains(old) {
            delete_links(old.clone().into(), id.clone().into(), link_type.clone())?;
        }
    }
    for base in new_bases {
        update_link(base.clone().into(), revision.clone(), link_type.clone(), id.clone().into())?;
    }
    Ok(())
}

#[hdk_extern]
pub fn update_rea_economic_event(input: UpdateReaEconomicEventInput) -> ExternResult<Record> {
    let latest_record =
        get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the latest record".to_string())
        ))?;
    let latest_record_decoded = ReaEconomicEvent::try_from(latest_record.clone())?;

    reject_immutable_change(&latest_record_decoded.input_of, &input.entry.input_of, "inputOf")?;
    reject_immutable_change(&latest_record_decoded.output_of, &input.entry.output_of, "outputOf")?;
    reject_immutable_change(&latest_record_decoded.corrects, &input.entry.corrects, "corrects")?;
    ensure_reciprocal_agreement(
        &latest_record_decoded.reciprocal_realization_of,
        &input.entry.reciprocal_realization_of,
    )?;

    let mut updated_rea_entry: ReaEconomicEvent =
        merge_partial(input.entry, latest_record_decoded.clone())?;
    updated_rea_entry.id = latest_record_decoded
        .id
        .clone()
        .or(Some(input.revision_id.clone()));
    let id = updated_rea_entry
        .id
        .clone()
        .unwrap_or(input.revision_id.clone());
    let updated_rea_action_hash = update_entry(id.clone(), &updated_rea_entry)?;
    create_link(
        id.clone(),
        updated_rea_action_hash.clone(),
        LinkTypes::ReaEconomicEventUpdates,
        (),
    )?;

    // Relationship indexes are maintained against old and new values, so a
    // changed base loses its link instead of keeping one to a stale revision.
    let old = &latest_record_decoded;
    let new = &updated_rea_entry;
    let rev = &updated_rea_action_hash;
    reindex_single(&old.input_of, &new.input_of, LinkTypes::ReaProcessToReaEconomicEventInputs, rev, &id)?;
    reindex_single(&old.output_of, &new.output_of, LinkTypes::ReaProcessToReaEconomicEventOutputs, rev, &id)?;
    reindex_single(&old.provider, &new.provider, LinkTypes::ProviderToReaEconomicEvents, rev, &id)?;
    reindex_single(&old.receiver, &new.receiver, LinkTypes::ReceiverToReaEconomicEvents, rev, &id)?;
    reindex_single(&old.realization_of, &new.realization_of, LinkTypes::ReaAgreementToReaEconomicEvents, rev, &id)?;
    reindex_single(&old.triggered_by, &new.triggered_by, LinkTypes::ReaEconomicEventToReaEconomicEvents, rev, &id)?;
    reindex_single(&old.settles, &new.settles, LinkTypes::ClaimToSettlingEvents, rev, &id)?;
    reindex_many(&old.fulfills, &new.fulfills, LinkTypes::CommitmentToFulfillingEconomicEvents, rev, &id)?;
    // same pairing as create: events index under their own type
    reindex_many(&old.satisfies, &new.satisfies, LinkTypes::IntentToSatisfyingEconomicEvents, rev, &id)?;

    let path = Path::from("all_economic_events");
    update_link(
        AnyLinkableHash::from(path.path_entry_hash()?),
        updated_rea_action_hash.clone(),
        LinkTypes::AllEconomicEvents,
        id.clone().into(),
    )?;

    let record =
        get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the newly updated ReaEconomicEvent".to_string())
        ))?;
    Ok(record)
}

// The create and update paths above already write these three link families.
// Only the readers were missing, so `process.observedInputs`,
// `process.observedOutputs` and `agreement.economicEvents` resolved to a
// "zome function that doesn't exist" error in the GraphQL adapter.

#[hdk_extern]
pub fn get_rea_economic_event_inputs_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_process_hash,
            LinkTypes::ReaProcessToReaEconomicEventInputs,
        )?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_rea_economic_event_outputs_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_process_hash,
            LinkTypes::ReaProcessToReaEconomicEventOutputs,
        )?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_rea_economic_events_for_rea_agreement(
    rea_agreement_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_agreement_hash,
            LinkTypes::ReaAgreementToReaEconomicEvents,
        )?,
        GetStrategy::Local,
    )
}
