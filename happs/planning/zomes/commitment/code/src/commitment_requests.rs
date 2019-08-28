/**
 * Handling for `Commitment`-related requests
 */

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::link::LinkMatch::Exactly,
    error::ZomeApiResult,
    utils::get_links_and_load_type,
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
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
    COMMITMENT_ENTRY_TYPE,
    COMMITMENT_BASE_ENTRY_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    COMMITMENT_FULFILLEDBY_LINK_TAG,
};

pub fn handle_get_commitment(address: Address) -> ZomeApiResult<CommitmentResponse> {
    let entry = read_record_entry(&address)?;

    // read reference fields
    let fulfillment_links = get_fulfillment_ids(&address)?;
    // let satisfaction_links = get_satisfactions(&address)?;

    // construct output response
    Ok(construct_response(&address, entry,
        &Some(fulfillment_links),
        // &Some(satisfaction_links),
        &None,
    ))
}

pub fn handle_create_commitment(commitment: CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    let (base_address, entry_resp): (Address, CommitmentEntry) = create_record(COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, commitment)?;

    // return entire record structure
    Ok(construct_response(&base_address, entry_resp, &None, &None))
}

pub fn handle_update_commitment(commitment: CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    let address = commitment.get_id();
    let new_entry = update_record(COMMITMENT_ENTRY_TYPE, &address, &commitment)?;

    // read reference fields
    let fulfillment_links = get_fulfillment_ids(&address)?;
    // let satisfaction_links = get_satisfactions(&address)?;

    Ok(construct_response(address, new_entry,
        &Some(fulfillment_links),
        // &Some(satisfaction_links),
        &None,
    ))
}

pub fn handle_delete_commitment(address: Address) -> ZomeApiResult<bool> {
    delete_record::<CommitmentEntry>(&address)
}

/// Used to load the list of linked Fulfillment IDs
pub fn get_fulfillment_ids(commitment: &Address) -> ZomeApiResult<Vec<Address>> {
    get_links_and_load_type(&commitment, Exactly(COMMITMENT_FULFILLEDBY_LINK_TYPE), Exactly(COMMITMENT_FULFILLEDBY_LINK_TAG))
}
