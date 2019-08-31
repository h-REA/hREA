/**
 * Handling for `Commitment`-related requests
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    error::ZomeApiError,
    get_links,
};

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
// use super::satisfaction_requests::{
//     get_satisfactions,
// };
use vf_planning::identifiers::{
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_INITIAL_ENTRY_LINK_TYPE,
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TAG,
    FULFILLMENT_FULFILLS_LINK_TYPE,
    FULFILLMENT_FULFILLS_LINK_TAG,
};

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

pub fn receive_query_commitments(fulfillment: Address) -> ZomeApiResult<Vec<CommitmentResponse>> {
    handle_query_commitments(&fulfillment)
}

fn handle_get_commitment(address: &Address) -> ZomeApiResult<CommitmentResponse> {
    let entry = read_record_entry(&address)?;

    // read reference fields
    let fulfillment_links = get_fulfillment_ids(&address)?;
    // let satisfaction_links = get_satisfactions(&address)?;

    // construct output response
    Ok(construct_response(&address, &entry,
        &Some(fulfillment_links),
        // &Some(satisfaction_links),
        &None,
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

    // read reference fields
    let fulfillment_links = get_fulfillment_ids(&address)?;
    // let satisfaction_links = get_satisfactions(&address)?;

    Ok(construct_response(address, &new_entry,
        &Some(fulfillment_links),
        // &Some(satisfaction_links),
        &None,
    ))
}

fn handle_query_commitments(fulfilled_by: &Address) -> ZomeApiResult<Vec<CommitmentResponse>> {
    let entries_result: ZomeApiResult<Vec<(Address, Option<CommitmentEntry>)>> = get_links_and_load_entry_data(
        &fulfilled_by,
        FULFILLMENT_FULFILLS_LINK_TYPE, FULFILLMENT_FULFILLS_LINK_TAG,
    );

    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address,
                            &entry,
                            &Some(get_fulfillment_ids(&entry_base_address)?),
                            &None,
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
