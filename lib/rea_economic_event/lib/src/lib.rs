/**
 * Holo-REA 'economic event' zome library API
 *
 * Contains helper methods that can be used to manipulate economic event data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use std::borrow::Cow;
use hdk::{
    PUBLIC_TOKEN,
    prelude::Address,
    error::{ ZomeApiResult, ZomeApiError },
};

use hdk_graph_helpers::{
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        link_entries,
        get_linked_addresses_as_type,
    },
    anchors::{
        create_anchor_index,
        read_anchored_record_entries,
    },
    local_indexes::{
        create_direct_index,
        delete_direct_index,
        query_direct_index_with_foreign_key,
        query_direct_remote_index_with_foreign_key,
    },
    remote_indexes::{
        create_direct_remote_index,
        update_direct_remote_index,
        remove_direct_remote_index,
        create_direct_remote_index_destination,
    },
};

use vf_core::type_aliases::{
    EventAddress,
    ResourceAddress,
    ActionId,
    FulfillmentAddress,
    SatisfactionAddress,
    ProcessSpecificationAddress,
};

use hc_zome_rea_fulfillment_storage_consts::{FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG};
use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_resource_specification_storage_consts::{
    ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
    RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG,
};

use hc_zome_rea_economic_event_storage_consts::*;
use hc_zome_rea_economic_event_storage::{
    Entry,
};
use hc_zome_rea_economic_event_rpc::{
    ResourceInventoryType,
    QueryParams,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
    Response,
    ResponseData,
};

use hc_zome_rea_economic_resource_storage_consts::*;
use hc_zome_rea_economic_resource_storage::{
    Entry as EconomicResourceEntry,
};
use hc_zome_rea_economic_resource_rpc::{
    CreateRequest as EconomicResourceCreateRequest,
    CreationPayload as ResourceCreationPayload,
};
use hc_zome_rea_economic_resource_lib::{
    resource_creation,
    construct_response_record as construct_resource_response,
    get_link_fields as get_resource_link_fields,
};

use hc_zome_rea_process_storage_consts::*;
use hc_zome_rea_agreement_storage_consts::{
    AGREEMENT_BASE_ENTRY_TYPE,
    AGREEMENT_EVENTS_LINK_TYPE,
    AGREEMENT_EVENTS_LINK_TAG,
};

// API gateway entrypoints. All methods must accept parameters by value.

pub fn receive_create_economic_event(event: EconomicEventCreateRequest, new_inventoried_resource: Option<EconomicResourceCreateRequest>) -> ZomeApiResult<ResponseData> {
    let mut resources_affected: Vec<(ResourceAddress, EconomicResourceEntry)> = vec![];
    let mut resource_created: Option<(ResourceAddress, EconomicResourceEntry)> = None;

    // if the event observes a new resource, create that resource & return it in the response
    if let Some(economic_resource) = new_inventoried_resource {
        let new_resource = handle_create_economic_resource(&economic_resource, &event)?;
        resource_created = Some(new_resource.clone());
        resources_affected.push(new_resource);
    }

    // if the event is a transfer-like event, run the receiver's update first
    if let MaybeUndefined::Some(receiver_inventory) = event.to_resource_inventoried_as.to_owned() {
        resources_affected.push(handle_update_economic_resource(&receiver_inventory, ResourceInventoryType::ReceivingInventory, &event)?);
    }
    // after receiver, run provider. This entry data will be returned in the response.
    if let MaybeUndefined::Some(provider_inventory) = event.resource_inventoried_as.to_owned() {
        resources_affected.push(handle_update_economic_resource(&provider_inventory, ResourceInventoryType::ProvidingInventory, &event)?);
    }

    // now that the resource updates have succeeded, write the event
    // :TODO: rethinking this, it's probably the event that should be written first, and the resource
    // validation should eventually depend on an event already having been authored.
    let (event_address, event_entry) = handle_create_economic_event(&event, match resource_created.clone() {
        Some(data) => Some(data.0.to_owned()),
        None => None,
    })?;

    // :IMPORTANT: we don't create indexes until after the event has saved since the event storage may fail validation and
    // we don't want dangling resources without events to be visible to callers.
    // This is dirty and introduces some non-obvious complexity, but works around transactionality limitations for now.

    // Index the event for retrieval via `get_all` API endpoints
    // :TODO: change to use DAG indexes for time ordering & pagination
    create_anchor_index(&EVENT_INDEX_ROOT_ENTRY_TYPE.to_string(), EVENT_INDEX_ENTRY_LINK_TYPE, &EVENT_INDEX_ROOT_ENTRY_ID.to_string(), &event_address.as_ref())?;
    // Index any new resource for retrieval via `get_all` API endpoints
    if let Some(resource_data) = &resource_created {
        let resource_addr = resource_data.0.to_owned();
        create_anchor_index(&RESOURCE_INDEX_ROOT_ENTRY_TYPE.to_string(), RESOURCE_INDEX_ENTRY_LINK_TYPE, &RESOURCE_INDEX_ROOT_ENTRY_ID.to_string(), &resource_addr.as_ref())?;
    }
    // Link any affected resources to this event so that we can pull all the events which affect any resource
    // :TODO: error handling
    for resource_data in resources_affected.iter() {
        let _ = link_entries(
            resource_data.0.as_ref(),
            event_address.as_ref(),
            RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE, RESOURCE_AFFECTED_BY_EVENT_LINK_TAG,
        );
    }

    match resource_created {
        Some(resource_data) => {
            let resource_addr = resource_data.0.to_owned();
            let resource_entry = resource_data.1;

            Ok(construct_response_with_resource(
                &event_address, &event_entry, get_link_fields(&event_address),
                Some(resource_addr.clone()), Some(resource_entry), get_resource_link_fields(&resource_addr)
            ))
        },
        None => {
            // :TODO: pass results from link creation rather than re-reading
            Ok(construct_response(&event_address, &event_entry, get_link_fields(&event_address)))
        },
    }
}

pub fn receive_get_economic_event(address: EventAddress) -> ZomeApiResult<ResponseData> {
    handle_get_economic_event(&address)
}

pub fn receive_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<ResponseData> {
    handle_update_economic_event(&event)
}

pub fn receive_delete_economic_event(address: EventAddress) -> ZomeApiResult<bool> {
    handle_delete_economic_event(&address)
}

pub fn receive_get_all_economic_events() -> ZomeApiResult<Vec<ResponseData>> {
    handle_get_all_economic_events()
}

pub fn receive_query_events(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    handle_query_events(&params)
}

// API logic handlers

fn handle_create_economic_event(event: &EconomicEventCreateRequest, resource_address: Option<ResourceAddress>) -> ZomeApiResult<(EventAddress, Entry)> {
    let (base_address, entry_resp): (EventAddress, Entry) = create_record(
        EVENT_BASE_ENTRY_TYPE, EVENT_ENTRY_TYPE,
        EVENT_INITIAL_ENTRY_LINK_TYPE,
        match resource_address {
            Some(addr) => event.with_inventoried_resource(&addr),
            None => event.to_owned(),
        }
    )?;

    // handle link fields
    // :TODO: propagate errors
    if let EconomicEventCreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = event {
        let _results = create_direct_index(
            base_address.as_ref(),
            input_of.as_ref(),
            EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
            PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
        );
    };
    if let EconomicEventCreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = event {
        let _results = create_direct_index(
            base_address.as_ref(),
            output_of.as_ref(),
            EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
            PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
        );
    };
    if let EconomicEventCreateRequest { realization_of: MaybeUndefined::Some(realization_of), .. } = event {
        let _results = create_direct_remote_index(
            BRIDGED_AGREEMENT_DHT, "economic_event_idx", "index_events", Address::from(PUBLIC_TOKEN.to_string()),
            AGREEMENT_BASE_ENTRY_TYPE,
            EVENT_REALIZATION_OF_LINK_TYPE, EVENT_REALIZATION_OF_LINK_TAG,
            AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG,
            base_address.as_ref(),
            vec![(realization_of.as_ref()).clone()],
        );
    };

    Ok((base_address, entry_resp))
}

/// Handle creation of new resources via events + resource metadata
///
fn handle_create_economic_resource(economic_resource: &EconomicResourceCreateRequest, event: &EconomicEventCreateRequest) -> ZomeApiResult<(ResourceAddress, EconomicResourceEntry)> {
    // :TODO: move this assertion to validation callback
    if let MaybeUndefined::Some(_sent_inventory_id) = &event.resource_inventoried_as {
        panic!("cannot create a new EconomicResource and specify an inventoried resource ID in the same event");
    }

    let params: ResourceCreationPayload = resource_creation(
        &event.with_inventory_type(ResourceInventoryType::ProvidingInventory),
        &economic_resource
    );

    let (base_address, entry_resp): (ResourceAddress, EconomicResourceEntry) = create_record(
        RESOURCE_BASE_ENTRY_TYPE, RESOURCE_ENTRY_TYPE, RESOURCE_INITIAL_ENTRY_LINK_TYPE,
        EconomicResourceEntry::from(params.clone())
    )?;

    let resource_params = params.get_resource_params();
    let event_params = params.get_event_params();

    // :NOTE: this will always run- resource without a specification ID would fail entry validation (implicit in the above)
    if let Some(conforms_to) = params.get_resource_specification_id() {
        let _results = create_direct_remote_index_destination(
            ECONOMIC_RESOURCE_SPECIFICATION_BASE_ENTRY_TYPE,
            RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TYPE, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG,
            RESOURCE_CONFORMS_TO_LINK_TYPE, RESOURCE_CONFORMS_TO_LINK_TAG,
            &conforms_to,
            vec![base_address.clone()],
        );
    }
    if let Some(contained_in) = resource_params.get_contained_in() {
        let _results = create_direct_index(
            base_address.as_ref(),
            contained_in.as_ref(),
            RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
            RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
        );
    };
    if let MaybeUndefined::Some(realization_of) = event_params.get_realization_of() {
        let _results = update_direct_remote_index(
            BRIDGED_AGREEMENT_DHT, "economic_event_idx", "index_events", Address::from(PUBLIC_TOKEN.to_string()),
            AGREEMENT_BASE_ENTRY_TYPE,
            EVENT_REALIZATION_OF_LINK_TYPE, EVENT_REALIZATION_OF_LINK_TAG,
            AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG,
            &base_address, &MaybeUndefined::Some(realization_of),
        );
    };

    Ok((base_address, entry_resp))
}

fn handle_get_economic_event(address: &EventAddress) -> ZomeApiResult<ResponseData> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(address, &entry, get_link_fields(address)))
}

fn handle_update_economic_event(event: &EconomicEventUpdateRequest) -> ZomeApiResult<ResponseData> {
    let address = event.get_id();
    let new_entry = update_record(EVENT_ENTRY_TYPE, &address, event)?;

    // :TODO: optimise this- should pass results from `replace_direct_index` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
}

/// Handle alteration of existing resources via events
///
fn handle_update_economic_resource(resource_addr: &ResourceAddress, inventory_type: ResourceInventoryType, event: &EconomicEventCreateRequest) -> ZomeApiResult<(ResourceAddress, EconomicResourceEntry)> {
    let context_event = event.with_inventory_type(inventory_type);

    let new_resource = update_record(RESOURCE_ENTRY_TYPE, &resource_addr.to_owned(), &context_event)?;

    Ok((resource_addr.to_owned(), new_resource))
}

fn handle_delete_economic_event(address: &EventAddress) -> ZomeApiResult<bool> {
    // read any referencing indexes
    let entry: Entry = read_record_entry(&address)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let _results = delete_direct_index(
            address.as_ref(), process_address.as_ref(),
            EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
            PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
        );
    }
    if let Some(process_address) = entry.output_of {
        let _results = delete_direct_index(
            address.as_ref(), process_address.as_ref(),
            EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
            PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
        );
    }
    if let Some(agreement_address) = entry.realization_of {
        let _results = remove_direct_remote_index(
            BRIDGED_AGREEMENT_DHT, "event_idx", "index_events", Address::from(PUBLIC_TOKEN.to_string()),
            AGREEMENT_BASE_ENTRY_TYPE,
            EVENT_REALIZATION_OF_LINK_TYPE, EVENT_REALIZATION_OF_LINK_TAG,
            AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG,
            address, &agreement_address,
        );
    }

    // delete entry last as it must be present in order for links to be removed
    delete_record::<Entry>(&address)
}

fn handle_get_all_economic_events() -> ZomeApiResult<Vec<ResponseData>> {
    let entries_result: ZomeApiResult<Vec<(EventAddress, Option<Entry>)>> = read_anchored_record_entries(
        &EVENT_INDEX_ROOT_ENTRY_TYPE.to_string(), EVENT_INDEX_ENTRY_LINK_TYPE, &EVENT_INDEX_ROOT_ENTRY_ID.to_string(),
    );

    handle_list_output(entries_result)
}

fn handle_query_events(params: &QueryParams) -> ZomeApiResult<Vec<ResponseData>> {
    let mut entries_result: ZomeApiResult<Vec<(EventAddress, Option<Entry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = query_direct_index_with_foreign_key(
                satisfies, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.fulfills {
        Some(fulfills) => {
            entries_result = query_direct_index_with_foreign_key(
                fulfills, FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = query_direct_index_with_foreign_key(
                input_of, PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = query_direct_index_with_foreign_key(
                output_of, PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.realization_of {
        Some(realization_of) => {
            entries_result = query_direct_remote_index_with_foreign_key(
                realization_of, AGREEMENT_BASE_ENTRY_TYPE,
                AGREEMENT_EVENTS_LINK_TYPE, AGREEMENT_EVENTS_LINK_TAG,
            );
        },
        _ => (),
    };

    handle_list_output(entries_result)
}

fn handle_list_output(entries_result: ZomeApiResult<Vec<(EventAddress, Option<Entry>)>>) -> ZomeApiResult<Vec<ResponseData>> {
    match entries_result {
        Ok(entries) => Ok(
            entries.iter()
                .map(|(entry_base_address, maybe_entry)| {
                    match maybe_entry {
                        Some(entry) => Ok(construct_response(
                            entry_base_address, &entry, get_link_fields(entry_base_address),
                        )),
                        None => Err(ZomeApiError::Internal("referenced entry not found".to_string()))
                    }
                })
                .filter_map(Result::ok)
                .collect()
        ),
        _ => Err(ZomeApiError::Internal("could not load linked addresses".to_string()))
    }
}

/**
 * Create response from input DHT primitives
 *
 * :TODO: determine if possible to construct `Response` with refs to fields of `e`, rather than cloning memory
 */
