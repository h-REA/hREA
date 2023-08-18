/**
 * hREA agent zome library API
 *
 * Contains helper methods that can be used to manipulate `Agent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package hREA
 */
use paste::paste;
use hdk::prelude::*;
use hdk_records::{
    RecordAPIResult,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_action,
        update_record,
        delete_record,
    },
    metadata::read_revision_metadata_abbreviated,
    SignedActionHashed,
    DataIntegrityError,
    DnaAddressable,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_agent_storage::*;
use hc_zome_rea_agent_rpc::*;

pub use hc_zome_rea_agent_storage::AGENT_ENTRY_TYPE;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.agent.index_zome)
}

pub fn handle_create_agent<S>(entry_def_id: S, agent: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display
{
    let agent_type = agent.agent_type.clone();
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, agent)?;
    let e = update_string_index!(agent(&base_address).agent_type(vec![agent_type])<AgentTypeId>);
    hdk::prelude::debug!("handle_create_agent::agent_type index {:?}", e);
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

/*
This function exists to create a linkage between
the holochain `AgentPubKey` of the active user, and a particular
Valueflows Agent, which can act as the profile for that user.
This should error if one has already been associated.
*/
pub fn handle_associate_my_agent(agent_address: AgentAddress) -> RecordAPIResult<()>
{
    match handle_get_my_agent() {
        Ok(_agent) => {
            Err(DataIntegrityError::AgentAlreadyLinked)
        },
        Err(DataIntegrityError::AgentNotLinked) => {
            // good, continue
            let pub_key = agent_info()?.agent_latest_pubkey;
            // link to the entry external identity. the dna hash can always be recovered from
            // the host context by calling dna_info! and the internal identity recovered
            // from the combination of the two
            create_link(pub_key, agent_address.1, LinkTypes::MyAgent, ())?;
            Ok(())
        },
        Err(e) => Err(e)
    }
}

pub fn handle_get_my_agent() -> RecordAPIResult<ResponseData>
{
    let my_pub_key = agent_info()?.agent_latest_pubkey;
    handle_whois_query(my_pub_key)
}

pub fn handle_whois_query(agent_pubkey: AgentPubKey) -> RecordAPIResult<ResponseData>
{
    let mut links = get_links(agent_pubkey, LinkTypes::MyAgent, None)?;
    match links.pop() {
        Some(link) => {
            // reconstruct the full internal use identity, as it was the external use identity that
            // was written to the Link (see associate_my_agent)
            let identity_address = AgentAddress::new(dna_info()?.hash, link.target.into_entry_hash().unwrap());
            handle_get_agent(identity_address)
        },
        None => Err(DataIntegrityError::AgentNotLinked)
    }
}

pub fn handle_get_agent(address: AgentAddress) -> RecordAPIResult<ResponseData>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&base_address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_agent(agent: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let revision_hash = agent.get_revision_id().clone();
    let (meta, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&revision_hash, agent)?;
    construct_response(&identity_address, &meta, &entry, get_link_fields(&identity_address)?)
}

pub fn handle_delete_agent(revision_id: ActionHash) -> RecordAPIResult<bool> {

    // load the record to ensure it is of the correct type
    let (_revision, _base_address, _entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    // This is where indexes would be updated if necessary

    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &AgentAddress, meta: &SignedActionHashed, e: &EntryData, (
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
        inventoried_economic_resources,
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
        Vec<EconomicResourceAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        agent: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
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
            inventoried_economic_resources: inventoried_economic_resources.to_owned(),
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
    Vec<EconomicResourceAddress>,
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
        read_index!(agent(base_address).inventoried_economic_resources)?,
    ))
}
