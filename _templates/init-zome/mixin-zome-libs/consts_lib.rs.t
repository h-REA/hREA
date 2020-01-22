---
to: lib/<%= h.changeCase.snake(zome_name) %>/storage_consts/src/lib.rs
---
/**
 * Storage constants for zome entry & link type identifiers
 *
 * Used by modules interfacing with the underlying Holochain storage system directly.
 *
 * @package Holo-REA
 */
pub const <%= h.changeCase.constant(record_type_name) %>_BASE_ENTRY_TYPE: &str = "vf_<%= h.changeCase.snake(record_type_name) %>_baseurl";
pub const <%= h.changeCase.constant(record_type_name) %>_INITIAL_ENTRY_LINK_TYPE: &str = "vf_<%= h.changeCase.snake(record_type_name) %>_entry";
pub const <%= h.changeCase.constant(record_type_name) %>_ENTRY_TYPE: &str = "vf_<%= h.changeCase.snake(record_type_name) %>";
// :TODO: replace with correct links for record type
pub const <%= h.changeCase.constant(record_type_name) %>_PARENTS_LINK_TYPE: &str = "vf_<%= h.changeCase.snake(record_type_name) %>_parents";
pub const <%= h.changeCase.constant(record_type_name) %>_PARENTS_LINK_TAG: &str = "parents";