pub fn construct_response_with_resource<'a>(
    event_address: &EventAddress,
    event: &Entry, (
    fulfillments,
        satisfactions,
    ): (
        Option<Cow<'a, Vec<FulfillmentAddress>>>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
    ),
    resource_address: Option<ResourceAddress>,
    resource: Option<EconomicResourceEntry>, (
        contained_in,
        stage,
        state,
        contains,
     ): (
        Option<ResourceAddress>,
        Option<ProcessSpecificationAddress>,
        Option<ActionId>,
        Option<Cow<'a, Vec<ResourceAddress>>>,
    ),
) -> ResponseData {
    ResponseData {
        economic_event: Response {
            id: event_address.to_owned(),
            action: event.action.to_owned(),
            note: event.note.to_owned(),
            input_of: event.input_of.to_owned(),
            output_of: event.output_of.to_owned(),
            provider: event.provider.to_owned(),
            receiver: event.receiver.to_owned(),
            resource_inventoried_as: event.resource_inventoried_as.to_owned(),
            to_resource_inventoried_as: event.to_resource_inventoried_as.to_owned(),
            resource_classified_as: event.resource_classified_as.to_owned(),
            resource_conforms_to: event.resource_conforms_to.to_owned(),
            resource_quantity: event.resource_quantity.to_owned(),
            effort_quantity: event.effort_quantity.to_owned(),
            has_beginning: event.has_beginning.to_owned(),
            has_end: event.has_end.to_owned(),
            has_point_in_time: event.has_point_in_time.to_owned(),
            at_location: event.at_location.to_owned(),
            agreed_in: event.agreed_in.to_owned(),
            triggered_by: event.triggered_by.to_owned(),
            realization_of: event.realization_of.to_owned(),
            in_scope_of: event.in_scope_of.to_owned(),
            fulfills: fulfillments.map(Cow::into_owned),
            satisfies: satisfactions.map(Cow::into_owned),
        },
        economic_resource: match resource_address {
            Some(addr) => Some(construct_resource_response(&addr, &(resource.unwrap()), (contained_in, stage, state, contains))),
            None => None,
        },
    }
}

