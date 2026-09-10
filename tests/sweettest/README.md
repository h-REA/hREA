# hREA Sweettest suite

Zome-boundary tests. These call the coordinator zome directly through Holochain's [Sweettest](https://docs.rs/holochain/latest/holochain/sweettest/) conductor, which is the layer no other suite in this repo reaches.

## Where a test belongs

Three suites, and one rule: **a behaviour is tested one layer below where it is observable.**

| Suite | Drives | Owns | A test here reads like |
|---|---|---|---|
| `tests/sweettest` | the DNA, through `SweetConductor` | integrity validation, link-index invariants, source-chain semantics | "this write should have been rejected, with this message" |
| `clients/acceptance` | the packed hApp, over `@holochain/client` | the GraphQL adapter: resolvers, field shapes, collections, pagination | "this query should return this shape" |
| `clients/playground-e2e` | a browser | the app shell | "a person can see it" |

The rule earns its keep on negative tests. A rejection asserted through GraphQL can only see *that* something failed, never *why*: during the harness rewrite every "REJECTED: ..." step in the acceptance battery stayed green while each zome call was dying of a missing capability grant. A rejection belongs here, where the validation message is the assertion. The acceptance battery keeps one step proving that a rejection still reads as its own message after the trip up through the adapter, and its `expectRejection` helper now requires the expected text so it can never again pass on infrastructure noise.

## Running

The DNA is an input, not something the tests build:

```bash
# from the repo root
yarn run test:sweettest
```

That packs the DNA and runs the suite. To run the tests against a DNA you have already built:

```bash
cargo test -p hrea-sweettest
```

The crate is a workspace member, excluded from `default-members` so the wasm build never sees it, which is why this is `-p` rather than `--manifest-path`.

Point `HREA_DNA` at a bundle to override the default (`dnas/hrea/workdir/hrea.dna`):

```bash
HREA_DNA=/path/to/hrea.dna cargo test -p hrea-sweettest
```

## Layout

`src/lib.rs` is the shared fixture: runtime, conductor, DNA resolution, mirrored coordinator payload types, entry builders, and the `rejection` helper that renders a zome-call error for substring assertions.

Each file below is one test binary, and therefore one conductor.

| File | Covers |
|---|---|
| `tests/integrity_gate.rs` | `SpatialThing` required name and WGS84 coordinate bounds; `Commitment` action vocabulary, the create-side "must have an action" rule, and action immutability on update |
| `tests/proposal.rs` | `vf:Proposal.purpose`: the `offer`/`request` vocabulary and its immutability on update |
| `tests/economic_event.rs` | the shared VF validators on the entity that runs the most of them: action vocabulary, temporal consistency (both messages, both boundaries), quantity positivity per field, the transfer two-agent shape, and the collection bound at 1024 and 1025 |
| `tests/intent.rs` | `Intent` required action, action vocabulary, quantity positivity, point-versus-interval, the `minimumQuantity <= availableQuantity` rule, and provider/action immutability on update |
| `tests/agent.rs` | `Agent` required name and `agentType`, the `Person`/`Organization` vocabulary on create and on update, and the collection bound on `classifiedAs` |
| `tests/process.rs` | `Process` required name on create and on update, interval ordering, and the collection bound on `classifiedAs` |

Every rejection asserts on the substring the rule itself emits, never on the mere fact of failure. That is the whole point of the suite: a rejection test that does not read the reason is satisfied by a dead conductor, by a missing capability grant, or by a payload that never deserialized.

Every rule with a boundary also has a case on the accepting side, so a rule cannot be tightened past its specification and stay green: a zero-length interval, a zero quantity, a list of exactly `MAX_COLLECTION_LEN`, a minimum equal to the available amount, an update that leaves the immutable fields alone.

## How the fixture works, and why

Write tests as plain `#[test]` functions that hand their body to `run`, and take the environment from `shared_env`:

```rust
#[test]
fn my_case() {
    run(async {
        let env = shared_env().await;
        let zome = env.zome();
        let record: Record = env.conductor.call(&zome, "create_rea_x", input).await;
        // assert
    })
}
```

Three constraints shape that, each found by breaking it:

**One conductor per test binary.** Every `SweetConductor` gets a fresh temp dir and compiles the 6.3 MB hREA wasm on its first zome call. A zome call's nonce expires five minutes after it is stamped, and the stamp happens before the compile, so four conductors compiling at once on a four-core CI runner returned `Unauthorized(BadNonce("Expired"))` for every call. `shared_env` boots one conductor and spends a throwaway read to force the compile before any test signs anything. Cargo runs test binaries one at a time, so splitting the suite by domain file keeps at most one compile in flight as it grows.

**One runtime per test binary.** `#[tokio::test]` builds a runtime per test and drops it on return, taking the conductor's background tasks with it. That is why these are `#[test]` plus `run`, over a single `LazyLock<Runtime>`.

**One writer at a time.** One conductor means one agent, and a source chain is a linear log, so concurrent writes fail with `SourceChainError(HeadMoved(..))`. `shared_env` returns a guard, so test bodies serialise without anyone having to pass `--test-threads=1`.

A test that needs a conductor nobody else has written to (asserting on collection counts, for instance) should call `setup_single_agent` instead and pay its own compile.

## Known flake

The test binary sometimes dies with `SIGSEGV` inside sqlcipher's OpenSSL provider during conductor teardown, after every assertion has already passed. All tests report `ok` and then the process reports signal 11. Re-run; it is not a test failure.

## Payload types

Payload types that live in the coordinator zome (`UpdateReaProposalInput`, `EconomicEventWithResource`, the `*UpdateParams` structs) are mirrored in `src/lib.rs` rather than imported: the coordinator crate is a `cdylib`, and only the integrity crate exposes an `rlib`. Entry types come from `hrea_integrity` directly.

Where a test sets only two or three fields, it can declare its own narrower struct instead: serde fills a missing `Option` field with `None`, so a sparse payload deserializes into the coordinator's full entry type. `SpatialThingInput` and `CommitmentInput` in `tests/integrity_gate.rs` are the examples.

## Adding cases

Reach for this suite when the behaviour is a validation rule, a link-index invariant, or anything whose failure mode is "the write should have been rejected". Behaviour that is really about GraphQL shape belongs in `clients/acceptance` instead.

Two habits are worth keeping. Assert on the rule's own message, not on the fact of failure. And check what the rule under test sits behind: `validate_create_*` resolves every `ActionHash` relation with `must_get_valid_record` before any field rule runs, so a fabricated hash makes the write fail on the reference and the field rule is never consulted. `create_agent` in `src/lib.rs` exists for exactly that reason.
