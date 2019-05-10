// trace_macros!(true);

use hdk::holochain_core_types::{
    json::JsonString,
    cas::content::Address,
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

vfRecord! {
    pub struct EconomicEventEntry {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_fields() {
        let e = EconomicEventEntry { note: Some("a note".into()), ..EconomicEventEntry::default() };
        assert_eq!(e.note, Some("a note".into()))
    }
}
