
use hdk::{
    holochain_json_api::{
        json::JsonString,
    },
};

pub mod builtins;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
enum ActionEffect {
    Neutral,
    Increment,
    Decrement,
}

impl Default for ActionEffect {
    fn default() -> ActionEffect {
        ActionEffect::Neutral
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
pub struct Action<'a> {
    id: &'a str,
    name: &'a str,
    resource_effect: ActionEffect,
}

// impl<'a> TryFrom<JsonString> for Action<'a> {
//     type Error = HolochainError;
//     fn try_from(j: JsonString) -> Result<Self, Self::Error> {
//         default_try_from_json(j)
//     }
// }

impl<'a> From<Action<'a>> for JsonString {
    fn from(result: Action<'a>) -> JsonString {
        JsonString::from_json(&serde_json::to_string(&result).expect("could not Jsonify Action"))
    }
}
