/**
* Holo-REA proposal zome library API
*
* Contains helper methods that can be used to manipulate `Proposal` data
* structures in either the local Holochain zome, or a separate DNA-local zome.
*
* @package Holo-REA
*/
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    foreign_indexes::read_foreign_index,
    local_indexes::query_index,
    records::{
        create_record,
        delete_record,
        read_record_entry,
        update_record,
    },
};

use hc_zome_rea_proposal_rpc::*;
use hc_zome_rea_proposal_storage::*;
use hc_zome_rea_proposal_storage_consts::*;
use hc_zome_rea_proposed_intent_storage_consts::PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG;
use hc_zome_rea_proposed_to_storage_consts::PROPOSED_TO_PROPOSED_LINK_TAG;

pub fn handle_create_proposal<S>(entry_def_id: S, proposal: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision_id, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, proposal)?;
    Ok(construct_response(&base_address, &revision_id, &entry_resp, get_link_fields(&base_address)?))
}

pub fn handle_get_proposal<S>(entry_def_id: S, address: ProposalAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    Ok(construct_response(&base_address, &revision, &entry, get_link_fields(&base_address)?))
}

pub fn handle_update_proposal<S>(entry_def_id: S, proposal: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let old_revision = proposal.get_revision_id().to_owned();
    let (revision_id, base_address, new_entry, _prev_entry): (_, ProposalAddress, EntryData, EntryData) = update_record(entry_def_id, &old_revision, proposal)?;
    Ok(construct_response(&base_address, &revision_id, &new_entry, get_link_fields(&base_address)?))
}

pub fn handle_delete_proposal(address: RevisionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage,_>(&address)
}

const READ_FN_NAME: &str = "get_proposal";

pub fn generate_query_handler<S, C, F>(
    foreign_zome_name_from_config: F,
    proposed_intent_entry_def_id: S,
    proposed_to_entry_def_id: S,
) -> impl FnOnce(&QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |params| {
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        match &params.publishes {
            Some(publishes) => {
                entries_result = query_index::<ResponseData, ProposalAddress, C,F,_,_,_,_>(
                    &proposed_intent_entry_def_id,
                    publishes, PROPOSED_INTENT_PUBLISHED_IN_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            }
            _ => (),
        };

        match &params.published_to {
            Some(published_to) => {
                entries_result = query_index::<ResponseData, ProposalAddress, C,F,_,_,_,_>(
                    &proposed_to_entry_def_id,
                    published_to, PROPOSED_TO_PROPOSED_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME
                );
            }
            _ => (),
        };

        // :TODO: return errors for UI, rather than filtering
        Ok(entries_result?.iter()
            .cloned()
            .filter_map(Result::ok)
            .collect())
    }
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &ProposalAddress,
    revision_id: &RevisionHash,
    e: &EntryData,
    (publishes, published_to): (
        Vec<ProposedIntentAddress>,
        Vec<ProposedToAddress>,
    ),
) -> ResponseData {
    ResponseData {
        proposal: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
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
    }
}

/// Properties accessor for zome config
fn read_foreign_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposal.index_zome)
}

fn get_link_fields<'a>(
    proposal: &ProposalAddress,
) -> RecordAPIResult<(
    Vec<ProposedIntentAddress>,
    Vec<ProposedToAddress>,
)> {
    Ok((
        read_foreign_index(read_foreign_index_zome, &PROPOSAL_PUBLISHES_READ_API_METHOD, proposal)?,
        read_foreign_index(read_foreign_index_zome, &PROPOSAL_PUBLISHED_TO_READ_API_METHOD, proposal)?,
    ))
}
