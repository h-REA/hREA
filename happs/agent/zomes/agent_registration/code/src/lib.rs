#![feature(proc_macro_hygiene)]
/**
 * Holo-REA agent zome API definition
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
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

// use hc_zome_rea_agent_defs::{ entry_def, base_entry_def };
use hc_zome_rea_agent_rpc::*;
// use hc_zome_rea_agent_lib::*;

use hc_zome_agent_registration_storage;
use hc_zome_agent_registration_lib;

#[zome]
mod rea_agent_zome {

    #[init]
    fn init() {
        hc_zome_agent_registration_storage::init()
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        hc_zome_agent_registration_storage::handle_agent_registration(validation_data)
    }

    #[entry_def]
    pub fn agents_root_entry_def() -> ValidatingEntryType {
        hc_zome_agent_registration_storage::agents_root_entry_def()
    }

    #[zome_fn("hc_public")]
    fn get_my_agent() -> ZomeApiResult<ResponseData> {
        // :TODO:
        Ok(ResponseData {
            agent: Response {
                id: AgentAddress::from(AGENT_ADDRESS.to_string()),
            },
        })
    }

    #[zome_fn("hc_public")]
    pub fn is_registered_agent(address: Address) -> ZomeApiResult<bool> {
        hc_zome_agent_registration_lib::is_registered_agent(address)
    }

    #[zome_fn("hc_public")]
    pub fn get_registered_agents() -> ZomeApiResult<Vec<Address>> {
        hc_zome_agent_registration_lib::get_registered_agents()
    }
}
