/**
 * ValueFlows Actions zome
 *
 * Provides read-only access to built-in action struct metadata.
 *
 * @package: HoloREA
 * @since:   2019-12-23
 */
use hdk::prelude::*;

use vf_actions::{get_all_builtin_actions, get_builtin_action, Action};

#[hdk_extern]
fn get_action(id: String) -> ExternResult<Action> {
    match get_builtin_action(id.as_ref()) {
        Some(action) => Ok(action),
        None => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "No action with ID '{}' available",
            id
        )))),
    }
}

#[hdk_extern]
fn get_all_actions(_: ()) -> ExternResult<Vec<Action>> {
    Ok(get_all_builtin_actions())
}
