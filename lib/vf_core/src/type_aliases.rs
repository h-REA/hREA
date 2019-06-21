/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 */
use hdk::holochain_core_types::{
    cas::content::Address,
    json::JsonString,
    error::HolochainError,
    time::Iso8601,
};
use holochain_core_types_derive::{ DefaultJson };

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
simple_alias!(ProcessOrTransferAddress => Address);

simple_alias!(CommitmentAddress => Address);
simple_alias!(PlanAddress => Address);

simple_alias!(ResourceSpecificationAddress => Address);
