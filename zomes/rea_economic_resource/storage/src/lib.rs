use hc_zome_dna_auth_resolver_core::AvailableCapability;
/**
 * hREA 'economic resource' zome internal data structures
 *
 * Required by the zome itself, and for any DNA-local zomes interacting with its
 * storage API directly.
 *
 * @package hREA
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
use hc_zome_rea_resource_specification_rpc::{ResponseData as ResourceSpecificationResponseData, Response as ResourceSpecificationResponse};

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
    #[entry_def(visibility = "private")]
    AvailableCapability(AvailableCapability)
}

impl From<EntryStorage> for EntryTypes
{
    fn from(e: EntryStorage) -> EntryTypes
    {
        EntryTypes::EconomicResource(e)
    }
}
impl TryFrom<AvailableCapability> for EntryTypes {
    type Error = WasmError;

    fn try_from(e: AvailableCapability) -> Result<EntryTypes, Self::Error>
    {
        Ok(EntryTypes::AvailableCapability(e))
    }
}

#[hdk_link_types(skip_no_mangle = true)]
pub enum LinkTypes {
    // relates to dna-auth-resolver mixin
    // and remote authorizations
    AvailableCapability
}

//---------------- CREATE ----------------

/// Handles create operations via observed event resource inspection parameter
/// @see https://github.com/h-REA/hREA/issues/65
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
        // first choice are the units passed in on the event
        // value, fallback is the default_unit_of_resource and
        // default_unit_of_effort
        // set on the resource specification, if there is one
        let (unit_of_resource, unit_of_effort) = get_units_for_resource(
            e.resource_quantity.to_owned(),
            e.effort_quantity.to_owned(),
            conforming.clone(),
        )?;
        let quantity_value = match e.resource_quantity.to_owned() {
            MaybeUndefined::Some(resource_quantity) => {
                Some(QuantityValue::new(resource_quantity.get_numerical_value(), unit_of_resource))
            },
            _ => None,
        };
        Ok(EntryData {
            name: r.name.to_option(),
            conforms_to: conforming,
            classified_as: if e.resource_classified_as == MaybeUndefined::Undefined { None } else { e.resource_classified_as.to_owned().to_option() },
            tracking_identifier: if r.tracking_identifier == MaybeUndefined::Undefined { None } else { r.tracking_identifier.to_owned().to_option() },
            lot: if r.lot == MaybeUndefined::Undefined { None } else { r.lot.to_owned().to_option() },
            image: if r.image == MaybeUndefined::Undefined { None } else { r.image.to_owned().to_option() },
            accounting_quantity: match quantity_value.clone() {
                Some(resource_quantity) => update_quantity(
                    // instantiate with the correct units
                    Some(QuantityValue::new(0.0, resource_quantity.get_unit())), 
                    MaybeUndefined::Some(resource_quantity),
                    &e.action,
                    ResourceValueType::AccountingValue,
                    match &e.target_inventory_type {
                        Some(inventory_type) => inventory_type.to_owned(),
                        None => panic!("Developer error: EconomicEvent inventory type must be provided when creating EconomicResource!"),
                    },
                ),
                None => None,
            },
            onhand_quantity: match quantity_value {
                Some(resource_quantity) => update_quantity(
                    // instantiate with the correct units
                    Some(QuantityValue::new(0.0, resource_quantity.get_unit())),
                    MaybeUndefined::Some(resource_quantity),
                    &e.action,
                    ResourceValueType::OnhandValue,
                    match &e.target_inventory_type {
                        Some(inventory_type) => inventory_type.to_owned(),
                        None => panic!("Developer error: EconomicEvent inventory type must be provided when updating EconomicResource!"),
                    },
                ),
                None => None,
            },
            unit_of_effort,
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

// in the tuple response
// first is Resource Unit, second is Effort Unit
// (Option<UnitId>, Option<UnitId>)
fn get_units_for_resource(
  event_resource_quantity: MaybeUndefined<QuantityValue>,
  event_effort_quantity: MaybeUndefined<QuantityValue>,
  conforming: Option<ResourceSpecificationAddress>,
) -> RecordAPIResult<(Option<UnitId>, Option<UnitId>)> {
    match (event_resource_quantity.clone(), event_effort_quantity.clone()) {
        // if both resource_quantity and effort_quantity have a defined unit, then we don't
        // need to fetch the resource_specification to look for default units
        // just take the overrides
        (MaybeUndefined::Some(resource_quantity), MaybeUndefined::Some(effort_quantity)) if (resource_quantity.get_unit().is_some() && effort_quantity.get_unit().is_some())  => {
            Ok((resource_quantity.get_unit(), effort_quantity.get_unit()))
        },
        (_, _)  => {
            let (default_resource_unit, default_effort_unit) = match conforming.clone() {
                Some(conforms_to_spec) => {
                    let resource_spec = get_resource_specification(conforms_to_spec)?;
                    (resource_spec.default_unit_of_resource, resource_spec.default_unit_of_effort)
                },
                None => (None, None),
            };
            let resource_unit = if let MaybeUndefined::Some(resource_quantity) = event_resource_quantity {
                if resource_quantity.get_unit().is_some() { resource_quantity.get_unit() }
                else { default_resource_unit }
            } else {
                default_resource_unit
            };
            let effort_unit = if let MaybeUndefined::Some(effort_quantity) = event_effort_quantity {
                if effort_quantity.get_unit().is_some() { effort_quantity.get_unit() }
                else { default_effort_unit }
            } else {
                default_effort_unit
            };
            Ok((resource_unit, effort_unit))
        }
    }
}

fn get_resource_specification(specification_id: ResourceSpecificationAddress) -> RecordAPIResult<ResourceSpecificationResponse> {
    let spec_data: OtherCellResult<ResourceSpecificationResponseData> = call_zome_method::<EntryTypes, _, _, _, _, _, _, _>(
        &specification_id,
        &String::from("read_resource_specification"),
        GetSpecificationRequest { address: specification_id.to_owned() },
        LinkTypes::AvailableCapability
    );

    match spec_data {
        Ok(spec_response) => Ok(spec_response.resource_specification),
        Err(e) => Err(e.into()),
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
        // just work from the configured effect and reverse for the receiver
        Some(action_obj) => {
            let action_effect = match which_qty_type {
                ResourceValueType::AccountingValue => action_obj.accounting_effect,
                ResourceValueType::OnhandValue => action_obj.onhand_effect
            };
            match which_inventory_type {
                ResourceInventoryType::ProvidingInventory => match action_effect {
                    ActionEffect::DecrementIncrement => ActionInventoryEffect::Decrement,
                    ActionEffect::NoEffect => ActionInventoryEffect::NoEffect,
                    ActionEffect::Increment => ActionInventoryEffect::Increment,
                    ActionEffect::Decrement => ActionInventoryEffect::Decrement,
                },
                ResourceInventoryType::ReceivingInventory => match action_effect {
                    ActionEffect::DecrementIncrement => ActionInventoryEffect::Increment,
                    ActionEffect::NoEffect => ActionInventoryEffect::NoEffect,
                    ActionEffect::Increment => ActionInventoryEffect::Decrement,
                    ActionEffect::Decrement => ActionInventoryEffect::Increment,
                },
            }
        },
        None => {
            let mut err_string: String = "unknown EconomicEvent action type: ".to_string();
            err_string.push_str(action.as_ref());
            panic!("{:?}", err_string);
        }
    }
}
