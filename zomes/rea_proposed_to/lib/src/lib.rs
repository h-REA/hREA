/**
* Holo-REA proposal addresses zome library API
*
* Contains helper methods that can be used to manipulate `ProposedTo` data
* structures in either the local Holochain zome, or a separate DNA-local zome.
*
* @package Holo-REA
*/
use paste::paste;
use hdk_records::{
    RecordAPIResult, SignedHeaderHashed,
    records::{
        create_record,
        delete_record,
        read_record_entry,
        read_record_entry_by_action,
    },
    metadata::read_revision_metadata_abbreviated,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_storage::*;
use hc_zome_rea_proposed_to_integrity::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.index_zome)
}

pub fn handle_create_proposed_to<S>(entry_def_id: S, proposed_to: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_, ProposedToAddress, EntryData) = create_record(read_index_zome, &entry_def_id, proposed_to.to_owned())?;

    // handle link fields
    let r1 = create_index!(proposed_to.proposed(&proposed_to.proposed), proposal.published_to(&base_address));
    hdk::prelude::debug!("handle_create_proposed_to::proposed index {:?}", r1);

    // :TODO: create index for retrieving all proposals for an agent

    construct_response(&base_address, &meta, &entry_resp)
}

pub fn handle_get_proposed_to<S>(entry_def_id: S, address: ProposedToAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &meta, &entry)
}

pub fn handle_delete_proposed_to(revision_id: &ActionHash) -> RecordAPIResult<bool>
{
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    let e = update_index!(proposed_to.proposed.not(&vec![entry.proposed]), proposal.published_to(&base_address));
    hdk::prelude::debug!("handle_delete_proposed_to::proposed index {:?}", e);

    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(address: &ProposedToAddress, meta: &SignedHeaderHashed, e: &EntryData) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        proposed_to: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            proposed_to: e.proposed_to.to_owned(),
            proposed: e.proposed.to_owned(),
        },
    })
}

/// Properties accessor for zome config.
fn read_proposed_to_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.index_zome)
}

/// Properties accessor for zome config.
fn read_proposal_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.proposal_index_zome)
}
