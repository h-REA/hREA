/**
 * Holo-REA 'economic event' zome library API
 *
 * Contains helper methods that can be used to manipulate economic event data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk_records::{
    RecordAPIResult, OtherCellResult, MaybeUndefined, SignedActionHashed,
    rpc::{
        call_local_zome_method,
    },
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

pub use hc_zome_rea_economic_event_storage_consts::*;

use hc_zome_rea_economic_event_zome_api::*;
use hc_zome_rea_economic_event_storage::*;
use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EconomicEventCreateRequest,
    UpdateRequest as EconomicEventUpdateRequest,
};
use hc_zome_rea_economic_resource_rpc::{ CreationPayload as ResourceCreationPayload };

use hc_zome_rea_economic_resource_storage::{
    EntryData as EconomicResourceData,
};
use hc_zome_rea_economic_resource_lib::{
    construct_response_record as construct_resource_response,
    get_link_fields as get_resource_link_fields,
};
use hc_zome_rea_economic_event_integrity::*;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.economic_event.index_zome)
}

/// Properties accessor for zome config.
fn read_economic_resource_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_event.economic_resource_index_zome
}

/// Properties accessor for zome config.
fn read_agent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_event.agent_index_zome
}

/// Trait object defining the default ValueFlows EconomicResource zome API.
/// 'Permissable' denotes the interface as a highly-permissable one, where little
/// validation on entry contents is performed.
pub struct EconomicEventZomePermissableDefault;

impl API for EconomicEventZomePermissableDefault {
    type S = &'static str;

    fn create_economic_event(
        entry_def_id: Self::S,
        event: EconomicEventCreateRequest, new_inventoried_resource: Option<ResourceCreateRequest>
    ) -> RecordAPIResult<ResponseData> {
        let mut resources_affected: Vec<(SignedActionHashed, EconomicResourceAddress, EconomicResourceData, EconomicResourceData)> = vec![];
        let mut resource_created: Option<(SignedActionHashed, EconomicResourceAddress, EconomicResourceData)> = None;

        // if the event observes a new resource, create that resource & return it in the response
        if let Some(economic_resource) = new_inventoried_resource {
            let new_resource = handle_create_inventory_from_event(
                &economic_resource, &event,
            )?;
            resource_created = Some(new_resource.clone());
            resources_affected.push((new_resource.0, new_resource.1, new_resource.2.clone(), new_resource.2));
        }

        // update any linked resources affected by the event
        resources_affected.append(&mut handle_update_resource_inventory(&event)?);

        // Now that the resource updates have succeeded, write the event.
        // Note we ignore the revision ID because events can't be edited (only underwritten by subsequent events)
        // :TODO: rethinking this, it's probably the event that should be written first, and the resource
        // validation should eventually depend on an event already having been authored.
        let (meta, event_address, event_entry) = handle_create_economic_event_record(
            &entry_def_id,
            &event, match &resource_created {
                Some(data) => Some(data.1.to_owned()),
                None => None,
            },
        )?;

        // Link any affected resources to this event so that we can pull all the events which affect any resource
        for resource_data in resources_affected.iter() {
            let e = create_index!(economic_event.affects(resource_data.1), economic_resource.affected_by(&event_address));
            hdk::prelude::debug!("create_economic_event::affects index {:?}", e);
        }

        match resource_created {
            Some((resource_meta, resource_addr, resource_entry)) => {
                construct_response_with_resource(
                    &event_address, &meta, &event_entry, get_link_fields(&event_address)?,
                    Some(resource_addr.clone()), &resource_meta, resource_entry, get_resource_link_fields(&resource_addr)?
                )
            },
            None => {
                // :TODO: pass results from link creation rather than re-reading
                construct_response(&event_address, &meta, &event_entry, get_link_fields(&event_address)?)
            },
        }
    }

    fn get_economic_event(address: EconomicEventAddress) -> RecordAPIResult<ResponseData> {
        let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(address.as_ref())?;
        construct_response(&base_address, &meta, &entry, get_link_fields(&address)?)
    }

    fn update_economic_event(event: EconomicEventUpdateRequest) -> RecordAPIResult<ResponseData> {
        let address = event.get_revision_id().to_owned();
        let (meta, identity_address, new_entry, _prev_entry): (_, EconomicEventAddress, EntryData, EntryData) = update_record(&address, event)?;

        // :TODO: optimise this- should pass results from `replace_direct_index` instead of retrieving from `get_link_fields` where updates
        construct_response(&identity_address, &meta, &new_entry, get_link_fields(&identity_address)?)
    }

    fn delete_economic_event(revision_id: ActionHash) -> RecordAPIResult<bool> {
        // read any referencing indexes
        let (_meta, base_address, entry) = read_record_entry_by_action::<EntryData, EntryStorage, _>(&revision_id)?;

        // handle link fields
        if let Some(process_address) = entry.input_of {
            let e = update_index!(economic_event.input_of.not(&vec![process_address.to_owned()]), process.observed_inputs(&base_address));
            hdk::prelude::debug!("delete_economic_event::input_of index {:?}", e);
        }
        if let Some(process_address) = entry.output_of {
            let e = update_index!(economic_event.output_of.not(&vec![process_address.to_owned()]), process.observed_outputs(&base_address));
            hdk::prelude::debug!("delete_economic_event::output_of index {:?}", e);
        }
        if let Some(agreement_address) = entry.realization_of {
            let e = update_index!(economic_event.realization_of.not(&vec![agreement_address.to_owned()]), agreement.economic_events(&base_address));
            hdk::prelude::debug!("delete_economic_event::realization_of index {:?}", e);
        }
        let e = update_index!(economic_event.provider.not(&vec![entry.provider]), agent.economic_events_as_provider(&base_address));
        hdk::prelude::debug!("delete_economic_event::provider index {:?}", e);
        let e = update_index!(economic_event.receiver.not(&vec![entry.receiver]), agent.economic_events_as_receiver(&base_address));
        hdk::prelude::debug!("delete_economic_event::receiver index {:?}", e);

        // :TODO: handle cleanup of foreign key fields? (fulfillment, satisfaction)
        // May not be needed due to cross-record deletion validation logic.

        // delete entry last as it must be present in order for links to be removed
        delete_record::<EntryStorage>(&revision_id)
    }
}

// API logic handlers

/// Properties accessor for zome config.
fn read_economic_event_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.economic_event.index_zome)
}
/// Properties accessor for zome config.
///
/// :TODO: should this be configurable as an array, to allow shared process planning spaces to be driven by multiple event logs?
///
fn read_process_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_event.process_index_zome
}

fn read_agreement_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_event.agreement_index_zome
}

fn handle_create_economic_event_record<S>(entry_def_id: S, event: &EconomicEventCreateRequest, resource_address: Option<EconomicResourceAddress>,
) -> RecordAPIResult<(SignedActionHashed, EconomicEventAddress, EntryData)>
    where S: AsRef<str> + std::fmt::Display,
{
    let (meta, base_address, entry_resp): (_, EconomicEventAddress, EntryData) = create_record(
        read_index_zome,
        &entry_def_id,
        match resource_address {
            Some(addr) => event.with_inventoried_resource(&addr),
            None => event.to_owned(),
        }
    )?;

    // handle link fields
    // :TODO: handle errors better https://github.com/h-REA/hREA/issues/264
    let e1 = create_index!(economic_event.provider(event.provider), agent.economic_events_as_provider(&base_address))?;
    let e2 = create_index!(economic_event.receiver(event.receiver), agent.economic_events_as_receiver(&base_address))?;
    hdk::prelude::debug!("handle_create_economic_event::provider index {:?}", e1);
    hdk::prelude::debug!("handle_create_economic_event::receiver index {:?}", e2);

    if let EconomicEventCreateRequest { input_of: MaybeUndefined::Some(input_of), .. } = event {
        let e = create_index!(economic_event.input_of(input_of), process.observed_inputs(&base_address));
        hdk::prelude::debug!("handle_create_economic_event_record::input_of index {:?}", e);
      };
      if let EconomicEventCreateRequest { output_of: MaybeUndefined::Some(output_of), .. } = event {
        let e = create_index!(economic_event.output_of(output_of), process.observed_outputs(&base_address));
        hdk::prelude::debug!("handle_create_economic_event_record::output_of index {:?}", e);
      };
      if let EconomicEventCreateRequest { realization_of: MaybeUndefined::Some(realization_of), .. } = event {
        let e = create_index!(economic_event.realization_of(realization_of), agreement.economic_events(&base_address));
        hdk::prelude::debug!("handle_create_economic_event_record::realization_of index {:?}", e);
    };

    Ok((meta, base_address, entry_resp))
}

/// Properties accessor for zome config.
///
/// :TODO: should this be configurable as an array, to allow multiple inventories to be driven by the same event log?
///
fn read_resource_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_event.economic_resource_zome
}

/// Handle creation of new resources via events + resource metadata
///
fn handle_create_inventory_from_event(
    economic_resource: &ResourceCreateRequest, event: &CreateRequest,
) -> OtherCellResult<(SignedActionHashed, EconomicResourceAddress, EconomicResourceData)>
{
    Ok(call_local_zome_method(
        read_resource_zome,
        INVENTORY_CREATION_API_METHOD.to_string(),
        resource_creation(&event, &economic_resource),
    )?)
}

fn resource_creation(event: &CreateRequest, resource: &ResourceCreateRequest) -> ResourceCreationPayload {
    ResourceCreationPayload {
        event: event.to_owned(),
        resource: resource.to_owned(),
    }
}

/// Handle alteration of existing resources via events
///
fn handle_update_resource_inventory(
    event: &EconomicEventCreateRequest,
) -> RecordAPIResult<Vec<(SignedActionHashed, EconomicResourceAddress, EconomicResourceData, EconomicResourceData)>>
{
    Ok(call_local_zome_method(
        read_resource_zome,
        INVENTORY_UPDATE_API_METHOD.to_string(),
        event,
    )?)
}

/**
 * Create response from input DHT primitives
 *
 * :TODO: determine if possible to construct `Response` with refs to fields of `e`, rather than cloning memory
 */
