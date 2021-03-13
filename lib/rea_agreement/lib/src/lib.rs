/**
 * Holo-REA agreement zome library API
 *
 * Contains helper methods that can be used to manipulate `Agreement` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult,
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

pub use hc_zome_rea_agreement_storage_consts::*;
use hc_zome_rea_agreement_storage::*;
use hc_zome_rea_agreement_rpc::*;

pub fn receive_create_agreement<S>(entry_def_id: S, agreement: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_create_agreement(&entry_def_id, agreement)
}

pub fn receive_get_agreement<S>(entry_def_id: S, address: AgreementAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_agreement(&entry_def_id, &address)
}

pub fn receive_update_agreement<S>(entry_def_id: S, agreement: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_agreement(&entry_def_id, agreement)
}

pub fn receive_delete_agreement(address: RevisionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryData>(&address)
}

fn handle_get_agreement<S>(entry_def_id: S, address: &AgreementAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry): (_, AgreementAddress, EntryData) = read_record_entry::<EntryData, EntryStorage, AgreementAddress,_,_>(&entry_def_id, address)?;
    Ok(construct_response(&base_address, &revision, &entry, get_link_fields(&entry_def_id, &address)?))
}

fn handle_create_agreement<S>(entry_def_id: S, agreement: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (header_addr, base_address, entry_resp): (_, AgreementAddress, EntryData) = create_record(&entry_def_id, agreement)?;
    Ok(construct_response(&base_address, &header_addr, &entry_resp, get_link_fields(&entry_def_id, &base_address)?))
}

fn handle_update_agreement<S>(entry_def_id: S, agreement: UpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let revision_hash = agreement.get_revision_id().clone();
    let (revision_id, identity_address, entry, _prev_entry): (_, AgreementAddress, EntryData, EntryData) = update_record(&entry_def_id, &revision_hash, agreement)?;
    Ok(construct_response(&identity_address, &revision_id, &entry, get_link_fields(&entry_def_id, &identity_address)?))
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &AgreementAddress, revision: &RevisionHash, e: &EntryData, (
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
pub fn get_link_fields<S>(entry_def_id: S, agreement: &AgreementAddress) -> RecordAPIResult<(
    Vec<CommitmentAddress>,
    Vec<EventAddress>,
)>
    where S: AsRef<str>
{
    Ok((
        read_index(&entry_def_id, agreement.as_ref(), &AGREEMENT_COMMITMENTS_LINK_TAG)?,
        read_index(&entry_def_id, agreement.as_ref(), &AGREEMENT_EVENTS_LINK_TAG)?,
    ))
}
