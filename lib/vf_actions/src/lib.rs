/**
 * ValueFlows-compatible action type definitions and records
 *
 * @package Holo-REA
 * @since
 */
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use holochain_json_api::json::JsonString;

use vf_core::type_aliases::{ ActionId, ProcessAddress, ResourceAddress };

pub mod builtins;
pub use builtins::{ get_builtin_action, get_all_builtin_actions };

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ActionEffect {
    // for 'process' events
    NoEffect,
    Increment,
    Decrement,
    // for 'transfer' events
    DecrementIncrement,
}

// actual underlying operations applied to particular resources are a subset of higher-level ActionEffect
#[derive(Debug)]
pub enum ActionInventoryEffect {
    NoEffect,
    Increment,
    Decrement,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ProcessType {
    NotApplicable,
    Input,
    Output,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub id: String,
    pub label: String,
    pub resource_effect: ActionEffect,
    pub input_output: ProcessType,
    pub pairs_with: String, // any of the action labels, or "notApplicable"
}

/**
 * Validation for EconomicEvent, Commitment and Process to ensure correct use of actions & Processes
 */
pub fn validate_flow_action(action_id: ActionId, input_process: Option<ProcessAddress>, output_process: Option<ProcessAddress>) -> Result<(), String> {
    if let Some(action) = get_builtin_action(action_id.as_ref()) {
        match action.input_output {
            ProcessType::NotApplicable => if input_process.is_some() || output_process.is_some() {
                Err(format!("EconomicEvent of '{:}' action cannot link to processes", action.id).into())
            } else { Ok(()) },
            ProcessType::Input => if input_process.is_none() {
                Err(format!("EconomicEvent input process required for '{:}' action", action.id).into())
            } else { Ok(()) },
            ProcessType::Output => if output_process.is_none() {
                Err(format!("EconomicEvent output process required for '{:}' action", action.id).into())
            } else { Ok(()) },
        }
    } else {
        Err("Unknown action".to_string())
    }
}

pub fn validate_move_inventories(resouce_inventoried_as: Option<ResourceAddress>, to_resource_inventoried_as: Option<ResourceAddress>) -> Result<(), String> {
    match resouce_inventoried_as {
        Some(_) => match to_resource_inventoried_as {
            Some(_) => Ok(()),
            None => Err("inventoried move EconomicEvent requires both source and destination inventory fields".into()),
        },
        None => match to_resource_inventoried_as {
            None => Ok(()),
            Some(_) => Err("non-inventoried move EconomicEvent must omit inventory fields".into()),
        },
    }
}

// impl<'a> TryFrom<JsonString> for Action<'a> {
//     type Error = HolochainError;
//     fn try_from(j: JsonString) -> Result<Self, Self::Error> {
//         default_try_from_json(j)
//     }
// }

impl From<Action> for JsonString {
    fn from(result: Action) -> JsonString {
        JsonString::from_json(&serde_json::to_string(&result).expect("could not Jsonify Action"))
    }
}
