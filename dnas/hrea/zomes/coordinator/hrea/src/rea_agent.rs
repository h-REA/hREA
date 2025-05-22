use hdk::prelude::*;
use hrea_integrity::*;
use crate::helpers::*;

#[hdk_extern]
pub fn create_rea_agent(rea_agent: ReaAgent) -> ExternResult<Record> {
    let rea_agent_hash: ActionHash = create_entry(&EntryTypes::ReaAgent(rea_agent.clone()))?;
    let record = get(rea_agent_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created ReaAgent".to_string())
    ))?;
    let path = Path::from("all_agents");
    let tag_prefix = LinkTag(rea_agent_hash.get_raw_39().to_vec()); // Convert ActionHash to Vec<u8>
    create_link(
        path.path_entry_hash()?,
        rea_agent_hash.clone(),
        LinkTypes::AllAgents,
        tag_prefix.clone()
    )?;

    if rea_agent.agent_type == "Organization" {
        let path = Path::from("all_organizations");
        create_link(
            path.path_entry_hash()?,
            rea_agent_hash.clone(),
            LinkTypes::AllAgents,
            tag_prefix.clone()
        )?;
    } else if rea_agent.agent_type == "Person" {
        let path = Path::from("all_people");
        create_link(
            path.path_entry_hash()?,
            rea_agent_hash.clone(),
            LinkTypes::AllAgents,
            tag_prefix,
        )?;
    }

    Ok(record)
}

#[hdk_extern]
pub fn get_rea_agents_from_entry_hashes(
    entry_hashes: Vec<EntryHash>,
) -> ExternResult<Vec<Option<Record>>> {
    let get_input: Vec<GetInput> = entry_hashes
        .into_iter()
        .map(|entry_hash| {
            Ok(GetInput::new(
                entry_hash.into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    Ok(records)
}

#[hdk_extern]
pub fn get_rea_agents_from_action_hashes(
    action_hashes: Vec<ActionHash>,
) -> ExternResult<Vec<Option<Record>>> {
    let get_input: Vec<GetInput> = action_hashes
        .into_iter()
        .map(|action_hash| {
            Ok(GetInput::new(
                action_hash.into(),
                GetOptions::default(),
            ))
        })
        .collect::<ExternResult<Vec<GetInput>>>()?;
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    Ok(records)
}

#[hdk_extern]
pub fn get_latest_rea_agent(revision_id: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        GetLinksInputBuilder::try_new(revision_id.clone(), LinkTypes::ReaAgentUpdates)?
            .build(),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_rea_agent_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "No action hash associated with link".to_string()
                )))?
        }
        None => revision_id.clone(),
    };
    get(latest_rea_agent_hash, GetOptions::default())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateReaAgentInput {
    pub revision_id: ActionHash,
    pub entry: ReaAgent,
}

#[hdk_extern]
pub fn update_rea_agent(input: UpdateReaAgentInput) -> ExternResult<Record> {
    let latest_record = get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!( WasmErrorInner::Guest("Could not find the latest record".to_string()) ))?;
    let latest_record_decoded = ReaAgent::try_from(latest_record.clone())?;
    let mut updated_rea_entry = merge_fields(input.entry.clone(), latest_record_decoded.clone(),);
    let id = latest_record_decoded.id.clone().or(Some(input.revision_id.clone())).expect("Expected id to be Some, but found None");
    updated_rea_entry.id = Some(id.clone());
    let updated_rea_action_hash = update_entry( id.clone(), &updated_rea_entry, )?;
    create_link( id.clone(), updated_rea_action_hash.clone(), LinkTypes::ReaAgentUpdates, (), )?;

    update_link(
        AnyLinkableHash::from(Path::from("all_agents").path_entry_hash()?),
        updated_rea_action_hash.clone(),
        LinkTypes::AllAgents,
        id.clone().into(),
    )?;

    if latest_record_decoded.agent_type == "Organization" {
        let path = Path::from("all_organizations");
        update_link(
            AnyLinkableHash::from(path.path_entry_hash()?),
            updated_rea_action_hash.clone(),
            LinkTypes::AllAgents,
            id.clone().into(),
        )?;
    } else if latest_record_decoded.agent_type == "Person" {
        let path = Path::from("all_people");
        update_link(
            AnyLinkableHash::from(path.path_entry_hash()?),
            updated_rea_action_hash.clone(),
            LinkTypes::AllAgents,
            id.clone().into(),
        )?;
    }

    let record = get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated record".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_rea_agent(revision_id: ActionHash) -> ExternResult<ActionHash> {
    // get latest revision
    let latest_record = get(revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the latest record".to_string())
    ))?;
    // check if there is an id
    let latest_record_decoded = ReaAgent::try_from(latest_record.clone())?;
    let id = latest_record_decoded.id.clone()
        .or(Some(revision_id.clone()))
        .expect("Expected id to be Some, but found None");

    let path = AnyLinkableHash::from(Path::from("all_agents").path_entry_hash()?);
    delete_links(path, id.clone().into(), LinkTypes::AllAgents)?;
    let orgpath = AnyLinkableHash::from(Path::from("all_organizations").path_entry_hash()?);
    delete_links(orgpath, id.clone().into(), LinkTypes::AllAgents)?;
    let personpath = AnyLinkableHash::from(Path::from("all_people").path_entry_hash()?);
    delete_links(personpath, id.clone().into(), LinkTypes::AllAgents)?;
    delete_entry(id)
}

// ===============UNUSED FUNCTIONS===============
#[hdk_extern]
pub fn get_original_rea_agent(revision_id: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(revision_id, GetOptions::default())? else {
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
pub fn get_all_deletes_for_rea_agent(
    revision_id: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(revision_id, GetOptions::default())? else {
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
pub fn get_oldest_delete_for_rea_agent(
    revision_id: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_rea_agent(revision_id)? else {
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
pub fn get_all_revisions_for_rea_agent(
    revision_id: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_rea_agent(revision_id.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        GetLinksInputBuilder::try_new(revision_id.clone(), LinkTypes::ReaAgentUpdates)?
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