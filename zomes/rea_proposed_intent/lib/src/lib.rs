/**
 * hREA proposed intents: maintains relationships between coordinated proposals and the individual intents that describe their planned enaction. zome library API
 *
 * Contains helper methods that can be used to manipulate `ProposedIntent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package hREA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult, SignedActionHashed,
    records::{
        create_record,
        delete_record,
        read_record_entry,
        read_record_entry_by_action,
    },
    metadata::read_revision_metadata_abbreviated,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_proposed_intent_rpc::*;
use hc_zome_rea_proposed_intent_storage::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_intent.index_zome)
}

pub fn handle_create_proposed_intent<S>(entry_def_id: S, proposed_intent: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_, ProposedIntentAddress, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, proposed_intent.to_owned())?;

    // handle link fields
    let r1 = create_index!(proposed_intent.published_in(&proposed_intent.published_in), proposal.publishes(&base_address));
    hdk::prelude::debug!("handle_create_proposed_intent::published_in index {:?}", r1);
    let r2 = create_index!(proposed_intent.publishes(proposed_intent.publishes.to_owned()), intent.proposed_in(&base_address));
    hdk::prelude::debug!("handle_create_proposed_intent::publishes index {:?}", r2);

    construct_response(&base_address, &meta, &entry_resp)
}

pub fn handle_get_proposed_intent(address: ProposedIntentAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry)
}

pub fn handle_delete_proposed_intent(revision_id: &ActionHash) -> RecordAPIResult<bool>
{
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // Notify indexing zomes in local DNA (& validate).
    // Allows authors of indexing modules to intervene in the deletion of a record.
    let r1 = update_index!(proposed_intent.published_in.not(&vec![entry.published_in]), proposal.publishes(&base_address));
    hdk::prelude::debug!("handle_delete_proposed_intent::published_in index {:?}", r1);

    // manage record deletion
    let res = delete_record::<EntryStorage>(&revision_id);

    // Update in associated foreign DNAs as well.
    // :TODO: If we caught errors here, foreign cells can also intervene in record deletion, and cause rollback.
    //        Is this desirable? Should the behaviour be configurable?
    let r2 = update_index!(proposed_intent.publishes.not(&vec![entry.publishes]), intent.proposed_in(&base_address));
    hdk::prelude::debug!("handle_delete_proposed_intent::publishes index {:?}", r2);

    res
}

/// Create response from input DHT primitives
fn construct_response<'a>(address: &ProposedIntentAddress, meta: &SignedActionHashed, e: &EntryData) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        proposed_intent: Response {
            // entry fields
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            reciprocal: e.reciprocal,
            // link field
            published_in: e.published_in.to_owned(),
            publishes: e.publishes.to_owned(),
        },
    })
}

/// Properties accessor for zome config.
fn read_proposed_intent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_intent.index_zome)
}

/// Properties accessor for zome config.
fn read_proposal_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.proposed_intent.proposal_index_zome)
}

/// Properties accessor for zome config.
fn read_intent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.proposed_intent.intent_index_zome
}
