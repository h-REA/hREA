/**
* Holo-REA proposal zome library API
*
* Contains helper methods that can be used to manipulate `Proposal` data
* structures in either the local Holochain zome, or a separate DNA-local zome.
*
* @package Holo-REA
*/
use paste::paste;
use hdk_records::{
    RecordAPIResult, SignedActionHashed,
    records::{
        create_record,
        delete_record,
        read_record_entry,
        read_record_entry_by_action,
        update_record,
    },
    metadata::read_revision_metadata_abbreviated,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_proposal_rpc::*;
use hc_zome_rea_proposal_storage::*;


/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposal.index_zome)
}

pub fn handle_create_proposal<S>(entry_def_id: S, proposal: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, proposal)?;
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_proposal(address: ProposalAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_proposal(proposal: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let old_revision = proposal.get_revision_id().to_owned();
    let (meta, base_address, new_entry, _prev_entry): (_, ProposalAddress, EntryData, EntryData) = update_record(&old_revision, proposal)?;
    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_proposal(address: ActionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage>(&address)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ProposalAddress,
    meta: &SignedActionHashed,
    e: &EntryData,
    (publishes, published_to): (
        Vec<ProposedIntentAddress>,
        Vec<ProposedToAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        proposal: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.name.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            unit_based: e.unit_based.to_owned(),
            created: e.created.to_owned(),
            note: e.note.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            // link fields
            publishes: publishes.to_owned(),
            published_to: published_to.to_owned(),
        },
    })
}

/// Properties accessor for zome config
fn read_proposal_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposal.index_zome)
}

fn get_link_fields<'a>(
    proposal: &ProposalAddress,
) -> RecordAPIResult<(
    Vec<ProposedIntentAddress>,
    Vec<ProposedToAddress>,
)> {
    Ok((
        read_index!(proposal(proposal).publishes)?,
        read_index!(proposal(proposal).published_to)?,
    ))
}
