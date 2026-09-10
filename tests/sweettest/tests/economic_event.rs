//! Zome-boundary tests for `ReaEconomicEvent`.
//!
//! EconomicEvent runs more of the shared VF validators than any other entry
//! type, which is why the shared rules are pinned here: action vocabulary,
//! temporal consistency, quantity positivity, the transfer two-agent shape, and
//! the collection bound. Each rejection asserts on the words that rule emits,
//! so a write dying of something incidental (a dangling reference, a
//! deserialization error, a missing capability grant) cannot satisfy the test.
//!
//! `new_inventoried_resource` stays `None` throughout. That is the branch of
//! `create_rea_economic_event` that does not call the coordinator's own
//! `get_builtin_action`, so an unknown action reaches the integrity gate rather
//! than failing one layer earlier with a different message.

use hrea_sweettest::{
    classifications, create_agent, empty_economic_event, event_only, quantity, rejection, run,
    seconds, shared_env, EconomicEventCreateResponse,
};

#[test]
fn rejects_an_action_outside_the_vf_vocabulary() {
    run(async {
        let env = shared_env().await;
        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "create_rea_economic_event",
                event_only(empty_economic_event("banana")),
            )
            .await;
        let msg = rejection(result.expect_err("'banana' is not a ValueFlows action"));
        assert!(
            msg.contains("EconomicEvent action 'banana' is not a valid ValueFlows action"),
            "expected the action vocabulary message, got: {msg}"
        );
    })
}

#[test]
fn rejects_a_beginning_after_its_end() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.has_beginning = Some(seconds(2_000));
        event.has_end = Some(seconds(1_000));

        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        let msg = rejection(result.expect_err("an interval that ends before it starts is invalid"));
        assert!(
            msg.contains("EconomicEvent has_beginning must not be after has_end"),
            "expected the interval-ordering message, got: {msg}"
        );
    })
}

/// The rule is `>`, not `>=`. A zero-length interval is a legal VF interval, and
/// without this case the rule could be tightened to reject it and stay green.
#[test]
fn accepts_a_beginning_equal_to_its_end() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.has_beginning = Some(seconds(1_000));
        event.has_end = Some(seconds(1_000));

        let created: EconomicEventCreateResponse = env
            .conductor
            .call(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        assert!(
            created.event.entry().as_option().is_some(),
            "the created record carried no entry"
        );
    })
}

/// VF models an occurrence as a point *or* an interval, never both.
#[test]
fn rejects_a_point_in_time_combined_with_an_interval() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.has_point_in_time = Some(seconds(1_000));
        event.has_beginning = Some(seconds(900));

        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        let msg = rejection(result.expect_err("a point and an interval cannot coexist"));
        assert!(
            msg.contains(
                "EconomicEvent has_point_in_time cannot be combined with has_beginning/has_end"
            ),
            "expected the point-versus-interval message, got: {msg}"
        );
    })
}

/// The other side of the same rule: a point on its own is the normal shape for
/// an observed event, so the rule must not reject `has_point_in_time` itself.
#[test]
fn accepts_a_point_in_time_on_its_own() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.has_point_in_time = Some(seconds(1_000));

        let created: EconomicEventCreateResponse = env
            .conductor
            .call(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        assert!(
            created.event.entry().as_option().is_some(),
            "the created record carried no entry"
        );
    })
}

#[test]
fn rejects_a_negative_resource_quantity() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.resource_quantity = Some(quantity(-1.0));

        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        let msg = rejection(result.expect_err("a vf:Measure must not be negative"));
        assert!(
            msg.contains("EconomicEvent resourceQuantity must not be negative"),
            "expected the resourceQuantity message, got: {msg}"
        );
    })
}

/// The same rule is applied per field, so the message has to name the field the
/// write actually got wrong. Without this case both call sites could point at
/// `resourceQuantity` and nothing would notice.
#[test]
fn rejects_a_negative_effort_quantity_and_names_that_field() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("work");
        event.effort_quantity = Some(quantity(-0.5));

        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        let msg = rejection(result.expect_err("a negative effort is not a vf:Measure"));
        assert!(
            msg.contains("EconomicEvent effortQuantity must not be negative"),
            "expected the effortQuantity message, got: {msg}"
        );
    })
}

