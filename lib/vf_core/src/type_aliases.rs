/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 *
 * Note that all types are defined as optional by default, since most fields in VF are optional.
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
        #[derive(Serialize, Deserialize, DefaultJson, Debug, Default, Clone)]
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

simple_alias!(Timestamp => Option<Iso8601>);

simple_alias!(ExternalURL => Option<String>);

simple_alias!(LocationAddress => Option<Address>);

simple_alias!(UnitAddress => Option<Address>);

simple_alias!(AgentAddress => Option<Address>);

simple_alias!(ResourceAddress => Option<Address>);
simple_alias!(ProcessOrTransferAddress => Option<Address>);

simple_alias!(CommitmentAddress => Option<Address>);
simple_alias!(CommitmentAddress_Required => Address);
simple_alias!(PlanAddress => Option<Address>);

simple_alias!(ResourceSpecificationAddress => Option<Address>);

simple_alias!(IntentAddress_required => Address);
