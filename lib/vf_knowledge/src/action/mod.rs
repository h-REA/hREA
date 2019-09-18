
use hdk::{
    holochain_json_api::{
        json::JsonString,
    },
};

pub mod builtins;
pub use builtins::get_builtin_action;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ActionEffect {
    Neutral,
    Increment,
    Decrement,
}

impl Default for ActionEffect {
    fn default() -> ActionEffect {
        ActionEffect::Neutral
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct Action {
    pub id: String,
    name: String,
    pub resource_effect: ActionEffect,
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
