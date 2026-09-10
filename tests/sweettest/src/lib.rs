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
use std::future::Future;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::ops::Deref;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

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

/// A booted conductor with the hREA DNA installed, shared by every test in a
/// binary.
pub struct SharedEnv {
    pub conductor: SweetConductor,
    pub app: SweetApp,
    pub cell: SweetCell,
}

impl SharedEnv {
    /// The coordinator zome, ready to call.
    pub fn zome(&self) -> holochain::sweettest::SweetZome {
        self.cell.zome(ZOME)
    }

    /// Force the conductor to compile the hREA wasm before any test signs a
    /// call.
    ///
    /// Compilation happens lazily on the first zome call, and the nonce on that
    /// call is stamped before it. On a slow runner the compile outlives the
    /// five-minute nonce window and the call dies as
    /// `Unauthorized(BadNonce("Expired"))`. Spending one throwaway call here
    /// means the module is already compiled when the real ones are signed.
    ///
    /// It reads a hash that does not exist, so it leaves the shared DHT empty,
    /// which matters once the suite grows assertions about collection counts.
    /// The result is discarded on purpose: this call exists for its effect on
    /// the ribosome, and it is the one call allowed to lose the nonce race.
    async fn warm_up(&self) {
        let missing = ActionHash::from_raw_36(vec![0; 36]);
        let _: Result<Option<Record>, _> = self
            .conductor
            .call_fallible(&self.zome(), "get_latest_rea_proposal", missing)
            .await;
    }
}

/// One tokio runtime for the whole test binary.
///
/// `#[tokio::test]` builds a fresh runtime per test and drops it when that test
/// returns, which takes the conductor's background tasks with it: the first
/// test passes and every later one fails with
/// `CellError(WorkflowError(SendError(..)))`. A conductor outliving a single
/// test therefore needs a runtime that outlives one too. Tests are plain
/// `#[test]` functions that hand their body to [`run`].
static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build the shared tokio runtime")
});

/// Drive a test body on the shared runtime.
///
/// `Runtime::block_on` takes `&self`, so libtest still runs the cases on
/// parallel threads. What keeps them from colliding is [`shared_env`], not the
/// thread count.
pub fn run<F: Future>(future: F) -> F::Output {
    RUNTIME.block_on(future)
}

/// Serialises test bodies against the shared agent's source chain.
static CHAIN_LOCK: Mutex<()> = Mutex::const_new(());

/// The shared environment, held exclusively for the life of one test.
///
/// Derefs to [`SharedEnv`], so `env.conductor` and `env.zome()` read the same
/// either way.
pub struct ExclusiveEnv {
    env: &'static SharedEnv,
    _guard: MutexGuard<'static, ()>,
}

impl Deref for ExclusiveEnv {
    type Target = SharedEnv;
    fn deref(&self) -> &SharedEnv {
        self.env
    }
}

static SHARED: OnceCell<SharedEnv> = OnceCell::const_new();

/// One conductor per test binary.
///
/// Every `SweetConductor` gets a fresh temp dir, so it compiles the 6.3 MB hREA
/// wasm from scratch on its first zome call. A zome call's nonce expires after
/// five minutes (`FRESH_NONCE_EXPIRES_AFTER` in `holochain_nonce`), and four
/// conductors compiling in parallel on a four-core CI runner ran past that:
/// every call came back `Unauthorized(BadNonce("Expired"))` while the same
/// commit passed on a less contended runner.
///
/// Sharing one conductor per binary means one compile. Cargo already runs test
/// binaries one at a time, so splitting the suite by domain keeps that property
/// as it grows. Reach for [`setup_single_agent`] only when a test genuinely
/// needs a conductor nobody else has written to.
pub async fn shared_env() -> ExclusiveEnv {
    let env = SHARED
        .get_or_init(|| async {
            let (conductor, app, cell) = setup_single_agent().await;
            let env = SharedEnv { conductor, app, cell };
            env.warm_up().await;
            env
        })
        .await;

    // One conductor means one agent, and an agent's source chain is a linear
    // log: two tests writing at once make the second fail with
    // `SourceChainError(HeadMoved(..))`. Handing out the environment behind a
    // lock is what serialises them, rather than asking every caller to
    // remember `--test-threads=1`.
    ExclusiveEnv { env, _guard: CHAIN_LOCK.lock().await }
}

/// Boot a single-agent conductor with the hREA DNA installed.
///
/// Prefer [`shared_env`]: each call to this pays the wasm compile again.
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

    // 0.7 renamed this. `standard()` spawns a local rendezvous server and
    // points the conductor at it, so the suite is isolated from any public
    // bootstrap rather than joining one.
    let mut conductor = SweetConductor::standard().await;
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
