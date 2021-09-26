/**
 * Holo-REA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome library API
 *
 * Contains helper methods that can be used to manipulate `ProposedIntent` data
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
    local_indexes::query_index,
    foreign_indexes::{
        create_foreign_index,
        update_foreign_index,
    },
    remote_indexes::{
        create_remote_index,
        update_remote_index,
    },
};

use hc_zome_rea_proposed_intent_rpc::*;
use hc_zome_rea_proposed_intent_storage::*;
use hc_zome_rea_proposed_intent_storage_consts::*;
use hc_zome_rea_proposal_storage_consts::*;
use hc_zome_rea_proposed_intent_storage_consts::{INTENT_PUBLISHEDIN_INDEXING_API_METHOD, PROPOSED_INTENT_PROPOSES_INDEXING_API_METHOD};

pub fn receive_create_proposed_intent<S>(entry_def_id: S, proposed_intent: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    handle_create_proposed_intent(entry_def_id, &proposed_intent)
}

pub fn receive_get_proposed_intent<S>(entry_def_id: S, address: ProposedIntentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    Ok(construct_response(&base_address, &revision, &entry))
}

pub fn receive_delete_proposed_intent(revision_id: &RevisionHash) -> RecordAPIResult<bool> {
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // Notify indexing zomes in local DNA (& validate).
    // Allows authors of indexing modules to intervene in the deletion of a record.
    update_foreign_index(
        read_foreign_index_zome,
        &PROPOSED_INTENT_PROPOSAL_INDEXING_API_METHOD,
        &base_address,
        read_foreign_proposal_index_zome,
        &PROPOSAL_PROPOSED_INTENT_INDEXING_API_METHOD,
        &vec![], &vec![entry.published_in],
    )?;

    // manage record deletion
    let res = delete_record::<EntryStorage,_>(&revision_id);

    // Update in associated foreign DNAs as well.
    // :TODO: In this pattern, foreign cells can also intervene in record deletion, and cause rollback.
    //        Is this desirable? Should the behaviour be configurable?
    update_remote_index(
        read_foreign_index_zome,
        &PROPOSED_INTENT_PROPOSES_INDEXING_API_METHOD,
        &base_address,
        &INTENT_PUBLISHEDIN_INDEXING_API_METHOD,
        &vec![], &vec![entry.publishes],
    )?;

    res
}

fn handle_create_proposed_intent<S>(entry_def_id: S, proposed_intent: &CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision_id, base_address, entry_resp): (_, ProposedIntentAddress, EntryData) = create_record(&entry_def_id, proposed_intent.to_owned())?;

    // handle link fields
    create_foreign_index(
        read_foreign_index_zome,
        &PROPOSED_INTENT_PROPOSAL_INDEXING_API_METHOD,
        &base_address,
        read_foreign_proposal_index_zome,
        &PROPOSAL_PROPOSED_INTENT_INDEXING_API_METHOD,
        &proposed_intent.published_in,
    )?;

    // update in associated foreign DNA
    create_remote_index(
        read_foreign_index_zome,
        &PROPOSED_INTENT_PROPOSES_INDEXING_API_METHOD,
        &base_address,
        &INTENT_PUBLISHEDIN_INDEXING_API_METHOD,
        &vec![proposed_intent.publishes.to_owned()],
    )?;

    Ok(construct_response(&base_address, &revision_id, &entry_resp))
}

const READ_FN_NAME: &str = "get_proposed_intent";

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

        // :TODO: replace with real query filter logic
        match &params.published_in {
            Some(published_in) => {
                entries_result = query_index::<ResponseData, ProposedIntentAddress, C,F,_,_,_,_>(
                    &proposal_entry_def_id,
                    published_in, PROPOSAL_PUBLISHES_LINK_TAG,
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
pub fn construct_response<'a>(address: &ProposedIntentAddress, revision_id: &RevisionHash, e: &EntryData) -> ResponseData {
    ResponseData {
        proposed_intent: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
            reciprocal: e.reciprocal,
            // link field
            published_in: e.published_in.to_owned(),
            publishes: e.publishes.to_owned(),
        },
    }
}

/// Properties accessor for zome config.
fn read_foreign_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_intent.index_zome)
}

/// Properties accessor for zome config.
fn read_foreign_proposal_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_intent.proposal_index_zome)
}
