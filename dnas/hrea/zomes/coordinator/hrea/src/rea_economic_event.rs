use hdk::prelude::*;
use hrea_integrity::*;
use crate::helpers::*;
use vf_actions::{get_builtin_action, ActionEffect};

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct EconomicEventWithResource {
    pub economic_event: ReaEconomicEvent,
    pub resource: ReaEconomicResource,
}

#[hdk_extern]
pub fn create_economic_event_with_resource(
    event_with_resource: EconomicEventWithResource,
) -> ExternResult<Record> {
    let rea_economic_resource = ReaEconomicResource {
        primary_accountable: event_with_resource.economic_event.receiver.clone(),
        accounting_quantity: event_with_resource.economic_event.resource_quantity.clone(),
        onhand_quantity: event_with_resource.economic_event.resource_quantity.clone(),
        ..event_with_resource.resource
    };

    let rea_economic_event = event_with_resource.economic_event;

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
    let rea_economic_event = ReaEconomicEvent {
        resource_inventoried_as: Some(rea_economic_resource_hash.clone()),
        ..rea_economic_event
    };

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
                LinkTypes::IntentToSatisfyingCommitments,
                econ_tag_prefix.clone(),
            )?;
        }
    }

    create_link(
        rea_economic_resource_hash.clone(),
        rea_economic_event_hash.clone(),
        LinkTypes::ReaEconomicResourceToReaEconomicEvents,
        econ_tag_prefix.clone(),
    )?;

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
    Ok(record)
}

#[hdk_extern]
pub fn create_rea_economic_event(rea_economic_event: ReaEconomicEvent) -> ExternResult<Record> {
    let rea_economic_event_hash =
        create_entry(&EntryTypes::ReaEconomicEvent(rea_economic_event.clone()))?;

    // Affect the resource
    if let Some(resource_id) = rea_economic_event.resource_inventoried_as.clone() {

        let links = get_links(
            GetLinksInputBuilder::try_new(
                resource_id.clone(),
                LinkTypes::ReaEconomicResourceUpdates,
            )?
            .build(),
        )?;
        let latest_link = links
            .into_iter()
            .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
        let latest_rea_economic_resource_hash = match latest_link {
            Some(link) => {
                link.target
                    .clone()
                    .into_action_hash()
                    .ok_or(wasm_error!(WasmErrorInner::Guest(
                        "No action hash associated with link".to_string()
                    )))?
            }
            None => resource_id.clone(),
        };

        let resource_bytes =
            get(latest_rea_economic_resource_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
                WasmErrorInner::Guest("Could not find the resource".to_string())
            ))?;
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
        let resource_update_params = ReaEconomicResource {
            onhand_quantity: Some(QuantityValue {
                has_numerical_value: apply_effect(
                    action_info.clone().unwrap().onhand_effect.clone(),
                    resource
                        .clone()
                        .onhand_quantity
                        .unwrap()
                        .has_numerical_value,
                    rea_economic_event
                        .clone()
                        .resource_quantity
                        .unwrap()
                        .has_numerical_value,
                ),
                has_unit: resource.clone().onhand_quantity.unwrap().has_unit.clone(),
            }),
            accounting_quantity: Some(QuantityValue {
                has_numerical_value: apply_effect(
                    action_info.unwrap().accounting_effect.clone(),
                    resource
                        .clone()
                        .accounting_quantity
                        .unwrap()
                        .has_numerical_value,
                    rea_economic_event
                        .resource_quantity
                        .unwrap()
                        .has_numerical_value,
                ),
                has_unit: resource.accounting_quantity.unwrap().has_unit.clone(),
            }),
            ..resource
        };
        debug!("Resource update params: {:?}", resource_update_params);

        let updated_resource = update_entry(
            latest_rea_economic_resource_hash.clone(),
            &EntryTypes::ReaEconomicResource(resource_update_params),
        )?;
        create_link(
            resource_id.clone(),
            updated_resource.clone(),
            LinkTypes::ReaEconomicResourceUpdates,
            (),
        )?;

        delete_entry(resource_id.clone())?;

        create_link(
            resource_id,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaEconomicResourceToReaEconomicEvents,
            (),
        )?;
    }

    if let Some(base) = rea_economic_event.input_of.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaProcessToReaEconomicEventInputs,
            (),
        )?;
    }
    if let Some(base) = rea_economic_event.output_of.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaProcessToReaEconomicEventOutputs,
            (),
        )?;
    }
    if let Some(base) = rea_economic_event.provider.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ProviderToReaEconomicEvents,
            (),
        )?;
    }
    if let Some(base) = rea_economic_event.receiver.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReceiverToReaEconomicEvents,
            (),
        )?;
    }
    if let Some(base) = rea_economic_event.realization_of.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaAgreementToReaEconomicEvents,
            (),
        )?;
    }
    if let Some(base) = rea_economic_event.triggered_by.clone() {
        create_link(
            base,
            rea_economic_event_hash.clone(),
            LinkTypes::ReaEconomicEventToReaEconomicEvents,
            (),
        )?;
    }
    if let Some(base) = rea_economic_event.fulfills.clone() {
        for b in base {
            create_link(
                b,
                rea_economic_event_hash.clone(),
                LinkTypes::CommitmentToFulfillingEconomicEvents,
                (),
            )?;
        }
    }
    if let Some(base) = rea_economic_event.satisfies.clone() {
        for b in base {
            create_link(
                b,
                rea_economic_event_hash.clone(),
                LinkTypes::IntentToSatisfyingCommitments,
                (),
            )?;
        }
    }
    let record =
        get(rea_economic_event_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the newly created ReaEconomicEvent".to_string())
        ))?;
    let path = Path::from("all_economic_events");
    create_link(
        path.path_entry_hash()?,
        rea_economic_event_hash.clone(),
        LinkTypes::AllEconomicEvents,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_rea_economic_event(
    original_rea_economic_event_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            original_rea_economic_event_hash.clone(),
            LinkTypes::ReaEconomicEventUpdates,
        )?
        .build(),
    )?;
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
    let links = get_links(
        GetLinksInputBuilder::try_new(
            original_rea_economic_event_hash.clone(),
            LinkTypes::ReaEconomicEventUpdates,
        )?
        .build(),
    )?;
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
    let latest_record = get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the latest record".to_string())
    ))?;
    let latest_record_decoded = ReaEconomicEvent::try_from(latest_record.clone())?;
    let mut updated_rea_entry: ReaEconomicEvent = latest_record_decoded.clone();
    // only update note field
    updated_rea_entry.note = input.entry.note.clone();
    updated_rea_entry.id = latest_record_decoded.id.clone().or(Some(input.revision_id.clone()));
    let id = updated_rea_entry.id.clone().unwrap_or(input.revision_id.clone());
    let updated_rea_action_hash = update_entry( id.clone(), &updated_rea_entry, )?;
    create_link( id.clone(), updated_rea_action_hash.clone(), LinkTypes::ReaEconomicEventUpdates, (), )?;

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
                LinkTypes::IntentToSatisfyingCommitments,
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

    let record = get(
        updated_rea_action_hash.clone(),
        GetOptions::default(),
    )?
    .ok_or(wasm_error!(WasmErrorInner::Guest(
        "Could not find the newly updated ReaEconomicEvent".to_string()
    )))?;
    Ok(record)
}

