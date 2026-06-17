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
5. Run the test suite: `yarn test`.
6. Open a pull request (see the branch model below).

## Testing

The integration suite lives in `tests/` and uses [Vitest](https://vitest.dev) with
[Tryorama](https://github.com/holochain/tryorama), Holochain's multi-conductor test harness.
Tests exercise the GraphQL API end to end against ephemeral in-memory networks, so they
validate the zomes, the adapter, and their wiring together.

```bash
yarn test
```

This rebuilds the zomes and the GraphQL adapter, packs the hApp with `hc app pack`, and runs
Vitest. The suite is organized by domain under `tests/src/lib/` (for example agents, actions,
events, commitments, agreements, planning, plans, units, pagination, and recipe scenarios).
Each test has a 60 second timeout because spinning up conductors is not instantaneous.

When adding a feature, add or extend the matching `graphql.<domain>` test so the GraphQL
surface stays covered.

## Continuous integration

Two workflows live in `.github/workflows/`:

- **`test.yml`** runs on every push and pull request. On `ubuntu-latest` it installs Nix
  (with a Cachix cache), enters the flake shell, installs dependencies, builds the GraphQL
  adapter, builds the WASM and hApp, and runs the integration suite. The job has a 100 minute
  timeout.
- **`release.yml`** runs on tags matching `happ-*` (for example `happ-0.6.0`). It creates a
  GitHub release and builds and uploads the packaged hApp and DNA bundles.

Make sure `yarn test` passes locally before opening a PR, since CI runs the same suite.

## Branch and PR model

hREA development on the Holochain 0.6 line happens against the `main-0.6` branch, with
`sprout` used as an integration branch for in-progress work. Prefer small, atomic,
single-purpose pull requests that are easy to review and land independently, rather than large
combined changes.

When opening a PR:

- Base it on the correct branch (`main-0.6` for the 0.6 line).
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
- Tests: `tests/`.
