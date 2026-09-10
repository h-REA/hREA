# Contributing

This document covers the development workflow for contributing to hREA itself: testing, CI, the
branch and PR model, and releases. For environment setup see
[Getting Started](./getting-started.md); for the codebase map see
[Repository Structure](./repository-structure.md).

## Development workflow

1. Enter the Nix shell: `nix develop`.
2. Install dependencies: `yarn install`.
3. Make your change in the relevant zome, module, UI, or test.
4. Rebuild what you touched (`yarn run build:zomes`, `yarn run build:graphql:adapter`, or
   `yarn run build`).
5. Run the test suites: `yarn run test:sweettest` and `yarn run test:acceptance` (see [Testing](#testing)).
6. Open a pull request (see the branch model below).

## Testing

Tryorama is retired and cannot come back on this line: the last published `@holochain/tryorama`, 0.19.2, depends on `@holochain/client ^0.20.4`, and no version of Tryorama targets Holochain 0.7 or client 0.21. It was the thing blocking the client upgrade, so the suite it drove (`tests/src`, Vitest against multi-conductor Tryorama networks) is gone. Two independent surfaces replace it, each testing at a different layer:

- **`tests/sweettest`** is a native Rust crate that drives `SweetConductor` against the packed DNA, testing at the zome boundary. That is where VF 1.0's integrity validation actually runs, so a rejection can be asserted on its actual message rather than only on the fact that something failed. Files: `tests/sweettest/tests/proposal.rs` (the `vf:Proposal.purpose` rules) and `tests/sweettest/tests/integrity_gate.rs` (cross-entity integrity rules). It is a member of the root cargo workspace but not a `default-member`, which is what keeps it out of the wasm build: a bare `cargo build --release --target wasm32-unknown-unknown` (`yarn build:zomes`) compiles only the zome crates and never drags the conductor into the wasm graph, while `cargo test -p hrea-sweettest` still builds it for the host. The crate used to declare its own `[workspace]` instead, and that is exactly how its conductor came to sit a minor version behind the zomes it exercises: two `Cargo.lock` files recorded two Holochain versions, and the 0.7 bump only ever touched one of them.

  ```bash
  yarn run test:sweettest
  ```

- **`clients/acceptance`** is the GraphQL adapter surface. It spawns its own ephemeral `hc sandbox` conductor and drives the schema over `@holochain/client`, so it needs nothing from Tryorama. Modules are registered in `clients/acceptance/src/run.ts`: `core`, `rea-flows`, `crud`, `recipes`, `regressions`, `forward-refs`. Run a single module with `--only=<module>`; `--demo` narrates the same scenario instead of just asserting it.

  ```bash
  yarn run test:acceptance
  # equivalently, from the workspace directly:
  yarn workspace hrea-acceptance run check
  ```

  `clients/acceptance/src/scenarios/regressions.ts` exists specifically to hold the checks the Tryorama retirement would otherwise have dropped: `updateCommitment`, `updateAgreement`, `updatePlan`, `Intent.observedBy`, `Plan.nonProcessCommitments` with `Process.committedInputs`/`committedOutputs`, the `spatialThings` / `agreementBundles` / bare `economicEvents` collection queries, relay `before`/`last` pagination on `agents`, `Agreement.name` read-back, and the check that a resource stays listed exactly once after repeated events. It is kept as one file so the mapping from the retired suite stays auditable.

`test:sweettest` packs the DNA itself (`yarn run build:happ && cargo test -p hrea-sweettest`). `test:acceptance` expects the hApp and the adapter to already be built.

### Network isolation on 0.7, and how the acceptance battery gets it

Both test surfaces run against a conductor that is isolated from any public network, but they get there by different routes, and the acceptance one had to be built.

`hc sandbox generate network` exists on 0.7 and takes `mem` or `quic` as the transport, plus `-b` for the bootstrap service. What it does not do is default to anything local. Passing `network mem` alone sets the transport to memory and leaves peer discovery untouched, so the generated `conductor-config.yaml` keeps:

```yaml
network:
  bootstrap_url: https://dev-test-bootstrap2.holochain.org/
  relay_url: https://use1-1.relay.n0.iroh-canary.iroh.link./
  request_timeout_s: 60
```

That is not a tidiness problem. A validation dependency fetched during `must_get_valid_record` goes out over that network, and when it does not come back the call fails at `request_timeout_s`. On a CI runner it did not come back: the `chore/holochain-0.7` run failed 21 of 65 steps with `get response channel dropped: likely response timeout`, the failures spaced exactly 60 seconds apart, while the same suite passed locally in 20 seconds. Reachability of a public service, not logic, which is why it stayed hidden for so long.

`clients/acceptance/src/harness.ts` therefore starts its own `kitsune2-bootstrap-srv` on a free port and passes `network -b http://127.0.0.1:<port> mem`. Each run gets its own bootstrap, so two runs on the same machine cannot find each other either, and a collection count is meaningful again.

Sweettest never had the problem: `SweetConductor::standard()` spawns a local rendezvous server of its own. The acceptance battery now has the same property by the same means.

One related fix lives in the same file. `hc sandbox --run` execs `holochain` as a grandchild, so killing the sandbox used to leave the conductor running and still holding DHT membership. The conductor is spawned `detached` and teardown signals the whole process group, so a run leaves nothing behind.

## Continuous integration

Two workflows live in `.github/workflows/`:

- **`test.yml`** (workflow name `Checks`) runs on every push and pull request. On `ubuntu-latest` it installs Nix (with a Cachix cache), enters the flake shell, installs dependencies, builds the GraphQL adapter, builds the WASM and hApp, then runs the Sweettest suite and the acceptance suite in turn. The job has a 100 minute timeout, and caches `target/` alongside the usual Cargo directories so the conductor is not recompiled from scratch on every run. One `target/` covers both halves now that the workspaces are unified.

- **`release.yml`** fires on any pushed tag matching `happ-*` (for example `happ-0.4.0-beta`), from any branch. It creates a GitHub release and uploads two artifacts, `workdir/hrea.happ` and `dnas/hrea/workdir/hrea.dna`. Both names are worth checking against `.github/workflows/release.yml` before cutting a tag: until `0b7f36e6` the workflow still called a `build:holochain:release` script that no longer exists and still published the seven per-module DNA bundles of the retired multi-DNA layout, so the `happ-0.4.0-beta` run failed and its artifacts had to be uploaded by hand.

Make sure `yarn run test:sweettest` and `yarn run test:acceptance` both pass locally before opening a PR, since CI runs the same suites.

## Branch and PR model

`sprout` is the default branch. A release is cut from an integration branch built up as a stack of small PRs; at the time of writing that branch is `feat/vf-proposal-purpose` (#408), and the Holochain 0.7 upgrade lands as PRs #414 (zome dependencies) and #415 (the JS side) stacked on top of it. Prefer small, atomic, single-purpose pull requests that are easy to review and land independently, rather than large combined changes.

When opening a PR:

- Base it on the current integration branch, not `sprout` directly, unless your change genuinely has no dependency on in-flight work.
- Give it a clear title and a description covering intent, the changes, and how to test.
- Keep source changes and unrelated formatting or refactors separate.

## Publishing the GraphQL adapter

The adapter is published to npm as `@valueflows/vf-graphql-holochain`:

```bash
yarn run publish:graphql:adapter
```

This builds the module and runs `npm publish --access=public` from the `build/` directory. The
helper `scripts/upgrade-modules.sh` bumps the module version. Publishing is normally done by
maintainers as part of a release.

## Code layout reminders

- Rust zomes: `dnas/hrea/zomes/` (integrity and coordinator). See
  [Architecture](./architecture.md).
- GraphQL adapter: `modules/vf-graphql-holochain/`.
- Demo UI: `ui/`.
- Tests: `tests/sweettest/` (zome boundary), `clients/acceptance/` (GraphQL), `clients/playground-e2e/` (browser).