// #[hdk_extern]
// pub fn delete_rea_economic_event(
//     original_rea_economic_event_hash: ActionHash,
// ) -> ExternResult<ActionHash> {
//     let details = get_details(
//         original_rea_economic_event_hash.clone(),
//         GetOptions::default(),
//     )?
//     .ok_or(wasm_error!(WasmErrorInner::Guest(
//         "ReaEconomicEvent not found".to_string()
//     )))?;
//     let record = match details {
//         Details::Record(details) => Ok(details.record),
//         _ => Err(wasm_error!(WasmErrorInner::Guest(
//             "Malformed get details response".to_string()
//         ))),
//     }?;
//     let entry = record
//         .entry()
//         .as_option()
//         .ok_or(wasm_error!(WasmErrorInner::Guest(
//             "ReaEconomicEvent record has no entry".to_string()
//         )))?;
//     let rea_economic_event = <ReaEconomicEvent>::try_from(entry)?;
//     if let Some(base_address) = rea_economic_event.input_of.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(
//                 base_address,
//                 LinkTypes::ReaProcessToReaEconomicEventInputs,
//             )?
//             .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     if let Some(base_address) = rea_economic_event.output_of.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(
//                 base_address,
//                 LinkTypes::ReaProcessToReaEconomicEventOutputs,
//             )?
//             .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     if let Some(base_address) = rea_economic_event.provider.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(base_address, LinkTypes::ProviderToReaEconomicEvents)?
//                 .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     if let Some(base_address) = rea_economic_event.receiver.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(base_address, LinkTypes::ReceiverToReaEconomicEvents)?
//                 .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     if let Some(base_address) = rea_economic_event.realization_of.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(
//                 base_address,
//                 LinkTypes::ReaAgreementToReaEconomicEvents,
//             )?
//             .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     if let Some(base_address) = rea_economic_event.triggered_by.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(
//                 base_address,
//                 LinkTypes::ReaEconomicEventToReaEconomicEvents,
//             )?
//             .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     if let Some(base_address) = rea_economic_event.resource_inventoried_as.clone() {
//         let links = get_links(
//             GetLinksInputBuilder::try_new(
//                 base_address,
//                 LinkTypes::ReaEconomicResourceToReaEconomicEvents,
//             )?
//             .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_event_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     let path = Path::from("all_economic_events");
//     let links = get_links(
//         GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllEconomicEvents)?
//             .build(),
//     )?;
//     for link in links {
//         if let Some(hash) = link.target.into_action_hash() {
//             if hash == original_rea_economic_event_hash {
//                 delete_link(link.create_link_hash)?;
//             }
//         }
//     }
//     delete_entry(original_rea_economic_event_hash)
// }

