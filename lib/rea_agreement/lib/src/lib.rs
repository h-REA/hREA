/**
 * Holo-REA agreement zome library API
 *
 * Contains helper methods that can be used to manipulate `Agreement` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk3::prelude::HeaderHash;

use hdk_graph_helpers::{
    GraphAPIResult,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    local_indexes::{
        read_index,
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
    handle_create_agreement(agreement)
}

pub fn receive_get_agreement(address: AgreementAddress) -> GraphAPIResult<ResponseData> {
    handle_get_agreement(&address)
}

pub fn receive_update_agreement(agreement: UpdateRequest) -> GraphAPIResult<ResponseData> {
    handle_update_agreement(agreement)
}

pub fn receive_delete_agreement(address: HeaderHash) -> GraphAPIResult<bool> {
    delete_record::<EntryData>(&address)
}

fn handle_get_agreement(address: &AgreementAddress) -> GraphAPIResult<ResponseData> {
    let (revision, base_address, entry): (_, AgreementAddress, EntryData) = read_record_entry::<EntryData, EntryStorage, AgreementAddress,_,_>(&AGREEMENT_ENTRY_TYPE, address)?;
    Ok(construct_response(&base_address, &revision, &entry, get_link_fields(&address)?))
}

fn handle_create_agreement(agreement: CreateRequest) -> GraphAPIResult<ResponseData> {
    let (header_addr, base_address, entry_resp): (_, AgreementAddress, EntryData) = create_record(&AGREEMENT_ENTRY_TYPE, agreement)?;
    Ok(construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&base_address)?))
}

fn handle_update_agreement(agreement: UpdateRequest) -> GraphAPIResult<ResponseData> {
    let revision_hash = agreement.get_revision_id().clone();
    let (revision_id, identity_address, entry): (_, AgreementAddress, EntryData) = update_record(&AGREEMENT_ENTRY_TYPE, &revision_hash, agreement)?;
    Ok(construct_response(&identity_address, &revision_id, &entry, get_link_fields(&identity_address)?))
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &AgreementAddress, revision: &HeaderHash, e: &EntryData, (
        commitments,
        economic_events,
    ): (
        Vec<CommitmentAddress>,
        Vec<EventAddress>,
    ),
) -> ResponseData {
    ResponseData {
        agreement: Response {
            id: address.to_owned(),
            revision_id: revision.to_owned(),
            name: e.name.to_owned(),
            created: e.created.to_owned(),
            note: e.note.to_owned(),
            commitments: commitments.to_owned(),
            economic_events: economic_events.to_owned(),
        }
    }
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields(agreement: &AgreementAddress) -> GraphAPIResult<(
    Vec<CommitmentAddress>,
    Vec<EventAddress>,
)> {
    Ok((
        read_index(&AGREEMENT_ENTRY_TYPE, agreement.as_ref(), &AGREEMENT_COMMITMENTS_LINK_TAG)?,
        read_index(&AGREEMENT_ENTRY_TYPE, agreement.as_ref(), &AGREEMENT_EVENTS_LINK_TAG)?,
    ))
}
