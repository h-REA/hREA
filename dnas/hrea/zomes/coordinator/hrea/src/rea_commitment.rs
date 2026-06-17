use crate::helpers::*;
use hdk::prelude::*;
use hrea_integrity::*;

#[hdk_extern]
pub fn create_rea_commitment(rea_commitment: ReaCommitment) -> ExternResult<Record> {
    // if there is no action field, error
    if rea_commitment.rea_action.is_none() {
        return Err(wasm_error!(WasmErrorInner::Guest(
            "A new Commitment must have an action field".to_string()
        )));
    }
    let rea_commitment_hash = create_entry(&EntryTypes::ReaCommitment(rea_commitment.clone()))?;
    let tag_prefix = LinkTag(rea_commitment_hash.get_raw_39().to_vec());
    let all_commitments_path = Path::from("all_commitments");
    create_link(
        all_commitments_path.path_entry_hash()?,
        rea_commitment_hash.clone(),
        LinkTypes::AllCommitments,
        tag_prefix.clone(),
    )?;
    if let Some(base) = rea_commitment.input_of.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ReaProcessToInputs,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.output_of.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ReaProcessToOutputs,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.provider.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ProviderToReaCommitments,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.receiver.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ReceiverToReaCommitments,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.clause_of.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ReaAgreementToReaCommitments,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.planned_within.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ReaPlanToReaCommitments,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.independent_demand_of.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::ReaPlanToIndependentDemands,
            tag_prefix.clone(),
        )?;
    }
    if let Some(base) = rea_commitment.satisfies.clone() {
        create_link(
            base,
            rea_commitment_hash.clone(),
            LinkTypes::IntentToSatisfyingCommitments,
            tag_prefix,
        )?;
    }
    let record = get(rea_commitment_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created ReaCommitment".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_rea_commitment(
    original_rea_commitment_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links_query = LinkQuery::try_new(
        original_rea_commitment_hash.clone(),
        LinkTypes::ReaCommitmentUpdates,
    )?;
    let links = get_links(links_query, GetStrategy::Local)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_rea_commitment_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => original_rea_commitment_hash.clone(),
    };
    get(latest_rea_commitment_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_rea_commitment(
    original_rea_commitment_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_rea_commitment_hash, GetOptions::default())? else {
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
pub fn get_all_revisions_for_rea_commitment(
    original_rea_commitment_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_rea_commitment(original_rea_commitment_hash.clone())?
    else {
        return Ok(vec![]);
    };
    let links_query = LinkQuery::try_new(
        original_rea_commitment_hash.clone(),
        LinkTypes::ReaCommitmentUpdates,
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
pub struct UpdateReaCommitmentInput {
    pub revision_id: ActionHash,
    pub entry: ReaCommitment,
}

#[hdk_extern]
pub fn update_rea_commitment(input: UpdateReaCommitmentInput) -> ExternResult<Record> {
    let latest_record =
        get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the latest record".to_string())
        ))?;
    let latest_record_decoded = ReaCommitment::try_from(latest_record.clone())?;
    let mut updated_rea_entry = merge_fields(input.entry.clone(), latest_record_decoded.clone());
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
        LinkTypes::ReaCommitmentUpdates,
        (),
    )?;

    // update cross-entry links
    if let Some(base) = updated_rea_entry.input_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaProcessToInputs,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.output_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaProcessToOutputs,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.provider.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ProviderToReaCommitments,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.receiver.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReceiverToReaCommitments,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.clause_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaAgreementToReaCommitments,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.planned_within.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaPlanToReaCommitments,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.independent_demand_of.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaPlanToIndependentDemands,
            id.clone().into(),
        )?;
    }
    if let Some(base) = updated_rea_entry.satisfies.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::IntentToSatisfyingCommitments,
            id.clone().into(),
        )?;
    }

    let record =
        get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the newly updated record".to_string())
        ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_rea_commitment(revision_id: ActionHash) -> ExternResult<ActionHash> {
    // get latest revision
    let latest_record = get(revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the latest record".to_string())
    ))?;
    let latest_record_decoded = <ReaCommitment>::try_from(latest_record)?;
    let id = latest_record_decoded
        .id
        .clone()
        .or(Some(revision_id.clone()))
        .expect("Expected id to be Some, but found None");

    if let Some(base) = latest_record_decoded.input_of {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ReaProcessToInputs,
        )?;
    }
    if let Some(base) = latest_record_decoded.output_of {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ReaProcessToOutputs,
        )?;
    }
    if let Some(base) = latest_record_decoded.provider {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ProviderToReaCommitments,
        )?;
    }
    if let Some(base) = latest_record_decoded.receiver {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ReceiverToReaCommitments,
        )?;
    }
    if let Some(base) = latest_record_decoded.clause_of {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ReaAgreementToReaCommitments,
        )?;
    }
    if let Some(base) = latest_record_decoded.planned_within {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ReaPlanToReaCommitments,
        )?;
    }
    if let Some(base) = latest_record_decoded.independent_demand_of {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::ReaPlanToIndependentDemands,
        )?;
    }
    if let Some(base) = latest_record_decoded.satisfies {
        delete_links(
            AnyLinkableHash::from(base),
            id.clone().into(),
            LinkTypes::IntentToSatisfyingCommitments,
        )?;
    }
    // delete the entry
    delete_entry(id)
}

