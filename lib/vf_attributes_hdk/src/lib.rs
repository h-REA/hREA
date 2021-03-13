#[macro_use]
use hdk_type_serialization_macros::*;

use holo_hash::{ AgentPubKey, EntryHash, HeaderHash };

// re-exports for convenience
pub use chrono::{ Local, Utc, DateTime };
pub use holochain_zome_types::timestamp::Timestamp;

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
