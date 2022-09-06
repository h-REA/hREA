/**
 * hREA plan zome library API
 *
 * Contains helper methods that can be used to manipulate `Plan` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package hREA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult, SignedActionHashed,
    records::{
        create_record,
        read_record_entry,
        read_record_entry_by_action,
        update_record,
        delete_record,
    },
    metadata::read_revision_metadata_abbreviated,
};
use hdk_semantic_indexes_client_lib::*;

use hc_zome_rea_plan_storage::*;
use hc_zome_rea_plan_rpc::*;


pub use hc_zome_rea_plan_storage::PLAN_ENTRY_TYPE;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.plan.index_zome)
}

pub fn handle_create_plan<S>(entry_def_id: S, plan: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, plan)?;
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_plan(address: PlanAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_plan(plan: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let revision_hash = plan.get_revision_id().clone();
    let (meta, identity_address, entry, _prev_entry): (_,_, EntryData, EntryData) = update_record(&revision_hash, plan)?;
    construct_response(&identity_address, &meta, &entry, get_link_fields(&identity_address)?)
}

pub fn handle_delete_plan(address: ActionHash) -> RecordAPIResult<bool> {
    delete_record::<EntryStorage>(&address)
}

/// Create response from input DHT primitives
fn construct_response<'a>(
    address: &PlanAddress, meta: &SignedActionHashed, e: &EntryData, (
        processes,
        independent_demands,
    ): (
        Vec<ProcessAddress>,
        Vec<CommitmentAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        plan: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.name.to_owned(),
            created: e.created.to_owned(),
            due: e.due.to_owned(),
            note: e.note.to_owned(),
            deletable: e.deletable.to_owned(),
            processes: processes.to_owned(),
            independent_demands: independent_demands.to_owned(),
        }
    })
}

//---------------- READ ----------------

/// Properties accessor for zome config
fn read_plan_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.plan.index_zome)
}

// @see construct_response
fn get_link_fields(base_address: &PlanAddress) -> RecordAPIResult<(
    Vec<ProcessAddress>,
    Vec<CommitmentAddress>,
)> {
    Ok((
        read_index!(plan(base_address).processes)?,
        read_index!(plan(base_address).independent_demands)?,
    ))
}
