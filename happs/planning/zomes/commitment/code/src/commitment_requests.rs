/**
 * Handling for `Commitment`-related requests
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    error::ZomeApiError,
    get_links,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_links_and_load_entry_data,
    },
};
use vf_planning::commitment::{
    Entry as CommitmentEntry,
    CreateRequest as CommitmentCreateRequest,
    UpdateRequest as CommitmentUpdateRequest,
    ResponseData as CommitmentResponse,
    construct_response,
};
use vf_planning::identifiers::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TAG,
    FULFILLMENT_FULFILLS_LINK_TYPE,
    FULFILLMENT_FULFILLS_LINK_TAG,
    COMMITMENT_SATISFIES_LINK_TYPE,
    COMMITMENT_SATISFIES_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE,
    SATISFACTION_SATISFIEDBY_LINK_TAG,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    fulfilled_by: Option<Address>,
    satisfies: Option<Address>,
}

pub fn receive_create_commitment(commitment: CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    handle_create_commitment(&commitment)
}

pub fn receive_get_commitment(address: Address) -> ZomeApiResult<CommitmentResponse> {
    handle_get_commitment(&address)
}

pub fn receive_update_commitment(commitment: CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    handle_update_commitment(&commitment)
}

pub fn receive_delete_commitment(address: Address) -> ZomeApiResult<bool> {
    delete_record::<CommitmentEntry>(&address)
}

pub fn receive_query_commitments(params: QueryParams) -> ZomeApiResult<Vec<CommitmentResponse>> {
    handle_query_commitments(&params)
}

fn handle_get_commitment(address: &Address) -> ZomeApiResult<CommitmentResponse> {
    let entry = read_record_entry(&address)?;

    // construct output response
    Ok(construct_response(&address, &entry,
        &Some(get_fulfillment_ids(&address)?),
        &Some(get_satisfaction_ids(&address)?),
    ))
}

fn handle_create_commitment(commitment: &CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    let (base_address, entry_resp): (Address, CommitmentEntry) = create_record(
        COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE,
        COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
        commitment.to_owned()
    )?;

    // return entire record structure
    Ok(construct_response(&base_address, &entry_resp, &None, &None))
}

fn handle_update_commitment(commitment: &CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    let address = commitment.get_id();
    let new_entry = update_record(COMMITMENT_ENTRY_TYPE, &address, commitment)?;

    Ok(construct_response(address, &new_entry,
        &Some(get_fulfillment_ids(&address)?),
        &Some(get_satisfaction_ids(&address)?),
    ))
}

fn handle_query_commitments(params: &QueryParams) -> ZomeApiResult<Vec<CommitmentResponse>> {
    let mut entries_result: ZomeApiResult<Vec<(Address, Option<CommitmentEntry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.fulfilled_by {
        Some(fulfilled_by) => {
            entries_result = get_links_and_load_entry_data(
                &fulfilled_by, FULFILLMENT_FULFILLS_LINK_TYPE, FULFILLMENT_FULFILLS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = get_links_and_load_entry_data(
                &satisfies, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
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
                            &Some(get_fulfillment_ids(&entry_base_address)?),
                            &Some(get_satisfaction_ids(&entry_base_address)?),
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

/// Used to load the list of linked Fulfillment IDs
fn get_fulfillment_ids(commitment: &Address) -> ZomeApiResult<Vec<Address>> {
    Ok(get_links(&commitment, Exactly(COMMITMENT_FULFILLEDBY_LINK_TYPE), Exactly(COMMITMENT_FULFILLEDBY_LINK_TAG))?.addresses())
}

/// Used to load the list of linked Satisfaction IDs
fn get_satisfaction_ids(commitment: &Address) -> ZomeApiResult<Vec<Address>> {
    Ok(get_links(&commitment, Exactly(COMMITMENT_SATISFIES_LINK_TYPE), Exactly(COMMITMENT_SATISFIES_LINK_TAG))?.addresses())
}
