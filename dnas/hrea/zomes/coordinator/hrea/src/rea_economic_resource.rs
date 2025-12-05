use crate::helpers::*;
use hdk::prelude::*;
use hrea_integrity::*;

// #[hdk_extern]
// pub fn create_rea_economic_resource(
//     rea_economic_resource: ReaEconomicResource,
// ) -> ExternResult<Record> {
//     let rea_economic_resource_hash = create_entry(&EntryTypes::ReaEconomicResource(
//         rea_economic_resource.clone(),
//     ))?;
//     if let Some(base) = rea_economic_resource.contained_in.clone() {
//         create_link(
//             base,
//             rea_economic_resource_hash.clone(),
//             LinkTypes::ReaEconomicResourceToReaEconomicResources,
//             (),
//         )?;
//     }
//     let record = get(rea_economic_resource_hash.clone(), GetOptions::default())?.ok_or(
//         wasm_error!(WasmErrorInner::Guest(
//             "Could not find the newly created ReaEconomicResource".to_string()
//         )),
//     )?;
//     let path = Path::from("all_economic_resources");
//     create_link(
//         path.path_entry_hash()?,
//         rea_economic_resource_hash.clone(),
//         LinkTypes::AllEconomicResources,
//         (),
//     )?;
//     Ok(record)
// }

#[hdk_extern]
pub fn get_latest_rea_economic_resource(
    original_rea_economic_resource_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links_query = LinkQuery::try_new(
        original_rea_economic_resource_hash.clone(),
        LinkTypes::ReaEconomicResourceUpdates,
    )?;
    let links = get_links(links_query, GetStrategy::Local)?;
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
        None => original_rea_economic_resource_hash.clone(),
    };
    get(latest_rea_economic_resource_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_original_rea_economic_resource(
    original_rea_economic_resource_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(original_rea_economic_resource_hash, GetOptions::default())?
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
pub fn get_all_revisions_for_rea_economic_resource(
    original_rea_economic_resource_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let Some(original_record) =
        get_original_rea_economic_resource(original_rea_economic_resource_hash.clone())?
    else {
        return Ok(vec![]);
    };
    let links_query = LinkQuery::try_new(
        original_rea_economic_resource_hash.clone(),
        LinkTypes::ReaEconomicResourceUpdates,
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
pub struct UpdateReaEconomicResourceInput {
    pub revision_id: ActionHash,
    pub entry: ReaEconomicResource,
}

#[hdk_extern]
pub fn update_rea_economic_resource(input: UpdateReaEconomicResourceInput) -> ExternResult<Record> {
    let latest_record =
        get(input.revision_id.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest("Could not find the latest record".to_string())
        ))?;
    let latest_record_decoded = ReaEconomicResource::try_from(latest_record.clone())?;
    let mut updated_rea_entry = merge_fields(input.entry.clone(), latest_record_decoded.clone());
    let id = latest_record_decoded
        .id
        .clone()
        .or(Some(input.revision_id.clone()))
        .expect("Expected id to be Some, but found None");
    updated_rea_entry.id = Some(id.clone());
    let updated_rea_action_hash = update_entry(id.clone(), &updated_rea_entry)?;
    let tag_prefix: LinkTag = LinkTag(id.get_raw_39().to_vec());
    create_link(
        id.clone(),
        updated_rea_action_hash.clone(),
        LinkTypes::ReaEconomicResourceUpdates,
        tag_prefix.clone(),
    )?;

    if let Some(base) = updated_rea_entry.contained_in.clone() {
        update_link(
            AnyLinkableHash::from(base),
            updated_rea_action_hash.clone(),
            LinkTypes::ReaEconomicResourceToReaEconomicResources,
            id.clone().into(),
        )?;
    }

    update_link(
        AnyLinkableHash::from(Path::from("all_economic_resources").path_entry_hash()?),
        updated_rea_action_hash.clone(),
        LinkTypes::AllEconomicResources,
        id.clone().into(),
    )?;

    let record = get(updated_rea_action_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(
            "Could not find the newly updated ReaEconomicResource".to_string()
        )),
    )?;
    Ok(record)
}

// #[hdk_extern]
// pub fn delete_rea_economic_resource(
//     original_rea_economic_resource_hash: ActionHash,
// ) -> ExternResult<ActionHash> {
//     let details = get_details(
//         original_rea_economic_resource_hash.clone(),
//         GetOptions::default(),
//     )?
//     .ok_or(wasm_error!(WasmErrorInner::Guest(
//         "ReaEconomicResource not found".to_string()
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
//             "ReaEconomicResource record has no entry".to_string()
//         )))?;
//     let rea_economic_resource = <ReaEconomicResource>::try_from(entry)?;
//     if let Some(base_address) = rea_economic_resource.contained_in.clone() {
//         let links = get_links(
//             LinkQuery::try_new(
//                 base_address,
//                 LinkTypes::ReaEconomicResourceToReaEconomicResources,
//             )?
//             .build(),
//         )?;
//         for link in links {
//             if let Some(action_hash) = link.target.into_action_hash() {
//                 if action_hash == original_rea_economic_resource_hash {
//                     delete_link(link.create_link_hash)?;
//                 }
//             }
//         }
//     }
//     let path = Path::from("all_economic_resources");
//     let links = get_links(
//         LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllEconomicResources)?
//             .build(),
//     )?;
//     for link in links {
//         if let Some(hash) = link.target.into_action_hash() {
//             if hash == original_rea_economic_resource_hash {
//                 delete_link(link.create_link_hash)?;
//             }
//         }
//     }
//     delete_entry(original_rea_economic_resource_hash)
// }

// #[hdk_extern]
// pub fn get_all_deletes_for_rea_economic_resource(
//     original_rea_economic_resource_hash: ActionHash,
// ) -> ExternResult<Option<Vec<SignedActionHashed>>> {
//     let Some(details) = get_details(original_rea_economic_resource_hash, GetOptions::default())?
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
// pub fn get_oldest_delete_for_rea_economic_resource(
//     original_rea_economic_resource_hash: ActionHash,
// ) -> ExternResult<Option<SignedActionHashed>> {
//     let Some(mut deletes) =
//         get_all_deletes_for_rea_economic_resource(original_rea_economic_resource_hash)?
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
pub fn get_rea_economic_resources_for_rea_economic_resource(
    rea_economic_resource_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        LinkQuery::try_new(
            rea_economic_resource_hash,
            LinkTypes::ReaEconomicResourceToReaEconomicResources,
        )?,
        GetStrategy::Local,
    )
}

// #[hdk_extern]
// pub fn get_deleted_rea_economic_resources_for_rea_economic_resource(
//     rea_economic_resource_hash: ActionHash,
// ) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
//     let details = get_links_details(LinkQuery::try_new(//         rea_economic_resource_hash, //         LinkTypes::ReaEconomicResourceToReaEconomicResources)?, GetStrategy::Local),
//     )?;
//     Ok(details
//         .into_inner()
//         .into_iter()
//         .filter(|(_link, deletes)| !deletes.is_empty())
//         .collect())
// }
