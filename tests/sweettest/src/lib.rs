//! Shared fixtures for the hREA Sweettest suite.
//!
//! These tests call the coordinator zome directly, which is the layer the
//! GraphQL suites in `tests/` and `clients/acceptance` never reach. That
//! matters most for integrity-zome validation rules: through GraphQL a
//! rejected write surfaces as a generic error several layers up, so a rule can
//! silently stop firing without any suite going red.

use holochain::prelude::*;
use holochain::sweettest::{SweetApp, SweetCell, SweetConductor, SweetDnaFile};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Coordinator zome name, per `dnas/hrea/workdir/dna.yaml`.
pub const ZOME: &str = "hrea";

/// Where `hc app pack workdir --recursive` leaves the packed DNA.
const DEFAULT_DNA_RELATIVE: &str = "../../dnas/hrea/workdir/hrea.dna";

/// Resolve the packed DNA bundle.
///
/// Override with `HREA_DNA=/path/to/hrea.dna` when the bundle lives elsewhere.
/// The DNA is an input to these tests, not something they build: run
/// `yarn build:zomes && hc app pack workdir --recursive` first.
pub fn dna_path() -> PathBuf {
    match std::env::var("HREA_DNA") {
        Ok(p) => PathBuf::from(p),
        Err(_) => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_DNA_RELATIVE),
    }
}

/// Boot a single-agent conductor with the hREA DNA installed.
pub async fn setup_single_agent() -> (SweetConductor, SweetApp, SweetCell) {
    let path = dna_path();
    assert!(
        path.exists(),
        "packed DNA not found at {}.\n\
         Build it first:  yarn build:zomes && hc app pack workdir --recursive\n\
         Or point HREA_DNA at an existing bundle.",
        path.display()
    );

    let dna = SweetDnaFile::from_bundle(&path)
        .await
        .expect("failed to load the hREA DNA bundle");

    let mut conductor = SweetConductor::from_standard_config().await;
    let app = conductor
        .setup_app("hrea", &[dna])
        .await
        .expect("failed to install the hREA app");
    let cell = app.cells()[0].clone();

    (conductor, app, cell)
}

/// Mirrors `UpdateReaProposalInput` in the coordinator zome.
///
/// Defined here rather than imported because the coordinator crate is a
/// `cdylib` wasm target; only the integrity crate exposes an `rlib`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateReaProposalInput {
    pub revision_id: ActionHash,
    pub entry: hrea_integrity::ReaProposal,
}

/// A `ReaProposal` with every field cleared, so tests set only what they assert on.
pub fn empty_proposal() -> hrea_integrity::ReaProposal {
    hrea_integrity::ReaProposal {
        id: None,
        name: None,
        has_beginning: None,
        has_end: None,
        unit_based: None,
        created: None,
        note: None,
        in_scope_of: None,
        publishes: None,
        reciprocal: None,
        proposed_to: None,
        purpose: None,
    }
}

/// Decode the entry of a `Record` into a `ReaProposal`.
pub fn proposal_from_record(record: &Record) -> hrea_integrity::ReaProposal {
    record
        .entry()
        .to_app_option::<hrea_integrity::ReaProposal>()
        .expect("record entry failed to deserialize as ReaProposal")
        .expect("record carried no entry")
}
