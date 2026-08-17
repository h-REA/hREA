# hREA Sweettest suite

Zome-boundary tests. These call the coordinator zome directly through
Holochain's [Sweettest](https://docs.rs/holochain/latest/holochain/sweettest/)
conductor, which is the layer no other suite in this repo reaches.

## Why this exists

Every other suite enters through GraphQL:

| Suite | Layer |
|---|---|
| `tests/` (Tryorama) | GraphQL |
| `clients/acceptance` | GraphQL |
| `clients/playground-e2e` | browser |

That leaves the integrity zome's validation rules untested at the layer they run
on. Through GraphQL a rejected write surfaces as a generic error several frames
up, so a validation rule can stop firing without any suite going red. These
tests assert on the rejection itself.

It also matters for version upgrades: Holochain 0.7 rewrites every `validate_*`
signature from `EntryCreationAction` to `TypedAction<EntryCreationData>`. This
suite is the regression check for that port.

## Running

The DNA is an input, not something the tests build:

```bash
# from the repo root
yarn run test:sweettest
```

That packs the DNA and runs the suite. To run the tests against a DNA you have
already built:

```bash
cargo test --manifest-path tests/sweettest/Cargo.toml
```

Point `HREA_DNA` at a bundle to override the default
(`dnas/hrea/workdir/hrea.dna`):

```bash
HREA_DNA=/path/to/hrea.dna cargo test --manifest-path tests/sweettest/Cargo.toml
```

## Layout

- `src/lib.rs` — the shared fixture: runtime, conductor, DNA resolution, payload
  types and entry builders
- `tests/proposal.rs` — `vf:Proposal.purpose` validation rules

## How the fixture works, and why

Write tests as plain `#[test]` functions that hand their body to `run`, and take
the environment from `shared_env`:

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

**One conductor per test binary.** Every `SweetConductor` gets a fresh temp dir
and compiles the 6.3 MB hREA wasm on its first zome call. A zome call's nonce
expires five minutes after it is stamped, and the stamp happens before the
compile, so four conductors compiling at once on a four-core CI runner returned
`Unauthorized(BadNonce("Expired"))` for every call. `shared_env` boots one
conductor and spends a throwaway read to force the compile before any test
signs anything. Cargo runs test binaries one at a time, so splitting the suite
by domain file keeps at most one compile in flight as it grows.

**One runtime per test binary.** `#[tokio::test]` builds a runtime per test and
drops it on return, taking the conductor's background tasks with it. That is why
these are `#[test]` plus `run`, over a single `LazyLock<Runtime>`.

**One writer at a time.** One conductor means one agent, and a source chain is a
linear log, so concurrent writes fail with `SourceChainError(HeadMoved(..))`.
`shared_env` returns a guard, so test bodies serialise without anyone having to
pass `--test-threads=1`.

A test that needs a conductor nobody else has written to (asserting on
collection counts, for instance) should call `setup_single_agent` instead and
pay its own compile.

## Why its own workspace

This crate declares `[workspace]` in its own `Cargo.toml` and the repo root
excludes it. `yarn build:zomes` runs
`cargo build --release --target wasm32-unknown-unknown` across the root
workspace; as a member, this crate would make that try to compile Holochain
itself for wasm.

Payload types that live in the coordinator zome (for example
`UpdateReaProposalInput`) are mirrored in `src/lib.rs` rather than imported: the
coordinator crate is a `cdylib`, and only the integrity crate exposes an `rlib`.

## Adding cases

Reach for this suite when the behaviour is a validation rule, a link-index
invariant, or anything whose failure mode is "the write should have been
rejected". Behaviour that is really about GraphQL shape belongs in
`clients/acceptance` instead.