#[hdk_extern]
pub fn get_all_deletes_for_rea_commitment(
    original_rea_commitment_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_rea_commitment_hash, GetOptions::default())? else {
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
pub fn get_oldest_delete_for_rea_commitment(
    original_rea_commitment_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_rea_commitment(original_rea_commitment_hash)?
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
pub fn get_inputs_for_rea_process(rea_process_hash: ActionHash) -> ExternResult<Vec<Link>> {
    let links_query = LinkQuery::try_new(rea_process_hash, LinkTypes::ReaProcessToInputs)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_deleted_inputs_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_process_hash, LinkTypes::ReaProcessToInputs)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_outputs_for_rea_process(rea_process_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(rea_process_hash, LinkTypes::ReaProcessToOutputs)?,
        GetStrategy::Local,
    )
}

#[hdk_extern]
pub fn get_deleted_outputs_for_rea_process(
    rea_process_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_process_hash, LinkTypes::ReaProcessToOutputs)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_fulfilling_economic_events_for_commitment(
    commitment_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    let links_query = LinkQuery::try_new(
        commitment_hash,
        LinkTypes::CommitmentToFulfillingEconomicEvents,
    )?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_deleted_rea_commitments_for_provider(
    rea_agent_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_agent_hash, LinkTypes::ProviderToReaCommitments)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_deleted_rea_commitments_for_receiver(
    rea_agent_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_agent_hash, LinkTypes::ReceiverToReaCommitments)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_rea_commitments_for_rea_agreement(
    rea_agreement_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    let links_query =
        LinkQuery::try_new(rea_agreement_hash, LinkTypes::ReaAgreementToReaCommitments)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_deleted_rea_commitments_for_rea_agreement(
    rea_agreement_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_agreement_hash, LinkTypes::ReaAgreementToReaCommitments)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_rea_commitments_for_rea_plan(rea_plan_hash: ActionHash) -> ExternResult<Vec<Link>> {
    let links_query = LinkQuery::try_new(rea_plan_hash, LinkTypes::ReaPlanToReaCommitments)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_deleted_rea_commitments_for_rea_plan(
    rea_plan_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_plan_hash, LinkTypes::ReaPlanToReaCommitments)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}

#[hdk_extern]
pub fn get_independent_demands_for_rea_plan(rea_plan_hash: ActionHash) -> ExternResult<Vec<Link>> {
    let links_query = LinkQuery::try_new(rea_plan_hash, LinkTypes::ReaPlanToIndependentDemands)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_deleted_independent_demands_for_rea_plan(
    rea_plan_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_links_details(
        LinkQuery::try_new(rea_plan_hash, LinkTypes::ReaPlanToIndependentDemands)?,
        GetStrategy::Local,
    )?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| !deletes.is_empty())
        .collect())
}
