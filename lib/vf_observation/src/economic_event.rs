// :TODO: make this file DRY via use of macros
// But not until we have another record type to compare against for commonalities!
// EconomicEvent differs from most other record types as its response can optionall include EconomicResource data,
// so it's a good one to check against others for where the pattern deviates.

// trace_macros!(true);

use std::borrow::Cow;
use hdk::{
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
};
use holochain_json_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_core::{
    measurement::QuantityValue,
};
use vf_core::type_aliases::{
    EventAddress,
    ActionId,
    Timestamp,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessAddress,
    ResourceSpecificationAddress,
    FulfillmentAddress,
    SatisfactionAddress,
};
use super::economic_resource::{
    ResourceInventoryType,
    Entry as ResourceEntry,
    Response as ResourceResponse,
    construct_response_record as construct_resource_response,
};

// vfRecord! {
    #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
    pub struct Entry {
        pub action: ActionId,
        pub provider: Option<AgentAddress>,
        pub receiver: Option<AgentAddress>,
        pub resource_inventoried_as: Option<ResourceAddress>,
        pub resource_classified_as: Option<Vec<ExternalURL>>,
        pub resource_conforms_to: Option<ResourceSpecificationAddress>,
        pub resource_quantity: Option<QuantityValue>,
        pub effort_quantity: Option<QuantityValue>,
        pub has_beginning: Option<Timestamp>,
        pub has_end: Option<Timestamp>,
        pub has_point_in_time: Option<Timestamp>,
        pub before: Option<Timestamp>,
        pub after: Option<Timestamp>,
        pub at_location: Option<LocationAddress>,
        pub in_scope_of: Option<Vec<String>>,
        pub note: Option<String>,
    }
// }

impl<'a> Entry {
    pub fn get_action(&'a self) -> &str {
        &(self.action.as_ref())[..]
    }
}

/// Handles update operations by merging any newly provided fields into
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            action: self.action.to_owned(),
            provider: self.provider.clone(),
            receiver: self.receiver.clone(),
            resource_inventoried_as: self.resource_inventoried_as.clone(),
            resource_classified_as: self.resource_classified_as.clone(),
            resource_conforms_to: self.resource_conforms_to.clone(),
            resource_quantity: self.resource_quantity.clone(),
            effort_quantity: self.effort_quantity.clone(),
            has_beginning: self.has_beginning.clone(),
            has_end: self.has_end.clone(),
            has_point_in_time: self.has_point_in_time.clone(),
            before: self.before.clone(),
            after: self.after.clone(),
            at_location: if e.at_location == MaybeUndefined::Undefined { self.at_location.clone() } else { e.at_location.clone().into() },
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.clone() } else { e.in_scope_of.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub action: ActionId,
    #[serde(default)]
    note: MaybeUndefined<String>,
    #[serde(default)]
    pub input_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    pub output_of: MaybeUndefined<ProcessAddress>,
    #[serde(default)]
    provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    pub resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    pub to_resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    pub resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    pub resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    pub resource_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    effort_quantity: MaybeUndefined<QuantityValue>,
    #[serde(default)]
    has_beginning: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_end: MaybeUndefined<Timestamp>,
    #[serde(default)]
    has_point_in_time: MaybeUndefined<Timestamp>,
    #[serde(default)]
    before: MaybeUndefined<Timestamp>,
    #[serde(default)]
    after: MaybeUndefined<Timestamp>,
    #[serde(default)]
    at_location: MaybeUndefined<LocationAddress>,
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,

    // :SHONK: internal field used in updating linked resource quantities
    #[serde(default)]
    pub target_inventory_type: Option<ResourceInventoryType>,
}

impl<'a> CreateRequest {
    pub fn with_inventoried_resource(&self, resource_address: &ResourceAddress) -> Self {
        CreateRequest {
            resource_inventoried_as: MaybeUndefined::Some(resource_address.to_owned()),
            ..self.to_owned()
        }
    }

    pub fn with_inventory_type(&self, t: ResourceInventoryType) -> Self {
        CreateRequest {
            target_inventory_type: Some(t),
            ..self.to_owned()
        }
    }

    // :TODO: accessors for field data
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    id: EventAddress,
    // ENTRY FIELDS
    #[serde(default)]
    note: MaybeUndefined<String>,
    #[serde(default)]
    at_location: MaybeUndefined<LocationAddress>,
    // :TODO:
    // agreed_in
    // realization_of
    // triggered_by
    #[serde(default)]
    in_scope_of: MaybeUndefined<Vec<String>>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &EventAddress {
        &self.id
    }

    pub fn get_location(&'a self) -> MaybeUndefined<LocationAddress> {
        self.at_location.to_owned()
    }
}

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: EventAddress,
    action: ActionId,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_of: Option<ProcessAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<AgentAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver: Option<AgentAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_inventoried_as: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_classified_as: Option<Vec<ExternalURL>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_conforms_to: Option<ResourceSpecificationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effort_quantity: Option<QuantityValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_beginning: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_point_in_time: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    at_location: Option<LocationAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    #[serde(skip_serializing_if = "Option::is_none")]
    fulfills: Option<Vec<FulfillmentAddress>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    satisfies: Option<Vec<SatisfactionAddress>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    economic_event: Response,
    #[serde(skip_serializing_if = "Option::is_none")]
    economic_resource: Option<ResourceResponse>,
}

