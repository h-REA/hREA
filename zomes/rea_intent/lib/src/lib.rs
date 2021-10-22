/**
 * Holo-REA intent zome library API
 *
 * Contains helper methods that can be used to manipulate `Intent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult,
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_intent_storage::*;
use hc_zome_rea_intent_rpc::*;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

pub fn handle_create_intent<S>(entry_def_id: S, intent: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (header_addr, base_address, entry_resp): (_,_, EntryData) = create_record(&entry_def_id, intent.to_owned())?;

    // handle link fields
    if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = &intent {
        create_index!(Remote(intent.input_of(input_of), process.intended_inputs(&base_address)))?;
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = &intent {
        create_index!(Remote(intent.output_of(output_of), process.intended_outputs(&base_address)))?;
    };

    // return entire record structure
    construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_intent<S>(entry_def_id: S, address: IntentAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&address)?)
}

pub fn handle_update_intent<S>(entry_def_id: S, intent: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>,
{
    let address = intent.get_revision_id().to_owned();
    let (revision_id, base_address, new_entry, prev_entry): (_, IntentAddress, EntryData, EntryData) = update_record(&entry_def_id, &address, intent.to_owned())?;

    // handle link fields
    if new_entry.input_of != prev_entry.input_of {
        let new_value = match &new_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        update_index!(Remote(
            intent
                .input_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            process.intended_inputs(&base_address)
        ))?;
    }
    if new_entry.output_of != prev_entry.output_of {
        let new_value = match &new_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        update_index!(Remote(
            intent
                .output_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            process.intended_outputs(&base_address)
        ))?;
    }

    construct_response(&base_address, &revision_id, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_intent(revision_id: RevisionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        update_index!(Remote(intent.input_of.not(&vec![process_address]), process.intended_inputs(&base_address)))?;
    }
    if let Some(process_address) = entry.output_of {
        update_index!(Remote(intent.output_of.not(&vec![process_address]), process.intended_outputs(&base_address)))?;
    }

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage, _>(&revision_id)
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

/// Properties accessor for zome config
fn read_intent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.intent.index_zome)
}

// @see construct_response
pub fn get_link_fields(intent: &IntentAddress) -> RecordAPIResult<(
    Vec<SatisfactionAddress>,
)> {
    Ok((
        read_index!(intent(intent).satisfied_by)?,
    ))
}
