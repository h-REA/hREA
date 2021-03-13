#![feature(proc_macro_hygiene)]
/**
 * ValueFlows Actions zome
 *
 * Provides read-only access to built-in action struct metadata.
 *
 * @package: HoloREA
 * @since:   2019-12-23
 */
use hdk::prelude::*;
use hdk_proc_macros::zome;

use vf_attributes_hdk::{
    ActionId,
};
use vf_actions::{
    Action,
    get_builtin_action,
    get_all_builtin_actions,
};

#[zome]
mod rea_specification_actions_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData::<AgentId>) {
        Ok(())
    }

    // receive: |from, payload| {
    //   format!("Received: {} from {}", payload, from)
    // }

    #[zome_fn("hc_public")]
    fn get_action(id: ActionId) -> ZomeApiResult<Action> {
        match get_builtin_action(id.as_ref()) {
            Some(action) => Ok(action),
            None => Err(ZomeApiError::Internal(format!("No action with ID '{}' available", id.as_ref()))),
        }
    }

    #[zome_fn("hc_public")]
    fn get_all_actions() -> ZomeApiResult<Vec<Action>> {
        Ok(get_all_builtin_actions())
    }
}
