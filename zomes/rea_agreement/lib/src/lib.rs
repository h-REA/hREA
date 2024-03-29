/**
 * hREA agreement zome library API
 *
 * Contains helper methods that can be used to manipulate `Agreement` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package hREA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult, SignedActionHashed,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_action,
        update_record,
        delete_record,
    },
    metadata::read_revision_metadata_abbreviated,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_agreement_storage::*;
use hc_zome_rea_agreement_rpc::*;


pub use hc_zome_rea_agreement_storage::AGREEMENT_ENTRY_TYPE;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.agreement.index_zome)
}

pub fn handle_create_agreement<S>(entry_def_id: S, agreement: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, agreement)?;
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_agreement(address: AgreementAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_agreement(agreement: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let revision_hash = agreement.get_revision_id().clone();
    let (meta, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&revision_hash, agreement)?;
    construct_response(&identity_address, &meta, &entry, get_link_fields(&identity_address)?)
}

pub fn handle_delete_agreement(address: ActionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage>(&address)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &AgreementAddress, meta: &SignedActionHashed, e: &EntryData, (
        commitments,
        economic_events,
        // involved_agents,
    ): (
        Vec<CommitmentAddress>,
        Vec<EconomicEventAddress>,
        // Vec<AgentAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        agreement: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.name.to_owned(),
            created: e.created.to_owned(),
            note: e.note.to_owned(),
            commitments: commitments.to_owned(),
            economic_events: economic_events.to_owned(),
            // involved_agents: involved_agents.to_owned(),
        }
    })
}

//---------------- READ ----------------

/// Properties accessor for zome config
fn read_agreement_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.agreement.index_zome)
}

// @see construct_response
fn get_link_fields(base_address: &AgreementAddress) -> RecordAPIResult<(
    Vec<CommitmentAddress>,
    Vec<EconomicEventAddress>,
    // Vec<AgentAddress>,
)> {
    Ok((
        read_index!(agreement(base_address).commitments)?,
        read_index!(agreement(base_address).economic_events)?,
        // read_index!(agreement(base_address).involved_agents)?,
    ))
}
