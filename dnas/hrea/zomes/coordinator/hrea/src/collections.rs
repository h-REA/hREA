use hdk::prelude::*;
use hrea_integrity::*;

#[hdk_extern]
pub fn get_generic_entries(action_hashes: Vec<ActionHash>) -> ExternResult<Vec<Record>> {
    let get_input: Vec<GetInput> = action_hashes
        .into_iter()
        .map(|hash| GetInput::new(
            hash.into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}

#[hdk_extern]
pub fn get_all_agents() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_agents");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?.build(),
    )
}

#[hdk_extern]
pub fn get_all_organizations() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_organizations");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?.build(),
    )
}

#[hdk_extern]
pub fn get_all_people() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_people");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?.build(),
    )
}

#[hdk_extern]
pub fn get_all_units() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_units");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllUnits)?.build())
}

#[hdk_extern]
pub fn get_all_resource_specifications() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_resource_specifications");
    get_links(
        GetLinksInputBuilder::try_new(
            path.path_entry_hash()?,
            LinkTypes::AllResourceSpecifications,
        )?
        .build(),
    )
}

#[hdk_extern]
pub fn get_all_recipe_processes() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_recipe_processes");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllRecipeProcesses)?
            .build(),
    )
}


#[hdk_extern]
pub fn get_all_recipe_exchanges() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_recipe_exchanges");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllRecipeExchanges)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_all_proposals() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_proposals");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllProposals)?.build(),
    )
}

#[hdk_extern]
pub fn get_all_processes() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_processes");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllProcesses)?.build(),
    )
}

#[hdk_extern]
pub fn get_all_process_specifications() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_process_specifications");
    get_links(
        GetLinksInputBuilder::try_new(
            path.path_entry_hash()?,
            LinkTypes::AllProcessSpecifications,
        )?
        .build(),
    )
}

#[hdk_extern]
pub fn get_all_plans() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_plans");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllPlans)?.build())
}

#[hdk_extern]
pub fn get_all_economic_resources() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_economic_resources");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllEconomicResources)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_all_economic_events() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_economic_events");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllEconomicEvents)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_all_agreements() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_agreements");
    get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllAgreements)?.build(),
    )
}
