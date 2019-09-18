/**
 * Handling for external request structure for economic event records
 */

use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::ZomeApiResult,
    error::ZomeApiError,
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    records::{
        create_record,
        read_record_entry,
        update_record,
        delete_record,
    },
    links::{
        link_entries_bidir,
        get_links_and_load_entry_data,
        get_linked_addresses_as_type,
        replace_entry_link_set,
        remove_links_bidir,
    },
};

use vf_observation::type_aliases::{
    EventAddress,
    ResourceAddress,
    ProcessAddress,
    CommitmentAddress,
    IntentAddress,
    FulfillmentAddress,
    SatisfactionAddress,
};
use vf_observation::economic_event::{
    Entry as EconomicEventEntry,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
    ResponseData as EconomicEventResponse,
    construct_response,
    construct_response_with_resource,
};
use vf_observation::economic_resource::{
    ResourceInventoryType,
    Entry as EconomicResourceEntry,
    CreateRequest as EconomicResourceCreateRequest,
    CreationPayload as ResourceCreationPayload,
    resource_creation,
    get_link_fields as get_resource_link_fields,
};
use vf_observation::identifiers::{
    EVENT_BASE_ENTRY_TYPE, EVENT_ENTRY_TYPE, EVENT_INITIAL_ENTRY_LINK_TYPE,
    EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG,
    EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG,
    EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
    EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
    PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
    PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
    RESOURCE_BASE_ENTRY_TYPE, RESOURCE_ENTRY_TYPE, RESOURCE_INITIAL_ENTRY_LINK_TYPE,
    RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
    RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
};
use vf_planning::identifiers::{
    FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG,
    SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    input_of: Option<ProcessAddress>,
    output_of: Option<ProcessAddress>,
    satisfies: Option<IntentAddress>,
    fulfills: Option<CommitmentAddress>,
}

// API gateway entrypoints. All methods must accept parameters by value.

pub fn receive_create_economic_event(event: EconomicEventCreateRequest, create_resource: Option<EconomicResourceCreateRequest>) -> ZomeApiResult<EconomicEventResponse> {
    let mut resource_address: Option<ResourceAddress> = None;
    let mut resource_entry: Option<EconomicResourceEntry> = None;

    if let Some(economic_resource) = create_resource {
        // Handle creation of new resources via events + resource metadata

        // :TODO: move this assertion to validation callback
        if let MaybeUndefined::Some(_sent_inventory_id) = event.resource_inventoried_as {
            panic!("cannot create a new EconomicResource and specify an inventoried resource ID in the same event");
        }

        let resource_result = handle_create_economic_resource(resource_creation(&event, &economic_resource))?;
        resource_address = Some(resource_result.0);
        resource_entry = Some(resource_result.1);
    }

    if let MaybeUndefined::Some(provider_inventory) = event.resource_inventoried_as.to_owned() {
        // Handle alteration of existing resources via events

        let context_event = event.with_inventory_type(ResourceInventoryType::ProvidingInventory);

        let new_resource = update_record(RESOURCE_ENTRY_TYPE, &provider_inventory.to_owned(), &context_event)?;
        resource_address = Some(provider_inventory.to_owned());
        resource_entry = Some(new_resource);
    }

    if let MaybeUndefined::Some(receiver_inventory) = event.to_resource_inventoried_as.to_owned() {
        // Handle alteration of existing resources via events

        let context_event = event.with_inventory_type(ResourceInventoryType::ReceivingInventory);

        // :TODO: output in response?
        let _receiver_resource: EconomicResourceEntry = update_record(RESOURCE_ENTRY_TYPE, &receiver_inventory.to_owned(), &context_event)?;
        // resource_address = Some(receiver_inventory.to_owned());
        // resource_entry = Some(new_resource);
    }

    let (event_address, event_entry) = handle_create_economic_event(&event, resource_address.to_owned())?;

    // :TODO: pass results from link creation rather than re-reading
    Ok(match resource_address {
        None => construct_response(&event_address, &event_entry, get_link_fields(&event_address)),
        _ => construct_response_with_resource(
            &event_address, &event_entry, get_link_fields(&event_address),
            resource_address.clone(), resource_entry, match resource_address {
                Some(addr) => get_resource_link_fields(&addr),
                None => (None, None),
            }
        ),
    })
}

pub fn receive_get_economic_event(address: EventAddress) -> ZomeApiResult<EconomicEventResponse> {
    handle_get_economic_event(&address)
}

pub fn receive_update_economic_event(event: EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    handle_update_economic_event(&event)
}

pub fn receive_delete_economic_event(address: EventAddress) -> ZomeApiResult<bool> {
    handle_delete_economic_event(&address)
}

pub fn receive_query_events(params: QueryParams) -> ZomeApiResult<Vec<EconomicEventResponse>> {
    handle_query_events(&params)
}

// API logic handlers

