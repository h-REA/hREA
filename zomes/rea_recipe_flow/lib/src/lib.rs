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
    Some(conf.recipe_flow.index_zome)
}

pub fn handle_create_recipe_flow<S>(entry_def_id: S, recipe_flow: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, recipe_flow.to_owned())?;

    debug!("---------------------------------------------------------------------------------------------------------------")
    debug!("base_address {:?}", base_address);
    debug!("entry_resp {:?}", entry_resp);
    // // handle link fields
    // // :TODO: improve error handling

    // if let CreateRequest { provider: MaybeUndefined::Some(provider), .. } = &recipe_flow {
    //     create_index!(recipe_flow.provider(provider), agent.recipe_flows_as_provider(&base_address))?;
    // };
    // if let CreateRequest { receiver: MaybeUndefined::Some(receiver), .. } = &recipe_flow {
    //     create_index!(recipe_flow.receiver(receiver), agent.recipe_flows_as_receiver(&base_address))?;
    // };
    // if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = &recipe_flow {
    //     let e = create_index!(recipe_flow.input_of(input_of), process.intended_inputs(&base_address));
    //     hdk::prelude::debug!("handle_create_recipe_flow::input_of index {:?}", e);
    // };
    // if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = &recipe_flow {
    //     let e = create_index!(recipe_flow.output_of(output_of), process.intended_outputs(&base_address));
    //     hdk::prelude::debug!("handle_create_recipe_flow::output_of index {:?}", e);
    // };

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

    // handle link fields
    if new_entry.provider != prev_entry.provider {
        let new_value = match &new_entry.provider { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.provider { Some(val) => vec![val.to_owned()], None => vec![] };
        update_index!(
            recipe_flow
                .provider(new_value.as_slice())
                .not(prev_value.as_slice()),
            agent.recipe_flows_as_provider(&base_address)
        )?;
    }
    if new_entry.receiver != prev_entry.receiver {
        let new_value = match &new_entry.receiver { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.receiver { Some(val) => vec![val.to_owned()], None => vec![] };
        update_index!(
            recipe_flow
                .receiver(new_value.as_slice())
                .not(prev_value.as_slice()),
            agent.recipe_flows_as_receiver(&base_address)
        )?;
    }
    if new_entry.input_of != prev_entry.input_of {
        let new_value = match &new_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            recipe_flow
                .input_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            process.intended_inputs(&base_address)
        );
        hdk::prelude::debug!("handle_update_recipe_flow::input_of index {:?}", e);
    }
    if new_entry.output_of != prev_entry.output_of {
        let new_value = match &new_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let prev_value = match &prev_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
        let e = update_index!(
            recipe_flow
                .output_of(new_value.as_slice())
                .not(prev_value.as_slice()),
            process.intended_outputs(&base_address)
        );
        hdk::prelude::debug!("handle_update_recipe_flow::output_of index {:?}", e);
    }

    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_recipe_flow(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let e = update_index!(recipe_flow.input_of.not(&vec![process_address]), process.intended_inputs(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::input_of index {:?}", e);
    }
    if let Some(process_address) = entry.output_of {
        let e = update_index!(recipe_flow.output_of.not(&vec![process_address]), process.intended_outputs(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::output_of index {:?}", e);
    }
    if let Some(agent_address) = entry.provider {
        let e = update_index!(recipe_flow.provider.not(&vec![agent_address]), process.recipe_flows_as_provider(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::provider index {:?}", e);
    }
    if let Some(agent_address) = entry.receiver {
        let e = update_index!(recipe_flow.receiver.not(&vec![agent_address]), process.recipe_flows_as_receiver(&base_address));
        hdk::prelude::debug!("handle_delete_recipe_flow::receiver index {:?}", e);
    }

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &RecipeFlowAddress, meta: &SignedActionHashed, e: &EntryData, (
        satisfactions,
        // published_in,
    ): (
        Vec<SatisfactionAddress>,
        // Vec<ProposedRecipeFlowAddress>
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        recipe_flow: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            state: e.note.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            recipe_clause_of: e.recipe_clause_of.to_owned(),
            stage: e.stage.to_owned(),
            recipe_input_of: e.recipe_input_of.to_owned(),
            recipe_output_of: e.recipe_output_of.to_owned(),
        }
    })
}

//---------------- READ ----------------

/// Properties accessor for zome config
fn read_recipe_flow_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.recipe_flow.index_zome)
}

/// Properties accessor for zome config
fn read_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.recipe_flow.process_index_zome
}

/// Properties accessor for zome config
fn read_agent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.recipe_flow.agent_index_zome
}

// @see construct_response
pub fn get_link_fields(recipe_flow: &RecipeFlowAddress) -> RecordAPIResult<(
    Vec<SatisfactionAddress>,
)> {
    Ok((
        read_index!(recipe_flow(recipe_flow).satisfied_by)?,
    ))
}
