//! Integrity-gate rules, asserted on the rejection itself.
//!
//! These lived in `clients/acceptance` as steps named "REJECTED: ...", which
//! only checked that *something* failed. During the harness rewrite every one of
//! them stayed green while each zome call was dying of a missing capability
//! grant: a rejection test that does not read the reason is satisfied by a dead
//! conductor. Here the validation message is the assertion.

use holochain::prelude::*;
use hrea_sweettest::{run, shared_env};
use serde::{Deserialize, Serialize};

/// Only the fields each case sets. Absent `Option` fields decode as `None`, so
/// the coordinator's own entry types accept these narrower payloads.
#[derive(Serialize, Deserialize, Debug, Default)]
struct SpatialThingInput {
    name: String,
    lat: Option<f64>,
    long: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CommitmentInput {
    rea_action: Option<String>,
    note: Option<String>,
}

fn rejection(err: impl std::fmt::Debug) -> String {
    format!("{err:?}")
}

#[test]
fn spatial_thing_requires_a_name() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "create_rea_spatial_thing",
                SpatialThingInput { name: String::new(), ..Default::default() },
            )
            .await;
        let msg = rejection(result.expect_err("an empty name must be rejected"));
        assert!(
            msg.contains("SpatialThing name must not be empty"),
            "expected the required-string message, got: {msg}"
        );
    })
}

#[test]
fn spatial_thing_rejects_latitude_outside_wgs84() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "create_rea_spatial_thing",
                SpatialThingInput { name: "North of north".into(), lat: Some(91.0), long: Some(0.0) },
            )
            .await;
        let msg = rejection(result.expect_err("latitude 91 must be rejected"));
        assert!(
            msg.contains("SpatialThing lat must be between -90 and 90"),
            "expected the latitude bound message, got: {msg}"
        );
    })
}

#[test]
fn spatial_thing_rejects_longitude_outside_wgs84() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "create_rea_spatial_thing",
                SpatialThingInput { name: "East of east".into(), lat: Some(0.0), long: Some(181.0) },
            )
            .await;
        let msg = rejection(result.expect_err("longitude 181 must be rejected"));
        assert!(
            msg.contains("SpatialThing long must be between -180 and 180"),
            "expected the longitude bound message, got: {msg}"
        );
    })
}

/// The negative cases above only mean something if the same call shape succeeds
/// when the coordinates are legal.
#[test]
fn spatial_thing_accepts_coordinates_in_bounds() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(
                &env.zome(),
                "create_rea_spatial_thing",
                SpatialThingInput { name: "Bakery".into(), lat: Some(45.5), long: Some(-73.6) },
            )
            .await;
        assert!(record.entry().as_option().is_some(), "the created record carried no entry");
    })
}

#[test]
fn commitment_rejects_an_action_outside_the_vf_vocabulary() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "create_rea_commitment",
                CommitmentInput { rea_action: Some("banana".into()), note: None },
            )
            .await;
        let msg = rejection(result.expect_err("'banana' is not a ValueFlows action"));
        assert!(
            msg.contains("is not a valid ValueFlows action") && msg.contains("banana"),
            "expected the action vocabulary message, got: {msg}"
        );
    })
}

#[test]
fn commitment_accepts_a_vf_action() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(
                &env.zome(),
                "create_rea_commitment",
                CommitmentInput { rea_action: Some("transfer".into()), note: Some("in bounds".into()) },
            )
            .await;
        assert!(record.entry().as_option().is_some(), "the created record carried no entry");
    })
}

/// `revision_id` plus a sparse entry. The coordinator's `entry` field is a full
/// `ReaCommitment`, whose fields are all `Option`, and serde fills a missing
/// `Option` with `None`, which is exactly what `merge_fields` reads as "leave
/// this one alone".
#[derive(Serialize, Deserialize, Debug)]
struct UpdateCommitmentInput {
    revision_id: ActionHash,
    entry: CommitmentInput,
}

/// A commitment with no action at all is rejected before the vocabulary check
/// ever runs, by a rule of its own. The two failures read differently on
/// purpose, so this asserts the create-side message rather than the vocabulary
/// one.
#[test]
fn commitment_requires_an_action() {
    run(async {
        let env = shared_env().await;
        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "create_rea_commitment",
                CommitmentInput { rea_action: None, note: Some("promise of nothing".into()) },
            )
            .await;
        let msg = rejection(result.expect_err("a commitment with no action must be rejected"));
        assert!(
            msg.contains("Commitment must have an action"),
            "expected the missing-action message, got: {msg}"
        );
    })
}

/// The immutability rule guards provider, receiver and action on every entity
/// that carries them. Commitment is the second entity it is proven on here
/// (Intent is the first), which is what shows the rule is the shared validator
/// rather than one entity's own code.
#[test]
fn commitment_rejects_an_update_that_changes_the_action() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(
                &env.zome(),
                "create_rea_commitment",
                CommitmentInput { rea_action: Some("produce".into()), note: None },
            )
            .await;

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &env.zome(),
                "update_rea_commitment",
                UpdateCommitmentInput {
                    revision_id: record.action_address().clone(),
                    entry: CommitmentInput { rea_action: Some("consume".into()), note: None },
                },
            )
            .await;
        let msg = rejection(result.expect_err("a promise to produce does not become one to consume"));
        assert!(
            msg.contains("Commitment action cannot be changed after creation"),
            "expected the immutability message naming action, got: {msg}"
        );
    })
}

/// `merge_fields` reads an absent field as "leave unchanged", so an update that
/// only carries a note must succeed. Without this the immutability rule above
/// could be satisfied by rejecting every commitment update.
#[test]
fn commitment_allows_an_update_that_leaves_the_action_alone() {
    run(async {
        let env = shared_env().await;
        let record: Record = env
            .conductor
            .call(
                &env.zome(),
                "create_rea_commitment",
                CommitmentInput { rea_action: Some("produce".into()), note: None },
            )
            .await;

        let updated: Record = env
            .conductor
            .call(
                &env.zome(),
                "update_rea_commitment",
                UpdateCommitmentInput {
                    revision_id: record.action_address().clone(),
                    entry: CommitmentInput { rea_action: None, note: Some("same promise".into()) },
                },
            )
            .await;
        assert!(updated.entry().as_option().is_some(), "the updated record carried no entry");
    })
}
