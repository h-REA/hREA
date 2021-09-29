/**
* Holo-REA proposal addresses zome library API
*
* Contains helper methods that can be used to manipulate `ProposedTo` data
* structures in either the local Holochain zome, or a separate DNA-local zome.
*
* @package Holo-REA
*/
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    records::{
        create_record,
        delete_record,
        read_record_entry,
        read_record_entry_by_header,
    },
    foreign_indexes::{
        create_foreign_index,
        update_foreign_index,
    },
    local_indexes::query_index,
};

use hc_zome_rea_proposed_to_rpc::*;
use hc_zome_rea_proposed_to_storage::*;
use hc_zome_rea_proposed_to_storage_consts::*;

use hc_zome_rea_proposal_storage_consts::*;

pub fn handle_create_proposed_to<S>(entry_def_id: S, proposed_to: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision_id, base_address, entry_resp): (_, ProposedToAddress, EntryData) = create_record(&entry_def_id, proposed_to.to_owned())?;

    // handle link fields
    create_foreign_index(
        read_foreign_index_zome,
        &PROPOSED_TO_PROPOSAL_INDEXING_API_METHOD,
        &base_address,
        read_foreign_proposal_index_zome,
        &PROPOSAL_PROPOSED_TO_INDEXING_API_METHOD,
        &proposed_to.proposed,
    )?;

    Ok(construct_response(&base_address, &revision_id, &entry_resp))
}

pub fn handle_get_proposed_to<S>(entry_def_id: S, address: ProposedToAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    Ok(construct_response(&base_address, &revision, &entry))
}

pub fn handle_delete_proposed_to(revision_id: &RevisionHash) -> RecordAPIResult<bool>
{
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    update_foreign_index(
        read_foreign_index_zome,
        &PROPOSED_TO_PROPOSAL_INDEXING_API_METHOD,
        &base_address,
        read_foreign_proposal_index_zome,
        &PROPOSAL_PROPOSED_TO_INDEXING_API_METHOD,
        &vec![],
        &vec![entry.proposed],
    )?;

    delete_record::<EntryStorage,_>(&revision_id)
}

const READ_FN_NAME: &str = "get_proposed_to";

pub fn generate_query_handler<S, C, F>(
    foreign_zome_name_from_config: F,
    proposal_entry_def_id: S,
) -> impl FnOnce(&QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |params| {
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        match &params.proposed {
            Some(proposed) => {
                entries_result = query_index::<ResponseData, ProposedToAddress, C,F,_,_,_,_>(
                    &proposal_entry_def_id,
                    proposed, PROPOSAL_PUBLISHED_TO_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
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
fn construct_response<'a>(address: &ProposedToAddress, revision_id: &RevisionHash, e: &EntryData) -> ResponseData {
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
fn read_foreign_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.index_zome)
}

/// Properties accessor for zome config.
fn read_foreign_proposal_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_to.proposal_index_zome)
}
