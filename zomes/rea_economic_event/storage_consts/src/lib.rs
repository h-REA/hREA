/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const EVENT_ENTRY_TYPE: &str = "vf_economic_event";

pub const EVENT_FULFILLS_LINK_TAG: &str = "fulfills";
pub const EVENT_SATISFIES_LINK_TAG: &str = "satisfies";
pub const EVENT_INPUT_OF_LINK_TAG: &str = "input_of";
pub const EVENT_OUTPUT_OF_LINK_TAG: &str = "output_of";
pub const EVENT_REALIZATION_OF_LINK_TAG: &str = "realization_of";
pub const EVENT_AFFECTS_RESOURCE_LINK_TAG: &str = "affects";

pub const INVENTORY_CREATION_API_METHOD: &str = "_internal_create_inventory";
pub const INVENTORY_UPDATE_API_METHOD: &str = "_internal_update_inventory";

pub const EVENT_FULFILLS_READ_API_METHOD: &str = "_internal_read_economic_event_fulfills";
pub const EVENT_SATISFIES_READ_API_METHOD: &str = "_internal_read_economic_event_satisfies";

pub const PROCESS_INPUT_INDEXING_API_METHOD: &str = "_internal_reindex_input_events";
pub const PROCESS_OUTPUT_INDEXING_API_METHOD: &str = "_internal_reindex_output_events";
pub const EVENT_INPUTOF_INDEXING_API_METHOD: &str = "_internal_reindex_process_inputs";
pub const EVENT_OUTPUTOF_INDEXING_API_METHOD: &str = "_internal_reindex_process_outputs";

pub const AGREEMENT_REALIZED_INDEXING_API_METHOD: &str = "index_realized_events";
pub const EVENT_REALIZATION_OF_INDEXING_API_METHOD: &str = "_internal_reindex_realized_agreements";
pub const EVENT_REALIZATION_OF_READ_API_METHOD: &str = "_internal_read_realized_agreements";

pub const RESOURCE_AFFECTED_INDEXING_API_METHOD: &str = "_internal_reindex_affecting_events";
pub const EVENT_AFFECTS_INDEXING_API_METHOD: &str = "_internal_reindex_affected_resources";
pub const EVENT_AFFECTS_READ_API_METHOD: &str = "_internal_read_affected_resources";
