/**
 * Holo-REA intent zome library API
 *
 * Contains helper methods that can be used to manipulate `Intent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;
use hdk_records::{
    DataIntegrityError, RecordAPIResult,
    MaybeUndefined,
    local_indexes::{
        read_index,
        query_index,
    },
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
    remote_indexes::{
        create_remote_index,
        update_remote_index,
    }
};

use vf_attributes_hdk::{
    RevisionHash,
    SatisfactionAddress,
};

use hc_zome_rea_intent_storage_consts::*;
use hc_zome_rea_intent_storage::*;
use hc_zome_rea_intent_rpc::*;

use hc_zome_rea_process_storage_consts::{PROCESS_INTENT_INPUTS_LINK_TAG, PROCESS_INTENT_OUTPUTS_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_SATISFIES_LINK_TAG};

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

pub fn receive_create_intent<S>(entry_def_id: S, process_entry_def_id: S, intent: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    handle_create_intent(entry_def_id, process_entry_def_id, &intent)
}

pub fn receive_get_intent<S>(entry_def_id: S, address: IntentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    handle_get_intent(entry_def_id, &address)
}

pub fn receive_update_intent<S>(entry_def_id: S, process_entry_def_id: S, intent: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    handle_update_intent(entry_def_id, process_entry_def_id, &intent)
}

pub fn receive_delete_intent<S>(entry_def_id: S, process_entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>,
{
    handle_delete_intent(entry_def_id, process_entry_def_id, revision_id)
}

// :TODO: move to hdk_records module

fn handle_get_intent<S>(entry_def_id: S, address: &IntentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&entry_def_id, &address)?)
}

fn handle_create_intent<S>(
    entry_def_id: S, process_entry_def_id: S,
    intent: &CreateRequest,
) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, intent.to_owned())?;

    // handle link fields
    if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = intent {
        let _results = create_remote_index(
            &PROCESS_INPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![input_of.to_owned()].as_slice(),
            INTENT_INPUT_OF_LINK_TAG, PROCESS_INTENT_INPUTS_LINK_TAG,
        )?;
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = intent {
        let _results = create_remote_index(
            &PROCESS_OUTPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![output_of.to_owned()].as_slice(),
            INTENT_OUTPUT_OF_LINK_TAG, PROCESS_INTENT_OUTPUTS_LINK_TAG,
        )?;
    };

    // return entire record structure
    construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&entry_def_id, &base_address)?)
}

fn handle_update_intent<S>(entry_def_id: S, process_entry_def_id: S, intent: &UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let address = intent.get_revision_id().to_owned();
    let (revision_id, base_address, new_entry, _prev_entry): (_, IntentAddress, EntryData, EntryData) = update_record(&entry_def_id, &address, intent.to_owned())?;

    // handle link fields
    if let UpdateRequest { input_of: MaybeUndefined::Some(input_of), .. } = intent {
        let _results = update_remote_index(
            &PROCESS_INPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![input_of.to_owned()].as_slice(),
            vec![].as_slice(),
            INTENT_INPUT_OF_LINK_TAG, PROCESS_INTENT_INPUTS_LINK_TAG,
        );
    }
    if let UpdateRequest { output_of: MaybeUndefined::Some(output_of), .. } = intent {
        let _results = update_remote_index(
            &PROCESS_OUTPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![output_of.to_owned()].as_slice(),
            vec![].as_slice(),
            INTENT_OUTPUT_OF_LINK_TAG, PROCESS_INTENT_OUTPUTS_LINK_TAG,
        );
    }

    construct_response(&base_address, &revision_id, &new_entry, get_link_fields(&entry_def_id, &base_address)?)
}

fn handle_delete_intent<S>(entry_def_id: S, process_entry_def_id: S, revision_id: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>,
{
    // load the record to ensure it is of the correct type
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let _results = update_remote_index(
            &PROCESS_INPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![].as_slice(),
            vec![process_address].as_slice(),
            INTENT_INPUT_OF_LINK_TAG, PROCESS_INTENT_INPUTS_LINK_TAG,
        );
    }
    if let Some(process_address) = entry.output_of {
        let _results = update_remote_index(
            &PROCESS_OUTPUT_INDEXING_API_METHOD,
            &entry_def_id, &base_address,
            &process_entry_def_id,
            vec![].as_slice(),
            vec![process_address].as_slice(),
            INTENT_OUTPUT_OF_LINK_TAG, PROCESS_INTENT_OUTPUTS_LINK_TAG,
        );
    }

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage, _>(&revision_id)
}

const READ_FN_NAME: &str = "get_intent";

pub fn generate_query_handler<S, C, F>(
    foreign_zome_name_from_config: F,
    sastisfaction_entry_def_id:S,
    process_entry_def_id: S,
) -> impl FnOnce(&QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>,
        C: std::fmt::Debug,
        SerializedBytes: TryInto<C, Error = SerializedBytesError>,
        F: Fn(C) -> Option<String>,
{
    move |params| {
        let mut entries_result: RecordAPIResult<Vec<RecordAPIResult<ResponseData>>> = Err(DataIntegrityError::EmptyQuery);

        match &params.satisfied_by {
            Some(satisfied_by) => {
                entries_result = query_index::<ResponseData, IntentAddress, C,F,_,_,_,_>(
                    &sastisfaction_entry_def_id,
                    satisfied_by, SATISFACTION_SATISFIES_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
                );
            },
            _ => (),
        };
        match &params.input_of {
            Some(input_of) => {
                entries_result = query_index::<ResponseData, IntentAddress, C,F,_,_,_,_>(
                    &process_entry_def_id,
                    input_of, PROCESS_INTENT_INPUTS_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
                );
            },
            _ => (),
        };
        match &params.output_of {
            Some(output_of) => {
                entries_result = query_index::<ResponseData, IntentAddress, C,F,_,_,_,_>(
                    &process_entry_def_id,
                    output_of, PROCESS_INTENT_OUTPUTS_LINK_TAG,
                    &foreign_zome_name_from_config, &READ_FN_NAME,
                );
            },
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
pub fn construct_response<'a>(
    address: &IntentAddress, revision_id: &RevisionHash, e: &EntryData, (
        satisfactions,
        // published_in,
    ): (
        Vec<SatisfactionAddress>,
        // Vec<ProposedIntentAddress>
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        intent: Response {
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            image: e.image.to_owned(),
            input_of: e.input_of.to_owned(),
            output_of: e.output_of.to_owned(),
            provider: e.provider.to_owned(),
            receiver: e.receiver.to_owned(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned(),
            resource_classified_as: e.resource_classified_as.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            available_quantity: e.available_quantity.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            has_point_in_time: e.has_point_in_time.to_owned(),
            due: e.due.to_owned(),
            at_location: e.at_location.to_owned(),
            agreed_in: e.agreed_in.to_owned(),
            finished: e.finished.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            satisfied_by: satisfactions.to_owned(),
            // published_in: published_in.to_owned(),
        }
    })
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a, S>(entry_def_id: S, intent: &IntentAddress) -> RecordAPIResult<(
    Vec<SatisfactionAddress>,
)>
    where S: AsRef<str>,
{
    Ok((
        read_index(&entry_def_id, intent, INTENT_SATISFIEDBY_LINK_TAG)?,
    ))
}
