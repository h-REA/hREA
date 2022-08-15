/**
 * Core ValueFlows actions.
 *
 * VF has an extended set of built-in actions, which cover a wide variety of common
 * use-cases for the REA grammar. This module exists to predefine them so that they
 * can be used in the system without requiring an genesis action to populate them.
 *
 * @see https://github.com/valueflows/valueflows/issues/487#issuecomment-482161938
 */
use super::{Action, ActionEffect, ProcessType};

// setup for core actions as in-memory statics

macro_rules! generate_builtin_actions {
    ($key: expr; $( $a:ident => $e:expr, $f:expr, $g:expr, $h:expr );*) => {
        match &str::replace($key, "-", "_")[..] {
            $(
                stringify!($a) => Some(Action {
                    id: str::replace(stringify!($a), "_", "-"),
                    label: str::replace(stringify!($a), "_", "-"),
                    resource_effect: $e,
                    onhand_effect: $f,
                    input_output: $g,
                    pairs_with: stringify!($h).to_string(),
                })
            ),*,
            _ => None,
        }
    }
}

// The first ActionEffect here is what is called the 'resource_effect' or
// the 'accounting effect'. The second ActionEffect define the 'onhand_effect'.
// They are separate because they can differ from each other for a given action.
pub fn get_builtin_action(key: &str) -> Option<Action> {
    generate_builtin_actions!(
        key;
        dropoff => ActionEffect::Increment, ActionEffect::Increment, ProcessType::Output, pickup;
        pickup => ActionEffect::Decrement, ActionEffect::Decrement, ProcessType::Input, dropoff;
        consume => ActionEffect::Decrement, ActionEffect::Decrement, ProcessType::Input, notApplicable;
        use => ActionEffect::NoEffect, ActionEffect::NoEffect, ProcessType::Input, notApplicable;
        work => ActionEffect::NoEffect, ActionEffect::NoEffect, ProcessType::Input, notApplicable;
        cite => ActionEffect::NoEffect, ActionEffect::NoEffect, ProcessType::Input, notApplicable;
        produce => ActionEffect::Increment, ActionEffect::Increment, ProcessType::Output, notApplicable;
        accept => ActionEffect::NoEffect, ActionEffect::Decrement, ProcessType::Input, modify;
        modify => ActionEffect::NoEffect, ActionEffect::Increment, ProcessType::Output, accept;
        pass => ActionEffect::NoEffect, ActionEffect::NoEffect, ProcessType::Output, accept;
        fail => ActionEffect::NoEffect, ActionEffect::NoEffect, ProcessType::Output, accept;
        deliver_service => ActionEffect::NoEffect, ActionEffect::NoEffect, ProcessType::Output, notApplicable;
        transfer_all_rights => ActionEffect::DecrementIncrement, ActionEffect::NoEffect, ProcessType::NotApplicable, notApplicable;
        transfer_custody => ActionEffect::NoEffect, ActionEffect::DecrementIncrement, ProcessType::NotApplicable, notApplicable;
        transfer => ActionEffect::DecrementIncrement, ActionEffect::DecrementIncrement, ProcessType::NotApplicable, notApplicable;
        move => ActionEffect::DecrementIncrement, ActionEffect::DecrementIncrement, ProcessType::NotApplicable, notApplicable;
        raise => ActionEffect::Increment, ActionEffect::Increment, ProcessType::NotApplicable, notApplicable;
        lower => ActionEffect::Decrement, ActionEffect::Decrement, ProcessType::NotApplicable, notApplicable
    )
}

pub fn get_all_builtin_actions() -> Vec<Action> {
    vec![
        get_builtin_action("dropoff").unwrap(),
        get_builtin_action("pickup").unwrap(),
        get_builtin_action("consume").unwrap(),
        get_builtin_action("use").unwrap(),
        get_builtin_action("work").unwrap(),
        get_builtin_action("cite").unwrap(),
        get_builtin_action("produce").unwrap(),
        get_builtin_action("accept").unwrap(),
        get_builtin_action("modify").unwrap(),
        get_builtin_action("pass").unwrap(),
        get_builtin_action("fail").unwrap(),
        get_builtin_action("deliver_service").unwrap(),
        get_builtin_action("transfer_all_rights").unwrap(),
        get_builtin_action("transfer_custody").unwrap(),
        get_builtin_action("transfer").unwrap(),
        get_builtin_action("move").unwrap(),
        get_builtin_action("raise").unwrap(),
        get_builtin_action("lower").unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_action_generator() {
        let action = Action {
            id: "consume".to_string(),
            label: "consume".to_string(),
            resource_effect: ActionEffect::Decrement,
            onhand_effect: ActionEffect::Decrement,
            input_output: ProcessType::Input,
            pairs_with: "notApplicable".to_string(),
        };

        assert_eq!(get_builtin_action("consume").unwrap(), action);
    }
}
