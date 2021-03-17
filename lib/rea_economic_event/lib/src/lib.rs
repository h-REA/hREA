/**
 * Holo-REA 'economic event' zome library API
 *
 * Contains helper methods that can be used to manipulate economic event data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use hdk_records::{
    RecordAPIResult, DataIntegrityError, MaybeUndefined,
    local_indexes::{
        create_index,
        read_index,
        delete_index,
        // query_index,
        query_root_index,
    },
    remote_indexes::{
        create_remote_index,
        update_remote_index,
    },
    records::{
        get_latest_header_hash,
        create_record,
        read_record_entry,
        read_record_entry_by_header,
        update_record,
        delete_record,
    },
};

pub use hc_zome_rea_economic_event_storage_consts::*;
pub use hc_zome_rea_economic_resource_storage_consts::{RESOURCE_ENTRY_TYPE};
pub use hc_zome_rea_process_storage_consts::{PROCESS_ENTRY_TYPE};
pub use hc_zome_rea_agreement_storage_consts::{AGREEMENT_ENTRY_TYPE};
pub use hc_zome_rea_resource_specification_storage_consts::{ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE};

use hc_zome_rea_economic_event_storage::*;
use hc_zome_rea_economic_event_rpc::{
    *,
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
};

use hc_zome_rea_economic_resource_storage_consts::*;
use hc_zome_rea_economic_resource_storage::{
    EntryData as EconomicResourceData,
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

// use hc_zome_rea_fulfillment_storage_consts::{FULFILLMENT_FULFILLEDBY_LINK_TAG};
// use hc_zome_rea_satisfaction_storage_consts::{SATISFACTION_SATISFIEDBY_LINK_TAG};
use hc_zome_rea_resource_specification_storage_consts::{ RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG };

use hc_zome_rea_process_storage_consts::{ PROCESS_EVENT_INPUTS_LINK_TAG, PROCESS_EVENT_OUTPUTS_LINK_TAG };
use hc_zome_rea_agreement_storage_consts::{ AGREEMENT_EVENTS_LINK_TAG };

// API gateway entrypoints. All methods must accept parameters by value.

pub fn receive_create_economic_event<S>(
    entry_def_id: S, resource_entry_def_id: S, process_entry_def_id: S, agreement_entry_def_id: S, resource_specification_entry_def_id: S,
    event: EconomicEventCreateRequest, new_inventoried_resource: Option<EconomicResourceCreateRequest>
) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let mut resources_affected: Vec<(RevisionHash, ResourceAddress, EconomicResourceData)> = vec![];
    let mut resource_created: Option<(RevisionHash, ResourceAddress, EconomicResourceData)> = None;

    // if the event observes a new resource, create that resource & return it in the response
    if let Some(economic_resource) = new_inventoried_resource {
        let new_resource = handle_create_economic_resource(
            &resource_entry_def_id, &resource_specification_entry_def_id,
            &economic_resource, &event,
        )?;
        resource_created = Some(new_resource.clone());
        resources_affected.push(new_resource);
    }

    // if the event is a transfer-like event, run the receiver's update first
    if let MaybeUndefined::Some(receiver_inventory) = &event.to_resource_inventoried_as {
        let inv_entry_hash: &EntryHash = receiver_inventory.as_ref();
        resources_affected.push(handle_update_economic_resource(
            &resource_entry_def_id,
            &get_latest_header_hash(inv_entry_hash.clone())?,
            ResourceInventoryType::ReceivingInventory, &event,
        )?);
    }
    // after receiver, run provider. This entry data will be returned in the response.
    if let MaybeUndefined::Some(provider_inventory) = &event.resource_inventoried_as {
        let inv_entry_hash: &EntryHash = provider_inventory.as_ref();
        resources_affected.push(handle_update_economic_resource(
            &resource_entry_def_id,
            &get_latest_header_hash(inv_entry_hash.clone())?,
            ResourceInventoryType::ProvidingInventory, &event,
        )?);
    }

    // Now that the resource updates have succeeded, write the event.
    // Note we ignore the revision ID because events can't be edited (only underwritten by subsequent events)
    // :TODO: rethinking this, it's probably the event that should be written first, and the resource
    // validation should eventually depend on an event already having been authored.
    let (revision_id, event_address, event_entry) = handle_create_economic_event(
        &entry_def_id, &process_entry_def_id, &agreement_entry_def_id,
        &event, match &resource_created {
            Some(data) => Some(data.1.to_owned()),
            None => None,
        },
    )?;

    // Link any affected resources to this event so that we can pull all the events which affect any resource
    for resource_data in resources_affected.iter() {
        let _ = create_index(
            &resource_entry_def_id, &(resource_data.1),
            &entry_def_id, &event_address,
            RESOURCE_AFFECTED_BY_EVENT_LINK_TAG, EVENT_AFFECTS_RESOURCE_LINK_TAG,
        )?;
    }

    match resource_created {
        Some((resource_revision_id, resource_addr, resource_entry)) => {
            construct_response_with_resource(
                &event_address, &revision_id, &event_entry, get_link_fields(&entry_def_id, &event_address)?,
                Some(resource_addr.clone()), &resource_revision_id, resource_entry, get_resource_link_fields(
                    &resource_entry_def_id, &entry_def_id, &process_entry_def_id, &resource_addr
                )?
            )
        },
        None => {
            // :TODO: pass results from link creation rather than re-reading
            construct_response(&event_address, &revision_id, &event_entry, get_link_fields(&entry_def_id, &event_address)?)
        },
    }
}

pub fn receive_get_economic_event<S>(entry_def_id: S, address: EventAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_get_economic_event(&entry_def_id, &address)
}

pub fn receive_update_economic_event<S>(entry_def_id: S, event: EconomicEventUpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    handle_update_economic_event(&entry_def_id, event)
}

pub fn receive_delete_economic_event<S>(entry_def_id: S, process_entry_def_id: S, agreement_entry_def_id: S, address: RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    handle_delete_economic_event(&entry_def_id, &process_entry_def_id, &agreement_entry_def_id, &address)
}

pub fn receive_get_all_economic_events<S>(entry_def_id: S) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_get_all_economic_events(&entry_def_id)
}

pub fn receive_query_events<S>(entry_def_id: S, params: QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    handle_query_events(&entry_def_id, &params)
}

// API logic handlers

fn handle_create_economic_event<S>(
    entry_def_id: S, process_entry_def_id: S, agreement_entry_def_id: S,
    event: &EconomicEventCreateRequest, resource_address: Option<ResourceAddress>,
) -> RecordAPIResult<(RevisionHash, EventAddress, EntryData)>
    where S: AsRef<str>
{
    let (revision_id, base_address, entry_resp): (_, EventAddress, EntryData) = create_record(
        &entry_def_id,
        match resource_address {
            Some(addr) => event.with_inventoried_resource(&addr),
            None => event.to_owned(),
        }
    )?;

    // handle link fields
    // :TODO: propagate errors
    if let EconomicEventCreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = event {
        let _results = create_index(
            &entry_def_id, &base_address,
            &process_entry_def_id, input_of,
            EVENT_INPUT_OF_LINK_TAG, PROCESS_EVENT_INPUTS_LINK_TAG,
        )?;
    };
    if let EconomicEventCreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = event {
        let _results = create_index(
            &entry_def_id, &base_address,
            &process_entry_def_id, output_of,
            EVENT_OUTPUT_OF_LINK_TAG, PROCESS_EVENT_OUTPUTS_LINK_TAG,
        )?;
    };
    if let EconomicEventCreateRequest { realization_of: MaybeUndefined::Some(realization_of), .. } = event {
        let _results = create_remote_index(
            None, // :TODO: map Agreement cell ID from `realization_of`
            "economic_event_idx".into(), "index_events".into(), None, // :TODO:
            &entry_def_id, base_address.as_ref(),
            &agreement_entry_def_id, vec![(realization_of.as_ref()).clone()].as_slice(),
            EVENT_REALIZATION_OF_LINK_TAG, AGREEMENT_EVENTS_LINK_TAG,
        )?;
    };

    Ok((revision_id, base_address, entry_resp))
}

/// Handle creation of new resources via events + resource metadata
///
fn handle_create_economic_resource<S>(
    resource_entry_def_id: S, resource_specification_entry_def_id: S,
    economic_resource: &EconomicResourceCreateRequest, event: &EconomicEventCreateRequest,
) -> RecordAPIResult<(RevisionHash, ResourceAddress, EconomicResourceData)>
    where S: AsRef<str>
{
    // :TODO: move this assertion to validation callback
    if let MaybeUndefined::Some(_sent_inventory_id) = &event.resource_inventoried_as {
        panic!("cannot create a new EconomicResource and specify an inventoried resource ID in the same event");
    }

    let params: ResourceCreationPayload = resource_creation(&event.with_inventory_type(ResourceInventoryType::ProvidingInventory), &economic_resource);
    let (revision_id, base_address, entry_resp): (_, ResourceAddress, EconomicResourceData) = create_record(&resource_entry_def_id, params.clone())?;

    let resource_params = params.get_resource_params();

    // :NOTE: this will always run- resource without a specification ID would fail entry validation (implicit in the above)
    if let Some(conforms_to) = params.get_resource_specification_id() {
        let _results = create_index(
            &resource_entry_def_id, &base_address,
            &resource_specification_entry_def_id, &conforms_to,
            RESOURCE_CONFORMS_TO_LINK_TAG, RESOURCE_SPECIFICATION_CONFORMING_RESOURCE_LINK_TAG,
        )?;
    }
    if let Some(contained_in) = resource_params.get_contained_in() {
        let _results = create_index(
            &resource_entry_def_id, &base_address,
            &resource_entry_def_id, &contained_in,
            RESOURCE_CONTAINED_IN_LINK_TAG, RESOURCE_CONTAINS_LINK_TAG,
        )?;
    };

    Ok((revision_id, base_address, entry_resp))
}

fn handle_get_economic_event<S>(entry_def_id: S, address: &EventAddress) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let (revision, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_>(&entry_def_id, address.as_ref())?;
    construct_response(&base_address, &revision, &entry, get_link_fields(&entry_def_id, address)?)
}

fn handle_update_economic_event<S>(entry_def_id: S, event: EconomicEventUpdateRequest) -> RecordAPIResult<ResponseData>
    where S: AsRef<str>
{
    let address = event.get_revision_id().to_owned();
    let (revision_id, identity_address, new_entry, _prev_entry): (_, EventAddress, EntryData, EntryData) = update_record(&entry_def_id, &address, event)?;

    // :TODO: optimise this- should pass results from `replace_direct_index` instead of retrieving from `get_link_fields` where updates
    construct_response(&identity_address, &revision_id, &new_entry, get_link_fields(&entry_def_id, &identity_address)?)
}

/// Handle alteration of existing resources via events
///
fn handle_update_economic_resource<S>(entry_def_id: S, resource_addr: &RevisionHash, inventory_type: ResourceInventoryType, event: &EconomicEventCreateRequest) -> RecordAPIResult<(RevisionHash, ResourceAddress, EconomicResourceData)>
    where S: AsRef<str>
{
    let context_event = event.with_inventory_type(inventory_type);

    let (revision_id, identity_address, new_entry, _prev_entry): (_, ResourceAddress, EconomicResourceData, EconomicResourceData) = update_record(&entry_def_id, &resource_addr.to_owned(), context_event)?;

    Ok((revision_id, identity_address, new_entry))
}

fn handle_delete_economic_event<S>(entry_def_id: S, process_entry_def_id: S, agreement_entry_def_id: S, revision_id: &RevisionHash) -> RecordAPIResult<bool>
    where S: AsRef<str>
{
    // read any referencing indexes
    let (base_address, entry) = read_record_entry_by_header::<EntryData, EntryStorage, _>(revision_id)?;

    // handle link fields
    if let Some(process_address) = entry.input_of {
        let _results = delete_index(
            &entry_def_id, &base_address,
            &process_entry_def_id, &process_address,
            &EVENT_INPUT_OF_LINK_TAG, &PROCESS_EVENT_INPUTS_LINK_TAG,
        )?;
    }
    if let Some(process_address) = entry.output_of {
        let _results = delete_index(
            &entry_def_id, &base_address,
            &process_entry_def_id, &process_address,
            &EVENT_OUTPUT_OF_LINK_TAG, &PROCESS_EVENT_OUTPUTS_LINK_TAG,
        );
    }
    if let Some(agreement_address) = entry.realization_of {
        let _results = update_remote_index(
            None, // :TODO: wire Agreement CellId from `realization_of`
            "event_idx".into(), "index_events".into(), None, // :TODO:
            &entry_def_id, base_address.as_ref(),
            &agreement_entry_def_id,
            vec![].as_slice(), vec![agreement_address.clone()].as_slice(),
            EVENT_REALIZATION_OF_LINK_TAG, AGREEMENT_EVENTS_LINK_TAG,
        );
    }

    // :TODO: handle cleanup of foreign key fields? (fulfillment, satisfaction)
    // May not be needed due to cross-record deletion validation logic.

    // delete entry last as it must be present in order for links to be removed
    delete_record::<EntryStorage>(revision_id)
}

fn handle_get_all_economic_events<S>(entry_def_id: S) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let entries_result = query_root_index::<EntryData, EntryStorage, _,_>(&entry_def_id)?;

    Ok(handle_list_output(entry_def_id, entries_result)?.iter().cloned()
        .filter_map(Result::ok)
        .collect()
    )
}

fn handle_query_events<S>(entry_def_id: S, _params: &QueryParams) -> RecordAPIResult<Vec<ResponseData>>
    where S: AsRef<str>
{
    let entries_result: RecordAPIResult<Vec<RecordAPIResult<(RevisionHash, EventAddress, EntryData)>>> = Err(DataIntegrityError::EmptyQuery);

    // :TODO: implement proper AND search rather than exclusive operations
    /*
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
    */

    Ok(handle_list_output(entry_def_id, entries_result?)?.iter().cloned()
        .filter_map(Result::ok)
        .collect()
    )
}

