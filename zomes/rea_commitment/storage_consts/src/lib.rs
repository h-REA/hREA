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

pub const COMMITMENT_FULFILLEDBY_READ_API_METHOD: &str = "_internal_read_commitment_fulfillments";
pub const COMMITMENT_SATISFIES_READ_API_METHOD: &str = "_internal_read_commitment_satisfactions";

pub const COMMITMENT_INPUT_READ_API_METHOD: &str = "_internal_read_commitment_process_inputs";
pub const COMMITMENT_INPUT_INDEXING_API_METHOD: &str = "_internal_reindex_process_inputs";
pub const PROCESS_INPUT_INDEXING_API_METHOD: &str = "index_process_input_commitments";

pub const COMMITMENT_OUTPUT_READ_API_METHOD: &str = "_internal_read_commitment_process_outputs";
pub const COMMITMENT_OUTPUT_INDEXING_API_METHOD: &str = "_internal_reindex_process_outputs";
pub const PROCESS_OUTPUT_INDEXING_API_METHOD: &str = "index_process_output_commitments";

pub const COMMITMENT_CLAUSEOF_READ_API_METHOD: &str = "_internal_read_commitment_agreements";
pub const COMMITMENT_CLAUSEOF_INDEXING_API_METHOD: &str = "_internal_reindex_agreement_clauses";
pub const AGREEMENT_CLAUSE_INDEXING_API_METHOD: &str = "index_agreement_clauses";
