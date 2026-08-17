//! Zome-boundary tests for `ReaProposal`, focused on the VF 1.0 validation
//! rules that no existing suite exercises directly.
//!
//! The rule under test is `vf:Proposal.purpose` immutability. It exists because
//! the coordinator indexes proposals under a purpose path at creation and
//! deliberately skips link cleanup on update, so a purpose change would corrupt
//! the offers/requests indexes silently.
//!
//! These are plain `#[test]` functions handed to `run`, not `#[tokio::test]`,
//! so they share one runtime and therefore one conductor. See `run` in the
//! crate root for why.

use holochain::prelude::*;
use hrea_sweettest::{empty_proposal, proposal_from_record, run, shared_env, UpdateReaProposalInput};

#[test]
fn creates_a_proposal_with_a_valid_purpose() {
    run(async {
        let env = shared_env().await;
        let zome = env.zome();

        let mut input = empty_proposal();
        input.name = Some("bike repair".into());
        input.purpose = Some("offer".into());

        let record: Record = env.conductor.call(&zome, "create_rea_proposal", input).await;

        let stored = proposal_from_record(&record);
        assert_eq!(stored.purpose.as_deref(), Some("offer"));
        assert_eq!(stored.name.as_deref(), Some("bike repair"));
    })
}

#[test]
fn rejects_a_proposal_with_an_unknown_purpose() {
    run(async {
        let env = shared_env().await;
        let zome = env.zome();

        let mut input = empty_proposal();
        input.purpose = Some("barter".into());

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(&zome, "create_rea_proposal", input)
            .await;

        let err = result.expect_err("'barter' is not a vf:ProposalPurpose and must be rejected");
        let msg = format!("{err:?}");
        assert!(
            msg.contains("barter") && msg.contains("must be 'offer' or 'request'"),
            "expected the purpose validation message, got: {msg}"
        );
    })
}

#[test]
fn rejects_an_update_that_changes_purpose() {
    run(async {
        let env = shared_env().await;
        let zome = env.zome();

        let mut created = empty_proposal();
        created.name = Some("bike repair".into());
        created.purpose = Some("offer".into());
        let record: Record = env
            .conductor
            .call(&zome, "create_rea_proposal", created)
            .await;
        let revision_id = record.action_address().clone();

        // An offer does not become a request.
        let mut changed = empty_proposal();
        changed.purpose = Some("request".into());

        let result: Result<Record, _> = env
            .conductor
            .call_fallible(
                &zome,
                "update_rea_proposal",
                UpdateReaProposalInput { revision_id, entry: changed },
            )
            .await;

        let err = result.expect_err("purpose is immutable on update and the change must be rejected");
        let msg = format!("{err:?}");
        assert!(
            msg.contains("purpose"),
            "expected the immutability message to name the purpose field, got: {msg}"
        );
    })
}

/// The coordinator's `merge_fields` treats `None` as "leave unchanged", so an
/// update that omits `purpose` must still succeed. Without this test the
/// immutability rule could be satisfied by rejecting every update.
#[test]
fn allows_an_update_that_leaves_purpose_untouched() {
    run(async {
        let env = shared_env().await;
        let zome = env.zome();

        let mut created = empty_proposal();
        created.name = Some("bike repair".into());
        created.purpose = Some("offer".into());
        let record: Record = env
            .conductor
            .call(&zome, "create_rea_proposal", created)
            .await;
        let revision_id = record.action_address().clone();

        let mut renamed = empty_proposal();
        renamed.name = Some("bicycle repair".into());

        let updated: Record = env
            .conductor
            .call(
                &zome,
                "update_rea_proposal",
                UpdateReaProposalInput { revision_id, entry: renamed },
            )
            .await;

        let stored = proposal_from_record(&updated);
        assert_eq!(stored.name.as_deref(), Some("bicycle repair"));
        assert_eq!(
            stored.purpose.as_deref(),
            Some("offer"),
            "the merge must carry the original purpose forward"
        );
    })
}
