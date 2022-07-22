use hdk_uuid_types::*;

// re-exports for convenience
pub use chrono::{ FixedOffset, Utc, DateTime };
pub use holo_hash::{ AgentPubKey, EntryHash, HeaderHash };
pub use holochain_zome_types::timestamp::Timestamp;
pub use hdk_uuid_types::{DnaAddressable, DnaIdentifiable};
pub use hdk_semantic_indexes_zome_rpc::{ByHeader, ByAddress, ByRevision};

simple_alias!(ActionId => String);

simple_alias!(ExternalURL => String);

addressable_identifier!(LocationAddress => EntryHash);

dna_scoped_string!(UnitId);
addressable_identifier!(UnitInternalAddress => EntryHash);

addressable_identifier!(AgentAddress => EntryHash);

addressable_identifier!(EconomicEventAddress => EntryHash);
addressable_identifier!(EconomicResourceAddress => EntryHash);
addressable_identifier!(ProductBatchAddress => EntryHash);
addressable_identifier!(ProcessAddress => EntryHash);

addressable_identifier!(CommitmentAddress => EntryHash);
addressable_identifier!(FulfillmentAddress => EntryHash);
addressable_identifier!(IntentAddress => EntryHash);
addressable_identifier!(SatisfactionAddress => EntryHash);

addressable_identifier!(PlanAddress => EntryHash);
addressable_identifier!(AgreementAddress => EntryHash);

addressable_identifier!(ResourceSpecificationAddress => EntryHash);
addressable_identifier!(ProcessSpecificationAddress => EntryHash);

addressable_identifier!(ProposedIntentAddress => EntryHash);
addressable_identifier!(ProposalAddress => EntryHash);
addressable_identifier!(ProposedToAddress => EntryHash);

addressable_identifier!(EventOrCommitmentAddress => EntryHash);
