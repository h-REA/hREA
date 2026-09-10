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

- **`tests/sweettest`** is a native Rust crate that drives `SweetConductor` against the packed DNA, testing at the zome boundary. That is where VF 1.0's integrity validation actually runs, so a rejection can be asserted on its actual message rather than only on the fact that something failed. Files: `tests/sweettest/tests/proposal.rs` (the `vf:Proposal.purpose` rules) and `tests/sweettest/tests/integrity_gate.rs` (cross-entity integrity rules). It declares its own `[workspace]`, and the root `Cargo.toml` excludes it, deliberately: without the split, `yarn build:zomes` (`cargo build --target wasm32-unknown-unknown`) would try to compile Holochain itself for wasm. CI caches `tests/sweettest/target/` separately for the same reason.

  ```bash
  yarn run test:sweettest
  ```

- **`clients/acceptance`** is the GraphQL adapter surface. It spawns its own ephemeral `hc sandbox` conductor and drives the schema over `@holochain/client`, so it needs nothing from Tryorama. Modules are registered in `clients/acceptance/src/run.ts`: `core`, `rea-flows`, `crud`, `recipes`, `regressions`. Run a single module with `--only=<module>`; `--demo` narrates the same scenario instead of just asserting it.

  ```bash
  yarn run test:acceptance
  # equivalently, from the workspace directly:
  yarn workspace hrea-acceptance run check
  ```

  `clients/acceptance/src/scenarios/regressions.ts` exists specifically to hold the checks the Tryorama retirement would otherwise have dropped: `updateCommitment`, `updateAgreement`, `updatePlan`, `Intent.observedBy`, `Plan.nonProcessCommitments` with `Process.committedInputs`/`committedOutputs`, the `spatialThings` / `agreementBundles` / bare `economicEvents` collection queries, relay `before`/`last` pagination on `agents`, `Agreement.name` read-back, and the check that a resource stays listed exactly once after repeated events. It is kept as one file so the mapping from the retired suite stays auditable.

`test:sweettest` packs the DNA itself (`yarn run build:happ && cargo test --manifest-path tests/sweettest/Cargo.toml`). `test:acceptance` expects the hApp and the adapter to already be built.

### Known limitation: local test conductors are not network-isolated

On Holochain 0.7, `hc sandbox` has no `network` subcommand at any level: `generate` accepts only `-n`, `--root`, `-d`, `--in-process-lair`, `-r`/`--run`, `-s`/`--network-seed`, and `--roles-settings`. Arguments like `network mem` (still present in `clients/acceptance/src/harness.ts` and in the root `launch:happ1` script) are silently ignored, and the generated conductor config keeps its defaults: `bootstrap_url: https://dev-test-bootstrap2.holochain.org/` and an iroh relay. Local test conductors therefore join a public bootstrap and can gossip with each other, and with anyone else running the same DNA against that bootstrap.

This is a known limitation, not a solved one. In practice it means an assertion that counts a whole collection is not deterministic; assert on specific ids instead, the way `clients/acceptance` already does.

When adding a feature, add the zome-boundary rule to `tests/sweettest` if it is a validation rule, and the GraphQL-shape behavior to the matching `clients/acceptance` scenario module.

## Continuous integration

Two workflows live in `.github/workflows/`:

- **`test.yml`** (workflow name `Checks`) runs on every push and pull request. On `ubuntu-latest` it installs Nix (with a Cachix cache), enters the flake shell, installs dependencies, builds the GraphQL adapter, builds the WASM and hApp, then runs the Sweettest suite and the acceptance suite in turn. The job has a 100 minute timeout, and caches `tests/sweettest/target/` alongside the usual Cargo directories so the conductor is not recompiled from scratch on every run.

- **`release.yml`** fires on any pushed tag matching `happ-*` (for example `happ-0.4.0-beta`), from any branch. It creates a GitHub release and uploads `bundles/app/full_suite/hrea_suite.happ` plus seven per-module DNA bundles: agent, agreement, observation, plan, planning, proposal, and specification.

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
