/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";
pub const COMMITMENT_FULFILLEDBY_LINK_TAG: &str = "fulfilled_by";
pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const COMMITMENT_INPUT_OF_LINK_TAG: &str = "input_of";
pub const COMMITMENT_OUTPUT_OF_LINK_TAG: &str = "output_of";
pub const COMMITMENT_CLAUSE_OF_LINK_TAG: &str = "clause_of";

pub const BRIDGED_OBSERVATION_DHT: &str = "vf_observation";
pub const BRIDGED_AGREEMENT_DHT: &str = "vf_agreement";
