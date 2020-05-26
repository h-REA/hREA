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

#[zome]
mod rea_agent_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_my_agent() -> ZomeApiResult<ResponseData> {
        Ok(ResponseData {
            agent: Response {
                id: AgentAddress::from(AGENT_ADDRESS.to_string()),
            },
        })
    }
}
