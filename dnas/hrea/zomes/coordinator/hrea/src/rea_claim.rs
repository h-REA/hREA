use hdk::prelude::*;
use hrea_integrity::*;
use crate::helpers::*;

#[hdk_extern]
pub fn create_rea_claim(rea_claim: ReaClaim) -> ExternResult<Record> {
    let rea_claim_hash = create_entry(&EntryTypes::ReaClaim(rea_claim.clone()))?;
    let tag_prefix = LinkTag(rea_claim_hash.get_raw_39().to_vec());
    let record = get(rea_claim_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created ReaClaim".to_string())
    ))?;
    let path = Path::from("all_claims");
    create_link(
        path.path_entry_hash()?,
        rea_claim_hash.clone(),
        LinkTypes::AllClaims,
        tag_prefix
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_latest_rea_claim(original_rea_claim_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        LinkQuery::try_new(original_rea_claim_hash.clone(), LinkTypes::ReaClaimUpdates)?,
        GetStrategy::Local,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_rea_claim_hash = match latest_link {
        Some(link) => link
            .target
            .clone()
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?,
        None => original_rea_claim_hash.clone(),
    };
    get(latest_rea_claim_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_rea_claim(original_rea_claim_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_rea_claim_hash, GetOptions::default())? else {
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
pub fn get_all_revisions_for_rea_claim(
    original_rea_claim_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) = get_original_rea_claim(original_rea_claim_hash.clone())? else {
        return Ok(vec![]);
    };
    let links = get_links(
        LinkQuery::try_new(original_rea_claim_hash.clone(), LinkTypes::ReaClaimUpdates)?,
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
pub struct UpdateReaClaimInput {
    pub revision_id: ActionHash,
    pub entry: ReaClaim,
}

#[hdk_extern]
pub fn update_rea_claim(input: UpdateReaClaimInput) -> ExternResult<Record> {
    let latest_record = get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the latest record".to_string())
    ))?;
    let latest_record_decoded = ReaClaim::try_from(latest_record.clone())?;
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
        LinkTypes::ReaClaimUpdates,
        (),
    )?;

    update_link(
        AnyLinkableHash::from(Path::from("all_claims").path_entry_hash()?),
        updated_rea_action_hash.clone(),
        LinkTypes::AllClaims,
        id.clone().into(),
    )?;

    let record = get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly updated ReaClaim".to_string())
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn delete_rea_claim(revision_id: ActionHash) -> ExternResult<ActionHash> {
    let latest_record = get(revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the latest record".to_string())
    ))?;
    let latest_record_decoded = <ReaClaim>::try_from(latest_record)?;
    let id = latest_record_decoded
        .id
        .clone()
        .or(Some(revision_id.clone()))
        .expect("Expected id to be Some, but found None");

    let path = Path::from("all_claims");
    delete_links(
        AnyLinkableHash::from(path.path_entry_hash()?),
        id.clone().into(),
        LinkTypes::AllClaims,
    )?;
    delete_entry(id)
}
