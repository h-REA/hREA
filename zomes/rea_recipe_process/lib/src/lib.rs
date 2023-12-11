/**
 * hREA recipe_process zome library API
 *
 * Contains helper methods that can be used to manipulate `RecipeProcess` data
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

use hc_zome_rea_recipe_process_storage::*;
use hc_zome_rea_recipe_process_rpc::*;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.recipe_process.index_zome)
}

pub fn handle_create_recipe_process<S>(entry_def_id: S, recipe_process: CreateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_,_, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(read_index_zome, &entry_def_id, recipe_process.to_owned())?;

    // handle link fields
    // :TODO: improve error handling

    // if let CreateRequest { provider: MaybeUndefined::Some(provider), .. } = &recipe_process {
    //     create_index!(recipe_process.provider(provider), agent.recipe_processs_as_provider(&base_address))?;
    // };
    // if let CreateRequest { receiver: MaybeUndefined::Some(receiver), .. } = &recipe_process {
    //     create_index!(recipe_process.receiver(receiver), agent.recipe_processs_as_receiver(&base_address))?;
    // };
    // if let CreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = &recipe_process {
    //     let e = create_index!(recipe_process.input_of(input_of), process.intended_inputs(&base_address));
    //     hdk::prelude::debug!("handle_create_recipe_process::input_of index {:?}", e);
    // };
    // if let CreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = &recipe_process {
    //     let e = create_index!(recipe_process.output_of(output_of), process.intended_outputs(&base_address));
    //     hdk::prelude::debug!("handle_create_recipe_process::output_of index {:?}", e);
    // };

    // return entire record structure
    construct_response(&base_address, &meta, &entry_resp, get_link_fields(&base_address)?)
}

pub fn handle_get_recipe_process(address: RecipeProcessAddress) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _>(address.as_ref())?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&address)?)
}

pub fn handle_get_revision(revision_id: ActionHash) -> RecordAPIResult<ResponseData>
{
    let (meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;
    construct_response(&base_address, &meta, &entry, get_link_fields(&base_address)?)
}

pub fn handle_update_recipe_process(recipe_process: UpdateRequest) -> RecordAPIResult<ResponseData>
{
    let address = recipe_process.get_revision_id().to_owned();
    let (meta, base_address, new_entry, prev_entry): (_, RecipeProcessAddress, EntryData, EntryData) = update_record(&address, recipe_process.to_owned())?;

    // handle link fields
    // if new_entry.provider != prev_entry.provider {
    //     let new_value = match &new_entry.provider { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let prev_value = match &prev_entry.provider { Some(val) => vec![val.to_owned()], None => vec![] };
    //     update_index!(
    //         recipe_process
    //             .provider(new_value.as_slice())
    //             .not(prev_value.as_slice()),
    //         agent.recipe_processs_as_provider(&base_address)
    //     )?;
    // }
    // if new_entry.receiver != prev_entry.receiver {
    //     let new_value = match &new_entry.receiver { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let prev_value = match &prev_entry.receiver { Some(val) => vec![val.to_owned()], None => vec![] };
    //     update_index!(
    //         recipe_process
    //             .receiver(new_value.as_slice())
    //             .not(prev_value.as_slice()),
    //         agent.recipe_processs_as_receiver(&base_address)
    //     )?;
    // }
    // if new_entry.input_of != prev_entry.input_of {
    //     let new_value = match &new_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let prev_value = match &prev_entry.input_of { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let e = update_index!(
    //         recipe_process
    //             .input_of(new_value.as_slice())
    //             .not(prev_value.as_slice()),
    //         process.intended_inputs(&base_address)
    //     );
    //     hdk::prelude::debug!("handle_update_recipe_process::input_of index {:?}", e);
    // }
    // if new_entry.output_of != prev_entry.output_of {
    //     let new_value = match &new_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let prev_value = match &prev_entry.output_of { Some(val) => vec![val.to_owned()], None => vec![] };
    //     let e = update_index!(
    //         recipe_process
    //             .output_of(new_value.as_slice())
    //             .not(prev_value.as_slice()),
    //         process.intended_outputs(&base_address)
    //     );
    //     hdk::prelude::debug!("handle_update_recipe_process::output_of index {:?}", e);
    // }

    construct_response(&base_address, &meta, &new_entry, get_link_fields(&base_address)?)
}

pub fn handle_delete_recipe_process(revision_id: ActionHash) -> RecordAPIResult<bool>
{
    // load the record to ensure it is of the correct type
    let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

    // handle link fields
    // if let Some(process_address) = entry.input_of {
    //     let e = update_index!(recipe_process.input_of.not(&vec![process_address]), process.intended_inputs(&base_address));
    //     hdk::prelude::debug!("handle_delete_recipe_process::input_of index {:?}", e);
    // }
    // if let Some(process_address) = entry.output_of {
    //     let e = update_index!(recipe_process.output_of.not(&vec![process_address]), process.intended_outputs(&base_address));
    //     hdk::prelude::debug!("handle_delete_recipe_process::output_of index {:?}", e);
    // }
    // if let Some(agent_address) = entry.provider {
    //     let e = update_index!(recipe_process.provider.not(&vec![agent_address]), process.recipe_processs_as_provider(&base_address));
    //     hdk::prelude::debug!("handle_delete_recipe_process::provider index {:?}", e);
    // }
    // if let Some(agent_address) = entry.receiver {
    //     let e = update_index!(recipe_process.receiver.not(&vec![agent_address]), process.recipe_processs_as_receiver(&base_address));
    //     hdk::prelude::debug!("handle_delete_recipe_process::receiver index {:?}", e);
    // }

    // delete entry last, as it must be present in order for links to be removed
    delete_record::<EntryStorage>(&revision_id)
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &RecipeProcessAddress, meta: &SignedActionHashed, e: &EntryData, (
        satisfactions,
        // published_in,
    ): (
        Vec<SatisfactionAddress>,
        // Vec<ProposedRecipeProcessAddress>
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        recipe_process: Response {
            id: address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
            name: e.note.to_owned(),
            image: e.note.to_owned(),
            note: e.note.to_owned(),
            process_conforms_to: e.process_conforms_to.to_owned(),
        }
    })
}

//---------------- READ ----------------

/// Properties accessor for zome config
fn read_recipe_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.recipe_process.index_zome)
}

/// Properties accessor for zome config
fn read_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.recipe_process.process_index_zome
}

/// Properties accessor for zome config
fn read_agent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.recipe_process.agent_index_zome
}

// @see construct_response
pub fn get_link_fields(recipe_process: &RecipeProcessAddress) -> RecordAPIResult<(
    Vec<SatisfactionAddress>,
)> {
    Ok((
        read_index!(recipe_process(recipe_process).satisfied_by)?,
    ))
}
