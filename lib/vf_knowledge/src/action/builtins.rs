/**
 * Core ValueFlows actions.
 *
 * VF has an extended set of built-in actions, which cover a wide variety of common
 * use-cases for the REA grammar. This module exists to predefine them so that they
 * can be used in the system without requiring an genesis action to populate them.
 *
 * @see https://github.com/valueflows/valueflows/issues/487#issuecomment-482161938
 */
use std::collections::HashMap;

use super::{
    Action,
    ActionEffect,
};

// setup for core actions as in-memory statics
macro_rules! builtin_action {
    ($m:ident, $id:expr, $effect_type:expr) => {
        $m.insert($id, Action {
            id: $id.into(),
            name: $id.into(),
            resource_effect: $effect_type,
        });
    };
}

lazy_static! {
    pub static ref BUILTIN_ACTIONS: HashMap<&'static str, Action<'static>> = {
        let mut m = HashMap::new();
        builtin_action!( m, "unload", ActionEffect::Increment );
        builtin_action!( m, "load", ActionEffect::Decrement );
        builtin_action!( m, "consume", ActionEffect::Decrement );
        builtin_action!( m, "use", ActionEffect::Neutral );
        builtin_action!( m, "work", ActionEffect::Neutral );
        builtin_action!( m, "cite", ActionEffect::Neutral );
        builtin_action!( m, "produce", ActionEffect::Increment );
        builtin_action!( m, "accept", ActionEffect::Neutral );
        builtin_action!( m, "improve", ActionEffect::Neutral );
        builtin_action!( m, "give", ActionEffect::Decrement );
        builtin_action!( m, "receive", ActionEffect::Increment );
        builtin_action!( m, "raise", ActionEffect::Increment );
        builtin_action!( m, "lower", ActionEffect::Decrement );
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_action_statics() {
        let action = Action {
            id: "unload",
            name: "unload",
            resource_effect: ActionEffect::Increment,
        };

        assert_eq!(BUILTIN_ACTIONS.get("unload").unwrap(), &action);
    }
}
