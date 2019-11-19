
use hdk::{
    holochain_json_api::{
        json::JsonString,
    },
};

use vf_core::type_aliases::{ ActionId, ProcessAddress };

pub mod builtins;
pub use builtins::get_builtin_action;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ActionEffect {
    // for 'process' events
    NoEffect,
    Increment,
    Decrement,
    // for 'transfer' events
    DecrementIncrement,
}

// actual underlying operations applied to particular resources are a subset of higher-level ActionEffect
pub enum ActionInventoryEffect {
    NoEffect,
    Increment,
    Decrement,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ProcessType {
    NotApplicable,
    Input,
    Output,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Action {
    pub id: String,
    pub label: String,
    pub resource_effect: ActionEffect,
    pub input_output: ProcessType,
    pub pairs_with: String, // any of the action labels, or "notApplicable"
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