pub fn construct_response_with_resource<'a>(
    event_address: &EconomicEventAddress,
    meta: &SignedActionHashed,
    event: &EntryData, (
        fulfillments,
        satisfactions,
    ): (
        Vec<FulfillmentAddress>,
        Vec<SatisfactionAddress>,
    ),
    resource_address: Option<EconomicResourceAddress>,
    resource_meta: &SignedActionHashed,
    resource: EconomicResourceData, (
        contained_in,
        stage,
        state,
        contains,
     ): (
        Option<EconomicResourceAddress>,
        Option<ProcessSpecificationAddress>,
        Option<ActionId>,
        Vec<EconomicResourceAddress>,
    ),
) -> RecordAPIResult<ResponseData> {
    Ok(ResponseData {
        economic_event: Response {
            id: event_address.to_owned(),
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
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
            Some(addr) => Some(construct_resource_response(&addr, &resource_meta, &resource, (contained_in, stage, state, contains))?),
            None => None,
        },
    })
}

// Same as above, but omits EconomicResource object
pub fn construct_response<'a>(
    address: &EconomicEventAddress, meta: &SignedActionHashed, e: &EntryData, (
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
            revision_id: meta.as_hash().to_owned(),
            meta: read_revision_metadata_abbreviated(meta)?,
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
pub fn get_link_fields(event: &EconomicEventAddress) -> RecordAPIResult<(
    Vec<FulfillmentAddress>,
    Vec<SatisfactionAddress>,
)> {
    Ok((
        read_index!(economic_event(event).fulfills)?,
        read_index!(economic_event(event).satisfies)?,
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
