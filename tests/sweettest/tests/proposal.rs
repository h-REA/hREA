//! Zome-boundary tests for `ReaProposal`, focused on the VF 1.0 validation
//! rules that no existing suite exercises directly.
//!
//! The rule under test is `vf:Proposal.purpose` immutability. It exists because
//! the coordinator indexes proposals under a purpose path at creation and
//! deliberately skips link cleanup on update, so a purpose change would corrupt
//! the offers/requests indexes silently.

use hrea_sweettest::{empty_proposal, proposal_from_record, setup_single_agent, UpdateReaProposalInput, ZOME};
use holochain::prelude::*;

#[tokio::test(flavor = "multi_thread")]
async fn creates_a_proposal_with_a_valid_purpose() {
    let (conductor, _app, cell) = setup_single_agent().await;
    let zome = cell.zome(ZOME);

    let mut input = empty_proposal();
    input.name = Some("bike repair".into());
    input.purpose = Some("offer".into());

    let record: Record = conductor.call(&zome, "create_rea_proposal", input).await;

    let stored = proposal_from_record(&record);
    assert_eq!(stored.purpose.as_deref(), Some("offer"));
    assert_eq!(stored.name.as_deref(), Some("bike repair"));
}

#[tokio::test(flavor = "multi_thread")]
async fn rejects_a_proposal_with_an_unknown_purpose() {
    let (conductor, _app, cell) = setup_single_agent().await;
    let zome = cell.zome(ZOME);

    let mut input = empty_proposal();
    input.purpose = Some("barter".into());

    let result: Result<Record, _> = conductor
        .call_fallible(&zome, "create_rea_proposal", input)
        .await;

    let err = result.expect_err("'barter' is not a vf:ProposalPurpose and must be rejected");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("barter") && msg.contains("must be 'offer' or 'request'"),
        "expected the purpose validation message, got: {msg}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn rejects_an_update_that_changes_purpose() {
    let (conductor, _app, cell) = setup_single_agent().await;
    let zome = cell.zome(ZOME);

    let mut created = empty_proposal();
    created.name = Some("bike repair".into());
    created.purpose = Some("offer".into());
    let record: Record = conductor.call(&zome, "create_rea_proposal", created).await;
    let revision_id = record.action_address().clone();

    // An offer does not become a request.
    let mut changed = empty_proposal();
    changed.purpose = Some("request".into());

    let result: Result<Record, _> = conductor
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
}

/// The coordinator's `merge_fields` treats `None` as "leave unchanged", so an
/// update that omits `purpose` must still succeed. Without this test the
/// immutability rule could be satisfied by rejecting every update.
#[tokio::test(flavor = "multi_thread")]
async fn allows_an_update_that_leaves_purpose_untouched() {
    let (conductor, _app, cell) = setup_single_agent().await;
    let zome = cell.zome(ZOME);

    let mut created = empty_proposal();
    created.name = Some("bike repair".into());
    created.purpose = Some("offer".into());
    let record: Record = conductor.call(&zome, "create_rea_proposal", created).await;
    let revision_id = record.action_address().clone();

    let mut renamed = empty_proposal();
    renamed.name = Some("bicycle repair".into());

    let updated: Record = conductor
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
}
