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
        match $key {
            $(
                stringify!($a) => Some(Action {
                    id: stringify!($a).into(),
                    name: stringify!($a).into(),
                    resource_effect: $e,
                })
            ),*,
            _ => None,
        }
    }
}

pub fn get_builtin_action<'a>(key: &str) -> Option<Action<'a>> {
    generate_builtin_actions!(
        key;
        unload => ActionEffect::Increment,
        load => ActionEffect::Decrement,
        consume => ActionEffect::Decrement,
        use => ActionEffect::Neutral,
        work => ActionEffect::Neutral,
        cite => ActionEffect::Neutral,
        produce => ActionEffect::Increment,
        accept => ActionEffect::Neutral,
        improve => ActionEffect::Neutral,
        give => ActionEffect::Decrement,
        receive => ActionEffect::Increment,
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
            id: "unload",
            name: "unload",
            resource_effect: ActionEffect::Increment,
        };

        assert_eq!(get_builtin_action("unload").unwrap(), action);
    }
}
