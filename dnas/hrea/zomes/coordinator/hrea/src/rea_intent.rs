use hdk::prelude::*;
use hrea_integrity::*;
use crate::helpers::*;

#[hdk_extern]
pub fn create_rea_intent(rea_intent: ReaIntent) -> ExternResult<Record> {
    let rea_intent_hash = create_entry(&EntryTypes::ReaIntent(rea_intent.clone()))?;
    let tag_prefix: LinkTag = LinkTag(rea_intent_hash.get_raw_39().to_vec());
    if let Some(base) = rea_intent.input_of.clone() {
        create_link(
            base,
            rea_intent_hash.clone(),
            LinkTypes::ReaProcessToReaIntentInputs,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_intent.output_of.clone() {
        create_link(
            base,
            rea_intent_hash.clone(),
            LinkTypes::ReaProcessToReaIntentOutputs,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_intent.provider.clone() {
        create_link(
            base,
            rea_intent_hash.clone(),
            LinkTypes::ProviderToReaIntents,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_intent.receiver.clone() {
        create_link(
            base,
            rea_intent_hash.clone(),
            LinkTypes::ReceiverToReaIntents,
            tag_prefix.clone(),
        )?;
    }
    let record = get(rea_intent_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created ReaIntent".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_rea_intent(original_rea_intent_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
            original_rea_intent_hash.clone(),
            LinkTypes::ReaIntentUpdates,
        )?
        .build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_rea_intent_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_rea_intent_hash.clone(),
    };
    get(latest_rea_intent_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_rea_intent(
    original_rea_intent_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_rea_intent_hash, GetOptions::default())? else {
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
pub fn get_all_revisions_for_rea_intent(
    original_rea_intent_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_rea_intent(original_rea_intent_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(
            original_rea_intent_hash.clone(),
            LinkTypes::ReaIntentUpdates,
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
pub struct UpdateReaIntentInput {
    pub revision_id: ActionHash,
    pub entry: ReaIntent,
}

#[hdk_extern]
pub fn update_rea_intent(input: UpdateReaIntentInput) -> ExternResult<Record> {
    let latest_record = get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!( WasmErrorInner::Guest("Could not find the latest record".to_string()) ))?;
    let latest_record_decoded = ReaIntent::try_from(latest_record.clone())?;
    let mut updated_rea_entry = merge_fields(input.entry.clone(), latest_record_decoded.clone(),);
    let id = latest_record_decoded.id.clone().or(Some(input.revision_id.clone())).expect("Expected id to be Some, but found None");
    updated_rea_entry.id = Some(id.clone());
    let updated_rea_action_hash = update_entry( id.clone(), &updated_rea_entry, )?;
    create_link( id.clone(), updated_rea_action_hash.clone(), LinkTypes::ReaIntentUpdates, (), )?;

    if let Some(base) = updated_rea_entry.input_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaProcessToReaIntentInputs,
            id.clone().into(),
        )?;
    }

    if let Some(base) = updated_rea_entry.output_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaProcessToReaIntentOutputs,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.provider.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ProviderToReaIntents,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.receiver.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReceiverToReaIntents,
            id.clone().into(),
        )?;
    }

    let record =
        get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the newly updated ReaIntent".to_string())
        ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_rea_intent(revision_id: ActionHash) -> ExternResult<ActionHash> {
    let latest_record = get(revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(WasmErrorInner::Guest("Could not find the latest record".to_string())))?;
    let latest_record_decoded = <ReaIntent>::try_from(latest_record)?;
    let id = latest_record_decoded.id.clone().or(Some(revision_id.clone())).expect("Expected id to be Some, but found None");
 
    if let Some(base_address) = latest_record_decoded.input_of.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReaProcessToReaIntentInputs,
        )?;
    }
    if let Some(base_address) = latest_record_decoded.output_of.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReaProcessToReaIntentOutputs,
        )?;
    }
    if let Some(base_address) = latest_record_decoded.provider.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ProviderToReaIntents,
        )?;
    }
    if let Some(base_address) = latest_record_decoded.receiver.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReceiverToReaIntents,
        )?;
    }
    delete_entry(id)
}

#[hdk_extern]
pub fn get_all_deletes_for_rea_intent(
    original_rea_intent_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_rea_intent_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}

#[hdk_extern]
pub fn get_oldest_delete_for_rea_intent(
    original_rea_intent_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_rea_intent(original_rea_intent_hash)? else {
        return Ok(None);
    };
    deletes.sort_by(|delete_a, delete_b| {
        delete_a
            .action()
            .timestamp()
            .cmp(&delete_b.action().timestamp())
    });
    Ok(deletes.first().cloned())
}

#[hdk_extern]
pub fn get_rea_intents_for_rea_process_inputs(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_process_hash, LinkTypes::ReaProcessToReaIntentInputs)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_deleted_rea_intents_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        rea_process_hash,
        LinkTypes::ReaProcessToReaIntentInputs,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_rea_intents_for_rea_process_outputs(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_process_hash, LinkTypes::ReaProcessToReaIntentOutputs)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_deleted_rea_intents_for_rea_process_outputs(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        rea_process_hash,
        LinkTypes::ReaProcessToReaIntentOutputs,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_rea_intents_for_provider(rea_agent_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_agent_hash, LinkTypes::ProviderToReaIntents)?.build(),
    )
}

#[hdk_extern]
pub fn get_deleted_rea_intents_for_provider(
    rea_agent_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        rea_agent_hash,
        LinkTypes::ProviderToReaIntents,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_rea_intents_for_receiver(rea_agent_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_agent_hash, LinkTypes::ReceiverToReaIntents)?.build(),
    )
}

#[hdk_extern]
pub fn get_deleted_rea_intents_for_receiver(
    rea_agent_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        rea_agent_hash,
        LinkTypes::ReceiverToReaIntents,
        None,
        GetOptions::default(),
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_satisfying_comitments_for_rea_intent(
    rea_intent_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_intent_hash, LinkTypes::IntentToSatisfyingCommitments)?.build(),
    )
}

#[hdk_extern]
pub fn get_satisfying_economic_events_for_rea_intent(
    rea_intent_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(rea_intent_hash, LinkTypes::IntentToSatisfyingEconomicEvents)?.build(),
    )
}