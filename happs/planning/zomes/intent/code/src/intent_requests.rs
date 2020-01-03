/**
 * Handling for `Intent`-related requests
 */
use hdk::{
    PUBLIC_TOKEN,
    holochain_persistence_api::cas::content::Address,
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiResult, ZomeApiError },
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
    local_indexes::{
        query_direct_index_with_foreign_key,
        query_direct_remote_index_with_foreign_key,
    },
    rpc::{
        create_remote_index_pair,
        update_remote_index_pair,
        remove_remote_index_pair,
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
    BRIDGED_OBSERVATION_DHT,
    INTENT_BASE_ENTRY_TYPE,
    INTENT_INITIAL_ENTRY_LINK_TYPE,
    INTENT_ENTRY_TYPE,
    INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
    INTENT_OUTPUT_OF_LINK_TYPE, INTENT_OUTPUT_OF_LINK_TAG,
    SATISFACTION_SATISFIES_LINK_TYPE, SATISFACTION_SATISFIES_LINK_TAG,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
    PROCESS_INTENT_OUTPUTS_LINK_TYPE, PROCESS_INTENT_OUTPUTS_LINK_TAG,
};
use vf_planning::intent::{
    Entry,
    CreateRequest,
    UpdateRequest,
    ResponseData as Response,
    construct_response,
    get_link_fields,
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
    handle_delete_intent(&address)
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
        let _results = create_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            base_address.as_ref(),
            vec![(input_of.as_ref()).clone()],
        );
    };
    if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = intent {
        let _results = create_remote_index_pair(
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

fn handle_update_intent(intent: &UpdateRequest) -> ZomeApiResult<Response> {
    let address = intent.get_id();
    let new_entry = update_record(INTENT_ENTRY_TYPE, address, intent)?;

    // handle link fields
    if MaybeUndefined::Undefined != intent.input_of {
        let _results = update_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            address, &intent.input_of,
        );
    }
    if MaybeUndefined::Undefined != intent.output_of {
        let _results = update_remote_index_pair(
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
        let _results = remove_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_intended_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            INTENT_INPUT_OF_LINK_TYPE, INTENT_INPUT_OF_LINK_TAG,
            PROCESS_INTENT_INPUTS_LINK_TYPE, PROCESS_INTENT_INPUTS_LINK_TAG,
            address, &process_address,
        );
    }
    if let Some(process_address) = entry.output_of {
        let _results = remove_remote_index_pair(
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

fn handle_query_intents(params: &QueryParams) -> ZomeApiResult<Vec<Response>> {
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
