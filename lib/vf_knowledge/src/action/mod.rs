pub mod builtins;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Action {
    id: String,
    name: String,
    resource_effect: ActionEffect,
}
