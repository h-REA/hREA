/**
 * Type aliases used to ensure explicit awareness of applicable record types in VF structs
 *
 * To convert wrapped values to an `EntryHash`, use `aliased_val.as_ref()`.
 * To convert a plain `EntryHash` to its wrapped form, use `raw_address.into()`.
 */
use holochain_serialized_bytes::prelude::*;
use holo_hash::{ AgentPubKey, EntryHash, HeaderHash };
pub use holochain_zome_types::timestamp::Timestamp;

// A string wrapper around binary IDs which serializes to a string
#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct HashString(String);

#[macro_export]
macro_rules! newtype_wrapper {
    ($id:ident => $base:ty) => {
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
    }
}

#[macro_export]
macro_rules! simple_alias {
    ($id:ident => $base:ty) => {
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
        pub struct $id(pub $base);

        newtype_wrapper!($id => $base);
    }
}

#[macro_export]
macro_rules! hashed_identifier {
    ($id:ident => $base:ty) => {
        #[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
        #[serde(try_from = "HashString")]
        #[serde(into = "HashString")]
        pub struct $id(pub $base);

        newtype_wrapper!($id => $base);

        impl TryFrom<HashString> for $id {
            type Error = String;
            fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
                match <$base>::try_from(ui_string_hash.0) {
                    Ok(address) => Ok(Self(address)),
                    Err(e) => Err(format!("{:?}", e)),
                }
            }
        }
        impl From<$id> for HashString {
            fn from(wrapped_entry_hash: $id) -> Self {
                Self(wrapped_entry_hash.0.to_string())
            }
        }
    }
}

hashed_identifier!(RevisionHash => HeaderHash);

simple_alias!(ActionId => String);

simple_alias!(ExternalURL => String);

hashed_identifier!(LocationAddress => EntryHash);

simple_alias!(UnitId => String);

hashed_identifier!(AgentAddress => AgentPubKey);

hashed_identifier!(EventAddress => EntryHash);
hashed_identifier!(ResourceAddress => EntryHash);
hashed_identifier!(ProductBatchAddress => EntryHash);
hashed_identifier!(ProcessAddress => EntryHash);

hashed_identifier!(CommitmentAddress => EntryHash);
hashed_identifier!(FulfillmentAddress => EntryHash);
hashed_identifier!(IntentAddress => EntryHash);
hashed_identifier!(SatisfactionAddress => EntryHash);
hashed_identifier!(EventOrCommitmentAddress => EntryHash);

hashed_identifier!(PlanAddress => EntryHash);
hashed_identifier!(AgreementAddress => EntryHash);

hashed_identifier!(ResourceSpecificationAddress => EntryHash);
hashed_identifier!(ProcessSpecificationAddress => EntryHash);

hashed_identifier!(ProposedIntentAddress => EntryHash);
hashed_identifier!(ProposalAddress => EntryHash);
hashed_identifier!(ProposedToAddress => EntryHash);
