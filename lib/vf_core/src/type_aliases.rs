/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 *
 * To convert wrapped values to an `Address`, use `aliased_val.as_ref()`.
 * To convert plain Addresses to aliases, use `raw_address.into()`.
*/
use hdk::{
    holochain_json_api::{
        json::JsonString,
        error::JsonError,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_core_types::{
        time::Iso8601,
    },
};
use holochain_json_derive::{ DefaultJson };

macro_rules! simple_alias {
    ($id:ident => $base:ty) => {
        #[derive(Serialize, Deserialize, DefaultJson, Debug, Clone, PartialEq)]
        pub struct $id($base);

        impl From<$id> for $base {
            fn from(v: $id) -> $base {
                v.0
            }
        }

        impl From<$base> for $id {
            fn from (v: $base) -> $id {
                $id(v)
            }
        }

        impl AsRef<$base> for $id {
            fn as_ref(&self) -> &$base {
                &self.0
            }
        }
    };
}

simple_alias!(ActionId => String);

simple_alias!(Timestamp => Iso8601);

simple_alias!(ExternalURL => String);

simple_alias!(LocationAddress => Address);

simple_alias!(UnitAddress => Address);

simple_alias!(AgentAddress => Address);

simple_alias!(EventAddress => Address);
simple_alias!(ResourceAddress => Address);
simple_alias!(ProcessAddress => Address);

simple_alias!(CommitmentAddress => Address);
simple_alias!(FulfillmentAddress => Address);
simple_alias!(IntentAddress => Address);
simple_alias!(SatisfactionAddress => Address);
simple_alias!(EventOrCommitmentAddress => Address);

simple_alias!(PlanAddress => Address);
simple_alias!(AgreementAddress => Address);

simple_alias!(ResourceSpecificationAddress => Address);
simple_alias!(ProcessSpecificationAddress => Address);
