/**
 * Handling for `Intent`-related requests
 */
use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::ZomeApiResult,
    error::ZomeApiError,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_links_and_load_entry_data,
        get_linked_addresses_as_type,
        replace_entry_link_set,
        link_entries_bidir,
    },
};

use vf_observation::type_aliases::{
    ProcessAddress,
};
use vf_planning::type_aliases::{
    IntentAddress,
    SatisfactionAddress,
};
use vf_planning::identifiers::{
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INITIAL_ENTRY_LINK_TYPE,
    INTENT_ENTRY_TYPE,
    INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
    INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
    INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG,
    SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
};
use vf_observation::identifiers::{
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
};
use vf_planning::intent::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    input_of: Option<ProcessAddress>,
    output_of: Option<ProcessAddress>,
    satisfied_by: Option<SatisfactionAddress>,
}

pub fn receive_create_intent(intent: CreateRequest) -> ZomeApiResult<Response> {
    handle_create_intent(&intent)
}

pub fn receive_get_intent(address: IntentAddress) -> ZomeApiResult<Response> {
    handle_get_intent(&address)
}

pub fn receive_update_intent(intent: UpdateRequest) -> ZomeApiResult<Response> {
    handle_update_intent(&intent)
}

pub fn receive_delete_intent(address: IntentAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

pub fn receive_query_intents(params: QueryParams) -> ZomeApiResult<Vec<Response>> {
    handle_query_intents(&params)
}

// :TODO: move to hdk_graph_helpers module

fn handle_get_intent(address: &IntentAddress) -> ZomeApiResult<Response> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(&address, &entry, get_link_fields(&address)))
}

fn handle_create_intent(intent: &CreateRequest) -> ZomeApiResult<Response> {
    let (base_address, entry_resp): (IntentAddress, Entry) = create_record(
        INTENT_BASE_ENTRY_TYPE, INTENT_ENTRY_TYPE,
        INTENT_INITIAL_ENTRY_LINK_TYPE,
        intent.to_owned(),
    )?;

    // handle link fields
    if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = intent {
        let _results = link_entries_bidir(
            base_address.as_ref(),
            input_of.as_ref(),
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
        );
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = intent {
        let _results = link_entries_bidir(
            base_address.as_ref(),
            output_of.as_ref(),
            INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
            PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
        );
    };

    // return entire record structure
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_intent(intent: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = intent.get_id();
    let new_entry = update_record(INTENT_ENTRY_TYPE, address, intent)?;

    // handle link fields
    replace_entry_link_set(address, &intent.input_of,
        INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
        PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
    );
    replace_entry_link_set(address, &intent.output_of,
        INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
        PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
    );

    // :TODO: optimise this- should pass results from `replace_entry_link_set` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
}

fn handle_query_intents(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
    let mut entries_result: ZomeApiResult<Vec<(IntentAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    match &params.satisfied_by {
        Some(satisfied_by) => {
            entries_result = get_links_and_load_entry_data(
                &satisfied_by, SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = get_links_and_load_entry_data(
                input_of, PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = get_links_and_load_entry_data(
                output_of, PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
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

// field list retrieval internals

// @see construct_response
fn get_link_fields<'a>(intent: &IntentAddress) -> (
    Option<ProcessAddress>,
    Option<ProcessAddress>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
) {
    (
        get_linked_addresses_as_type(intent, INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG).into_owned().pop(),
        get_linked_addresses_as_type(intent, INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(intent, INTENT_SATISFIEDBY_LINK_TYPE, INTENT_SATISFIEDBY_LINK_TAG)),
    )
}
