use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
    error::{ ZomeApiResult },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
    links::get_linked_addresses_as_type,
    records::read_record_entry,
};

use vf_core::type_aliases::{
    ResourceAddress,
    ExternalURL,
    LocationAddress,
    ProcessSpecificationAddress,
    ResourceSpecificationAddress,
    UnitAddress,
    ProductBatchAddress,
    EventAddress,
    ActionId,
};
use vf_core::measurement::{ QuantityValue, add, subtract };
use vf_knowledge::action::{ ActionEffect, ActionInventoryEffect, get_builtin_action };

use super::identifiers::{
    RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
    RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
    RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE, RESOURCE_AFFECTED_BY_EVENT_LINK_TAG,
};
use super::economic_event::{
    Entry as EventEntry,
    CreateRequest as EventCreateRequest,
};

//---------------- RECORD INTERNALS & VALIDATION ----------------

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
  conforms_to: Option<ResourceSpecificationAddress>,
  classified_as: Option<Vec<ExternalURL>>,
  tracking_identifier: Option<String>,
  lot: Option<ProductBatchAddress>,
  image: Option<ExternalURL>,
  accounting_quantity: Option<QuantityValue>,
  onhand_quantity: Option<QuantityValue>,
  unit_of_effort: Option<UnitAddress>,
  stage: Option<ProcessSpecificationAddress>,
  current_location: Option<LocationAddress>,
  note: Option<String>,
}

impl Entry {
    pub fn validate(&self) -> Result<(), String> {
        if !(self.classified_as.is_some() || self.conforms_to.is_some()) {
            return Err("EconomicResource must have either a specification or classification".into());
        }
        Ok(())
    }
}

//---------------- CREATE ----------------

// used in EconomicEvent API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    tracking_identifier: MaybeUndefined<String>,
    #[serde(default)]
    lot: MaybeUndefined<ProductBatchAddress>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    contained_in: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    current_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> CreateRequest {
    pub fn get_contained_in(&'a self) -> Option<ResourceAddress> {
        self.contained_in.to_owned().to_option()
    }
}

#[derive(Clone)]
pub struct CreationPayload {
    event: EventCreateRequest,
    resource: CreateRequest,
}

impl<'a> CreationPayload {
    pub fn get_event_params(&'a self) -> &EventCreateRequest {
        &self.event
    }

    pub fn get_resource_params(&'a self) -> &CreateRequest {
        &self.resource
    }
}

pub fn resource_creation(event: &EventCreateRequest, resource: &CreateRequest) -> CreationPayload {
    CreationPayload {
        event: event.to_owned(),
        resource: resource.to_owned(),
    }
}

/// Handles create operations via observed event resource inspection parameter
/// @see https://github.com/holo-rea/holo-rea/issues/65
impl From<CreationPayload> for Entry
{
    fn from(t: CreationPayload) -> Entry {
        let r = t.resource;
        let e = t.event;
        Entry {
            conforms_to: if r.conforms_to.is_some() { r.conforms_to.to_owned().to_option() } else { e.resource_conforms_to.to_owned().to_option() },
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
            unit_of_effort: None, // :TODO: pull from e.resource_conforms_to.unit_of_effort if present
            stage: None, // :TODO: pull from e.output_of.based_on if present. Undecided whether this should only happen on 'modify' events, or on everything.
            current_location: if r.current_location == MaybeUndefined::Undefined { None } else { r.current_location.to_owned().to_option() },
            note: if r.note == MaybeUndefined::Undefined { None } else { r.note.clone().into() },
        }
    }
}

//---------------- UPDATE ----------------

// used in EconomicResource API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ResourceAddress,
    #[serde(default)]
    classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    contained_in: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    unit_of_effort: MaybeUndefined<UnitAddress>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ResourceAddress {
        &self.id
    }

    pub fn get_contained_in(&'a self) -> MaybeUndefined<ResourceAddress> {
        self.contained_in.to_owned()
    }
}

