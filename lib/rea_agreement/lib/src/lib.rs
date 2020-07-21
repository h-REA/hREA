/**
 * Holo-REA agreement zome library API
 *
 * Contains helper methods that can be used to manipulate `Agreement` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::error::{
    ZomeApiResult,
    // ZomeApiError,
};

use hdk_graph_helpers::{
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        get_linked_addresses_with_foreign_key_as_type,
    },
};

use vf_core::type_aliases::{
    AgreementAddress,
    CommitmentAddress,
    EventAddress,
};

use hc_zome_rea_agreement_storage_consts::*;
use hc_zome_rea_agreement_storage::*;
use hc_zome_rea_agreement_rpc::*;

pub fn receive_create_agreement(agreement: CreateRequest) -> ZomeApiResult<ResponseData> {
    handle_create_agreement(&agreement)
}

pub fn receive_get_agreement(address: AgreementAddress) -> ZomeApiResult<ResponseData> {
    handle_get_agreement(&address)
}

pub fn receive_update_agreement(agreement: UpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_agreement(&agreement)
}

pub fn receive_delete_agreement(address: AgreementAddress) -> ZomeApiResult<bool> {
    delete_record::<Entry>(&address)
}

fn handle_get_agreement(address: &AgreementAddress) -> ZomeApiResult<ResponseData> {
    Ok(construct_response(address, &read_record_entry(address)?, get_link_fields(&address)))
}

fn handle_create_agreement(agreement: &CreateRequest) -> ZomeApiResult<ResponseData> {
    let (base_address, entry_resp): (AgreementAddress, Entry) = create_record(
        AGREEMENT_BASE_ENTRY_TYPE, AGREEMENT_ENTRY_TYPE,
        AGREEMENT_INITIAL_ENTRY_LINK_TYPE,
        agreement.to_owned(),
    )?;
    Ok(construct_response(&base_address, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_agreement(agreement: &UpdateRequest) -> ZomeApiResult<ResponseData> {
    let base_address = agreement.get_id();
    let new_entry = update_record(AGREEMENT_ENTRY_TYPE, base_address, agreement)?;
    Ok(construct_response(&base_address, &new_entry, get_link_fields(&base_address)))
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &AgreementAddress, e: &Entry, (
        commitments,
        economic_events,
    ): (
        Option<Cow<'a, Vec<CommitmentAddress>>>,
        Option<Cow<'a, Vec<EventAddress>>>,
    ),
) -> ResponseData {
    ResponseData {
        agreement: Response {
            id: address.to_owned(),
            name: e.name.to_owned(),
            created: e.created.to_owned(),
            note: e.note.to_owned(),
            commitments: commitments.map(Cow::into_owned),
            economic_events: economic_events.map(Cow::into_owned),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields<'a>(agreement: &AgreementAddress) -> (
        Option<Cow<'a, Vec<CommitmentAddress>>>,
        Option<Cow<'a, Vec<EventAddress>>>,
) {
    (
        Some(get_linked_addresses_with_foreign_key_as_type(agreement, AGREEMENT_COMMITMENTS_LINK_TYPE, AGREEMENT_COMMITMENTS_LINK_TAG)),
        Some(get_linked_addresses_with_foreign_key_as_type(agreement, AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG)),
    )
}
