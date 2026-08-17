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

- `src/lib.rs` — conductor setup, DNA resolution, payload types and entry builders
- `tests/proposal.rs` — `vf:Proposal.purpose` validation rules

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
