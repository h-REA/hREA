/**
 * hREA recipe_flow zome library API
 *
 * Contains helper methods that can be used to manipulate `RecipeFlow` data
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

use hc_zome_rea_recipe_flow_storage::*;
use hc_zome_rea_recipe_flow_rpc::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some("indexing".to_string())
}

fn read_recipe_flow_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some("indexing".to_string())
}

fn read_recipe_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some("indexing".to_string())
}

fn read_recipe_exchange_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some("indexing".to_string())
}

pub fn handle_create_recipe_flow<S>(entry_def_id: S, recipe_flow: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    hdk::prelude::debug!("------------------------2---------------------------------------------------------------------------------------{:?}", recipe_flow);
    
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryDefinitions,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, recipe_flow.to_owned())?;
    hdk::prelude::debug!("------------------------3---------------------------------------------------------------------------------------");
    
    // hdk::prelude::debug!("base_address {:?}", base_address.clone());
    // hdk::prelude::debug!("entry_resp {:?}", entry_resp.clone());
    // // handle link fields
    // // :TODO: improve error handling

    // if let CreateRequest { provider: MaybeUndefined::Some(provider), .. } = &recipe_flow {
    //     create_index!(recipe_flow.provider(provider), agent.recipe_flows_as_provider(&base_address))?;
    // };
    // if let CreateRequest { receiver: MaybeUndefined::Some(receiver), .. } = &recipe_flow {
    //     create_index!(recipe_flow.receiver(receiver), agent.recipe_flows_as_receiver(&base_address))?;
    // };
    if let CreateRequest { recipe_input_of: MaybeUndefined::Some(recipe_input_of), .. } = &recipe_flow {
        let e = create_index!(recipe_flow.recipe_input_of(recipe_input_of), recipe_process.recipe_inputs(&base_address));
        // let e = create_index!(fulfillment.fulfills(fulfillment.get_fulfills()), commitment.fulfilled_by(&fulfillment_address));
        hdk::prelude::debug!("handle_create_recipe_flow::recipe_input_of index {:?}", e);
    };
    if let CreateRequest { recipe_output_of: MaybeUndefined::Some(recipe_output_of), .. } = &recipe_flow {
        let e = create_index!(recipe_flow.recipe_output_of(recipe_output_of), recipe_process.recipe_outputs(&base_address));
        hdk::prelude::debug!("handle_create_recipe_flow::recipe_output_of index {:?}", e);
    };
    if let CreateRequest { recipe_clause_of: MaybeUndefined::Some(recipe_clause_of), .. } = &recipe_flow {
        let e = create_index!(recipe_flow.recipe_clause_of(recipe_clause_of), recipe_exchange.recipe_exchanges(&base_address));
        hdk::prelude::debug!("handle_create_recipe_flow::recipe_clause_of index {:?}", e);
    };
    if let CreateRequest { recipe_reciprocal_clause_of: MaybeUndefined::Some(recipe_reciprocal_clause_of), .. } = &recipe_flow {
        let e = create_index!(recipe_flow.recipe_reciprocal_clause_of(recipe_reciprocal_clause_of), recipe_exchange.recipe_exchanges(&base_address));
        hdk::prelude::debug!("handle_create_recipe_flow::recipe_reciprocal_clause_of index {:?}", e);
    };

    // return entire record structure
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_recipe_flow(address: RecipeFlowAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_recipe_flow(recipe_flow: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let address = recipe_flow.get_revision_id().to_owned();
    let (meta, base_address, new_entry, prev_entry): (_, RecipeFlowAddress, EntryData, EntryData) = update_record(&address, recipe_flow.to_owned())?;

    if new_entry.recipe_input_of != prev_entry.recipe_input_of {
        let new_value = match &new_entry.recipe_input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.recipe_input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            recipe_flow
                .recipe_input_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            recipe_process.recipe_inputs(&base_address)
        );
        hdk::prelude::debug!("handle_update_recipe_flow::input_of index {:?}", e);
    }
    if new_entry.recipe_output_of != prev_entry.recipe_output_of {
        let new_value = match &new_entry.recipe_output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.recipe_output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            recipe_flow
                .recipe_output_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            recipe_process.recipe_outputs(&base_address)
        );
        hdk::prelude::debug!("handle_update_recipe_flow::output_of index {:?}", e);
    }

    if new_entry.recipe_clause_of != prev_entry.recipe_clause_of {
        let new_value = match &new_entry.recipe_clause_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.recipe_clause_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            recipe_flow
                .recipe_clause_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            recipe_exchange.recipe_exchanges(&base_address)
        );
        hdk::prelude::debug!("handle_update_recipe_flow::recipe_clause_of index {:?}", e);
    }

    if new_entry.recipe_reciprocal_clause_of != prev_entry.recipe_reciprocal_clause_of {
        let new_value = match &new_entry.recipe_reciprocal_clause_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.recipe_reciprocal_clause_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            recipe_flow
                .recipe_reciprocal_clause_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            recipe_exchange.recipe_exchanges(&base_address)
        );
        hdk::prelude::debug!("handle_update_recipe_flow::recipe_reciprocal_clause_of index {:?}", e);
    }


    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_recipe_flow(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.recipe_input_of {
        let e = update_index!(recipe_flow.recipe_input_of.not(&vec![process_address]), recipe_process.recipe_inputs(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::recipe_input_of index {:?}", e);
    }
    if let Some(process_address) = entry.recipe_output_of {
        let e = update_index!(recipe_flow.recipe_output_of.not(&vec![process_address]), recipe_process.recipe_outputs(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::recipe_output_of index {:?}", e);
    }
    if let Some(recipe_address) = entry.recipe_clause_of {
        let e = update_index!(recipe_flow.recipe_clause_of.not(&vec![recipe_address]), recipe_exchange.recipe_exchanges(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::recipe_clause_of index {:?}", e);
    }
    if let Some(recipe_address) = entry.recipe_reciprocal_clause_of {
        let e = update_index!(recipe_flow.recipe_reciprocal_clause_of.not(&vec![recipe_address]), recipe_exchange.recipe_exchanges(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::recipe_reciprocal_clause_of index {:?}", e);
    }

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &RecipeFlowAddress, meta: &SignedActionHashed, e: &EntryData, (
    ): (
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        recipe_flow: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            provider_role: e.provider_role.to_owned(),
            receiver_role: e.receiver_role.to_owned(),
            instructions: e.instructions.to_owned(),
            state: e.note.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            recipe_clause_of: e.recipe_clause_of.to_owned(),
            recipe_reciprocal_clause_of: e.recipe_reciprocal_clause_of.to_owned(),
            stage: e.stage.to_owned(),
            recipe_input_of: e.recipe_input_of.to_owned(),
            recipe_output_of: e.recipe_output_of.to_owned(),
        }
    })
}

//---------------- READ ----------------

// @see construct_response
pub fn get_link_fields(recipe_flow: &RecipeFlowAddress) -> RecordAPIResult<(
)> {
    Ok(())
}
