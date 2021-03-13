/**
 * Holo-REA 'economic resource' zome library API
 *
 * Contains helper methods that can be used to manipulate economic resource data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    DataIntegrityError, GraphAPIResult,
    local_indexes::{
        read_index,
        update_index,
        // query_index,
        query_root_index,
    },
    records::{
        read_record_entry,
        update_record,
    },
};

use vf_core::type_aliases::{
    ResourceAddress,
    EventAddress,
    ActionId,
    ProcessAddress,
    ProcessSpecificationAddress,
};

pub use hc_zome_rea_economic_resource_storage_consts::*;
pub use hc_zome_rea_economic_event_storage_consts::{EVENT_ENTRY_TYPE};
pub use hc_zome_rea_process_storage_consts::{PROCESS_ENTRY_TYPE};
pub use hc_zome_rea_resource_specification_storage_consts::{ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE};

use hc_zome_rea_economic_resource_storage::*;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_process_storage::{
    EntryData as ProcessData,
    EntryStorage as ProcessStorage,
};
use hc_zome_rea_economic_event_storage::{
    EntryData as EventData,
    EntryStorage as EventStorage,
};
use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EventCreateRequest,
    ResourceResponse as Response,
    ResourceResponseData as ResponseData,
};

pub fn receive_get_economic_resource<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, address: ResourceAddress) -> GraphAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_economic_resource(entry_def_id, event_entry_def_id, process_entry_def_id, &address)
}

pub fn receive_update_economic_resource<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, resource: UpdateRequest) -> GraphAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_economic_resource(entry_def_id, event_entry_def_id, process_entry_def_id, resource)
}

pub fn receive_get_all_economic_resources<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S) -> GraphAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_get_all_economic_resources(entry_def_id, event_entry_def_id, process_entry_def_id)
}

pub fn receive_query_economic_resources<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, params: QueryParams) -> GraphAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_query_economic_resources(entry_def_id, event_entry_def_id, process_entry_def_id, &params)
}

fn handle_get_economic_resource<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, address: &ResourceAddress) -> GraphAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, ResourceAddress, _,_>(&entry_def_id, address)?;
    Ok(construct_response(&base_address, &revision, &entry, get_link_fields(&entry_def_id, &event_entry_def_id, &process_entry_def_id, &address)?))
}

fn handle_update_economic_resource<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, resource: UpdateRequest) -> GraphAPIResult<ResponseData>
    where S: AsRef<str>
{
    let address = resource.get_revision_id().clone();
    let (revision_id, identity_address, entry, prev_entry): (_, ResourceAddress, EntryData, EntryData) = update_record(&entry_def_id, &address, resource)?;

    // :TODO: this may eventually be moved to an EconomicEvent update, see https://lab.allmende.io/valueflows/valueflows/-/issues/637
    let now_contained = if let Some(contained) = &entry.contained_in { vec![(*contained).as_ref().clone()] } else { vec![] };
    let prev_contained = if let Some(contained) = &prev_entry.contained_in { vec![(*contained).as_ref().clone()] } else { vec![] };
    update_index(&entry_def_id, identity_address.as_ref(), &entry_def_id,
        &RESOURCE_CONTAINED_IN_LINK_TAG, &RESOURCE_CONTAINS_LINK_TAG,
        now_contained.as_slice(), prev_contained.as_slice(),
    )?;

    // :TODO: optimise this- should pass results from `replace_direct_index` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(&identity_address, &revision_id, &entry, get_link_fields(&entry_def_id, &event_entry_def_id, &process_entry_def_id, &identity_address)?))
}

fn handle_get_all_economic_resources<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S) -> GraphAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let entries_result = query_root_index::<EntryData, EntryStorage, _,_>(&entry_def_id)?;

    Ok(handle_list_output(entry_def_id, event_entry_def_id, process_entry_def_id, entries_result)?.iter().cloned()
        .filter_map(Result::ok)
        .collect()
    )
}

fn handle_query_economic_resources<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, params: &QueryParams) -> GraphAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let entries_result: GraphAPIResult<Vec<GraphAPIResult<(RevisionHash, ResourceAddress, EntryData)>>> = Err(DataIntegrityError::EmptyQuery);

    /* :TODO:
    match &params.contains {
        Some(contains) => {
            entries_result = query_direct_index_with_foreign_key(
                &contains, RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.contained_in {
        Some(contained_in) => {
            entries_result = query_direct_index_with_foreign_key(
                contained_in, RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.conforms_to {
        Some(conforms_to) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                conforms_to, ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG,
            );
        },
        _ => (),
    };
    */

    Ok(handle_list_output(entry_def_id, event_entry_def_id, process_entry_def_id, entries_result?)?.iter().cloned()
        .filter_map(Result::ok)
        .collect()
    )
}

fn handle_list_output<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, entries_result: Vec<GraphAPIResult<(RevisionHash, ResourceAddress, EntryData)>>) -> GraphAPIResult<Vec<GraphAPIResult<ResponseData>>>
    where S: AsRef<str>
{
    Ok(entries_result.iter()
        .cloned()
        .filter_map(Result::ok)
        .map(|(revision_id, entry_base_address, entry)| {
            Ok(construct_response(
                &entry_base_address, &revision_id, &entry, get_link_fields(&entry_def_id, &event_entry_def_id, &process_entry_def_id, &entry_base_address)?
            ))
        })
        .collect()
    )
}

