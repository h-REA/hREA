use hdk_type_serialization_macros::*;

use holo_hash::{ AgentPubKey, EntryHash, HeaderHash };

// re-exports for convenience
pub use chrono::{ Local, Utc, DateTime };
pub use holochain_zome_types::timestamp::Timestamp;

addressable_identifier!(RevisionHash, RevisionHashRemote => HeaderHash);

simple_alias!(ActionId => String);

simple_alias!(ExternalURL => String);

addressable_identifier!(LocationAddress, LocationAddressRemote => EntryHash);

simple_alias!(UnitId => String);

addressable_identifier!(AgentAddress, AgentAddressRemote => AgentPubKey);

addressable_identifier!(EventAddress, EventAddressRemote => EntryHash);
addressable_identifier!(ResourceAddress, ResourceAddressRemote => EntryHash);
addressable_identifier!(ProductBatchAddress, ProductBatchAddressRemote => EntryHash);
addressable_identifier!(ProcessAddress, ProcessAddressRemote => EntryHash);

addressable_identifier!(CommitmentAddress, CommitmentAddressRemote => EntryHash);
addressable_identifier!(FulfillmentAddress, FulfillmentAddressRemote => EntryHash);
addressable_identifier!(IntentAddress, IntentAddressRemote => EntryHash);
addressable_identifier!(SatisfactionAddress, SatisfactionAddressRemote => EntryHash);
addressable_identifier!(EventOrCommitmentAddress, EventOrCommitmentAddressRemote => EntryHash);

addressable_identifier!(PlanAddress, PlanAddressRemote => EntryHash);
addressable_identifier!(AgreementAddress, AgreementAddressRemote => EntryHash);

addressable_identifier!(ResourceSpecificationAddress, ResourceSpecificationAddressRemote => EntryHash);
addressable_identifier!(ProcessSpecificationAddress, ProcessSpecificationAddressRemote => EntryHash);

addressable_identifier!(ProposedIntentAddress, ProposedIntentAddressRemote => EntryHash);
addressable_identifier!(ProposalAddress, ProposalAddressRemote => EntryHash);
addressable_identifier!(ProposedToAddress, ProposedToAddressRemote => EntryHash);
