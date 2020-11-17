/**
 * Holo-REA agreement zome library API
 *
 * Contains helper methods that can be used to manipulate `Agreement` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk3::prelude::{HeaderHash};

use hdk_graph_helpers::{
    GraphAPIResult,
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

pub fn receive_create_agreement(agreement: CreateRequest) -> GraphAPIResult<ResponseData> {
    handle_create_agreement(&agreement)
}

pub fn receive_get_agreement(address: AgreementAddress) -> GraphAPIResult<ResponseData> {
    handle_get_agreement(&address)
}

pub fn receive_update_agreement(agreement: UpdateRequest) -> GraphAPIResult<ResponseData> {
    handle_update_agreement(&agreement)
}

pub fn receive_delete_agreement(address: HeaderHash) -> GraphAPIResult<bool> {
    delete_record::<Entry,_>(&address)
}

fn handle_get_agreement(address: &AgreementAddress) -> GraphAPIResult<ResponseData> {
    let (revision, entry): (HeaderHash, Entry) = read_record_entry(address)?;
    Ok(construct_response(address, &revision, &entry, get_link_fields(&address)))
}

fn handle_create_agreement(agreement: &CreateRequest) -> GraphAPIResult<ResponseData> {
    let (header_addr, base_address, entry_resp): (_, AgreementAddress, Entry) = create_record(AGREEMENT_ENTRY_TYPE.to_string().as_ref(), agreement)?;
    Ok(construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&base_address)))
}

fn handle_update_agreement(agreement: &UpdateRequest) -> GraphAPIResult<ResponseData> {
    let revision_hash = agreement.get_revision_id();
    let (revision_id, identity_address, entry): (_,_,Entry) = update_record(revision_hash, agreement)?;
    Ok(construct_response(&identity_address, &revision_id, &entry, get_link_fields(&base_address)))
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &AgreementAddress, revision: &HeaderHash, e: &Entry, (
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
            revision_id: revision.to_owned(),
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
