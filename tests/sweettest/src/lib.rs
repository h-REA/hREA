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

/// Mirrors `EconomicEventWithResource` in the coordinator zome.
///
/// `create_rea_economic_event` does not take a bare entry: it takes the event
/// plus an optional resource to bring into inventory in the same call. Tests
/// that only care about field validation leave `new_inventoried_resource` at
/// `None`, which is also the branch that skips the coordinator's own
/// `get_builtin_action` lookup, so an unknown action reaches the integrity gate
/// instead of dying earlier with a different message.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EconomicEventWithResource {
    pub event: hrea_integrity::ReaEconomicEvent,
    pub new_inventoried_resource: Option<hrea_integrity::ReaEconomicResource>,
}

/// Mirrors `EconomicEventCreateResponse` in the coordinator zome.
#[derive(Serialize, Deserialize, Debug)]
pub struct EconomicEventCreateResponse {
    pub event: Record,
    pub resource: Option<Record>,
}

/// Mirrors `ReaIntentUpdateParams` in the coordinator zome: every field
/// optional, because `merge_partial` reads `null` as "leave unchanged".
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReaIntentUpdateParams {
    pub id: Option<ActionHash>,
    pub rea_action: Option<String>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub image: Option<String>,
    pub input_of: Option<ActionHash>,
    pub output_of: Option<ActionHash>,
    pub provider: Option<ActionHash>,
    pub receiver: Option<ActionHash>,
    pub resource_classified_as: Option<Vec<String>>,
    pub resource_conforms_to: Option<ActionHash>,
    pub resource_quantity: Option<hrea_integrity::QuantityValue>,
    pub effort_quantity: Option<hrea_integrity::QuantityValue>,
    pub available_quantity: Option<hrea_integrity::QuantityValue>,
    pub minimum_quantity: Option<hrea_integrity::QuantityValue>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub has_point_in_time: Option<Timestamp>,
    pub due: Option<Timestamp>,
    pub at_location: Option<String>,
    pub agreed_in: Option<String>,
    pub finished: Option<bool>,
    pub in_scope_of: Option<Vec<ActionHash>>,
}

/// Mirrors `UpdateReaIntentInput` in the coordinator zome.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateReaIntentInput {
    pub revision_id: ActionHash,
    pub entry: ReaIntentUpdateParams,
}

/// Mirrors `UpdateReaProcessInput` in the coordinator zome. `ReaProcessUpdateParams`
/// is the all-`Option` mirror of `ReaProcess`; only the fields tests set are
/// declared here, and the rest decode as `None`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReaProcessUpdateParams {
    pub name: Option<String>,
    pub has_beginning: Option<Timestamp>,
    pub has_end: Option<Timestamp>,
    pub classified_as: Option<Vec<String>>,
    pub note: Option<String>,
}

/// Mirrors `UpdateReaProcessInput` in the coordinator zome.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateReaProcessInput {
    pub revision_id: ActionHash,
    pub entry: ReaProcessUpdateParams,
}

/// Mirrors `ReaAgentUpdateParams` in the coordinator zome.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReaAgentUpdateParams {
    pub id: Option<ActionHash>,
    pub name: Option<String>,
    pub agent_type: Option<String>,
    pub image: Option<String>,
    pub classified_as: Option<Vec<String>>,
    pub note: Option<String>,
}

/// Mirrors `UpdateReaAgentInput` in the coordinator zome.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateReaAgentInput {
    pub revision_id: ActionHash,
    pub entry: ReaAgentUpdateParams,
}

/// A `vf:Measure` carrying only a numerical value.
///
/// `has_unit` stays `None` on purpose: it is an `ActionHash` pointing at a
/// `ReaUnit`, and none of the quantity rules read it, so requiring one would
/// mean an extra write per test for nothing.
pub fn quantity(value: f64) -> hrea_integrity::QuantityValue {
    hrea_integrity::QuantityValue { has_numerical_value: value, has_unit: None }
}

/// A `Timestamp` from whole seconds, for readable temporal fixtures.
pub fn seconds(s: i64) -> Timestamp {
    Timestamp::from_micros(s * 1_000_000)
}

