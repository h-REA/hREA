/**
 * Holo-REA 'economic resource' zome library API
 *
 * Contains helper methods that can be used to manipulate economic resource data
 * structures in either the local Holochain zome, or a separate DNA-local zome.
 *
 * @package Holo-REA
 */
use paste::paste;
use hdk_records::{
    DataIntegrityError, RecordAPIResult, MaybeUndefined,
    records::{
        get_latest_action_hash,
        create_record,
        read_record_entry,
        update_record,
    },
    metadata::read_revision_metadata_abbreviated,
    EntryHash, SignedActionHashed,
};
use hdk_semantic_indexes_client_lib::*;

use vf_attributes_hdk::{
    EconomicResourceAddress,
    EconomicEventAddress,
    ActionId,
    ProcessSpecificationAddress,
};

pub use hc_zome_rea_economic_resource_storage_consts::*;
pub use hc_zome_rea_economic_event_storage_consts::{EVENT_ENTRY_TYPE};
pub use hc_zome_rea_process_storage_consts::{PROCESS_ENTRY_TYPE};
pub use hc_zome_rea_resource_specification_storage_consts::{ECONOMIC_RESOURCE_SPECIFICATION_ENTRY_TYPE};

use hc_zome_rea_economic_resource_zome_api::*;
use hc_zome_rea_economic_resource_storage::*;
use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_process_storage::{
    EntryData as ProcessData,
    EntryStorage as ProcessStorage,
};
use hc_zome_rea_economic_event_storage::{EntryData as EventData, EntryStorage as EventStorage};
use hc_zome_rea_economic_event_rpc::{
    ResourceResponse as Response,
    ResourceResponseData as ResponseData,
    ResourceInventoryType,
    CreateRequest as EventCreateRequest,
};
use hc_zome_rea_economic_resource_integrity::*;

// :SHONK: needed to re-export for zome `entry_defs()` where macro-assigned defs are overridden
pub use hdk_records::CAP_STORAGE_ENTRY_DEF_ID;

/// Trait object defining the default ValueFlows EconomicResource zome API.
/// 'Permissable' denotes the interface as a highly-permissable one, where little
/// validation on entry contents is performed.
pub struct EconomicResourceZomePermissableDefault;

/// properties accessor for zome config
fn read_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.economic_resource.index_zome)
}

impl API for EconomicResourceZomePermissableDefault {
    type S = &'static str;

    /// Handle creation of new resources via events + resource metadata.
    ///
    /// :WARNING: Should only ever be wired up as the dependency of an EconomicEvent zome.
    ///           API is not for direct use and could lead to an inconsistent database state.
    ///
    /// :TODO: assess whether this should use the same standardised API format as external endpoints
    ///
    fn create_inventory_from_event(resource_entry_def_id: Self::S, params: CreationPayload) -> RecordAPIResult<(SignedActionHashed, EconomicResourceAddress, EntryData)>
    {
        let event_params = params.get_event_params().clone();
        let resource_params = params.get_resource_params().clone();
        let resource_spec = params.get_resource_specification_id();
        // :TODO: move this assertion to validation callback
        if let MaybeUndefined::Some(_sent_inventory_id) = event_params.resource_inventoried_as {
            return Err(DataIntegrityError::RemoteRequestError("cannot create a new EconomicResource and specify an inventoried resource ID in the same event".to_string()));
        }

        let (meta, base_address, entry_resp): (_, EconomicResourceAddress, EntryData) = create_record::<EntryTypes,_,_,_,_,_,_,_,_>(
            read_index_zome,
            &resource_entry_def_id,
            params.with_inventory_type(ResourceInventoryType::ProvidingInventory),  // inventories can only be inited by their owners initially
        )?;

        // :NOTE: this will always run- resource without a specification ID would fail entry validation (implicit in the above)
        if let Some(conforms_to) = resource_spec {
            let e = create_index!(economic_resource.conforms_to(conforms_to), resource_specification.conforming_resources(&base_address));
            hdk::prelude::debug!("create_inventory_from_event::conforms_to index {:?}", e);
          }
          if let Some(contained_in) = resource_params.get_contained_in() {
            let e = create_index!(economic_resource(&base_address).contained_in(&contained_in));
            hdk::prelude::debug!("create_inventory_from_event::contained_in index {:?}", e);
        };

        if entry_resp.primary_accountable.is_some() {
            let e = create_index!(economic_resource.primary_accountable(&event_params.receiver), agent.inventoried_economic_resources(&base_address));
            hdk::prelude::debug!("create_inventory_from_event::new_inventoried_resource::primary_accountable index {:?}", e);
        }

        Ok((meta, base_address, entry_resp))
    }

