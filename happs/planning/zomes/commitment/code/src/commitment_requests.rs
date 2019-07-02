/**
 * Handling for `Commitment`-related requests
 */

use hdk::{
    commit_entry,
    update_entry,
    remove_entry,
    link_entries,
    get_links,
    holochain_core_types::{
        cas::content::Address,
        entry::Entry,
    },
    error::ZomeApiResult,
    utils::{
        get_as_type,
    },
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
    COMMITMENT_FULFILLEDBY_LINK_TYPE,
    LINK_TAG_COMMITMENT_FULFILLEDBY,
    link_fulfillments,
};

// Entry types

pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_base";
pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";

pub fn handle_get_commitment(address: Address) -> ZomeApiResult<CommitmentResponse> {
    let entry = read_record_entry(&address)?;

    // read reference fields
    let fulfillment_links = get_links(&address, Some(COMMITMENT_FULFILLEDBY_LINK_TYPE.to_string()), Some(LINK_TAG_COMMITMENT_FULFILLEDBY.to_string()))?;

    // construct output response
    Ok(construct_response(&address, entry, Some(fulfillment_links.addresses())))
}

pub fn handle_create_commitment(commitment: CommitmentCreateRequest) -> ZomeApiResult<CommitmentResponse> {
    // copy necessary fields for link processing first, since `commitment.into()` will borrow the fields into the target Entry
    let fulfills = commitment.get_fulfills();

    let (base_address, entry_resp): (Address, CommitmentEntry) = create_record(COMMITMENT_BASE_ENTRY_TYPE, COMMITMENT_ENTRY_TYPE, commitment)?;

    // handle cross-DHT link fields
    match fulfills.clone() {
        Some(f) => { link_fulfillments(&base_address, &f); },
        None => ()
    };

    // return entire record structure
    Ok(construct_response(&base_address, entry_resp, fulfills))
}

pub fn handle_update_commitment(commitment: CommitmentUpdateRequest) -> ZomeApiResult<CommitmentResponse> {
    let base_address = commitment.get_id();
    let new_entry = update_record(COMMITMENT_ENTRY_TYPE, &base_address, &commitment)?;

    // copy necessary fields for link processing first, since `commitment.into()` will borrow the fields into the target Entry
    // :TODO: link field handling
    let fulfills = commitment.get_fulfills();

    Ok(construct_response(base_address, new_entry, fulfills))
}

pub fn handle_delete_commitment(address: Address) -> ZomeApiResult<bool> {
    delete_record::<CommitmentEntry>(&address)
}
