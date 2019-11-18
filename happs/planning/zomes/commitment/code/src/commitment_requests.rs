/**
 * Handling for `Commitment`-related requests
 */

use std::borrow::Cow;
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
    links::{
        get_links_and_load_entry_data,
        get_remote_links_and_load_entry_data,
        get_linked_addresses_as_type,
        get_linked_remote_addresses_as_type,
    },
    rpc::{
        create_remote_index_pair,
        update_remote_index_pair,
        remove_remote_index_pair,
    },
};

use vf_observation::type_aliases::{
    CommitmentAddress,
    ProcessAddress,
    FulfillmentAddress,
    SatisfactionAddress,
    AgentAddress,
};
use vf_planning::commitment::{
    Entry as CommitmentEntry,
    CreateRequest as CommitmentCreateRequest,
    UpdateRequest as CommitmentUpdateRequest,
    ResponseData as CommitmentResponse,
    construct_response,
};
use vf_planning::identifiers::{
    BRIDGED_OBSERVATION_DHT,
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
    COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
    COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG,
    FULFILLMENT_FULFILLS_LINK_TYPE, FULFILLMENT_FULFILLS_LINK_TAG,
    COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
};
use vf_observation::identifiers::{
    PROCESS_BASE_ENTRY_TYPE,
    PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
    PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    input_of: Option<ProcessAddress>,
    output_of: Option<ProcessAddress>,
    fulfilled_by: Option<FulfillmentAddress>,
    satisfies: Option<SatisfactionAddress>,
}

pub fn receive_create_commitment(commitment: CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    handle_create_commitment(&commitment)
}

pub fn receive_get_commitment(address: CommitmentAddress) -> ZomeApiResult<CommitmentResponse> {
    handle_get_commitment(&address)
}

pub fn receive_update_commitment(commitment: CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    handle_update_commitment(&commitment)
}

pub fn receive_delete_commitment(address: CommitmentAddress) -> ZomeApiResult<bool> {
    handle_delete_commitment(&address)
}

pub fn receive_query_commitments(params: QueryParams) -> ZomeApiResult<Vec<CommitmentResponse>> {
    handle_query_commitments(&params)
}

fn handle_get_commitment(address: &CommitmentAddress) -> ZomeApiResult<CommitmentResponse> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(&address, &entry, get_link_fields(&address)))
}

fn handle_create_commitment(commitment: &CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    let (base_address, entry_resp): (CommitmentAddress, CommitmentEntry) = create_record(
        COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE,
        COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
        commitment.to_owned()
    )?;

    // handle link fields
    if let CommitmentCreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = commitment {
        let _results = create_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_committed_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
            PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
            base_address.as_ref(),
            vec![(input_of.as_ref()).clone()],
        );
    };
    if let CommitmentCreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = commitment {
        let _results = create_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_committed_outputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
            PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
            base_address.as_ref(),
            vec![(output_of.as_ref()).clone()],
        );
    };

    // :TODO: pass results from link creation rather than re-reading
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_commitment(commitment: &CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    let address = commitment.get_id();
    let new_entry = update_record(COMMITMENT_ENTRY_TYPE, &address, commitment)?;

    // handle link fields
    if MaybeUndefined::Undefined != commitment.input_of {
        let _results = update_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_committed_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
            PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
            address, &commitment.input_of,
        );
    }
    if MaybeUndefined::Undefined != commitment.output_of {
        let _results = update_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_committed_outputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
            PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
            address, &commitment.output_of,
        );
    }

    // :TODO: optimise this- should pass results from `replace_entry_link_set` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
}

fn handle_delete_commitment(address: &CommitmentAddress) -> ZomeApiResult<bool> {
    // read any referencing indexes
    let (
        input_of, output_of,
        ..
        // :NOTE: These aren't managed- they should be retained to allow exploring the deleted data:
        // fulfillments, satisfactions
    ) = get_link_fields(address);

    // handle link fields
    if let Some(process_address) = input_of {
        let _results = remove_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_committed_inputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG,
            PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
            address, &process_address,
        );
    }
    if let Some(process_address) = output_of {
        let _results = remove_remote_index_pair(
            BRIDGED_OBSERVATION_DHT, "process", "index_committed_outputs", Address::from(PUBLIC_TOKEN.to_string()),
            PROCESS_BASE_ENTRY_TYPE,
            COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG,
            PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
            address, &process_address,
        );
    }

    // delete entry last as it must be present in order for links to be removed
    delete_record::<CommitmentEntry>(&address)
}

fn handle_query_commitments(params: &QueryParams) -> ZomeApiResult<Vec<CommitmentResponse>> {
    let mut entries_result: ZomeApiResult<Vec<(CommitmentAddress, Option<CommitmentEntry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.fulfilled_by {
        Some(fulfilled_by) => {
            entries_result = get_links_and_load_entry_data(
                fulfilled_by, FULFILLMENT_FULFILLS_LINK_TYPE, FULFILLMENT_FULFILLS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = get_links_and_load_entry_data(
                satisfies, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = get_remote_links_and_load_entry_data(
                input_of, PROCESS_BASE_ENTRY_TYPE,
                PROCESS_COMMITMENT_INPUTS_LINK_TYPE, PROCESS_COMMITMENT_INPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = get_remote_links_and_load_entry_data(
                output_of, PROCESS_BASE_ENTRY_TYPE,
                PROCESS_COMMITMENT_OUTPUTS_LINK_TYPE, PROCESS_COMMITMENT_OUTPUTS_LINK_TAG,
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
                            entry_base_address,
                            &entry,
                            get_link_fields(entry_base_address),
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
fn get_link_fields<'a>(commitment: &CommitmentAddress) -> (
    Option<ProcessAddress>,
    Option<ProcessAddress>,
    Option<Cow<'a, Vec<FulfillmentAddress>>>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
    Option<Cow<'a, Vec<AgentAddress>>>,
) {
    (
        get_linked_remote_addresses_as_type(commitment, COMMITMENT_INPUT_OF_LINK_TYPE, COMMITMENT_INPUT_OF_LINK_TAG).into_owned().pop(),
        get_linked_remote_addresses_as_type(commitment, COMMITMENT_OUTPUT_OF_LINK_TYPE, COMMITMENT_OUTPUT_OF_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(commitment, COMMITMENT_FULFILLEDBY_LINK_TYPE, COMMITMENT_FULFILLEDBY_LINK_TAG)),
        Some(get_linked_addresses_as_type(commitment, COMMITMENT_SATISFIES_LINK_TYPE, COMMITMENT_SATISFIES_LINK_TAG)),
        None,   // :TODO:
    )
}
