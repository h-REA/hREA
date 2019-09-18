/**
 * Core ValueFlows actions.
 *
 * VF has an extended set of built-in actions, which cover a wide variety of common
 * use-cases for the REA grammar. This module exists to predefine them so that they
 * can be used in the system without requiring an genesis action to populate them.
 *
 * @see https://github.com/valueflows/valueflows/issues/487#issuecomment-482161938
 */
use super::{
    Action,
    ActionEffect,
};

// setup for core actions as in-memory statics

macro_rules! generate_builtin_actions {
    ($key: expr; $( $a:ident => $e:expr ),*) => {
        match &str::replace($key, "-", "_")[..] {
            $(
                stringify!($a) => Some(Action {
                    id: str::replace(stringify!($a), "_", "-"),
                    name: str::replace(stringify!($a), "_", "-"),
                    resource_effect: $e,
                })
            ),*,
            _ => None,
        }
    }
}

pub fn get_builtin_action(key: &str) -> Option<Action> {
    generate_builtin_actions!(
        key;
        dropoff => ActionEffect::Increment,
        pickup => ActionEffect::Decrement,
        consume => ActionEffect::Decrement,
        use => ActionEffect::Neutral,
        work => ActionEffect::Neutral,
        cite => ActionEffect::Neutral,
        produce => ActionEffect::Increment,
        accept => ActionEffect::Neutral,
        modify => ActionEffect::Neutral,
        pass => ActionEffect::Neutral,
        fail => ActionEffect::Neutral,
        deliver_service => ActionEffect::Neutral,
        transfer_all_rights => ActionEffect::Decrement, // :NOTE: given in context of the providing agent, opposite is true for toResourceInventoriedAs
        transfer_custody => ActionEffect::Decrement,    // :NOTE: given in context of the providing agent, opposite is true for toResourceInventoriedAs
        transfer_complete => ActionEffect::Decrement,   // :NOTE: given in context of the providing agent, opposite is true for toResourceInventoriedAs
        move => ActionEffect::Decrement,                // :NOTE: given in context of the providing agent, opposite is true for toResourceInventoriedAs
        raise => ActionEffect::Increment,
        lower => ActionEffect::Decrement
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_action_generator() {
        let action = Action {
            id: "consume".to_string(),
            name: "consume".to_string(),
            resource_effect: ActionEffect::Decrement,
        };

        assert_eq!(get_builtin_action("consume").unwrap(), action);
    }
}
