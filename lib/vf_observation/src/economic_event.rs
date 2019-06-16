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

use vf_knowledge::action::Action;

use vf_core::{
    measurement::QuantityValue,
};

use vf_core::type_aliases::{
    EventAddressRequired,
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
        pub input_of: ProcessOrTransferAddress,
        pub output_of: ProcessOrTransferAddress,
        pub provider: AgentAddress,
        pub receiver: AgentAddress,
        pub resource_inventoried_as: ResourceAddress,
        pub resource_classified_as: Option<Vec<ExternalURL>>,
        pub resource_conforms_to: ResourceSpecificationAddress,
        pub affected_quantity: Option<QuantityValue>,
        pub has_beginning: Timestamp,
        pub has_end: Timestamp,
        pub has_point_in_time: Timestamp,
        pub before: Timestamp,
        pub after: Timestamp,
        pub at_location: LocationAddress,
        pub in_scope_of: Option<Vec<String>>,
        pub note: Option<String>,
    }
// }

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct CreateRequest {
    // ENTRY FIELDS
    note: Option<String>,
    // action: Action, :TODO:
    input_of: ProcessOrTransferAddress,
    output_of: ProcessOrTransferAddress,
    provider: AgentAddress,
    receiver: AgentAddress,
    resource_inventoried_as: ResourceAddress,
    resource_classified_as: Option<Vec<ExternalURL>>,
    resource_conforms_to: ResourceSpecificationAddress,
    affected_quantity: Option<QuantityValue>,
    has_beginning: Timestamp,
    has_end: Timestamp,
    has_point_in_time: Timestamp,
    before: Timestamp,
    after: Timestamp,
    at_location: LocationAddress,
    in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    // :TODO: I am glossing over the intermediary Fulfillment for now, just experimenting!
    // :TODO: use newtype alias when HDK supports such type coercion better
    pub fulfills: Option<Vec<Address>>,
}

/// I/O struct to describe the complete input record, including all managed links
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct UpdateRequest {
    id: Address,
    // ENTRY FIELDS
    note: Option<String>,
    // action: Action, :TODO:
    input_of: ProcessOrTransferAddress,
    output_of: ProcessOrTransferAddress,
    provider: AgentAddress,
    receiver: AgentAddress,
    resource_inventoried_as: ResourceAddress,
    resource_classified_as: Option<Vec<ExternalURL>>,
    resource_conforms_to: ResourceSpecificationAddress,
    affected_quantity: Option<QuantityValue>,
    has_beginning: Timestamp,
    has_end: Timestamp,
    has_point_in_time: Timestamp,
    before: Timestamp,
    after: Timestamp,
    at_location: LocationAddress,
    in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    pub fulfills: Option<Vec<Address>>,
}

impl UpdateRequest {
    pub fn get_id(&self) -> Address {
        self.id.clone()
    }
}

/// I/O struct to describe the complete output record, including all managed link fields
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
pub struct Response {
    id: EventAddressRequired,
    // ENTRY FIELDS
    note: Option<String>,
    // action: Action, :TODO:
    input_of: ProcessOrTransferAddress,
    output_of: ProcessOrTransferAddress,
    provider: AgentAddress,
    receiver: AgentAddress,
    resource_inventoried_as: ResourceAddress,
    resource_classified_as: Option<Vec<ExternalURL>>,
    resource_conforms_to: ResourceSpecificationAddress,
    affected_quantity: Option<QuantityValue>,
    has_beginning: Timestamp,
    has_end: Timestamp,
    has_point_in_time: Timestamp,
    before: Timestamp,
    after: Timestamp,
    at_location: LocationAddress,
    in_scope_of: Option<Vec<String>>,

    // LINK FIELDS
    fulfills: Option<Vec<Address>>,
}

/// I/O struct to describe what is returned outside the gateway
#[derive(Serialize, Deserialize, Debug, DefaultJson, Default, Clone)]
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
        }
    }
}

impl From<UpdateRequest> for Entry {
    fn from(e: UpdateRequest) -> Entry {
        Entry {
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