/// The bound is `< 0`, not `<= 0`. A zero quantity is meaningful in VF (an event
/// that moved nothing), so it must survive.
#[test]
fn accepts_a_zero_quantity() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.resource_quantity = Some(quantity(0.0));

        let created: EconomicEventCreateResponse = env
            .conductor
            .call(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        assert!(
            created.event.entry().as_option().is_some(),
            "the created record carried no entry"
        );
    })
}

/// A transfer moves something from someone to someone else, so an event
/// recording one names both parties.
///
/// The provider here is a real agent on purpose: `validate_create` resolves
/// every `ActionHash` relation with `must_get_valid_record` before any field
/// rule runs, so a fabricated hash would fail on the reference and the test
/// would pass without the transfer rule ever being consulted.
#[test]
fn rejects_a_transfer_missing_its_receiver() {
    run(async {
        let env = shared_env().await;
        let provider = create_agent(&env, "Provider co-op").await;

        let mut event = empty_economic_event("transfer");
        event.provider = Some(provider);

        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        let msg = rejection(result.expect_err("a transfer with no receiver is one-sided"));
        assert!(
            msg.contains(
                "EconomicEvent action 'transfer' is a transfer and requires both provider and receiver"
            ),
            "expected the transfer two-agent message, got: {msg}"
        );
    })
}

#[test]
fn accepts_a_transfer_naming_both_parties() {
    run(async {
        let env = shared_env().await;
        let provider = create_agent(&env, "Provider co-op").await;
        let receiver = create_agent(&env, "Receiver co-op").await;

        let mut event = empty_economic_event("transfer");
        event.provider = Some(provider);
        event.receiver = Some(receiver);

        let created: EconomicEventCreateResponse = env
            .conductor
            .call(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        assert!(
            created.event.entry().as_option().is_some(),
            "the created record carried no entry"
        );
    })
}

/// The rule is scoped to the three transfer-class actions. A `produce` event
/// legitimately has no counterparty, and the same empty provider/receiver pair
/// that the transfer case rejects must go through here.
#[test]
fn accepts_a_non_transfer_action_with_no_agents() {
    run(async {
        let env = shared_env().await;
        let created: EconomicEventCreateResponse = env
            .conductor
            .call(
                &env.zome(),
                "create_rea_economic_event",
                event_only(empty_economic_event("produce")),
            )
            .await;
        assert!(
            created.event.entry().as_option().is_some(),
            "the created record carried no entry"
        );
    })
}

/// `MAX_COLLECTION_LEN` bounds gossip fan-out below Holochain's blunt 4MB entry
/// limit, so the rule has to fire on element count rather than on byte size.
/// The strings here are short enough that the entry is nowhere near 4MB, which
/// is what makes the count the only thing that can have rejected it.
#[test]
fn rejects_a_classification_list_over_the_collection_bound() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.resource_classified_as = Some(classifications(1025));

        let result: Result<EconomicEventCreateResponse, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        let msg = rejection(result.expect_err("1025 entries is over MAX_COLLECTION_LEN"));
        assert!(
            msg.contains("EconomicEvent resourceClassifiedAs exceeds the maximum of 1024 entries"),
            "expected the collection bound message, got: {msg}"
        );
    })
}

/// The bound is inclusive. Exactly `MAX_COLLECTION_LEN` elements is legal, and
/// this is the case that would go red if the comparison were changed to `>=`.
#[test]
fn accepts_a_classification_list_exactly_at_the_collection_bound() {
    run(async {
        let env = shared_env().await;
        let mut event = empty_economic_event("produce");
        event.resource_classified_as = Some(classifications(1024));

        let created: EconomicEventCreateResponse = env
            .conductor
            .call(&env.zome(), "create_rea_economic_event", event_only(event))
            .await;
        assert!(
            created.event.entry().as_option().is_some(),
            "the created record carried no entry"
        );
    })
}
