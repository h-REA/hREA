/**
 * hREA recipe_exchange zome library API
 *
 * Contains helper methods that can be used to manipulate `RecipeExchange` data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package hREA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult, MaybeUndefined, SignedActionHashed,
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

use hc_zome_rea_recipe_exchange_storage::*;
use hc_zome_rea_recipe_exchange_rpc::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.recipe_exchange.index_zome)
}

pub fn handle_create_recipe_exchange<S>(entry_def_id: S, recipe_exchange: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryDefinitions,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, recipe_exchange.to_owned())?;

    // return entire record structure
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_recipe_exchange(address: RecipeExchangeAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_recipe_exchange(recipe_exchange: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let address = recipe_exchange.get_revision_id().to_owned();
    let (meta, base_address, new_entry, prev_entry): (_, RecipeExchangeAddress, EntryData, EntryData) = update_record(&address, recipe_exchange.to_owned())?;

    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_recipe_exchange(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &RecipeExchangeAddress, meta: &SignedActionHashed, e: &EntryData, (
        recipe_clauses, recipe_reciprocal_clauses,
    ): (
        Vec<RecipeFlowAddress>,
        Vec<RecipeFlowAddress>,
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        recipe_exchange: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.name.to_owned(),
            note: e.note.to_owned(),
            recipe_clauses: recipe_clauses.to_owned(),
            recipe_reciprocal_clauses: recipe_reciprocal_clauses.to_owned(),
        }
    })
}

//---------------- READ ----------------


/// Properties accessor for zome config
fn read_recipe_exchange_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.recipe_exchange.index_zome)
}

pub fn get_link_fields(recipe_exchange: &RecipeExchangeAddress) -> RecordAPIResult<(
    Vec<RecipeFlowAddress>,
    Vec<RecipeFlowAddress>,
)> {
    Ok((
        read_index!(recipe_exchange(recipe_exchange).recipe_clauses)?,
        read_index!(recipe_exchange(recipe_exchange).recipe_reciprocal_clauses)?,
    ))
}
