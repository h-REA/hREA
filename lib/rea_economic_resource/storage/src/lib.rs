/**
 * Holo-REA 'economic resource' zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_graph_helpers::{
    MaybeUndefined, OtherCellResult,
    generate_record_entry,
    record_interface::Updateable,
    rpc::call_zome_method,
};

use vf_core::measurement::*;
use vf_core::type_aliases::{
    ExternalURL,
    LocationAddress,
    ResourceSpecificationAddress,
    UnitId,
    ProductBatchAddress,
    ActionId,
};
use vf_actions::{ ActionEffect, ActionInventoryEffect, get_builtin_action };
use hc_zome_rea_resource_specification_rpc::{ResponseData as ResourceSpecificationResponse};

use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EventCreateRequest,
    ResourceInventoryType,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct EntryData {
    pub conforms_to: Option<ResourceSpecificationAddress>,
    pub classified_as: Option<Vec<ExternalURL>>,
    pub tracking_identifier: Option<String>,
    pub lot: Option<ProductBatchAddress>,
    pub image: Option<ExternalURL>,
    pub accounting_quantity: Option<QuantityValue>,
    pub onhand_quantity: Option<QuantityValue>,
    pub unit_of_effort: Option<UnitId>,
    pub current_location: Option<LocationAddress>,
    pub note: Option<String>,
}

impl EntryData {
    pub fn validate(&self) -> Result<(), String> {
        if !(self.classified_as.is_some() || self.conforms_to.is_some()) {
            return Err("EconomicResource must have either a specification or classification".into());
        }
        Ok(())
    }
}

generate_record_entry!(EntryData, EntryStorage);

//---------------- CREATE ----------------

/// Handles create operations via observed event resource inspection parameter
/// @see https://github.com/holo-rea/holo-rea/issues/65
impl From<CreationPayload> for EntryData
{
    fn from(t: CreationPayload) -> EntryData {
        let conforming = t.get_resource_specification_id();
        let r = t.resource;
        let e = t.event;
        EntryData {
            conforms_to: conforming.clone(),
            classified_as: if e.resource_classified_as == MaybeUndefined::Undefined { None } else { e.resource_classified_as.to_owned().to_option() },
            tracking_identifier: if r.tracking_identifier == MaybeUndefined::Undefined { None } else { r.tracking_identifier.to_owned().to_option() },
            lot: if r.lot == MaybeUndefined::Undefined { None } else { r.lot.to_owned().to_option() },
            image: if r.image == MaybeUndefined::Undefined { None } else { r.image.to_owned().to_option() },
            accounting_quantity: match e.resource_quantity.to_owned() {
                MaybeUndefined::Some(resource_quantity) => update_quantity(
                    Some(QuantityValue::new(0.0, resource_quantity.get_unit())), // :TODO: pull from e.resource_conforms_to.unit_of_effort if present
                    e.resource_quantity.to_owned(),
                    &e.action,
                    ResourceValueType::AccountingValue,
                    match &e.target_inventory_type {
                        Some(inventory_type) => inventory_type.to_owned(),
                        None => panic!("Developer error: EconomicEvent inventory type must be provided when creating EconomicResource!"),
                    },
                ),
                _ => None,
            },
            onhand_quantity: match e.resource_quantity.to_owned() {
                MaybeUndefined::Some(resource_quantity) => update_quantity(
                    Some(QuantityValue::new(0.0, resource_quantity.get_unit())), // :TODO: pull from e.resource_conforms_to.unit_of_effort if present
                    e.resource_quantity.to_owned(),
                    &e.action,
                    ResourceValueType::OnhandValue,
                    match &e.target_inventory_type {
                        Some(inventory_type) => inventory_type.to_owned(),
                        None => panic!("Developer error: EconomicEvent inventory type must be provided when updating EconomicResource!"),
                    },
                ),
                _ => None,
            },
            unit_of_effort: match conforming {
                Some(conforms_to_spec) => get_default_unit_for_specification(conforms_to_spec),
                None => None,
            },
            current_location: if r.current_location == MaybeUndefined::Undefined { None } else { r.current_location.to_owned().to_option() },
            note: if r.note == MaybeUndefined::Undefined { None } else { r.note.clone().into() },
        }
    }
}

/// I/O struct for forwarding records to other DNAs via zome API
#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetSpecificationRequest {
    pub address: ResourceSpecificationAddress,
}

fn get_default_unit_for_specification(specification_id: ResourceSpecificationAddress) -> Option<UnitId> {
    let spec_data: OtherCellResult<ResourceSpecificationResponse> = call_zome_method(
        // :TODO: pass appropriate params
        None,
        "resource_specification".into(),
        "get_resource_specification".into(),
        None, // :TODO:
        GetSpecificationRequest { address: specification_id.to_owned().into() },
    );

    match spec_data {
        Ok(spec_response) => spec_response.resource_specification.default_unit_of_effort,
        Err(_) => None,     // :TODO: error handling
    }
}

//---------------- UPDATE ----------------

/// Handles update operations for correcting data entry errors
impl Updateable<UpdateRequest> for EntryData {
    fn update_with(&self, e: UpdateRequest) -> EntryData {
        EntryData {
            conforms_to: self.conforms_to.to_owned(),
            classified_as: if e.classified_as == MaybeUndefined::Undefined { self.classified_as.to_owned() } else { e.classified_as.to_owned().to_option() },
            tracking_identifier: self.tracking_identifier.to_owned(),
            lot: self.lot.to_owned(),
            image: if e.image == MaybeUndefined::Undefined { self.image.to_owned() } else { e.image.to_owned().to_option() },
            accounting_quantity: self.accounting_quantity.to_owned(),
            onhand_quantity: self.onhand_quantity.to_owned(),
            unit_of_effort: if e.unit_of_effort == MaybeUndefined::Undefined { self.unit_of_effort.to_owned() } else { e.unit_of_effort.to_owned().to_option() },
            current_location: self.current_location.to_owned(),
            note: if e.note == MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().to_option() },
        }
    }
}

/// Handle update operations by observed events
///
/// :WARNING: we presume the event has already been determined to relate to the
/// resource, and this method will panic if that is not the case.
///
/// Currently it is only called within `hdk_graph_helpers::record_helpers::update_record`,
/// where this check is already implicitly performed.
///
impl Updateable<EventCreateRequest> for EntryData {
    fn update_with(&self, e: EventCreateRequest) -> EntryData {
        EntryData {
            conforms_to: self.conforms_to.to_owned(),
            classified_as: {
                if let MaybeUndefined::Some(classified_as) = e.resource_classified_as.to_owned() {
                    // get previous list as starting set
                    let mut results: Vec<String> = if let Some(already_classified) = &self.classified_as {
                        // :NOTE: need to convert ExternalURL to String in order to perform the de-duplication
                        already_classified.to_owned().into_iter().map(|url| { url.as_ref().clone() }).collect()
                    } else {
                        vec![]
                    };
                    // add new list
                    results.append(&mut classified_as.into_iter().map(|url| { url.as_ref().clone() }).collect());
                    // de-duplicate values
                    results.sort_unstable();
                    results.dedup();
                    // convert output back to ExternalURL type wrappers
                    Some(results.into_iter().map(|url| { url.into() }).collect())
                } else {
                    self.classified_as.to_owned()
                }
            },
            tracking_identifier: self.tracking_identifier.to_owned(),
            lot: self.lot.to_owned(),
            image: self.image.to_owned(),
            accounting_quantity: update_quantity(
                self.accounting_quantity.to_owned(), e.resource_quantity.to_owned(),
                &e.action, ResourceValueType::AccountingValue, match &e.target_inventory_type {
                    Some(inventory_type) => inventory_type.to_owned(),
                    None => panic!("Developer error: EconomicEvent inventory type must be provided when updating EconomicResource!"),
                },
            ),
            onhand_quantity: update_quantity(
                self.onhand_quantity.to_owned(), e.resource_quantity.to_owned(),
                &e.action, ResourceValueType::OnhandValue, match &e.target_inventory_type {
                    Some(inventory_type) => inventory_type.to_owned(),
                    None => panic!("Developer error: EconomicEvent inventory type must be provided when updating EconomicResource!"),
                },
            ),
            unit_of_effort: self.unit_of_effort.to_owned(), // :TODO: pull from e.resource_conforms_to.unit_of_effort
            current_location: if e.get_action() == "move" {
                if let MaybeUndefined::Some(at_location) = e.get_location() {
                    Some(at_location)
                } else {
                    self.current_location.to_owned()
                }
            } else { self.current_location.to_owned() },
            note: self.note.to_owned(),
        }
    }
}

/// Encapsulates the logic for updating EconomicResource quantities in response to event triggers
fn update_quantity(
    current_val: Option<QuantityValue>,
    event_val: MaybeUndefined<QuantityValue>,
    action: &ActionId,
    which_qty_type: ResourceValueType,
    which_inventory_type: ResourceInventoryType,
) -> Option<QuantityValue> {
    if None == current_val {
        return None;
    }
    if MaybeUndefined::None == event_val || MaybeUndefined::Undefined == event_val {
        return current_val;
    }
    let current = current_val.unwrap();
    let event_qty = event_val.unwrap();

    let action_to_perform = get_event_action(action, which_qty_type, which_inventory_type);

    match action_to_perform {
        ActionInventoryEffect::NoEffect => Some(current),
        ActionInventoryEffect::Increment => Some(add(current, event_qty)),
        ActionInventoryEffect::Decrement => Some(subtract(current, event_qty)),
    }
}

enum ResourceValueType {
    AccountingValue,
    OnhandValue,
}

/// Determines the `ActionInventoryEffect` to apply to a resource, based on the input event
/// action type, the type of inventory quantity ("accounting" or "on hand"),
/// and the side of the event that the resource is on (providing or receiving).
fn get_event_action(
    action: &ActionId,
    which_qty_type: ResourceValueType,
    which_inventory_type: ResourceInventoryType,
) -> ActionInventoryEffect {
    let action_str: &str = (*action).as_ref();

    match get_builtin_action(action_str) {
        Some(action_obj) => match &action_str[..] {
            // 'transfer-custody' updates onHand but not Accounting
            "transfer-custody" => match which_qty_type {
                ResourceValueType::AccountingValue => ActionInventoryEffect::NoEffect,
                ResourceValueType::OnhandValue => match which_inventory_type {
                    ResourceInventoryType::ProvidingInventory => ActionInventoryEffect::Decrement,
                    ResourceInventoryType::ReceivingInventory => ActionInventoryEffect::Increment,
                },
            },
            // 'transfer-all-rights' updates Accounting but not onHand
            "transfer-all-rights" => match which_qty_type {
                ResourceValueType::AccountingValue => match which_inventory_type {
                    ResourceInventoryType::ProvidingInventory => ActionInventoryEffect::Decrement,
                    ResourceInventoryType::ReceivingInventory => ActionInventoryEffect::Increment,
                },
                ResourceValueType::OnhandValue => ActionInventoryEffect::NoEffect,
            },
            // 'transfer' is a full swap
            "transfer" => match which_inventory_type {
                ResourceInventoryType::ProvidingInventory => ActionInventoryEffect::Decrement,
                ResourceInventoryType::ReceivingInventory => ActionInventoryEffect::Increment,
            },
            // 'normal' action, just work from the configured effect and reverse for the receiver
            _ => {
                match which_inventory_type {
                    ResourceInventoryType::ProvidingInventory => match action_obj.resource_effect {
                        ActionEffect::DecrementIncrement => ActionInventoryEffect::Decrement,
                        ActionEffect::NoEffect => ActionInventoryEffect::NoEffect,
                        ActionEffect::Increment => ActionInventoryEffect::Increment,
                        ActionEffect::Decrement => ActionInventoryEffect::Decrement,
                    },
                    ResourceInventoryType::ReceivingInventory => match action_obj.resource_effect {
                        ActionEffect::DecrementIncrement => ActionInventoryEffect::Increment,
                        ActionEffect::NoEffect => ActionInventoryEffect::NoEffect,
                        ActionEffect::Increment => ActionInventoryEffect::Decrement,
                        ActionEffect::Decrement => ActionInventoryEffect::Increment,
                    },
                }
            }
        },
        None => {
            let mut err_string: String = "unknown EconomicEvent action type: ".to_string();
            err_string.push_str(action.as_ref());
            panic!(err_string);
        }
    }
}
