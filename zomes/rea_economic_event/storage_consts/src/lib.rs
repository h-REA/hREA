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

pub const BRIDGED_AGREEMENT_DHT: &str = "vf_agreement";

pub const INVENTORY_CREATION_API_METHOD: &str = "_internal_create_inventory";
pub const INVENTORY_UPDATE_API_METHOD: &str = "_internal_update_inventory";
