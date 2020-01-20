---
to: <%=dna_path%>/zomes/<%= h.changeCase.snake(zome_name) %>/code/src/lib.rs
---
#![feature(proc_macro_hygiene)]
/**
 * <%=zome_friendly_name%> zome API definition
 *
 * Defines the top-level zome configuration needed by Holochain's build system
 * to bundle the app. This basically involves wiring up the helper methods from the
 * related `_lib` module into a packaged zome WASM binary.
 *
 * @package Holo-REA
 */
extern crate serde;
extern crate hdk;
extern crate hdk_proc_macros;

use hdk::prelude::*;
use hdk_proc_macros::zome;

use hdk_graph_helpers::remote_indexes::RemoteEntryLinkResponse; // :TODO: wire up remote indexing API if necessary

use hc_zome_<%= h.changeCase.snake(zome_name) %>_defs::{ entry_def, base_entry_def };
use hc_zome_<%= h.changeCase.snake(zome_name) %>_rpc::*;
use hc_zome_<%= h.changeCase.snake(zome_name) %>_lib::*;


// Zome entry type wrappers
#[zome]
mod <%= h.changeCase.snake(zome_name) %>_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn <%= h.changeCase.snake(record_type_name) %>_entry_def() -> ValidatingEntryType {
        entry_def()
    }

    #[entry_def]
    fn <%= h.changeCase.snake(record_type_name) %>_base_entry_def() -> ValidatingEntryType {
        base_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>: CreateRequest) -> ZomeApiResult<ResponseData> {
        receive_create_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>)
    }

    #[zome_fn("hc_public")]
    fn get_<%= h.changeCase.snake(record_type_name) %>(address: <%= h.changeCase.pascal(record_type_name) %>Address) -> ZomeApiResult<ResponseData> {
        receive_get_<%= h.changeCase.snake(record_type_name) %>(address)
    }

    #[zome_fn("hc_public")]
    fn update_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>: UpdateRequest) -> ZomeApiResult<ResponseData> {
        receive_update_<%= h.changeCase.snake(record_type_name) %>(<%= h.changeCase.snake(record_type_name) %>)
    }

    #[zome_fn("hc_public")]
    fn delete_<%= h.changeCase.snake(record_type_name) %>(address: <%= h.changeCase.pascal(record_type_name) %>Address) -> ZomeApiResult<bool> {
        receive_delete_<%= h.changeCase.snake(record_type_name) %>(address)
    }

    #[zome_fn("hc_public")]
    fn query_<%= h.inflection.pluralize(h.changeCase.snake(record_type_name)) %>(params: QueryParams) -> ZomeApiResult<Vec<ResponseData>>{
        receive_query_<%= h.inflection.pluralize(h.changeCase.snake(record_type_name)) %>(params)
    }

    // :TODO: wire up remote indexing API if necessary

    // :TODO:
    // receive: |from, payload| {
    //     format!("Received: {} from {}", payload, from)
    // }
}
