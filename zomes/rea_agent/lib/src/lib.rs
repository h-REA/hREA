/**
 * Holo-REA agent zome library API
 *
 * Contains helper methods that can be used to manipulate `Agent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record, read_record_entry_by_header,
    }, agent_info, links::create_link,
    get_links, HdkLinkType, DataIntegrityError,
    WasmError,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_agent_storage::*;
use hc_zome_rea_agent_rpc::*;

pub use hc_zome_rea_agent_storage::AGENT_ENTRY_TYPE;

pub fn handle_create_agent<S>(entry_def_id: S, agent: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, agent)?;
    // TODO: check if contains `link_pub_key` field
    let pub_key = agent_info()?.agent_latest_pubkey;
    // TODO: error handling to match expect error type
    let _pub_key_link = create_link(pub_key, header_addr.clone(), HdkLinkType::Any, ())?;
    construct_response(&base_address, header_addr, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_my_agent() -> RecordAPIResult<ResponseData>
{
    let my_pub_key = agent_info()?.agent_latest_pubkey;
    let mut links = get_links(my_pub_key, None)?;
    // validation rules allow us to assume only one link
    match links.pop() {
        Some(link) => {
            let header_hash: HeaderHash = link.target.into();
            let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&header_hash)?;
            construct_response(&base_address, header_hash, &entry, get_link_fields(&base_address)?)
        },
        None => Err(DataIntegrityError::AgentNotLinked)
    }
}

pub fn handle_get_agent<S>(entry_def_id: S, address: AgentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, revision, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_agent<S>(entry_def_id: S, agent: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let revision_hash = agent.get_revision_id().clone();
    let (revision_id, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&entry_def_id, &revision_hash, agent)?;
    construct_response(&identity_address, revision_id, &entry, get_link_fields(&identity_address)?)
}

pub fn handle_delete_agent(revision_id: HeaderHash) -> RecordAPIResult<bool> {

    // load the record to ensure it is of the correct type
    let (_base_address, _entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;
    // This is where indexes would be updated if necessary

    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &AgentAddress, revision: HeaderHash, e: &EntryData, (
        // commitments,
        // intents,
        // economic_events,
        // inventoried_economic_resources,
        // plans,
        // processes,
        // proposals,
        commitments_as_provider,
        commitments_as_receiver,
        intents_as_provider,
        intents_as_receiver,
        economic_events_as_provider,
        economic_events_as_receiver,
    ): (
        // Vec<CommitmentAddress>,
        // Vec<EconomicEventAddress>,
        // Vec<IntentAddress>,
        // Vec<EconomicResourceAddress>,
        // Vec<PlanAddress>,
        // Vec<ProcessAddress>,
        // Vec<ProposalAddress>,
        Vec<CommitmentAddress>,
        Vec<CommitmentAddress>,
        Vec<IntentAddress>,
        Vec<IntentAddress>,
        Vec<EconomicEventAddress>,
        Vec<EconomicEventAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        agent: Response {
            id: address.to_owned(),
            revision_id: revision.to_owned(),
            name: e.name.to_owned(),
            agent_type: e.agent_type.to_owned(),
            image: e.image.to_owned(),
            classified_as: e.classified_as.to_owned(),
            note: e.note.to_owned(),
            // commitments: commitments.to_owned(),
            // economic_events: economic_events.to_owned(),
            // intents: intents.to_owned(),
            // inventoried_economic_resources: inventoried_economic_resources.to_owned(),
            // plans: plans.to_owned(),
            // processes: processes.to_owned(),
            // proposals: proposals.to_owned(),
            commitments_as_provider: commitments_as_provider.to_owned(),
            commitments_as_receiver: commitments_as_receiver.to_owned(),
            intents_as_provider: intents_as_provider.to_owned(),
            intents_as_receiver: intents_as_receiver.to_owned(),
            economic_events_as_provider: economic_events_as_provider.to_owned(),
            economic_events_as_receiver: economic_events_as_receiver.to_owned(),
        }
    })
}

//---------------- READ ----------------

/// Properties accessor for zome config
fn read_agent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.agent.index_zome)
}

// @see construct_response
fn get_link_fields(base_address: &AgentAddress) -> RecordAPIResult<(
    // Vec<CommitmentAddress>,
    // Vec<EconomicEventAddress>,
    // Vec<IntentAddress>,
    // Vec<EconomicResourceAddress>,
    // Vec<PlanAddress>,
    // Vec<ProcessAddress>,
    // Vec<ProposalAddress>,
    Vec<CommitmentAddress>,
    Vec<CommitmentAddress>,
    Vec<IntentAddress>,
    Vec<IntentAddress>,
    Vec<EconomicEventAddress>,
    Vec<EconomicEventAddress>,
)> {
    Ok((
        // read_index!(agent(base_address).commitments)?,
        // read_index!(agent(base_address).economic_events)?,
        // read_index!(agent(base_address).intents)?,
        // read_index!(agent(base_address).inventoried_economic_resources)?,
        // read_index!(agent(base_address).plans)?,
        // read_index!(agent(base_address).processes)?,
        // read_index!(agent(base_address).proposals)?,
        read_index!(agent(base_address).commitments_as_provider)?,
        read_index!(agent(base_address).commitments_as_receiver)?,
        read_index!(agent(base_address).intents_as_provider)?,
        read_index!(agent(base_address).intents_as_receiver)?,
        read_index!(agent(base_address).economic_events_as_provider)?,
        read_index!(agent(base_address).economic_events_as_receiver)?,
    ))
}