    fn get_economic_resource(address: EconomicResourceAddress) -> RecordAPIResult<ResponseData>
    {
        let (meta, base_address, entry) = read_record_entry::<EntryData, EntryStorage, _,_,_>(address.as_ref())?;
        construct_response(&base_address, &meta, &entry, get_link_fields(&address)?)
    }

    /// Handle update of resources by iterative reduction of event records over time.
    ///
    fn update_inventory_from_event(
        event: EventCreateRequest,
    ) -> RecordAPIResult<Vec<(SignedActionHashed, EconomicResourceAddress, EntryData, EntryData)>>
    {
        let mut resources_affected: Vec<(SignedActionHashed, EconomicResourceAddress, EntryData, EntryData)> = vec![];

        // if the event is a transfer-like event, run the receiver's update first
        if let MaybeUndefined::Some(receiver_inventory) = &event.to_resource_inventoried_as {
            let inv_entry_hash: &EntryHash = receiver_inventory.as_ref();
            let (meta, resource_address, new_resource, prev_resource) = handle_update_inventory_resource(
                &get_latest_action_hash(inv_entry_hash.clone())?,   // :TODO: temporal reduction here! Should error on mismatch and return latest valid ID
                event.with_inventory_type(ResourceInventoryType::ReceivingInventory),
            )?;
            resources_affected.push((meta, resource_address.clone(), new_resource.clone(), prev_resource.clone()));
            if new_resource.primary_accountable != prev_resource.primary_accountable {
                let new_value = if let Some(val) = &new_resource.primary_accountable { vec![val.to_owned()] } else { vec![] };
                let prev_value = if let Some(val) = &prev_resource.primary_accountable { vec![val.to_owned()] } else { vec![] };
                let e = update_index!(
                    economic_resource
                        .primary_accountable(new_value.as_slice())
                        .not(prev_value.as_slice()),
                    agent.inventoried_economic_resources(&resource_address));
                hdk::prelude::debug!("update_economic_resource::to_resource_inventoried_as::primary_accountable index {:?}", e);
            }
        }
        // after receiver, run provider. This entry data will be returned in the response.
        if let MaybeUndefined::Some(provider_inventory) = &event.resource_inventoried_as {
            let inv_entry_hash: &EntryHash = provider_inventory.as_ref();
            resources_affected.push(handle_update_inventory_resource(
                &get_latest_action_hash(inv_entry_hash.clone())?,   // :TODO: temporal reduction here! Should error on mismatch and return latest valid ID
                event.with_inventory_type(ResourceInventoryType::ProvidingInventory),
            )?);
        }

        Ok(resources_affected)
    }

    fn update_economic_resource(resource: UpdateRequest) -> RecordAPIResult<ResponseData>
    {
        let address = resource.get_revision_id().clone();
        let (meta, identity_address, entry, prev_entry): (_,_, EntryData, EntryData) = update_record(&address, resource)?;

        // :TODO: this may eventually be moved to an EconomicEvent update, see https://lab.allmende.io/valueflows/valueflows/-/issues/637
        if entry.contained_in != prev_entry.contained_in {
            let now_contained = if let Some(contained) = &entry.contained_in { vec![contained.clone()] } else { vec![] };
            let prev_contained = if let Some(contained) = &prev_entry.contained_in { vec![contained.clone()] } else { vec![] };
            let e = update_index!(economic_resource(&identity_address).contained_in(now_contained.as_slice()).not(prev_contained.as_slice()));
            hdk::prelude::debug!("update_economic_resource::contained_in index {:?}", e);
        }


        // :TODO: optimise this- should pass results from `replace_direct_index` instead of retrieving from `get_link_fields` where updates
        construct_response(&identity_address, &meta, &entry, get_link_fields(&identity_address)?)
    }
}

