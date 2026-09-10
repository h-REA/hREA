//! Zome-boundary tests for `ReaIntent`.
//!
//! Intent is where two rules are reachable that no other entry type exposes
//! well: the required-string check on `action` (Intent's action is a bare
//! `String`, so an empty one gets past deserialization), and the immutability
//! of the parties and the action on update, which `update_rea_intent` reaches
//! because `merge_partial` overlays whatever the caller sends.
//!
//! `minimumQuantity <= availableQuantity` is Intent's own rule and had no test
//! before this file.

use holochain::prelude::*;
use hrea_sweettest::{
    create_agent, empty_intent, quantity, rejection, run, seconds, shared_env,
    ReaIntentUpdateParams, UpdateReaIntentInput,
};

/// The required-string check sits one line above the vocabulary check, so an
/// empty action must produce the emptiness message rather than the "not a valid
/// ValueFlows action" one. Asserting the exact message is what separates them.
#[test]
fn rejects_an_empty_action() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_intent", empty_intent(""))
            .await;
        let msg = rejection(result.expect_err("an intent with no action is not an intent"));
        assert!(
            msg.contains("Intent action must not be empty"),
            "expected the required-string message, got: {msg}"
        );
    })
}

/// The rule trims before testing, so whitespace is not a way past it.
#[test]
fn rejects_a_whitespace_only_action() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_intent", empty_intent("   "))
            .await;
        let msg = rejection(result.expect_err("blank is empty once trimmed"));
        assert!(
            msg.contains("Intent action must not be empty"),
            "expected the required-string message, got: {msg}"
        );
    })
}

/// Past the emptiness check, the vocabulary check is the next thing an action
/// has to clear, and it names the offending value.
#[test]
fn rejects_an_action_outside_the_vf_vocabulary() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_intent", empty_intent("barter"))
            .await;
        let msg = rejection(result.expect_err("'barter' is not a ValueFlows action"));
        assert!(
            msg.contains("Intent action 'barter' is not a valid ValueFlows action"),
            "expected the action vocabulary message, got: {msg}"
        );
    })
}

#[test]
fn rejects_a_negative_available_quantity() {
    run(async {
        let env = shared_env().await;
        let mut intent = empty_intent("produce");
        intent.available_quantity = Some(quantity(-3.0));

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_intent", intent)
            .await;
        let msg = rejection(result.expect_err("a vf:Measure must not be negative"));
        assert!(
            msg.contains("Intent availableQuantity must not be negative"),
            "expected the availableQuantity message, got: {msg}"
        );
    })
}

#[test]
fn rejects_a_point_in_time_combined_with_an_interval() {
    run(async {
        let env = shared_env().await;
        let mut intent = empty_intent("produce");
        intent.has_point_in_time = Some(seconds(1_000));
        intent.has_end = Some(seconds(1_500));

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_intent", intent)
            .await;
        let msg = rejection(result.expect_err("a point and an interval cannot coexist"));
        assert!(
            msg.contains("Intent has_point_in_time cannot be combined with has_beginning/has_end"),
            "expected the point-versus-interval message, got: {msg}"
        );
    })
}

/// Offering a minimum larger than what is available describes an intent nobody
/// can ever satisfy.
#[test]
fn rejects_a_minimum_quantity_above_the_available_quantity() {
    run(async {
        let env = shared_env().await;
        let mut intent = empty_intent("produce");
        intent.available_quantity = Some(quantity(5.0));
        intent.minimum_quantity = Some(quantity(10.0));

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_intent", intent)
            .await;
        let msg = rejection(result.expect_err("a minimum above the available amount is unsatisfiable"));
        assert!(
            msg.contains("Intent minimumQuantity must not exceed availableQuantity"),
            "expected the minimum-versus-available message, got: {msg}"
        );
    })
}

/// The comparison is `>`, not `>=`: taking the whole available amount is the
/// ordinary all-or-nothing offer, and it must go through.
#[test]
fn accepts_a_minimum_quantity_equal_to_the_available_quantity() {
    run(async {
        let env = shared_env().await;
        let mut intent = empty_intent("produce");
        intent.available_quantity = Some(quantity(5.0));
        intent.minimum_quantity = Some(quantity(5.0));

        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_intent", intent)
            .await;
        assert!(record.entry().as_option().is_some(), "the created record carried no entry");
    })
}

/// Both agents are real records because `create_rea_intent` resolves `provider`
/// with `must_get_valid_record`: a fabricated hash on the create would fail on
/// the reference, and the update would never happen.
#[test]
fn rejects_an_update_that_changes_the_provider() {
    run(async {
        let env = shared_env().await;
        let original_provider = create_agent(&env, "First provider").await;
        let other_provider = create_agent(&env, "Second provider").await;

        let mut intent = empty_intent("produce");
        intent.provider = Some(original_provider);
        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_intent", intent)
            .await;

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "update_rea_intent",
                UpdateReaIntentInput {
                    revision_id: record.action_address().clone(),
                    entry: ReaIntentUpdateParams {
                        provider: Some(other_provider),
                        ..Default::default()
                    },
                },
            )
            .await;
        let msg = rejection(result.expect_err("an intent does not change hands by edit"));
        assert!(
            msg.contains("Intent provider cannot be changed after creation"),
            "expected the immutability message naming provider, got: {msg}"
        );
    })
}

/// The same rule guards the action, and the message has to name the field that
/// actually moved. Without this case the provider message could be hardcoded
/// and the suite would not notice.
#[test]
fn rejects_an_update_that_changes_the_action() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_intent", empty_intent("produce"))
            .await;

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "update_rea_intent",
                UpdateReaIntentInput {
                    revision_id: record.action_address().clone(),
                    entry: ReaIntentUpdateParams {
                        rea_action: Some("consume".into()),
                        ..Default::default()
                    },
                },
            )
            .await;
        let msg = rejection(result.expect_err("producing does not become consuming by edit"));
        assert!(
            msg.contains("Intent action cannot be changed after creation"),
            "expected the immutability message naming action, got: {msg}"
        );
    })
}

/// `merge_partial` reads an absent field as "leave unchanged", so an update that
/// touches only the note must succeed. Without this the immutability rule could
/// be satisfied by rejecting every update.
#[test]
fn allows_an_update_that_leaves_the_immutable_fields_alone() {
    run(async {
        let env = shared_env().await;
        let provider = create_agent(&env, "Steady provider").await;

        let mut intent = empty_intent("produce");
        intent.provider = Some(provider.clone());
        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_intent", intent)
            .await;

        let updated: Record = env
            .conductor
            .call(
                &env.zome(),
                "update_rea_intent",
                UpdateReaIntentInput {
                    revision_id: record.action_address().clone(),
                    entry: ReaIntentUpdateParams {
                        note: Some("still the same offer".into()),
                        ..Default::default()
                    },
                },
            )
            .await;

        let stored = updated
            .entry()
            .to_app_option::<hrea_integrity::ReaIntent>()
            .expect("record entry failed to deserialize as ReaIntent")
            .expect("record carried no entry");
        assert_eq!(stored.note.as_deref(), Some("still the same offer"));
        assert_eq!(
            stored.provider,
            Some(provider),
            "the merge must carry the original provider forward"
        );
        assert_eq!(stored.rea_action, "produce");
    })
}
