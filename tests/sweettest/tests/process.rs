//! Zome-boundary tests for `ReaProcess`.
//!
//! Process is the plainest caller of the shared rules: a required name, an
//! interval, and two bounded lists. It is included because the same validators
//! have to behave identically on an entity that does nothing else, which is
//! where a rule accidentally coupled to EconomicEvent's field names would show.
//!
//! Process passes `None` for `has_point_in_time`, so only the interval half of
//! the temporal rule is reachable here; the point-versus-interval half is
//! covered on EconomicEvent and Intent.

use holochain::prelude::*;
use hrea_sweettest::{
    classifications, empty_process, rejection, run, seconds, shared_env, ReaProcessUpdateParams,
    UpdateReaProcessInput,
};

#[test]
fn rejects_an_empty_name() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_process", empty_process(""))
            .await;
        let msg = rejection(result.expect_err("a process with no name must be rejected"));
        assert!(
            msg.contains("Process name must not be empty"),
            "expected the required-string message, got: {msg}"
        );
    })
}

#[test]
fn rejects_a_beginning_after_its_end() {
    run(async {
        let env = shared_env().await;
        let mut process = empty_process("Bread baking");
        process.has_beginning = Some(seconds(9_000));
        process.has_end = Some(seconds(8_000));

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_process", process)
            .await;
        let msg = rejection(result.expect_err("a process cannot end before it starts"));
        assert!(
            msg.contains("Process has_beginning must not be after has_end"),
            "expected the interval-ordering message, got: {msg}"
        );
    })
}

#[test]
fn rejects_a_classification_list_over_the_collection_bound() {
    run(async {
        let env = shared_env().await;
        let mut process = empty_process("Over-classified process");
        process.classified_as = Some(classifications(1025));

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&env.zome(), "create_rea_process", process)
            .await;
        let msg = rejection(result.expect_err("1025 entries is over MAX_COLLECTION_LEN"));
        assert!(
            msg.contains("Process classifiedAs exceeds the maximum of 1024 entries"),
            "expected the collection bound message, got: {msg}"
        );
    })
}

/// The negative cases above only mean something if the same call shape, with a
/// name and an ordered interval, still goes through.
#[test]
fn accepts_a_named_process_over_an_ordered_interval() {
    run(async {
        let env = shared_env().await;
        let mut process = empty_process("Bread baking");
        process.has_beginning = Some(seconds(8_000));
        process.has_end = Some(seconds(9_000));
        process.classified_as = Some(classifications(3));

        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_process", process)
            .await;
        assert!(record.entry().as_option().is_some(), "the created record carried no entry");
    })
}

/// `validate_update_rea_process` re-runs the field checks, and `merge_partial`
/// lets an update send a name on its own. A rule wired only into the create
/// callback would let a process be renamed to nothing.
#[test]
fn rejects_an_update_that_blanks_the_name() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(&env.zome(), "create_rea_process", empty_process("Bread baking"))
            .await;

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "update_rea_process",
                UpdateReaProcessInput {
                    revision_id: record.action_address().clone(),
                    entry: ReaProcessUpdateParams {
                        name: Some("   ".into()),
                        ..Default::default()
                    },
                },
            )
            .await;
        let msg = rejection(result.expect_err("a process cannot be renamed to whitespace"));
        assert!(
            msg.contains("Process name must not be empty"),
            "expected the required-string message, got: {msg}"
        );
    })
}