/// Properties accessor for zome config
fn read_economic_resource_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.economic_resource.index_zome)
}

/// Properties accessor for zome config
fn read_resource_specification_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_resource.resource_specification_index_zome
}

fn handle_update_inventory_resource(
    resource_addr: &ActionHash,
    event: EventCreateRequest,
) -> RecordAPIResult<(SignedActionHashed, EconomicResourceAddress, EntryData, EntryData)>
{
    Ok(update_record(resource_addr, event)?)
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &EconomicResourceAddress, meta: &SignedActionHashed, e: &EntryData, (
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
        economic_resource: construct_response_record(address, meta, e, (contained_in, stage, state, contains))?
    })
}

/// Create response from input DHT primitives
pub fn construct_response_record<'a>(
    address: &EconomicResourceAddress, meta: &SignedActionHashed, e: &EntryData, (
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
) -> RecordAPIResult<Response> {
    Ok(Response {
        // entry fields
        id: address.to_owned(),
        name: e.name.to_owned(),
        revision_id: meta.as_hash().to_owned(),
        meta: read_revision_metadata_abbreviated(meta)?,
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
        primary_accountable: e.primary_accountable.to_owned(),

        // link fields
        contained_in: contained_in.to_owned(),
        contains: contains.to_owned(),
    })
}

fn read_agent_index_zome(conf: DnaConfigSlice) -> Option<String> {
    conf.economic_resource.agent_index_zome
}

// field list retrieval internals
// @see construct_response
pub fn get_link_fields<'a, S>(resource: &EconomicResourceAddress) -> RecordAPIResult<(
    Option<EconomicResourceAddress>,
    Option<ProcessSpecificationAddress>,
    Option<ActionId>,
    Vec<EconomicResourceAddress>,
)>
    where S: AsRef<str>
{
    Ok((
        read_index!(economic_resource(resource).contained_in)?.pop(),
        get_resource_stage(resource)?,
        get_resource_state(resource)?,
        read_index!(economic_resource(resource).contains)?,
    ))
}

fn get_resource_state(resource: &EconomicResourceAddress) -> RecordAPIResult<Option<ActionId>>
{
    let events: Vec<EconomicEventAddress> = get_affecting_events(resource)?;

    // grab the most recent "pass" or "fail" action
    Ok(events.iter()
        .fold(None, move |result, event| {
            // already found it, just fall through
            // :TODO: figure out the Rust STL method to abort on first Some() value
            if let Some(_) = result {
                return result;
            }

            let evt = read_record_entry::<EventData, EventStorage, _,_,_>(event.as_ref());
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

fn get_resource_stage(resource: &EconomicResourceAddress) -> RecordAPIResult<Option<ProcessSpecificationAddress>>
{
    let events: Vec<EconomicEventAddress> = get_affecting_events(resource)?;

    // grab the most recent event with a process output association
    Ok(events.iter()
        .fold(None, move |result, event| {
            // already found it, just fall through
            // :TODO: figure out the Rust STL method to abort on first Some() value
            if let Some(_) = result {
                return result;
            }

            let evt = read_record_entry::<EventData, EventStorage, _,_,_>(event.as_ref());
            match evt {
                Err(_) => result, // :TODO: this indicates some data integrity error
                Ok((_, _, entry)) => {
                    match &entry.output_of {
                        Some(output_of) => {
                            // get the associated process
                            let maybe_process_entry = read_record_entry::<ProcessData, ProcessStorage, _,_,_>(output_of.as_ref());
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
fn get_affecting_events(resource: &EconomicResourceAddress) -> RecordAPIResult<Vec<EconomicEventAddress>>
{
    read_index!(economic_resource(resource).affected_by)
}
