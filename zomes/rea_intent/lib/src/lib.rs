/**
 * Holo-REA intent zome library API
 *
 * Contains helper methods that can be used to manipulate `Intent` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::{
    PUBLIC_TOKEN,
    prelude::Address,
    error::{ ZomeApiResult, ZomeApiError },
};

use hdk_records::{
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_linked_addresses_as_type,
    },
    local_indexes::{
        query_direct_index_with_foreign_key,
        query_direct_remote_index_with_foreign_key,
    },
    remote_indexes::{
        create_direct_remote_index,
        update_direct_remote_index,
        remove_direct_remote_index,
    },
};

use vf_attributes_hdk::{
    SatisfactionAddress,
};

use hc_zome_rea_intent_storage_consts::*;
use hc_zome_rea_intent_storage::*;
use hc_zome_rea_intent_rpc::*;

use hc_zome_rea_process_storage_consts::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
};

use hc_zome_rea_satisfaction_storage_consts::{
    SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
};

pub fn receive_create_intent(intent: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_intent(&intent)
}

pub fn receive_get_intent(address: IntentAddress) -> ZomeApiResult<ResponseData> {
    handle_get_intent(&address)
}

pub fn receive_update_intent(intent: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_intent(&intent)
}

pub fn receive_delete_intent(address: IntentAddress) -> ZomeApiResult<bool> {
    handle_delete_intent(&address)
}

pub fn receive_query_intents(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_intents(&params)
}

// :TODO: move to hdk_records module

fn handle_get_intent(address: &IntentAddress) -> ZomeApiResult<ResponseData> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(&address, &entry, get_link_fields(&address)))
}

fn handle_create_intent(intent: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (IntentAddress, Entry) = create_record(
        INTENT_BASE_ENTRY_TYPE, INTENT_ENTRY_TYPE,
        INTENT_INITIAL_ENTRY_LINK_TYPE,
        intent.to_owned(),
    )?;

    // handle link fields
    if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = intent {
        let _results = create_direct_remote_index(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            base_address.as_ref(),
            vec![(input_of.as_ref()).clone()],
        );
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = intent {
        let _results = create_direct_remote_index(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_outputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
            PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
            base_address.as_ref(),
            vec![(output_of.as_ref()).clone()],
        );
    };

    // return entire record structure
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_intent(intent: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let address = intent.get_id();
    let new_entry = update_record(INTENT_ENTRY_TYPE, address, intent)?;

    // handle link fields
    if MaybeUndefined::Undefined != intent.input_of {
        let _results = update_direct_remote_index(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            address, &intent.input_of,
        );
    }
    if MaybeUndefined::Undefined != intent.output_of {
        let _results = update_direct_remote_index(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_outputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
            PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
            address, &intent.output_of,
        );
    }

    // :TODO: optimise this- should pass results from `replace_direct_index` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
}

fn handle_delete_intent(address: &IntentAddress) -> ZomeApiResult<bool> {
    // read any referencing indexes
    let entry: Entry = read_record_entry(&address)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let _results = remove_direct_remote_index(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            address, &process_address,
        );
    }
    if let Some(process_address) = entry.output_of {
        let _results = remove_direct_remote_index(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_outputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
            PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
            address, &process_address,
        );
    }

    // delete entry last as it must be present in order for links to be removed
    delete_record::<Entry>(&address)
}

fn handle_query_intents(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(IntentAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = query_direct_index_with_foreign_key(
                &satisfied_by, SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                input_of, PROCESS_BASE_ENTRY_TYPE,
                PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                output_of, PROCESS_BASE_ENTRY_TYPE,
                PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
            );
        },
        _ => (),
    };

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address, &entry, get_link_fields(entry_base_address)
                        )),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &IntentAddress, e: &Entry, (
        satisfactions,
        // published_in,
    ): (
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
        // Option<Cow<'a, Vec<ProposedIntentAddress>>>
    )
) -> ResponseData {
    ResponseData {
        intent: Response {
            id: address.to_owned(),
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
            satisfied_by: satisfactions.map(Cow::into_owned),
            // published_in: published_in.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(intent: &IntentAddress) -> (
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
) {
    (
        Some(get_linked_addresses_as_type(intent, INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG)),
    )
}
