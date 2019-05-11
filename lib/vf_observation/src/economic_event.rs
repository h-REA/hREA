// trace_macros!(true);

use hdk::holochain_core_types::{
    json::JsonString,
    error::HolochainError,
};
use hdk::holochain_core_types_derive::{ DefaultJson };

use vf_knowledge::action::Action;

use vf_core::{
    measurement::QuantityValue,
};

use vf_core::type_aliases::{
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

// :TODO: definitions for same-zome link fields & cross-DNA link fields

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_fields() {
        let e = Entry { note: Some("a note".into()), ..Entry::default() };
        assert_eq!(e.note, Some("a note".into()))
    }
}