/// A `ReaEconomicEvent` carrying only its required action, so tests set exactly
/// the fields they assert on.
pub fn empty_economic_event(action: &str) -> hrea_integrity::ReaEconomicEvent {
    hrea_integrity::ReaEconomicEvent {
        id: None,
        rea_action: action.to_string(),
        note: None,
        input_of: None,
        output_of: None,
        provider: None,
        receiver: None,
        resource_inventoried_as: None,
        to_resource_inventoried_as: None,
        resource_classified_as: None,
        resource_conforms_to: None,
        resource_quantity: None,
        effort_quantity: None,
        has_beginning: None,
        has_end: None,
        has_point_in_time: None,
        at_location: None,
        agreed_in: None,
        realization_of: None,
        reciprocal_realization_of: None,
        settles: None,
        in_scope_of: None,
        triggered_by: None,
        fulfills: None,
        satisfies: None,
        corrects: None,
    }
}

/// Wrap an event for `create_rea_economic_event` without bringing a resource
/// into inventory.
pub fn event_only(
    event: hrea_integrity::ReaEconomicEvent,
) -> EconomicEventWithResource {
    EconomicEventWithResource { event, new_inventoried_resource: None }
}

/// A `ReaIntent` carrying only its required action.
pub fn empty_intent(action: &str) -> hrea_integrity::ReaIntent {
    hrea_integrity::ReaIntent {
        id: None,
        rea_action: action.to_string(),
        name: None,
        note: None,
        image: None,
        input_of: None,
        output_of: None,
        provider: None,
        receiver: None,
        resource_classified_as: None,
        resource_conforms_to: None,
        resource_quantity: None,
        effort_quantity: None,
        available_quantity: None,
        minimum_quantity: None,
        has_beginning: None,
        has_end: None,
        has_point_in_time: None,
        due: None,
        at_location: None,
        agreed_in: None,
        finished: None,
        in_scope_of: None,
    }
}

/// A `ReaProcess` carrying only its required name.
pub fn empty_process(name: &str) -> hrea_integrity::ReaProcess {
    hrea_integrity::ReaProcess {
        id: None,
        name: name.to_string(),
        has_beginning: None,
        has_end: None,
        before: None,
        after: None,
        classified_as: None,
        based_on: None,
        planned_within: None,
        finished: None,
        in_scope_of: None,
        note: None,
    }
}

/// A `ReaAgent` carrying only its two required strings.
pub fn empty_agent(name: &str, agent_type: &str) -> hrea_integrity::ReaAgent {
    hrea_integrity::ReaAgent {
        id: None,
        name: name.to_string(),
        agent_type: agent_type.to_string(),
        image: None,
        classified_as: None,
        note: None,
    }
}

/// Create a `Person` agent and return the hash of its create action.
///
/// Several rules only become reachable once a real agent exists: an
/// EconomicEvent's `provider` and `receiver` are checked with
/// `must_get_valid_record` before any field rule runs, so a made-up hash there
/// fails on the reference rather than on the rule under test.
pub async fn create_agent(env: &SharedEnv, name: &str) -> ActionHash {
    let record: Record = env
        .conductor
        .call(&env.zome(), "create_rea_agent", empty_agent(name, "Person"))
        .await;
    record.action_address().clone()
}

/// Render a zome-call error for substring assertions.
///
/// A rejected write surfaces as a `ConductorApiError` whose payload nests the
/// validation message several layers down, and no accessor reaches it. The
/// `Debug` rendering does, and asserting on the rule's own words is what stops
/// a test passing on unrelated failure (a missing capability grant, a
/// deserialization error, a dangling reference).
pub fn rejection(err: impl std::fmt::Debug) -> String {
    format!("{err:?}")
}

/// A list of `n` distinct short classification strings.
///
/// `vf_validate_collection_bound` only reads `Vec::len`, so the contents are
/// irrelevant; they are made distinct anyway so a truncating bug in the
/// serialization path would show up as a shorter list rather than as a
/// deduplicated one.
pub fn classifications(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("https://example.org/class/{i}")).collect()
}
