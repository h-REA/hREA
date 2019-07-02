// :TODO: make this file DRY via use of macros
// But not until we have another record type to compare against for commonalities!
// EconomicEvent differs from most other record types as its response can optionall include EconomicResource data,
// so it's a good one to check against others for where the pattern deviates.

// trace_macros!(true);

use hdk::holochain_core_types::{
    cas::content::Address,
    json::JsonString,
    error::HolochainError,
};
use hdk::holochain_core_types_derive::{ DefaultJson };

use hdk_graph_helpers::{
    MaybeUndefined,
    record_interface::Updateable,
};

use vf_knowledge::action::Action;

use vf_core::{
    measurement::QuantityValue,
};

use vf_core::type_aliases::{
    EventAddress,
    Timestamp,
    ExternalURL,
    LocationAddress,
    AgentAddress,
    ResourceAddress,
    ProcessOrTransferAddress,
    ResourceSpecificationAddress,
};

// vfRecord! {
    #[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
    pub struct Entry {
        // action: Action, :TODO:
        pub input_of: Option<ProcessOrTransferAddress>,
        pub output_of: Option<ProcessOrTransferAddress>,
        pub provider: Option<AgentAddress>,
        pub receiver: Option<AgentAddress>,
        pub resource_inventoried_as: Option<ResourceAddress>,
        pub resource_classified_as: Option<Vec<ExternalURL>>,
        pub resource_conforms_to: Option<ResourceSpecificationAddress>,
        pub affected_quantity: Option<QuantityValue>,
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

/// Handles update operations by merging any newly provided fields into
impl Updateable<UpdateRequest> for Entry {
    fn update_with(&self, e: &UpdateRequest) -> Entry {
        Entry {
            input_of: if e.input_of == MaybeUndefined::Undefined { self.input_of.clone() } else { e.input_of.clone().into() },
            output_of: if e.output_of == MaybeUndefined::Undefined { self.output_of.clone() } else { e.output_of.clone().into() },
            provider: if e.provider == MaybeUndefined::Undefined { self.provider.clone() } else { e.provider.clone().into() },
            receiver: if e.receiver == MaybeUndefined::Undefined { self.receiver.clone() } else { e.receiver.clone().into() },
            resource_inventoried_as: if e.resource_inventoried_as == MaybeUndefined::Undefined { self.resource_inventoried_as.clone() } else { e.resource_inventoried_as.clone().into() },
            resource_classified_as: if e.resource_classified_as== MaybeUndefined::Undefined { self.resource_classified_as.clone() } else { e.resource_classified_as.clone().into() },
            resource_conforms_to: if e.resource_conforms_to == MaybeUndefined::Undefined { self.resource_conforms_to.clone() } else { e.resource_conforms_to.clone().into() },
            affected_quantity: if e.affected_quantity== MaybeUndefined::Undefined { self.affected_quantity.clone() } else { e.affected_quantity.clone().into() },
            has_beginning: if e.has_beginning == MaybeUndefined::Undefined { self.has_beginning.clone() } else { e.has_beginning.clone().into() },
            has_end: if e.has_end == MaybeUndefined::Undefined { self.has_end.clone() } else { e.has_end.clone().into() },
            has_point_in_time: if e.has_point_in_time == MaybeUndefined::Undefined { self.has_point_in_time.clone() } else { e.has_point_in_time.clone().into() },
            before: if e.before == MaybeUndefined::Undefined { self.before.clone() } else { e.before.clone().into() },
            after: if e.after == MaybeUndefined::Undefined { self.after.clone() } else { e.after.clone().into() },
            at_location: if e.at_location == MaybeUndefined::Undefined { self.at_location.clone() } else { e.at_location.clone().into() },
            in_scope_of: if e.in_scope_of== MaybeUndefined::Undefined { self.in_scope_of.clone() } else { e.in_scope_of.clone().into() },
            note: if e.note== MaybeUndefined::Undefined { self.note.clone() } else { e.note.clone().into() },
        }
    }
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct CreateRequest {
    #[serde(default)]
    note: MaybeUndefined<String>,
    // action: Action, :TODO:
    #[serde(default)]
    input_of: MaybeUndefined<ProcessOrTransferAddress>,
    #[serde(default)]
    output_of: MaybeUndefined<ProcessOrTransferAddress>,
    #[serde(default)]
    provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    affected_quantity: MaybeUndefined<QuantityValue>,
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

    // LINK FIELDS
    // :TODO: I am glossing over the intermediary Fulfillment for now, just experimenting!
    // :TODO: use newtype alias when HDK supports such type coercion better
    #[serde(default)]
    pub fulfills: MaybeUndefined<Vec<Address>>,
}

impl<'a> CreateRequest {
    // :TODO: accessors for other field data

    pub fn get_fulfills(&'a self) -> Option<Vec<Address>> {
        self.fulfills.clone().into()
    }
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct UpdateRequest {
    id: Address,
    // ENTRY FIELDS
    #[serde(default)]
    note: MaybeUndefined<String>,
    // action: Action, :TODO:
    #[serde(default)]
    input_of: MaybeUndefined<ProcessOrTransferAddress>,
    #[serde(default)]
    output_of: MaybeUndefined<ProcessOrTransferAddress>,
    #[serde(default)]
    provider: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    receiver: MaybeUndefined<AgentAddress>,
    #[serde(default)]
    resource_inventoried_as: MaybeUndefined<ResourceAddress>,
    #[serde(default)]
    resource_classified_as: MaybeUndefined<Vec<ExternalURL>>,
    #[serde(default)]
    resource_conforms_to: MaybeUndefined<ResourceSpecificationAddress>,
    #[serde(default)]
    affected_quantity: MaybeUndefined<QuantityValue>,
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

    // LINK FIELDS
    #[serde(default)]
    fulfills: MaybeUndefined<Vec<Address>>,
}

impl<'a> UpdateRequest {
    pub fn get_id(&'a self) -> &Address {
        &self.id
    }

    // :TODO: accessors for other field data

    pub fn get_fulfills(&'a self) -> Option<Vec<Address>> {
        self.fulfills.clone().into()
    }
}

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    id: EventAddress,
    // ENTRY FIELDS
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    // action: Action, :TODO:
    #[serde(skip_serializing_if = "Option::is_none")]
    input_of: Option<ProcessOrTransferAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_of: Option<ProcessOrTransferAddress>,
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
    affected_quantity: Option<QuantityValue>,
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
    fulfills: Option<Vec<Address>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseData {
    economic_event: Response,
    // :TODO: economic_resource: Option<EconomicResource>,
}

/**
 * Pick relevant fields out of I/O record into underlying DHT entry
 */
impl From<CreateRequest> for Entry {
    fn from(e: CreateRequest) -> Entry {
        Entry {
            note: e.note.into(),
            input_of: e.input_of.into(),
            output_of: e.output_of.into(),
            provider: e.provider.into(),
            receiver: e.receiver.into(),
            resource_inventoried_as: e.resource_inventoried_as.into(),
            resource_classified_as: e.resource_classified_as.into(),
            resource_conforms_to: e.resource_conforms_to.into(),
            affected_quantity: e.affected_quantity.into(),
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
pub fn construct_response(address: Address, e: Entry, fulfillments: Option<Vec<Address>>) -> ResponseData {
    ResponseData {
        economic_event: Response {
            id: address.into(),
            note: e.note,
            input_of: e.input_of,
            output_of: e.output_of,
            provider: e.provider,
            receiver: e.receiver,
            resource_inventoried_as: e.resource_inventoried_as,
            resource_classified_as: e.resource_classified_as,
            resource_conforms_to: e.resource_conforms_to,
            affected_quantity: e.affected_quantity,
            has_beginning: e.has_beginning,
            has_end: e.has_end,
            has_point_in_time: e.has_point_in_time,
            before: e.before,
            after: e.after,
            at_location: e.at_location,
            in_scope_of: e.in_scope_of,
            fulfills: fulfillments,
        }
    }
}

// :TODO: definitions for same-zome link fields & cross-DNA link fields

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_fields() {
        let e = Entry { note: Some("a note".into()), ..Entry::default() };
        assert_eq!(e.note, Some("a note".into()))
    }

    // :TODO: unit tests for type conversions... though maybe these should be macro tests, not tests for every single record type
}
