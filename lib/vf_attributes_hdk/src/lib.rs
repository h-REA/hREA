use hdk_type_serialization_macros::*;

// re-exports for convenience
pub use chrono::{ Local, Utc, DateTime };
pub use holo_hash::{ AgentPubKey, EntryHash, HeaderHash };
pub use holochain_zome_types::timestamp::Timestamp;
pub use hdk_type_serialization_macros::{RevisionHash, DnaAddressable};

simple_alias!(ActionId => String);

simple_alias!(ExternalURL => String);

addressable_identifier!(LocationAddress => EntryHash);

simple_alias!(UnitId => String);

addressable_identifier!(AgentAddress => AgentPubKey);

addressable_identifier!(EventAddress => EntryHash);
addressable_identifier!(ResourceAddress => EntryHash);
addressable_identifier!(ProductBatchAddress => EntryHash);
addressable_identifier!(ProcessAddress => EntryHash);

addressable_identifier!(CommitmentAddress => EntryHash);
addressable_identifier!(FulfillmentAddress => EntryHash);
addressable_identifier!(IntentAddress => EntryHash);
addressable_identifier!(SatisfactionAddress => EntryHash);
addressable_identifier!(EventOrCommitmentAddress => EntryHash);

addressable_identifier!(PlanAddress => EntryHash);
addressable_identifier!(AgreementAddress => EntryHash);

addressable_identifier!(ResourceSpecificationAddress => EntryHash);
addressable_identifier!(ProcessSpecificationAddress => EntryHash);

addressable_identifier!(ProposedIntentAddress => EntryHash);
addressable_identifier!(ProposalAddress => EntryHash);
addressable_identifier!(ProposedToAddress => EntryHash);
