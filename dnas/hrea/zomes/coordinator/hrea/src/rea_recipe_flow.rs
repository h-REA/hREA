use crate::helpers::*;
use hdk::prelude::*;
use hrea_integrity::*;

#[hdk_extern]
pub fn create_rea_recipe_flow(rea_recipe_flow: ReaRecipeFlow) -> ExternResult<Record> {
    let rea_recipe_flow_hash = create_entry(&EntryTypes::ReaRecipeFlow(rea_recipe_flow.clone()))?;
    let tag_prefix = LinkTag(rea_recipe_flow_hash.get_raw_39().to_vec());
    let all_recipe_flows_path = Path::from("all_recipe_flows");
    create_link(
        all_recipe_flows_path.path_entry_hash()?,
        rea_recipe_flow_hash.clone(),
        LinkTypes::AllRecipeFlows,
        tag_prefix.clone(),
    )?;
    if let Some(base) = rea_recipe_flow.recipe_clause_of.clone() {
        create_link(
            base,
            rea_recipe_flow_hash.clone(),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_recipe_flow.recipe_reciprocal_clause_of.clone() {
        create_link(
            base,
            rea_recipe_flow_hash.clone(),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_recipe_flow.recipe_input_of.clone() {
        create_link(
            base,
            rea_recipe_flow_hash.clone(),
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_recipe_flow.recipe_output_of.clone() {
        create_link(
            base,
            rea_recipe_flow_hash.clone(),
            LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs,
            tag_prefix,
        )?;
    }
    let record = get(rea_recipe_flow_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created ReaRecipeFlow".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_rea_recipe_flow(
    original_rea_recipe_flow_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        LinkQuery::try_new(
            original_rea_recipe_flow_hash.clone(),
            LinkTypes::ReaRecipeFlowUpdates,
        )?,
        GetStrategy::Local,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_rea_recipe_flow_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_rea_recipe_flow_hash.clone(),
    };
    get(latest_rea_recipe_flow_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_rea_recipe_flow(
    original_rea_recipe_flow_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_rea_recipe_flow_hash, GetOptions::default())? else {
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
pub fn get_all_revisions_for_rea_recipe_flow(
    original_rea_recipe_flow_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) =
        get_original_rea_recipe_flow(original_rea_recipe_flow_hash.clone())?
    else {
        return Ok(vec![]);
    };
    let links = get_links(
        LinkQuery::try_new(
            original_rea_recipe_flow_hash.clone(),
            LinkTypes::ReaRecipeFlowUpdates,
        )?,
        GetStrategy::Local,
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
pub struct UpdateReaRecipeFlowInput {
    pub revision_id: ActionHash,
    pub entry: ReaRecipeFlow,
}

#[hdk_extern]
pub fn update_rea_recipe_flow(input: UpdateReaRecipeFlowInput) -> ExternResult<Record> {
    let latest_record =
        get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the latest record".to_string())
        ))?;
    let latest_record_decoded = ReaRecipeFlow::try_from(latest_record.clone())?;
    let mut updated_rea_entry = merge_fields(input.entry.clone(), latest_record_decoded.clone());
    let id = latest_record_decoded
        .id
        .clone()
        .or(Some(input.revision_id.clone()))
        .expect("Expected id to be Some, but found None");
    updated_rea_entry.id = Some(id.clone());
    let updated_rea_action_hash = update_entry(id.clone(), &updated_rea_entry)?;
    create_link(
        id.clone(),
        updated_rea_action_hash.clone(),
        LinkTypes::ReaRecipeFlowUpdates,
        (),
    )?;

    if let Some(base) = updated_rea_entry.recipe_clause_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows,
            id.clone().into(),
        )?;
    }

    if let Some(base) = updated_rea_entry.recipe_reciprocal_clause_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.recipe_input_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.recipe_output_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs,
            id.into(),
        )?;
    }

    let record =
        get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the newly updated ReaRecipeFlow".to_string())
        ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_rea_recipe_flow(revision_id: ActionHash) -> ExternResult<ActionHash> {
    let latest_record = get(revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the latest record".to_string())
    ))?;
    let latest_record_decoded = <ReaRecipeFlow>::try_from(latest_record)?;
    let id = latest_record_decoded
        .id
        .clone()
        .or(Some(revision_id.clone()))
        .expect("Expected id to be Some, but found None");

    if let Some(base_address) = latest_record_decoded.recipe_clause_of.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows,
        )?;
    }
    if let Some(base_address) = latest_record_decoded.recipe_reciprocal_clause_of.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal,
        )?;
    }

    if let Some(base_address) = latest_record_decoded.recipe_input_of.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs,
        )?;
    }

    if let Some(base_address) = latest_record_decoded.recipe_output_of.clone() {
        delete_links(
            AnyLinkableHash::from(base_address),
            id.clone().into(),
            LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs,
        )?;
    }
    delete_entry(id)
}

#[hdk_extern]
pub fn get_all_deletes_for_rea_recipe_flow(
    original_rea_recipe_flow_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_rea_recipe_flow_hash, GetOptions::default())? else {
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
pub fn get_oldest_delete_for_rea_recipe_flow(
    original_rea_recipe_flow_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_rea_recipe_flow(original_rea_recipe_flow_hash)?
    else {
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
pub fn get_rea_recipe_clauses_for_rea_recipe_exchange(
    rea_recipe_exchange_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_recipe_exchange_hash,
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows,
        )?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_rea_recipe_reciprocal_clauses_for_rea_recipe_exchange(
    rea_recipe_exchange_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_recipe_exchange_hash,
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal,
        )?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_deleted_rea_recipe_clauses_for_rea_recipe_exchange(
    rea_recipe_exchange_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(
            rea_recipe_exchange_hash,
            LinkTypes::ReaRecipeExchangeToReaRecipeFlows,
        )?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_deleted_rea_recipe_reciprocal_clauses_for_rea_recipe_exchange(
    rea_recipe_exchange_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(
            rea_recipe_exchange_hash,
            LinkTypes::ReaRecipeExchangeToReaRecipeFlowsReciprocal,
        )?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_rea_recipe_flow_inputs_for_rea_recipe_process(
    rea_recipe_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_recipe_process_hash,
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs,
        )?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_rea_recipe_flow_outputs_for_rea_recipe_process(
    rea_recipe_process_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_recipe_process_hash,
            LinkTypes::ReaRecipeProcessToReaRecipeFlowOutputs,
        )?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_deleted_rea_recipe_flow_inputs_for_rea_recipe_process(
    rea_recipe_process_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(
            rea_recipe_process_hash,
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs,
        )?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_deleted_rea_recipe_flow_outputs_for_rea_recipe_process(
    rea_recipe_process_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(
            rea_recipe_process_hash,
            LinkTypes::ReaRecipeProcessToReaRecipeFlowInputs,
        )?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