fn handle_list_output<S>(entry_def_id: S, entries_result: Vec<RecordAPIResult<(RevisionHash, EventAddress, EntryData)>>) -> RecordAPIResult<Vec<RecordAPIResult<ResponseData>>>
    where S: AsRef<str>
{
    Ok(entries_result.iter()
        .cloned()
        .filter_map(Result::ok)
        .map(|(revision_id, entry_base_address, entry)| {
            construct_response(
                &entry_base_address, &revision_id, &entry,
                get_link_fields(&entry_def_id, &entry_base_address)?,
            )
        })
        .collect()
    )
}

/**
 * Create response from input DHT primitives
 *
 * :TODO: determine if possible to construct `Response` with refs to fields of `e`, rather than cloning memory
 */
pub fn construct_response_with_resource<'a>(
    event_address: &EventAddress,
    revision_id: &RevisionHash,
    event: &EntryData, (
        fulfillments,
        satisfactions,
    ): (
        Vec<FulfillmentAddress>,
        Vec<SatisfactionAddress>,
    ),
    resource_address: Option<ResourceAddress>,
    resource_revision_id: &RevisionHash,
    resource: EconomicResourceData, (
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
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        economic_event: Response {
            id: event_address.to_owned(),
            revision_id: revision_id.to_owned(),
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
            fulfills: fulfillments.to_owned(),
            satisfies: satisfactions.to_owned(),
        },
        economic_resource: match resource_address {
            Some(addr) => Some(construct_resource_response(&addr, &resource_revision_id, &resource, (contained_in, stage, state, contains))?),
            None => None,
        },
    })
}

// Same as above, but omits EconomicResource object
pub fn construct_response<'a>(
    address: &EventAddress, revision_id: &RevisionHash, e: &EntryData, (
        fulfillments,
        satisfactions,
    ): (
        Vec<FulfillmentAddress>,
        Vec<SatisfactionAddress>,
    )
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        economic_event: Response {
            id: address.to_owned(),
            revision_id: revision_id.to_owned(),
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
            fulfills: fulfillments.to_owned(),
            satisfies: satisfactions.to_owned(),
        },
        economic_resource: None,
    })
}

// @see construct_response
pub fn get_link_fields<'a, S>(entry_def_id: S, event: &EventAddress) -> RecordAPIResult<(
    Vec<FulfillmentAddress>,
    Vec<SatisfactionAddress>,
)>
    where S: AsRef<str>
{
    Ok((
        read_index(&entry_def_id, event, &EVENT_FULFILLS_LINK_TAG)?,
        read_index(&entry_def_id, event, &EVENT_SATISFIES_LINK_TAG)?,
    ))
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
