use std::borrow::Cow;
use hdk::{
    holochain_json_api::{ json::JsonString, error::JsonError },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
    links::get_linked_addresses_as_type,
};

use vf_core::type_aliases::{
    ResourceAddress,
    ExternalURL,
    LocationAddress,
    ProcessSpecificationAddress,
    ResourceSpecificationAddress,
    UnitAddress,
    ProductBatchAddress,
    ActionId,
};
use vf_core::measurement::{ QuantityValue, add, subtract };
use vf_knowledge::action::{ ActionEffect, get_builtin_action };

use super::identifiers::{
    RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG,
    RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG,
};
use super::economic_event::{
    CreateRequest as EventCreateRequest,
    UpdateRequest as EventUpdateRequest,
};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Entry {
  conforms_to: ResourceSpecificationAddress,
  classified_as: Option<Vec<ExternalURL>>,
  tracking_identifier: Option<String>,
  lot: Option<ProductBatchAddress>,
  image: Option<ExternalURL>,
  accounting_quantity: Option<QuantityValue>,
  onhand_quantity: Option<QuantityValue>,
  unit_of_effort: Option<UnitAddress>,
  stage: Option<ProcessSpecificationAddress>,
  state: Option<ActionId>,
  current_location: Option<LocationAddress>,
  note: Option<String>,
}

// used in EconomicEvent API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    conforms_to: ResourceSpecificationAddress,
    #[serde(default)]
    tracking_identifier: MaybeUndefined<String>,
    #[serde(default)]
    lot: MaybeUndefined<ProductBatchAddress>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    accounting_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    onhand_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    unit_of_effort: MaybeUndefined<UnitAddress>,
    #[serde(default)]
    contained_in: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    current_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

// used in EconomicResource API
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: ResourceAddress,
    #[serde(default)]
    conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    tracking_identifier: MaybeUndefined<String>,
    #[serde(default)]
    lot: MaybeUndefined<ProductBatchAddress>,
    #[serde(default)]
    image: MaybeUndefined<ExternalURL>,
    #[serde(default)]
    accounting_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    onhand_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    unit_of_effort: MaybeUndefined<UnitAddress>,
    #[serde(default)]
    contained_in: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    current_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    note: MaybeUndefined<String>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &ResourceAddress {
        &self.id
    }
}

pub struct CreationPayload {
    event: EventCreateRequest,
    resource: CreateRequest,
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
            conforms_to: r.conforms_to.to_owned(),
            classified_as: if e.resource_classified_as == MaybeUndefined::Undefined { None } else { e.resource_classified_as.to_owned().to_option() },
            tracking_identifier: if r.tracking_identifier == MaybeUndefined::Undefined { None } else { r.tracking_identifier.to_owned().to_option() },
            lot: if r.lot == MaybeUndefined::Undefined { None } else { r.lot.to_owned().to_option() },
            image: if r.image == MaybeUndefined::Undefined { None } else { r.image.to_owned().to_option() },
            accounting_quantity: if r.accounting_quantity == MaybeUndefined::Undefined { None } else { r.accounting_quantity.to_owned().to_option() },
            onhand_quantity: if r.onhand_quantity == MaybeUndefined::Undefined { None } else { r.onhand_quantity.to_owned().to_option() },
            unit_of_effort: if r.unit_of_effort == MaybeUndefined::Undefined {
                None // :TODO: pull from e.resource_conforms_to.unit_of_effort if present
            } else { r.unit_of_effort.to_owned().to_option() },
            stage: None, // :TODO: pull from e.output_of.based_on if present. Undecided whether this should only happen on 'modify' events, or on everything.
            state: match String::from(e.action.clone()).as_ref() {
                "pass" | "fail" => Some(e.action),
                _ => None,
            },
            current_location: if r.current_location == MaybeUndefined::Undefined { None } else { r.current_location.to_owned().to_option() },
            note: if r.note == MaybeUndefined::Undefined { None } else { r.note.clone().into() },
        }
    }
}

/// Handles update operations for correcting data entry errors
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            conforms_to: if e.conforms_to == MaybeUndefined::Undefined { self.conforms_to.to_owned() } else { e.conforms_to.to_owned().unwrap() },
            classified_as: if e.classified_as == MaybeUndefined::Undefined { self.classified_as.to_owned() } else { e.classified_as.to_owned().to_option() },
            tracking_identifier: if e.tracking_identifier == MaybeUndefined::Undefined { self.tracking_identifier.to_owned() } else { e.tracking_identifier.to_owned().to_option() },
            lot: if e.lot == MaybeUndefined::Undefined { self.lot.to_owned() } else { e.lot.to_owned().to_option() },
            image: if e.image == MaybeUndefined::Undefined { self.image.to_owned() } else { e.image.to_owned().to_option() },
            accounting_quantity: if e.accounting_quantity == MaybeUndefined::Undefined { self.accounting_quantity.to_owned() } else { e.accounting_quantity.to_owned().to_option() },
            onhand_quantity: if e.onhand_quantity == MaybeUndefined::Undefined { self.onhand_quantity.to_owned() } else { e.onhand_quantity.to_owned().to_option() },
            unit_of_effort: if e.unit_of_effort == MaybeUndefined::Undefined { self.unit_of_effort.to_owned() } else { e.unit_of_effort.to_owned().to_option() },
            stage: self.stage.to_owned(),
            state: self.state.to_owned(),
            current_location: if e.current_location == MaybeUndefined::Undefined { self.current_location.to_owned() } else { e.current_location.to_owned().to_option() },
            note: if e.note == MaybeUndefined::Undefined { self.note.to_owned() } else { e.note.to_owned().to_option() },
        }
    }
}

