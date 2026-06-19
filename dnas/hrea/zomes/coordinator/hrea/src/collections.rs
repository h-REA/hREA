use hdk::prelude::*;
use hrea_integrity::*;

#[hdk_extern]
pub fn get_generic_entries(action_hashes: Vec<ActionHash>) -> ExternResult<Vec<Record>> {
    let get_input: Vec<GetInput> = action_hashes
        .into_iter()
        .map(|hash| GetInput::new(hash.into(), GetOptions::default()))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}

#[hdk_extern]
pub fn get_all_agents() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_agents");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_organizations() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_organizations");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_people() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_people");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllAgents)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_units() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_units");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllUnits)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_resource_specifications() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_resource_specifications");
    let links_query = LinkQuery::try_new(
        path.path_entry_hash()?,
        LinkTypes::AllResourceSpecifications,
    )?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_recipe_processes() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_recipe_processes");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllRecipeProcesses)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_recipe_exchanges() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_recipe_exchanges");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllRecipeExchanges)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_proposals() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_proposals");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllProposals)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_claims() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_claims");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllClaims)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_spatial_things() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_spatial_things");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllSpatialThings)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_agreement_bundles() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_agreement_bundles");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllAgreementBundles)?;
    get_links(links_query, GetStrategy::Local)
}

/// VF 1.0: list proposals filtered by vf:Proposal.purpose ("offer" | "request").
/// Mirrors `get_all_proposals` but queries the purpose-specific index path, so a
/// caller can fetch only offers or only requests in a single DHT-native lookup.
#[hdk_extern]
pub fn get_proposals_by_purpose(purpose: String) -> ExternResult<Vec<Link>> {
    let path = match purpose.as_str() {
        "offer" => Path::from("all_offer_proposals"),
        "request" => Path::from("all_request_proposals"),
        _ => {
            return Err(wasm_error!(WasmErrorInner::Guest(format!(
                "Invalid Proposal purpose '{purpose}': must be 'offer' or 'request'"
            ))))
        }
    };
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllProposals)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_processes() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_processes");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllProcesses)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_process_specifications() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_process_specifications");
    let links_query =
        LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllProcessSpecifications)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_plans() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_plans");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllPlans)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_economic_resources() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_economic_resources");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllEconomicResources)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_economic_events() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_economic_events");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllEconomicEvents)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_agreements() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_agreements");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllAgreements)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_commitments() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_commitments");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllCommitments)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_intents() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_intents");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllIntents)?;
    get_links(links_query, GetStrategy::Local)
}

#[hdk_extern]
pub fn get_all_recipe_flows() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_recipe_flows");
    let links_query = LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllRecipeFlows)?;
    get_links(links_query, GetStrategy::Local)
}