/**
 * Pick relevant fields out of I/O record into underlying DHT entry
 */
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            action: e.action.into(),
            note: e.note.into(),
            provider: e.provider.into(),
            receiver: e.receiver.into(),
            resource_inventoried_as: e.resource_inventoried_as.into(),
            resource_classified_as: e.resource_classified_as.into(),
            resource_conforms_to: e.resource_conforms_to.into(),
            resource_quantity: e.resource_quantity.into(),
            effort_quantity: e.effort_quantity.into(),
            has_beginning: e.has_beginning.into(),
            has_end: e.has_end.into(),
            has_point_in_time: e.has_point_in_time.into(),
            before: e.before.into(),
            after: e.after.into(),
            at_location: e.at_location.into(),
            in_scope_of: e.in_scope_of.into(),
        }
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
        input_process, output_process,
        fulfillments,
        satisfactions,
    ): (
        Option<ProcessAddress>, Option<ProcessAddress>,
        Option<Cow<'a, Vec<FulfillmentAddress>>>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
    ),
    resource_address: Option<ResourceAddress>,
    resource: Option<ResourceEntry>, (
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
        economic_event: Response {
            id: event_address.to_owned(),
            action: event.action.to_owned(),
            note: event.note.to_owned(),
            input_of: input_process.to_owned(),
            output_of: output_process.to_owned(),
            provider: event.provider.to_owned(),
            receiver: event.receiver.to_owned(),
            resource_inventoried_as: event.resource_inventoried_as.to_owned(),
            resource_classified_as: event.resource_classified_as.to_owned(),
            resource_conforms_to: event.resource_conforms_to.to_owned(),
            resource_quantity: event.resource_quantity.to_owned(),
            effort_quantity: event.effort_quantity.to_owned(),
            has_beginning: event.has_beginning.to_owned(),
            has_end: event.has_end.to_owned(),
            has_point_in_time: event.has_point_in_time.to_owned(),
            before: event.before.to_owned(),
            after: event.after.to_owned(),
            at_location: event.at_location.to_owned(),
            in_scope_of: event.in_scope_of.to_owned(),
            fulfills: fulfillments.map(Cow::into_owned),
            satisfies: satisfactions.map(Cow::into_owned),
        },
        economic_resource: match resource_address {
            Some(addr) => Some(construct_resource_response(&addr, &(resource.unwrap()), (contained_in, state, contains))),
            None => None,
        },
    }
}

// Same as above, but omits EconomicResource object
pub fn construct_response<'a>(
    address: &EventAddress, e: &Entry, (
        input_process, output_process,
        fulfillments,
        satisfactions,
    ): (
        Option<ProcessAddress>, Option<ProcessAddress>,
        Option<Cow<'a, Vec<FulfillmentAddress>>>,
        Option<Cow<'a, Vec<SatisfactionAddress>>>,
    )
) -> ResponseData {
    ResponseData {
        economic_event: Response {
            id: address.to_owned().into(),
            action: e.action.to_owned(),
            note: e.note.to_owned(),
            input_of: input_process.to_owned(),
            output_of: output_process.to_owned(),
            provider: e.provider.to_owned(),
            receiver: e.receiver.to_owned(),
            resource_inventoried_as: e.resource_inventoried_as.to_owned(),
            resource_classified_as: e.resource_classified_as.to_owned(),
            resource_conforms_to: e.resource_conforms_to.to_owned(),
            resource_quantity: e.resource_quantity.to_owned(),
            effort_quantity: e.effort_quantity.to_owned(),
            has_beginning: e.has_beginning.to_owned(),
            has_end: e.has_end.to_owned(),
            has_point_in_time: e.has_point_in_time.to_owned(),
            before: e.before.to_owned(),
            after: e.after.to_owned(),
            at_location: e.at_location.to_owned(),
            in_scope_of: e.in_scope_of.to_owned(),
            fulfills: fulfillments.map(Cow::into_owned),
            satisfies: satisfactions.map(Cow::into_owned),
        },
        economic_resource: None,
    }
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
