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
    PlanAddress,
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
        pub quantified_as: Option<QuantityValue>,
        pub has_beginning: Timestamp,
        pub has_end: Timestamp,
        pub has_point_in_time: Timestamp,
        pub before: Timestamp,
        pub after: Timestamp,
        pub at_location: LocationAddress,
        pub plan: PlanAddress,
        pub finished: bool,
        pub in_scope_of: Option<Vec<String>>,
        pub note: Option<String>,
    }
// }
