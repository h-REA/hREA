/**
 * Holo-REA 'economic resource' zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package Holo-REA
 */
use hdk::prelude::*;

use hdk_records::{
    RecordAPIResult, DataIntegrityError,
    MaybeUndefined, OtherCellResult,
    generate_record_entry,
    record_interface::Updateable,
    rpc::call_zome_method,
};

use vf_measurement::*;
use vf_attributes_hdk::{
    EconomicResourceAddress,
    ExternalURL,
    LocationAddress,
    ResourceSpecificationAddress,
    UnitId,
    ProductBatchAddress,
    ActionId,
    AgentAddress,
};
use vf_actions::{ ActionEffect, ActionInventoryEffect};
pub use vf_actions::get_builtin_action;
use hc_zome_rea_resource_specification_rpc::{ResponseData as ResourceSpecificationResponse};

use hc_zome_rea_economic_resource_rpc::*;
use hc_zome_rea_economic_event_rpc::{
    CreateRequest as EventCreateRequest,
    ResourceInventoryType,
};

// :SHONK: needed as re-export in zome logic to allow validation logic to parse entries
pub use hdk_records::record_interface::Identified;

//--------------- ZOME CONFIGURATION ATTRIBUTES ----------------

// :TODO: remove this, replace with reference to appropriate namespacing of zome config
#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct DnaConfigSlice {
    pub economic_resource: EconomicResourceZomeConfig,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, PartialEq, Debug)]
pub struct EconomicResourceZomeConfig {
    pub index_zome: String,
    pub resource_specification_index_zome: Option<String>,
    pub agent_index_zome: Option<String>,
}

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct EntryData {
    pub name: Option<String>,
    pub conforms_to: Option<ResourceSpecificationAddress>,
    pub classified_as: Option<Vec<ExternalURL>>,
    pub tracking_identifier: Option<String>,
    pub lot: Option<ProductBatchAddress>,
    pub image: Option<ExternalURL>,
    pub accounting_quantity: Option<QuantityValue>,
    pub onhand_quantity: Option<QuantityValue>,
    pub unit_of_effort: Option<UnitId>,
    pub current_location: Option<LocationAddress>,
    pub contained_in: Option<EconomicResourceAddress>,
    pub note: Option<String>,
    pub primary_accountable: Option<AgentAddress>,
    pub _nonce: Bytes,
}

impl EntryData {
    pub fn validate(&self) -> Result<(), String> {
        if !(self.classified_as.is_some() || self.conforms_to.is_some()) {
            return Err("EconomicResource must have either a specification or classification".into());
        }
        Ok(())
    }
}

generate_record_entry!(EntryData, EconomicResourceAddress, EntryStorage);

//---------------- Holochain App Entry And Link Types Setup ----------------

#[hdk_entry_defs(skip_hdk_extern = true)]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    EconomicResource(EntryStorage),
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::EconomicResource(e)
    }
}


//---------------- CREATE ----------------

/// Handles create operations via observed event resource inspection parameter
/// @see https://github.com/holo-rea/holo-rea/issues/65
impl TryFrom<CreationPayload> for EntryData {
    type Error = DataIntegrityError;

    fn try_from(t: CreationPayload) -> RecordAPIResult<EntryData> {
        let conforming = t.get_resource_specification_id();
        let r = t.resource;
        let e = t.event;
        let produce_action = get_builtin_action("produce").unwrap();
        let raise_action = get_builtin_action("raise").unwrap();
        let lower_action = get_builtin_action("lower").unwrap();
        let action_id = String::from(e.get_action());
        Ok(EntryData {
            name: r.name.to_option(),
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
            contained_in: if r.contained_in == MaybeUndefined::Undefined { None } else { r.contained_in.to_owned().to_option() },
            note: if r.note == MaybeUndefined::Undefined { None } else { r.note.clone().into() },
            primary_accountable: if action_id == produce_action.id || action_id == raise_action.id || action_id == lower_action.id { Some(e.receiver) } else { None },
            _nonce: random_bytes(32)?,
        })
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
        &specification_id,
        &String::from("read_resource_specification"),
        GetSpecificationRequest { address: specification_id.to_owned() },
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
            name: self.name.to_owned(),
            conforms_to: self.conforms_to.to_owned(),
            classified_as: if e.classified_as == MaybeUndefined::Undefined { self.classified_as.to_owned() } else { e.classified_as.to_owned().to_option() },
            tracking_identifier: self.tracking_identifier.to_owned(),
            lot: self.lot.to_owned(),
            image: if e.image == MaybeUndefined::Undefined { self.image.to_owned() } else { e.image.to_owned().to_option() },
            accounting_quantity: self.accounting_quantity.to_owned(),
            onhand_quantity: self.onhand_quantity.to_owned(),
            unit_of_effort: if e.unit_of_effort == MaybeUndefined::Undefined { self.unit_of_effort.to_owned() } else { e.unit_of_effort.to_owned().to_option() },
            current_location: self.current_location.to_owned(),
            contained_in: if e.contained_in == MaybeUndefined::Undefined { self.contained_in.to_owned() } else { e.contained_in.to_owned().to_option() },
            note: if e.note == MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().to_option() },
            primary_accountable: self.primary_accountable.to_owned(),
            _nonce: self._nonce.to_owned(),
        }
    }
}

/// Handle update operations by observed events
///
/// :WARNING: we presume the event has already been determined to relate to the
/// resource, and this method will panic if that is not the case.
///
/// Currently it is only called within `hdk_records::record_helpers::update_record`,
/// where this check is already implicitly performed.
///
impl Updateable<EventCreateRequest> for EntryData {
    fn update_with(&self, e: EventCreateRequest) -> EntryData {
        EntryData {
            name: self.name.to_owned(),
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
            contained_in: self.contained_in.to_owned(),
            note: self.note.to_owned(),
            // NOTE: this could be "dangerous" in the sense that if not validated properly, this ability to update via events could be abused by third party agents transferring rights and 'ownership' to themselves, from resources currently controlled/owned/stewarded by other agents
            // relates to transfer all rights but not custody
            primary_accountable: if e.to_resource_inventoried_as.to_owned().is_some() && (e.to_resource_inventoried_as == e.resource_inventoried_as) && (e.get_action() == "transfer" || e.get_action() == "transfer_all_rights") {
                Some(e.receiver.to_owned())
            } else {
                self.primary_accountable.to_owned()
            },
            _nonce: self._nonce.to_owned(),
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
            panic!("{:?}", err_string);
        }
    }
}
