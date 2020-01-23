/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const COMMITMENT_BASE_ENTRY_TYPE: &str = "vf_commitment_baseurl";
pub const COMMITMENT_INITIAL_ENTRY_LINK_TYPE: &str = "vf_commitment_entry";
pub const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";
pub const COMMITMENT_FULFILLEDBY_LINK_TYPE: &str = "vf_commitment_fulfilled_by";
pub const COMMITMENT_FULFILLEDBY_LINK_TAG: &str = "fulfills";
pub const COMMITMENT_SATISFIES_LINK_TYPE: &str = "vf_commitment_satisfies";
pub const COMMITMENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const COMMITMENT_INPUT_OF_LINK_TYPE: &str = "vf_commitment_input_of";
pub const COMMITMENT_INPUT_OF_LINK_TAG: &str = "input_of";
pub const COMMITMENT_OUTPUT_OF_LINK_TYPE: &str = "vf_commitment_output_of";
pub const COMMITMENT_OUTPUT_OF_LINK_TAG: &str = "output_of";

pub const BRIDGED_OBSERVATION_DHT: &str = "vf_observation";
