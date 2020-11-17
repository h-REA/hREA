/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 *
 * To convert wrapped values to an `Address`, use `aliased_val.as_ref()`.
 * To convert plain Addresses to aliases, use `raw_address.into()`.
 */

use hdk3::prelude::*;
pub use hdk3::prelude::timestamp::Timestamp;
pub use hdk3::prelude::HeaderHash;

// :DUPE: newtype-macro-rules
macro_rules! simple_alias {
    ($id:ident => $base:ty) => {
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
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

simple_alias!(ExternalURL => String);

simple_alias!(LocationAddress => EntryHash);

simple_alias!(UnitId => String);

simple_alias!(AgentAddress => AgentPubKey);

simple_alias!(EventAddress => EntryHash);
simple_alias!(ResourceAddress => EntryHash);
simple_alias!(ProductBatchAddress => EntryHash);
simple_alias!(ProcessAddress => EntryHash);

simple_alias!(CommitmentAddress => EntryHash);
simple_alias!(FulfillmentAddress => EntryHash);
simple_alias!(IntentAddress => EntryHash);
simple_alias!(SatisfactionAddress => EntryHash);
simple_alias!(EventOrCommitmentAddress => EntryHash);

simple_alias!(PlanAddress => EntryHash);
simple_alias!(AgreementAddress => EntryHash);

simple_alias!(ResourceSpecificationAddress => EntryHash);
simple_alias!(ProcessSpecificationAddress => EntryHash);

simple_alias!(ProposedIntentAddress => EntryHash);
simple_alias!(ProposalAddress => EntryHash);
simple_alias!(ProposedToAddress => EntryHash);