fn handle_create_economic_event(event: &EconomicEventCreateRequest, resource_address: Option<ResourceAddress>) -> ZomeApiResult<(EventAddress, EconomicEventEntry)> {
    let (base_address, entry_resp): (EventAddress, EconomicEventEntry) = create_record(
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
        let _results = link_entries_bidir(
            base_address.as_ref(),
            input_of.as_ref(),
            EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
            PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
        );
    };
    if let EconomicEventCreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = event {
        let _results = link_entries_bidir(
            base_address.as_ref(),
            output_of.as_ref(),
            EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
            PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
        );
    };

    Ok((base_address, entry_resp))
}

fn handle_create_economic_resource(params: ResourceCreationPayload) -> ZomeApiResult<(ResourceAddress, EconomicResourceEntry)> {
    let (base_address, entry_resp): (ResourceAddress, EconomicResourceEntry) = create_record(
        RESOURCE_BASE_ENTRY_TYPE, RESOURCE_ENTRY_TYPE, RESOURCE_INITIAL_ENTRY_LINK_TYPE,
        EconomicResourceEntry::from(params.clone())
    )?;

    if let Some(contained_in) = params.get_resource_params().get_contained_in() {
        let _results = link_entries_bidir(
            base_address.as_ref(),
            contained_in.as_ref(),
            RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
            RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
        );
    };

    Ok((base_address, entry_resp))
}

fn handle_get_economic_event(address: &EventAddress) -> ZomeApiResult<EconomicEventResponse> {
    let entry = read_record_entry(&address)?;
    Ok(construct_response(address, &entry, get_link_fields(address)))
}

fn handle_update_economic_event(event: &EconomicEventUpdateRequest) -> ZomeApiResult<EconomicEventResponse> {
    let address = event.get_id();
    let new_entry = update_record(EVENT_ENTRY_TYPE, &address, event)?;

    // handle link fields
    replace_entry_link_set(address, &event.input_of,
        EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
        PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
    )?;
    replace_entry_link_set(address, &event.output_of,
        EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
        PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
    )?;

    // :TODO: optimise this- should pass results from `replace_entry_link_set` instead of retrieving from `get_link_fields` where updates
    Ok(construct_response(address, &new_entry, get_link_fields(address)))
}

fn handle_delete_economic_event(address: &EventAddress) -> ZomeApiResult<bool> {
    // read any referencing indexes
    let (
        input_of, output_of,
        ..
        // :NOTE: These aren't managed- they should be retained to allow exploring the deleted data:
        // fulfillments, satisfactions
    ) = get_link_fields(address);

    // handle link fields
    if let Some(process_address) = input_of {
        let _results = remove_links_bidir(
            address.as_ref(), process_address.as_ref(),
            EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG,
            PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
        );
    }
    if let Some(process_address) = output_of {
        let _results = remove_links_bidir(
            address.as_ref(), process_address.as_ref(),
            EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG,
            PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
        );
    }

    // delete entry last as it must be present in order for links to be removed
    delete_record::<EconomicEventEntry>(&address)
}

fn handle_query_events(params: &QueryParams) -> ZomeApiResult<Vec<EconomicEventResponse>> {
    let mut entries_result: ZomeApiResult<Vec<(EventAddress, Option<EconomicEventEntry>)>> = Err(ZomeApiError::Internal("No results found".to_string()));

    // :TODO: implement proper AND search rather than exclusive operations
    match &params.satisfies {
        Some(satisfies) => {
            entries_result = get_links_and_load_entry_data(
                satisfies, SATISFACTION_SATISFIEDBY_LINK_TYPE, SATISFACTION_SATISFIEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.fulfills {
        Some(fulfills) => {
            entries_result = get_links_and_load_entry_data(
                fulfills, FULFILLMENT_FULFILLEDBY_LINK_TYPE, FULFILLMENT_FULFILLEDBY_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.input_of {
        Some(input_of) => {
            entries_result = get_links_and_load_entry_data(
                input_of, PROCESS_EVENT_INPUTS_LINK_TYPE, PROCESS_EVENT_INPUTS_LINK_TAG,
            );
        },
        _ => (),
    };
    match &params.output_of {
        Some(output_of) => {
            entries_result = get_links_and_load_entry_data(
                output_of, PROCESS_EVENT_OUTPUTS_LINK_TYPE, PROCESS_EVENT_OUTPUTS_LINK_TAG,
            );
        },
        _ => (),
    };

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

// field list retrieval internals

// @see construct_response
fn get_link_fields<'a>(event: &EventAddress) -> (
    Option<ProcessAddress>,
    Option<ProcessAddress>,
    Option<Cow<'a, Vec<FulfillmentAddress>>>,
    Option<Cow<'a, Vec<SatisfactionAddress>>>,
) {
    (
        get_linked_addresses_as_type(event, EVENT_INPUT_OF_LINK_TYPE, EVENT_INPUT_OF_LINK_TAG).into_owned().pop(),
        get_linked_addresses_as_type(event, EVENT_OUTPUT_OF_LINK_TYPE, EVENT_OUTPUT_OF_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(event, EVENT_FULFILLS_LINK_TYPE, EVENT_FULFILLS_LINK_TAG)),
        Some(get_linked_addresses_as_type(event, EVENT_SATISFIES_LINK_TYPE, EVENT_SATISFIES_LINK_TAG)),
    )
}