/// Handles update operations for correcting data entry errors
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            conforms_to: self.conforms_to.to_owned(),
            classified_as: if e.classified_as == MaybeUndefined::Undefined { self.classified_as.to_owned() } else { e.classified_as.to_owned().to_option() },
            tracking_identifier: self.tracking_identifier.to_owned(),
            lot: self.lot.to_owned(),
            image: if e.image == MaybeUndefined::Undefined { self.image.to_owned() } else { e.image.to_owned().to_option() },
            accounting_quantity: self.accounting_quantity.to_owned(),
            onhand_quantity: self.onhand_quantity.to_owned(),
            unit_of_effort: if e.unit_of_effort == MaybeUndefined::Undefined { self.unit_of_effort.to_owned() } else { e.unit_of_effort.to_owned().to_option() },
            stage: self.stage.to_owned(),
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
impl Updateable<EventCreateRequest> for Entry {
    fn update_with(&self, e: &EventCreateRequest) -> Entry {
        Entry {
            conforms_to: self.conforms_to.to_owned(),
            classified_as: if let MaybeUndefined::Some(classified_as) = e.resource_classified_as.to_owned() {
                let mut results: Vec<ExternalURL> = if let Some(already_classified) = &self.classified_as { already_classified.to_owned() } else { vec![] };
                results.append(&mut classified_as.clone());
                Some(results)
            } else { self.classified_as.to_owned() },
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
            stage: self.stage.to_owned(), // :TODO: pull from e.output_of.based_on if present
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

#[derive(Serialize, Deserialize, DefaultJson, Clone, Debug)]
pub enum ResourceInventoryType {
    ProvidingInventory,
    ReceivingInventory,
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
            // 'transfer-complete' is a full swap
            "transfer-complete" => match which_inventory_type {
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

//---------------- EXTERNAL API ----------------

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ResourceAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracking_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lot: Option<ProductBatchAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<ExternalURL>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounting_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    onhand_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_effort: Option<UnitAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contained_in: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<ProcessSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<ActionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,

    // query edges
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<Vec<ResourceAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // trace: Option<Vec<EventAddress>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // track: Option<Vec<EventAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    economic_resource: Response,
}

/// Create response from input DHT primitives
pub fn construct_response<'a>(
    address: &ResourceAddress, e: &Entry, (
        contained_in,
        state,
        contains,
     ): (
        Option<ResourceAddress>,
        Option<ActionId>,
        Option<Cow<'a, Vec<ResourceAddress>>>,
    ),
) -> ResponseData {
    ResponseData {
        economic_resource: construct_response_record(address, e, (contained_in, state, contains))
    }
}

/// Create response from input DHT primitives
pub fn construct_response_record<'a>(
    address: &ResourceAddress, e: &Entry, (
        contained_in,
        state,
        contains,
     ): (
        Option<ResourceAddress>,
        Option<ActionId>,
        Option<Cow<'a, Vec<ResourceAddress>>>,
    ),
) -> Response {
    Response {
        // entry fields
        id: address.to_owned(),
        conforms_to: e.conforms_to.to_owned(),
        classified_as: e.classified_as.to_owned(),
        tracking_identifier: e.tracking_identifier.to_owned(),
        lot: e.lot.to_owned(),
        image: e.image.to_owned(),
        accounting_quantity: e.accounting_quantity.to_owned(),
        onhand_quantity: e.onhand_quantity.to_owned(),
        unit_of_effort: e.unit_of_effort.to_owned(),
        stage: e.stage.to_owned(),
        state: state.to_owned(),
        current_location: e.current_location.to_owned(),
        note: e.note.to_owned(),

        // link fields
        contained_in: contained_in.to_owned(),
        contains: contains.map(Cow::into_owned),
    }
}

//---------------- READ ----------------

// field list retrieval internals
// @see construct_response
pub fn get_link_fields<'a>(resource: &ResourceAddress) -> (
    Option<ResourceAddress>,
    Option<ActionId>,
    Option<Cow<'a, Vec<ResourceAddress>>>,
) {
    (
        get_linked_addresses_as_type(resource, RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG).into_owned().pop(),
        get_resource_state(resource),
        Some(get_linked_addresses_as_type(resource, RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG)),
    )
}

fn get_resource_state(resource: &ResourceAddress) -> Option<ActionId> {
    // read all the EconomicEvents affecting this resource
    let events: Vec<EventAddress> = get_linked_addresses_as_type(
        resource,
        RESOURCE_AFFECTED_BY_EVENT_LINK_TYPE,
        RESOURCE_AFFECTED_BY_EVENT_LINK_TAG,
    ).into_owned();

    // grab the most recent "pass" or "fail" action
    events.iter()
        .rev()
        .fold(None, move |result, event| {
            // already found it, just fall through
            // :TODO: figure out the Rust STL method to abort on first Some() value
            if let Some(_) = result {
                return result;
            }

            let entry: ZomeApiResult<EventEntry> = read_record_entry(event);
            match entry {
                Err(_) => result, // :TODO: this indicates some data integrity error
                Ok(entry) => {
                    match &*String::from(entry.action.clone()) {
                        "pass" | "fail" => Some(entry.action),  // found it! Return this as the current resource state.
                        _ => result,    // still not located, keep looking...
                    }
                },
            }
        })
}