// #[hdk_extern]
// pub fn get_all_deletes_for_rea_economic_event(
//     original_rea_economic_event_hash: ActionHash,
// ) -> ExternResult<Option<Vec<SignedActionHashed>>> {
//     let Some(details) = get_details(original_rea_economic_event_hash, GetOptions::default())?
//     else {
//         return Ok(None);
//     };
//     match details {
//         Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
//             "Malformed details".into()
//         ))),
//         Details::Record(record_details) => Ok(Some(record_details.deletes)),
//     }
// }

// #[hdk_extern]
// pub fn get_oldest_delete_for_rea_economic_event(
//     original_rea_economic_event_hash: ActionHash,
// ) -> ExternResult<Option<SignedActionHashed>> {
//     let Some(mut deletes) =
//         get_all_deletes_for_rea_economic_event(original_rea_economic_event_hash)?
//     else {
//         return Ok(None);
//     };
//     deletes.sort_by(|delete_a, delete_b| {
//         delete_a
//             .action()
//             .timestamp()
//             .cmp(&delete_b.action().timestamp())
//     });
//     Ok(deletes.first().cloned())
// }

#[hdk_extern]
pub fn get_rea_economic_event_inputs_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            rea_process_hash,
            LinkTypes::ReaProcessToReaEconomicEventInputs,
        )?
        .build(),
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_event_inputs_for_rea_process(
//     rea_process_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_link_details(
//         rea_process_hash,
//         LinkTypes::ReaProcessToReaEconomicEventInputs,
//         None,
//         GetOptions::default(),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }

#[hdk_extern]
pub fn get_rea_economic_event_outputs_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            rea_process_hash,
            LinkTypes::ReaProcessToReaEconomicEventOutputs,
        )?
        .build(),
    )
}

#[hdk_extern]
pub fn get_rea_economic_events_for_provider(rea_agent_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_agent_hash, LinkTypes::ProviderToReaEconomicEvents)?
            .build(),
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_events_for_provider(
//     rea_agent_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_link_details(
//         rea_agent_hash,
//         LinkTypes::ProviderToReaEconomicEvents,
//         None,
//         GetOptions::default(),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }

#[hdk_extern]
pub fn get_rea_economic_events_for_receiver(rea_agent_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_agent_hash, LinkTypes::ReceiverToReaEconomicEvents)?
            .build(),
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_events_for_receiver(
//     rea_agent_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_link_details(
//         rea_agent_hash,
//         LinkTypes::ReceiverToReaEconomicEvents,
//         None,
//         GetOptions::default(),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }

#[hdk_extern]
pub fn get_rea_economic_events_for_rea_agreement(
    rea_agreement_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            rea_agreement_hash,
            LinkTypes::ReaAgreementToReaEconomicEvents,
        )?
        .build(),
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_events_for_rea_agreement(
//     rea_agreement_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_link_details(
//         rea_agreement_hash,
//         LinkTypes::ReaAgreementToReaEconomicEvents,
//         None,
//         GetOptions::default(),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }

#[hdk_extern]
pub fn get_rea_economic_events_for_rea_economic_resource(
    rea_economic_resource_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            rea_economic_resource_hash,
            LinkTypes::ReaEconomicResourceToReaEconomicEvents,
        )?
        .build(),
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_events_for_rea_economic_resource(
//     rea_economic_resource_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_link_details(
//         rea_economic_resource_hash,
//         LinkTypes::ReaEconomicResourceToReaEconomicEvents,
//         None,
//         GetOptions::default(),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }

#[hdk_extern]
pub fn get_rea_economic_events_for_rea_economic_event(
    rea_economic_event_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            rea_economic_event_hash,
            LinkTypes::ReaEconomicEventToReaEconomicEvents,
        )?
        .build(),
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_events_for_rea_economic_event(
//     rea_economic_event_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_link_details(
//         rea_economic_event_hash,
//         LinkTypes::ReaEconomicEventToReaEconomicEvents,
//         None,
//         GetOptions::default(),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }
