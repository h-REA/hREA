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

use holochain::prelude::*;
use hrea_sweettest::{
    classifications, create_agent, economic_event_from_record, empty_economic_event, empty_intent,
    empty_process, event_only, quantity, rejection, run, seconds, shared_env,
    EconomicEventCreateResponse, ReaEconomicEventUpdateParams, UpdateReaEconomicEventInput,
};
use serde::{Deserialize, Serialize};

/// A commitment carrying only what the update tests need. `ReaCommitment`'s
/// fields are all `Option`, so absent ones decode as `None`.
#[derive(Serialize, Deserialize, Debug, Default)]
struct CommitmentInput {
    rea_action: Option<String>,
    note: Option<String>,
}

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

// ---------------------------------------------------------------------------
// update_rea_economic_event (#416)
//
// Before #416 the update applied `note` and discarded every other field while
// still returning a success record, so these tests read each field back rather
// than trusting the returned record's existence.
// ---------------------------------------------------------------------------

/// Create an event with nothing but its action and the fields `customise` sets,
/// returning the record of its create action.
async fn create_event(
    env: &hrea_sweettest::SharedEnv,
    customise: impl FnOnce(&mut hrea_integrity::ReaEconomicEvent),
) -> Record {
    let mut event = empty_economic_event("produce");
    customise(&mut event);
    let created: EconomicEventCreateResponse = env
        .conductor
        .call(&env.zome(), "create_rea_economic_event", event_only(event))
        .await;
    created.event
}

async fn create_agreement(env: &hrea_sweettest::SharedEnv, name: &str) -> ActionHash {
    let record: Record = env
        .conductor
        .call(
            &env.zome(),
            "create_rea_agreement",
            hrea_integrity::ReaAgreement { id: None, name: Some(name.into()), created: None, note: None },
        )
        .await;
    record.action_address().clone()
}

async fn update_event(
    env: &hrea_sweettest::SharedEnv,
    revision_id: &ActionHash,
    entry: ReaEconomicEventUpdateParams,
) -> Result<Record, String> {
    env.conductor
        .call_fallible::<_, Record>(
            &env.zome(),
            "update_rea_economic_event",
            UpdateReaEconomicEventInput { revision_id: revision_id.clone(), entry },
        )
        .await
        .map_err(rejection)
}

/// The link targets an index returns, so a test can assert which revision an
/// index points at rather than only how many links it holds.
async fn link_targets(
    env: &hrea_sweettest::SharedEnv,
    fn_name: &str,
    base: &ActionHash,
) -> Vec<ActionHash> {
    let links: Vec<Link> = env.conductor.call(&env.zome(), fn_name, base.clone()).await;
    links
        .into_iter()
        .map(|l| l.target.into_action_hash().expect("link target is not an action hash"))
        .collect()
}

/// Every field the update declares as mutable is written, and the stored entry
/// reads back with each value. This is the test #416 said was missing.
#[test]
fn update_applies_every_mutable_field() {
    run(async {
        let env = shared_env().await;
        let alice = create_agent(&env, "Alice").await;
        let agreement = create_agreement(&env, "Bakery supply").await;
        let reciprocal = create_agreement(&env, "Bakery payment").await;
        let trigger = create_event(&env, |_| {}).await.action_address().clone();
        let commitment: Record = env
            .conductor
            .call(&env.zome(), "create_rea_commitment", CommitmentInput { rea_action: Some("produce".into()), note: None })
            .await;
        let intent: Record = env
            .conductor
            .call(&env.zome(), "create_rea_intent", empty_intent("produce"))
            .await;
        let claim: Record = env
            .conductor
            .call(
                &env.zome(),
                "create_rea_claim",
                hrea_integrity::ReaClaim {
                    id: None,
                    rea_action: "transfer".into(),
                    resource_classified_as: None,
                    resource_quantity: None,
                    effort_quantity: None,
                    triggered_by: Some(trigger.clone()),
                    due: None,
                    created: None,
                    finished: None,
                    note: None,
                    agreed_in: None,
                },
            )
            .await;

        let created = create_event(&env, |e| e.note = Some("first pass".into())).await;
        let id = created.action_address().clone();

        let params = ReaEconomicEventUpdateParams {
            note: Some("second pass".into()),
            agreed_in: Some("https://example.org/terms".into()),
            realization_of: Some(agreement.clone()),
            reciprocal_realization_of: Some(reciprocal.clone()),
            settles: Some(claim.action_address().clone()),
            in_scope_of: Some(vec![alice.clone()]),
            triggered_by: Some(trigger.clone()),
            fulfills: Some(vec![commitment.action_address().clone()]),
            satisfies: Some(vec![intent.action_address().clone()]),
            ..Default::default()
        };
        let updated = update_event(&env, &id, params).await.expect("a context-only update must succeed");

        let latest: Option<Record> =
            env.conductor.call(&env.zome(), "get_latest_rea_economic_event", id.clone()).await;
        let latest = latest.expect("the event has no latest revision");
        assert_eq!(latest.action_address(), updated.action_address(), "latest revision is not the update");

        let stored = economic_event_from_record(&latest);
        assert_eq!(stored.id, Some(id.clone()), "the update lost the original id");
        assert_eq!(stored.note.as_deref(), Some("second pass"));
        assert_eq!(stored.agreed_in.as_deref(), Some("https://example.org/terms"));
        assert_eq!(stored.realization_of, Some(agreement.clone()));
        assert_eq!(stored.reciprocal_realization_of, Some(reciprocal));
        assert_eq!(stored.settles, Some(claim.action_address().clone()));
        assert_eq!(stored.in_scope_of, Some(vec![alice]));
        assert_eq!(stored.triggered_by, Some(trigger));
        assert_eq!(stored.fulfills, Some(vec![commitment.action_address().clone()]));
        assert_eq!(stored.satisfies, Some(vec![intent.action_address().clone()]));
        assert_eq!(stored.rea_action, "produce", "the action must survive a context update");

        // The reverse indexes are written for the new values and point at the new revision.
        let rev = updated.action_address().clone();
        assert_eq!(link_targets(&env, "get_rea_economic_events_for_rea_agreement", &agreement).await, vec![rev.clone()]);
        assert_eq!(link_targets(&env, "get_settling_events_for_claim", claim.action_address()).await, vec![rev.clone()]);
        assert_eq!(link_targets(&env, "get_fulfilling_economic_events_for_commitment", commitment.action_address()).await, vec![rev.clone()]);
        assert_eq!(link_targets(&env, "get_satisfying_economic_events_for_rea_intent", intent.action_address()).await, vec![rev]);
    })
}

/// An absent field is "leave unchanged", not "clear". The update before #416
/// assigned `note` unconditionally, so an update without a note erased it.
#[test]
fn update_leaves_absent_fields_alone() {
    run(async {
        let env = shared_env().await;
        let agreement = create_agreement(&env, "Kept").await;
        let created = create_event(&env, |e| {
            e.note = Some("keep me".into());
            e.realization_of = Some(agreement.clone());
        })
        .await;

        let params = ReaEconomicEventUpdateParams {
            agreed_in: Some("https://example.org/only-this".into()),
            ..Default::default()
        };
        let updated = update_event(&env, created.action_address(), params).await.expect("update failed");
        let stored = economic_event_from_record(&updated);
        assert_eq!(stored.note.as_deref(), Some("keep me"), "an absent note was cleared");
        assert_eq!(stored.realization_of, Some(agreement));
        assert_eq!(stored.agreed_in.as_deref(), Some("https://example.org/only-this"));
    })
}

/// Moving an event to another agreement moves its index entry: the old
/// agreement stops listing it, the new one lists the new revision.
#[test]
fn update_moves_the_agreement_index_with_realization_of() {
    run(async {
        let env = shared_env().await;
        let first = create_agreement(&env, "First").await;
        let second = create_agreement(&env, "Second").await;
        let created = create_event(&env, |e| e.realization_of = Some(first.clone())).await;
        assert_eq!(
            link_targets(&env, "get_rea_economic_events_for_rea_agreement", &first).await,
            vec![created.action_address().clone()]
        );

        let params = ReaEconomicEventUpdateParams { realization_of: Some(second.clone()), ..Default::default() };
        let updated = update_event(&env, created.action_address(), params).await.expect("update failed");

        assert!(
            link_targets(&env, "get_rea_economic_events_for_rea_agreement", &first).await.is_empty(),
            "the old agreement still lists the event"
        );
        assert_eq!(
            link_targets(&env, "get_rea_economic_events_for_rea_agreement", &second).await,
            vec![updated.action_address().clone()]
        );
    })
}

/// The integrity rule that keeps `provider` fixed was unreachable while the
/// coordinator discarded the field. It is reachable now, and this pins its message.
#[test]
fn update_rejects_a_change_of_provider() {
    run(async {
        let env = shared_env().await;
        let alice = create_agent(&env, "Alice").await;
        let bob = create_agent(&env, "Bob").await;
        let created = create_event(&env, |e| e.provider = Some(alice.clone())).await;

        let params = ReaEconomicEventUpdateParams { provider: Some(bob), ..Default::default() };
        let msg = update_event(&env, created.action_address(), params).await.expect_err("the provider of an observed event is fixed");
        assert!(
            msg.contains("EconomicEvent provider cannot be changed after creation"),
            "expected the immutability message naming provider, got: {msg}"
        );
    })
}

/// `inputOf`, `outputOf` and `corrects` have no integrity rule, so the
/// coordinator refuses the change itself instead of silently dropping it.
#[test]
fn update_rejects_a_change_of_input_of() {
    run(async {
        let env = shared_env().await;
        let first: Record = env.conductor.call(&env.zome(), "create_rea_process", empty_process("Bake")).await;
        let second: Record = env.conductor.call(&env.zome(), "create_rea_process", empty_process("Pack")).await;
        let created = create_event(&env, |e| {
            e.rea_action = "consume".into();
            e.input_of = Some(first.action_address().clone());
        })
        .await;

        let params = ReaEconomicEventUpdateParams {
            input_of: Some(second.action_address().clone()),
            ..Default::default()
        };
        let msg = update_event(&env, created.action_address(), params).await.expect_err("an observed input does not move process");
        assert!(
            msg.contains("EconomicEvent inputOf cannot be changed after creation"),
            "expected the coordinator's immutability message naming inputOf, got: {msg}"
        );
    })
}

#[test]
fn update_rejects_setting_corrects_after_creation() {
    run(async {
        let env = shared_env().await;
        let original = create_event(&env, |_| {}).await;
        let created = create_event(&env, |_| {}).await;

        let params = ReaEconomicEventUpdateParams {
            corrects: Some(original.action_address().clone()),
            ..Default::default()
        };
        let msg = update_event(&env, created.action_address(), params).await.expect_err("a correction is a new event, not an edit");
        assert!(
            msg.contains("EconomicEvent corrects cannot be changed after creation"),
            "expected the coordinator's immutability message naming corrects, got: {msg}"
        );
    })
}

/// Resending the stored value of every fixed field is a no-op, so a client that
/// echoes the whole entity back with one changed note still succeeds. Without
/// this the rejections above could be satisfied by refusing every update.
#[test]
fn update_accepts_fixed_fields_resent_unchanged() {
    run(async {
        let env = shared_env().await;
        let alice = create_agent(&env, "Alice").await;
        let process: Record = env.conductor.call(&env.zome(), "create_rea_process", empty_process("Mill")).await;
        let created = create_event(&env, |e| {
            e.rea_action = "consume".into();
            e.provider = Some(alice.clone());
            e.receiver = Some(alice.clone());
            e.input_of = Some(process.action_address().clone());
        })
        .await;

        let params = ReaEconomicEventUpdateParams {
            note: Some("echoed back".into()),
            provider: Some(alice.clone()),
            receiver: Some(alice.clone()),
            input_of: Some(process.action_address().clone()),
            ..Default::default()
        };
        let updated = update_event(&env, created.action_address(), params).await.expect("an unchanged echo must succeed");
        let stored = economic_event_from_record(&updated);
        assert_eq!(stored.note.as_deref(), Some("echoed back"));
        assert_eq!(stored.provider, Some(alice));
        assert_eq!(stored.input_of, Some(process.action_address().clone()));
    })
}
