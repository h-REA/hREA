/**
 * ValueFlows Actions zome
 *
 * Provides read-only access to built-in action struct metadata.
 *
 * @package: HoloREA
 * @since:   2019-12-23
 */
use hdk::prelude::*;

use vf_attributes_hdk::{
    ActionId,
};
use vf_actions::{
    Action,
    get_builtin_action,
    get_all_builtin_actions,
};

#[derive(Debug, Serialize, Deserialize)]
struct ById {
    id: ActionId,
}

#[hdk_extern]
fn get_action(ById { id }: ById) -> ExternResult<Action> {
    match get_builtin_action(id.as_ref()) {
        Some(action) => Ok(action),
        None => Err(WasmError::Guest(format!("No action with ID '{}' available", id.as_ref()))),
    }
}

#[hdk_extern]
fn get_all_actions(_: ()) -> ExternResult<Vec<Action>> {
    Ok(get_all_builtin_actions())
}
