/**
 * Holo-REA agreement zome library API
 *
 * Contains helper methods that can be used to manipulate `Agreement` data
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
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_agreement_storage::*;
use hc_zome_rea_agreement_rpc::*;
use hc_zome_rea_agreement_integrity::*;

pub use hc_zome_rea_agreement_storage::AGREEMENT_ENTRY_TYPE;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.agreement.index_zome)
}

pub fn handle_create_agreement<S>(entry_def_id: S, agreement: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record(read_index_zome, &entry_def_id, agreement)?;
    construct_response(&base_address, meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_agreement<S>(entry_def_id: S, address: AgreementAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_agreement<S>(entry_def_id: S, agreement: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let revision_hash = agreement.get_revision_id().clone();
    let (meta, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&entry_def_id, &revision_hash, agreement)?;
    construct_response(&identity_address, meta, &entry, get_link_fields(&identity_address)?)
}

pub fn handle_delete_agreement(address: HeaderHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage>(&address)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &AgreementAddress, meta: RevisionMeta, e: &EntryData, (
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
            revision_id: meta.id.to_owned(),
            meta: meta.to_owned(),
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
