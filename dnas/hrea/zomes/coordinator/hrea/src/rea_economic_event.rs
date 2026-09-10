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

#[derive(Serialize, Deserialize, Debug)]
pub struct ReaEconomicEventUpdateParams {
    pub note: Option<String>,
    pub input_of: Option<ActionHash>,
    pub output_of: Option<ActionHash>,
    pub provider: Option<ActionHash>,
    pub receiver: Option<ActionHash>,
    pub realization_of: Option<ActionHash>,
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

#[hdk_extern]
pub fn update_rea_economic_event(input: UpdateReaEconomicEventInput) -> ExternResult<Record> {
    let latest_record =
        get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the latest record".to_string())
        ))?;
    let latest_record_decoded = ReaEconomicEvent::try_from(latest_record.clone())?;
    let mut updated_rea_entry: ReaEconomicEvent = latest_record_decoded.clone();
    // only update note field
    updated_rea_entry.note = input.entry.note.clone();
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

    if let Some(base) = updated_rea_entry.input_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaProcessToReaEconomicEventInputs,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.output_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaProcessToReaEconomicEventOutputs,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.provider.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ProviderToReaEconomicEvents,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.receiver.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReceiverToReaEconomicEvents,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.realization_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaAgreementToReaEconomicEvents,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.triggered_by.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaEconomicEventToReaEconomicEvents,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.fulfills.clone() {
        for b in base {
            update_link(
                AnyLinkableHash::from(b),
                updated_rea_action_hash.clone(),
                LinkTypes::CommitmentToFulfillingEconomicEvents,
                id.clone().into(),
            )?;
        }
    }
    if let Some(base) = updated_rea_entry.satisfies.clone() {
        for b in base {
            update_link(
                AnyLinkableHash::from(b),
                updated_rea_action_hash.clone(),
                // same pairing as create: events index under their own type
                LinkTypes::IntentToSatisfyingEconomicEvents,
                id.clone().into(),
            )?;
        }
    }

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