// Same as above, but omits EconomicResource object
pub fn construct_response<'a>(
    address: &EventAddress, e: &Entry, (
        fulfillments,
        satisfactions,
    ): (
        Option<Cow<'a, Vec<FulfillmentAddress>>>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
    )
) -> ResponseData {
    ResponseData {
        economic_event: Response {
            id: address.to_owned().into(),
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            input_of: e.input_of.to_owned(),
            output_of: e.output_of.to_owned(),
            provider: e.provider.to_owned(),
            receiver: e.receiver.to_owned(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned(),
            to_resource_inventoried_as: e.to_resource_inventoried_as.to_owned(),
            resource_classified_as: e.resource_classified_as.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            has_point_in_time: e.has_point_in_time.to_owned(),
            at_location: e.at_location.to_owned(),
            agreed_in: e.agreed_in.to_owned(),
            triggered_by: e.triggered_by.to_owned(),
            realization_of: e.realization_of.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            fulfills: fulfillments.map(Cow::into_owned),
            satisfies: satisfactions.map(Cow::into_owned),
        },
        economic_resource: None,
    }
}

// @see construct_response
pub fn get_link_fields<'a>(event: &EventAddress) -> (
    Option<Cow<'a, Vec<FulfillmentAddress>>>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
) {
    (
        Some(get_linked_addresses_as_type(event, EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG)),
        Some(get_linked_addresses_as_type(event, EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG)),
    )
}

// #[cfg(test)]
// mod tests {
//     use super::*;

    // #[test]
    // fn test_derived_fields() {
    //     let e = Entry { note: Some("a note".into()), ..Entry::default() };
    //     assert_eq!(e.note, Some("a note".into()))
    // }

    // :TODO: unit tests for type conversions... though maybe these should be macro tests, not tests for every single record type
// }
