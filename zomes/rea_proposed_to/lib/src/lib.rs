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
    RecordAPIResult,
    records::{
        create_record,
        delete_record,
        read_record_entry,
        read_record_entry_by_header,
    },
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_storage::*;

pub fn handle_create_proposed_to<S>(entry_def_id: S, proposed_to: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision_id, base_address, entry_resp): (_, ProposedToAddress, EntryData) = create_record(&entry_def_id, proposed_to.to_owned())?;

    // handle link fields
    let r1 = create_index!(proposed_to.proposed(&proposed_to.proposed), proposal.published_to(&base_address));
    hdk::prelude::debug!("handle_create_proposed_to::proposed index {:?}", r1);

    // :TODO: create index for retrieving all proposals for an agent

    Ok(construct_response(&base_address, &revision_id, &entry_resp))
}

pub fn handle_get_proposed_to<S>(entry_def_id: S, address: ProposedToAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(&entry_def_id, address.as_ref())?;
    Ok(construct_response(&base_address, &revision, &entry))
}

pub fn handle_delete_proposed_to(revision_id: &HeaderHash) -> RecordAPIResult<bool>
{
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    let e = update_index!(proposed_to.proposed.not(&vec![entry.proposed]), proposal.published_to(&base_address));
    hdk::prelude::debug!("handle_delete_proposed_to::proposed index {:?}", e);

    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
fn construct_response<'a>(address: &ProposedToAddress, revision_id: &HeaderHash, e: &EntryData) -> ResponseData {
    ResponseData {
        proposed_to: Response {
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
            proposed_to: e.proposed_to.to_owned(),
            proposed: e.proposed.to_owned(),
        },
    }
}

/// Properties accessor for zome config.
fn read_proposed_to_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.index_zome)
}

/// Properties accessor for zome config.
fn read_proposal_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.proposal_index_zome)
}
