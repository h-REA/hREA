#[derive(Debug, Clone)]
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

#[derive(Debug, Default, Clone)]
pub struct Action {
    id: String,
    name: String,
    resource_effect: ActionEffect,
}