/// Handle update operations by observed events
impl Updateable<EventCreateRequest> for Entry {
    fn update_with(&self, e: &EventCreateRequest) -> Entry {
        Entry {
            conforms_to: if let MaybeUndefined::Some(conforms_to) = e.resource_conforms_to.to_owned() { conforms_to } else { self.conforms_to.to_owned() },
            classified_as: if let MaybeUndefined::Some(classified_as) = e.resource_classified_as.to_owned() {
                let mut results: Vec<ExternalURL> = if let Some(already_classified) = &self.classified_as { already_classified.to_owned() } else { vec![] };
                results.append(&mut classified_as.clone());
                Some(results)
            } else { self.classified_as.to_owned() },
            tracking_identifier: self.tracking_identifier.to_owned(),
            lot: self.lot.to_owned(),
            image: self.image.to_owned(),
            accounting_quantity: update_quantity(self.accounting_quantity.to_owned(), e.resource_quantity.to_owned(), e.action.as_ref(), false),
            onhand_quantity: update_quantity(self.onhand_quantity.to_owned(), e.resource_quantity.to_owned(), e.action.as_ref(), true),
            unit_of_effort: self.unit_of_effort.to_owned(), // :TODO: pull from e.resource_conforms_to.unit_of_effort
            stage: self.stage.to_owned(), // :TODO: pull from e.output_of.based_on if present
            state: self.state.to_owned(),
            current_location: self.current_location.to_owned(),
            note: self.note.to_owned(),
        }
    }
}

/// Encapsulates the logic for updating EconomicResource quantities in response to event triggers
/// :TODO: refactor this out to reduce the conditional complexity
fn update_quantity(
    current_val: Option<QuantityValue>,
    event_qty: MaybeUndefined<QuantityValue>,
    action: &str,
    is_onhand_value: bool,
) -> Option<QuantityValue> {
    if let Some(qty) = current_val.to_owned() {
        if let MaybeUndefined::Some(event_resource_qty) = event_qty.to_owned() {
            match get_builtin_action(action) {
                Some(action) => {
                    match &*(action.id) {
                        // 'transfer-custody' updates onHand but not Accounting
                        "transfer-custody" => match action.resource_effect {
                            ActionEffect::Decrement => if is_onhand_value { Some(subtract(qty, event_resource_qty)) } else { current_val.to_owned() },
                            _ => current_val.to_owned(),
                        },
                        // 'transfer-all-rights' updates Accounting but not onHand
                        "transfer-all-rights" => match action.resource_effect {
                            ActionEffect::Decrement => if is_onhand_value { current_val.to_owned() } else { Some(subtract(qty, event_resource_qty)) },
                            _ => current_val.to_owned(),
                        },
                        // 'normal' action, just work from the action effect
                        _ => {
                            match action.resource_effect {
                                ActionEffect::Increment => Some(add(qty, event_resource_qty)),
                                ActionEffect::Decrement => Some(subtract(qty, event_resource_qty)),
                                _ => current_val.to_owned(),
                            }
                        }
                    }
                },
                None => {
                    current_val.to_owned()
                },
            }
        } else { current_val.to_owned() }
    } else { None }
}

/// Handle updates when a previously logged event is altered after application
impl Updateable<EventUpdateRequest> for Entry {
    fn update_with(&self, _e: &EventUpdateRequest) -> Entry {
        self.clone() // :TODO:
    }
}

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: ResourceAddress,
    conforms_to: ResourceSpecificationAddress,
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
        contains,
     ): (
        Option<ResourceAddress>,
        Option<Cow<'a, Vec<ResourceAddress>>>,
    ),
) -> ResponseData {
    ResponseData {
        economic_resource: construct_response_record(address, e, (contained_in, contains))
    }
}

/// Create response from input DHT primitives
pub fn construct_response_record<'a>(
    address: &ResourceAddress, e: &Entry, (
        contained_in,
        contains,
     ): (
        Option<ResourceAddress>,
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
        state: e.state.to_owned(),
        current_location: e.current_location.to_owned(),
        note: e.note.to_owned(),

        // link fields
        contained_in: contained_in.to_owned(),
        contains: contains.map(Cow::into_owned),
    }
}

// field list retrieval internals
// @see construct_response
pub fn get_link_fields<'a>(resource: &ResourceAddress) -> (
    Option<ResourceAddress>,
    Option<Cow<'a, Vec<ResourceAddress>>>,
) {
    (
        get_linked_addresses_as_type(resource, RESOURCE_CONTAINED_IN_LINK_TYPE, RESOURCE_CONTAINED_IN_LINK_TAG).into_owned().pop(),
        Some(get_linked_addresses_as_type(resource, RESOURCE_CONTAINS_LINK_TYPE, RESOURCE_CONTAINS_LINK_TAG)),
    )
}