pub fn resource_creation(event: &EventCreateRequest, resource: &CreateRequest) -> CreationPayload {
    CreationPayload {
        event: event.to_owned(),
        resource: resource.to_owned(),
    }
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ResourceAddress, revision_id: &RevisionHash, e: &EntryData, (
        contained_in,
        stage,
        state,
        contains,
     ): (
        Option<ResourceAddress>,
        Option<ProcessSpecificationAddress>,
        Option<ActionId>,
        Vec<ResourceAddress>,
    ),
) -> ResponseData {
    ResponseData {
        economic_resource: construct_response_record(address, revision_id, e, (contained_in, stage, state, contains))
    }
}

/// Create response from input DHT primitives
pub fn construct_response_record<'a>(
    address: &ResourceAddress, revision_id: &RevisionHash, e: &EntryData, (
        contained_in,
        stage,
        state,
        contains,
     ): (
        Option<ResourceAddress>,
        Option<ProcessSpecificationAddress>,
        Option<ActionId>,
        Vec<ResourceAddress>,
    ),
) -> Response {
    Response {
        // entry fields
        id: address.to_owned(),
        revision_id: revision_id.to_owned(),
        conforms_to: e.conforms_to.to_owned(),
        classified_as: e.classified_as.to_owned(),
        tracking_identifier: e.tracking_identifier.to_owned(),
        lot: e.lot.to_owned(),
        image: e.image.to_owned(),
        accounting_quantity: e.accounting_quantity.to_owned(),
        onhand_quantity: e.onhand_quantity.to_owned(),
        unit_of_effort: e.unit_of_effort.to_owned(),
        stage: stage.to_owned(),
        state: state.to_owned(),
        current_location: e.current_location.to_owned(),
        note: e.note.to_owned(),

        // link fields
        contained_in: contained_in.to_owned(),
        contains: contains.to_owned(),
    }
}

// field list retrieval internals
// @see construct_response
pub fn get_link_fields<'a, S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, resource: &ResourceAddress) -> GraphAPIResult<(
    Option<ResourceAddress>,
    Option<ProcessSpecificationAddress>,
    Option<ActionId>,
    Vec<ResourceAddress>,
)>
    where S: AsRef<str>
{
    Ok((
        read_index(&entry_def_id, resource.as_ref(), &RESOURCE_CONTAINED_IN_LINK_TAG)?.pop(),
        get_resource_stage(&entry_def_id, &event_entry_def_id, &process_entry_def_id, resource)?,
        get_resource_state(&entry_def_id, &event_entry_def_id, resource)?,
        read_index(&entry_def_id, resource.as_ref(), &RESOURCE_CONTAINS_LINK_TAG)?,
    ))
}

fn get_resource_state<S>(entry_def_id: S, event_entry_def_id: S, resource: &ResourceAddress) -> GraphAPIResult<Option<ActionId>>
    where S: AsRef<str>
{
    let events: Vec<EventAddress> = get_affecting_events(entry_def_id, resource)?;

    // grab the most recent "pass" or "fail" action
    Ok(events.iter()
        .rev()
        .fold(None, move |result, event| {
            // already found it, just fall through
            // :TODO: figure out the Rust STL method to abort on first Some() value
            if let Some(_) = result {
                return result;
            }

            let evt = read_record_entry::<EventData, EventStorage, EventAddress,_,_>(&event_entry_def_id, event);
            match evt {
                Err(_) => result, // :TODO: this indicates some data integrity error
                Ok((_, _, entry)) => {
                    match &*String::from(entry.action.clone()) {
                        "pass" | "fail" => Some(entry.action),  // found it! Return this as the current resource state.
                        _ => result,    // still not located, keep looking...
                    }
                },
            }
        })
    )
}

fn get_resource_stage<S>(entry_def_id: S, event_entry_def_id: S, process_entry_def_id: S, resource: &ResourceAddress) -> GraphAPIResult<Option<ProcessSpecificationAddress>>
    where S: AsRef<str>
{
    let events: Vec<EventAddress> = get_affecting_events(entry_def_id, resource)?;

    // grab the most recent event with a process output association
    Ok(events.iter()
        .rev()
        .fold(None, move |result, event| {
            // already found it, just fall through
            // :TODO: figure out the Rust STL method to abort on first Some() value
            if let Some(_) = result {
                return result;
            }

            let evt = read_record_entry::<EventData, EventStorage, EventAddress,_,_>(&event_entry_def_id, event);
            match evt {
                Err(_) => result, // :TODO: this indicates some data integrity error
                Ok((_, _, entry)) => {
                    match &entry.output_of {
                        Some(output_of) => {
                            // get the associated process
                            let maybe_process_entry = read_record_entry::<ProcessData, ProcessStorage, ProcessAddress,_,_>(&process_entry_def_id, output_of);
                            // check to see if it has an associated specification
                            match &maybe_process_entry {
                                Ok((_,_, process_entry)) => match &process_entry.based_on {
                                    Some(based_on) => Some(based_on.to_owned()),   // found it!
                                    None => result, // still not located, keep looking...
                                },
                                Err(_) => result, // :TODO: this indicates some data integrity error
                            }
                        },
                        None => result,    // still not located, keep looking...
                    }
                },
            }
        })
    )
}

/// Read all the EconomicEvents affecting a given EconomicResource
fn get_affecting_events<S>(entry_def_id: S, resource: &ResourceAddress) -> GraphAPIResult<Vec<EventAddress>>
    where S: AsRef<str>
{
    read_index(
        &entry_def_id,
        resource.as_ref(),
        &RESOURCE_AFFECTED_BY_EVENT_LINK_TAG,
    )
}
