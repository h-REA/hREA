/**
 * Handling for `Commitment`-related requests
 */

use hdk::{
    get_links,
    holochain_core_types::{
        cas::content::Address,
    },
    error::ZomeApiResult,
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
use super::fulfillment_requests::{
    get_fulfilled_by,
};
use super::satisfaction_requests::{
    link_satisfactions,
    get_satisfactions,
};

// Entry types

pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_base";
pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";


pub fn handle_get_commitment(address: Address) -> ZomeApiResult<CommitmentResponse> {
    let entry = read_record_entry(&address)?;

    // read reference fields
    let fulfillment_links = get_fulfilled_by(&address)?;
    let satisfaction_links = get_satisfactions(&address)?;

    // construct output response
    Ok(construct_response(&address, entry,
        &Some(fulfillment_links),
        &Some(satisfaction_links),
    ))
}

pub fn handle_create_commitment(commitment: CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    // copy necessary fields for link processing first, since `commitment.into()` will borrow the fields into the target Entry
    let fulfills = commitment.get_fulfills();
    let satisfies = commitment.get_satisfies();

    let (base_address, entry_resp): (Address, CommitmentEntry) = create_record(COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, commitment)?;

    // handle cross-DHT link fields
    match &satisfies {
        Some(f) => { link_satisfactions(&base_address, &f)?; },
        None => ()
    }

    // return entire record structure
    Ok(construct_response(&base_address, entry_resp, &None, &satisfies))
}

pub fn handle_update_commitment(commitment: CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    let address = commitment.get_id();
    let new_entry = update_record(COMMITMENT_ENTRY_TYPE, &address, &commitment)?;

    // read reference fields
    let fulfillment_links = get_fulfilled_by(&address)?;
    let satisfaction_links = get_satisfactions(&address)?;

    Ok(construct_response(address, new_entry,
        &Some(fulfillment_links),
        &Some(satisfaction_links),
    ))
}

pub fn handle_delete_commitment(address: Address) -> ZomeApiResult<bool> {
    delete_record::<CommitmentEntry>(&address)
}
