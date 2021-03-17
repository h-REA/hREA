use hdk_type_serialization_macros::*;

// re-exports for convenience
pub use chrono::{ Local, Utc, DateTime };
pub use holo_hash::{ AgentPubKey, EntryHash, HeaderHash };
pub use holochain_zome_types::timestamp::Timestamp;
pub use hdk_type_serialization_macros::{RevisionHash, DnaAddressable};

simple_alias!(ActionId => String);

simple_alias!(ExternalURL => String);

addressable_identifier!(LocationRef => EntryHash);

simple_alias!(UnitId => String);

addressable_identifier!(AgentRef => AgentPubKey);

addressable_identifier!(EventRef => EntryHash);
addressable_identifier!(ResourceRef => EntryHash);
addressable_identifier!(ProductBatchRef => EntryHash);
addressable_identifier!(ProcessRef => EntryHash);

addressable_identifier!(CommitmentRef => EntryHash);
addressable_identifier!(FulfillmentRef => EntryHash);
addressable_identifier!(IntentRef => EntryHash);
addressable_identifier!(SatisfactionRef => EntryHash);
addressable_identifier!(EventOrCommitmentRef => EntryHash);

addressable_identifier!(PlanRef => EntryHash);
addressable_identifier!(AgreementRef => EntryHash);

addressable_identifier!(ResourceSpecificationRef => EntryHash);
addressable_identifier!(ProcessSpecificationRef => EntryHash);

addressable_identifier!(ProposedIntentRef => EntryHash);
addressable_identifier!(ProposalRef => EntryHash);
addressable_identifier!(ProposedToRef => EntryHash);
