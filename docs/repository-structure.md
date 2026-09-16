# Repository Structure

hREA is a monorepo containing the Holochain DNA (Rust zomes), the GraphQL adapter
(TypeScript), a demo UI, and an integration test suite. This document maps the layout and the
build artifact pipeline.

## Top-level layout

| Path | Purpose |
|------|---------|
| `dnas/` | Holochain DNA definitions. |
| `dnas/hrea/` | The single hREA DNA. |
| `dnas/hrea/zomes/integrity/hrea/` | Integrity zome (`hrea_integrity` crate): entry and link type definitions plus validation. |
| `dnas/hrea/zomes/coordinator/hrea/` | Coordinator zome (`hrea` crate): all application logic (CRUD, queries, signals). |
| `dnas/hrea/zomes/coordinator/hrea/vf_actions/` | Embedded `vf_actions` crate: the ValueFlows action system. |
| `dnas/hrea/workdir/dna.yaml` | DNA manifest wiring the two zomes together. |
| `modules/` | JavaScript/TypeScript workspace modules. |
| `modules/vf-graphql-holochain/` | The GraphQL adapter, published as `@valueflows/vf-graphql-holochain`. |
| `ui/` | Demo and explorer web app (Svelte + Vite). |
| `tests/sweettest/` | Zome-boundary test crate, native Rust, driving Holochain's `SweetConductor` against the packed DNA. Tryorama and the Vitest suite that used to live under `tests/src/` are retired. |
| `clients/acceptance/` | GraphQL adapter acceptance suite: spawns its own ephemeral `hc sandbox` conductor and drives the schema over `@holochain/client`. |
| `clients/playground-e2e/` | Playwright browser tests driving the demo UI against a live sandboxed conductor. |
| `workdir/` | App-level manifests: `happ.yaml` and `web-happ.yaml`, plus the built bundles. |
| `scripts/` | Build and release automation helpers. |
| `Cargo.toml` | Rust workspace definition. Members are the zome crates plus `tests/sweettest`; `default-members` is the zome crates alone. |
| `package.json` | Root workspace and the canonical script entry points. |
| `flake.nix` | Nix dev environment (Holonix, pinned to `main-0.7`). |
| `.github/workflows/` | CI (`test.yml`) and release (`release.yml`). |
| `README.md` | Project overview. |

Both `package-lock.json` and `yarn.lock` are present in the tree; the root scripts call
`yarn`, so use the commands as written in `package.json`.

## The DNA

The hREA DNA is composed of exactly two zomes:

- **`hrea_integrity`** defines all entry types and link types and their validation. Integrity
  zomes are the immutable, deterministic part of a Holochain app.
- **`hrea`** (coordinator) contains all callable logic: the `create_*` / `get_*` / `update_*`
  / `delete_*` functions, collection queries, link traversal, and post-commit signals.

The coordinator depends on the integrity zome, declared in `dnas/hrea/workdir/dna.yaml`:

```yaml
manifest_version: '0'
name: hrea
integrity:
  zomes:
  - name: hrea_integrity
    path: '../../../target/wasm32-unknown-unknown/release/hrea_integrity.wasm'
coordinator:
  zomes:
  - name: hrea
    path: '../../../target/wasm32-unknown-unknown/release/hrea.wasm'
    dependencies:
    - name: hrea_integrity
```

See [Architecture](./architecture.md) for what lives inside each zome.

## The Rust workspace

The root `Cargo.toml` defines a workspace pinning the Holochain 0.7 SDK:

```toml
[workspace.dependencies]
hdi = "=0.8.0"   # Holochain Deterministic Integrity, used by the integrity zome
hdk = "=0.7.0"   # Holochain Development Kit, used by the coordinator zome
holochain_serialized_bytes = "0.0.57"
```

The crates are: `hrea_integrity` (integrity), `hrea` (coordinator), and `vf_actions` (a local path dependency of the coordinator).

`tests/sweettest` is a fourth member, and it is deliberately not a `default-member`:

```toml
members = ["dnas/*/zomes/coordinator/*", "dnas/*/zomes/integrity/*", "tests/sweettest"]
default-members = ["dnas/*/zomes/coordinator/*", "dnas/*/zomes/integrity/*"]
```

That crate builds for the host and pulls in the full conductor (`holochain = { version = "=0.7.0", features = ["test_utils"] }`), which must never reach the wasm build. `default-members` is what prevents it: a bare `cargo build --release --target wasm32-unknown-unknown` (`yarn build:zomes`) resolves to the zome crates only, and the wasm dependency graph contains no conductor crate. `cargo test -p hrea-sweettest` names the member explicitly and gets it.

The crate used to be `exclude`d and carry its own `[workspace]`. Membership is the better shape for one reason that cost real time: two workspaces meant two `Cargo.lock` files, so the conductor's version lived in a file the 0.7 bump never read, and a 0.6.1 conductor was handed 0.7 wasm until every Sweettest test failed at module build with `unknown import "env"."__hc__get_init_properties_1"`. One lockfile makes that drift impossible to express.

One consequence to know: cargo ignores `[profile.*]` in a non-root member, so the test profile lives at the workspace root, where `[profile.test] opt-level = 0` opts the native conductor build back out of the `opt-level = "z"` that exists to keep the wasm small.

## Build artifact flow

The build transforms Rust source into a distributable hApp through a fixed chain of manifests:

```
Rust source (dnas/hrea/zomes/)
   │  build:zomes  ->  cargo build --release --target wasm32-unknown-unknown
   ▼
target/wasm32-unknown-unknown/release/{hrea_integrity.wasm, hrea.wasm}
   │  referenced by dnas/hrea/workdir/dna.yaml
   ▼
   │  build:happ  ->  hc app pack workdir --recursive
   ▼
workdir/hrea.dna           (DNA bundle)
workdir/hrea.happ          (hApp: workdir/happ.yaml + hrea.dna)
   │  package  ->  hc web-app pack workdir --recursive
   ▼
workdir/hrea.webhapp       (web hApp: web-happ.yaml + UI assets + hrea.happ)
```

The app manifest `workdir/happ.yaml` declares a single role named `hrea`:

```yaml
manifest_version: '0'
name: hrea
roles:
- name: hrea
  dna:
    path: '../dnas/hrea/workdir/hrea.dna'
```

That `hrea` role name is what application code passes as `roleName` when creating the GraphQL
schema (see [GraphQL API & Integration](./graphql-api.md)).

## The modules workspace

`modules/vf-graphql-holochain/` is the bridge between Holochain and GraphQL. It is built with
the TypeScript compiler into `build/` and published to npm. Its single public export,
`createHolochainSchema`, binds the ValueFlows GraphQL schema to the hREA zomes. See
[GraphQL API & Integration](./graphql-api.md) for the full API.

## The demo UI

`ui/` is a Svelte + Vite application used to explore an hREA network during development. It uses `@apollo/client` with `svelte-apollo` and visualizes flows with `@xyflow/svelte`. Run it standalone with `yarn workspace ui start` (it serves on `$UI_PORT`, default 8888), or as part of the full `yarn dev` flow.

## The clients workspace

`clients/` holds two consumer-side test surfaces, each named `hrea-acceptance` and `hrea-playground-e2e` in their own `package.json`:

- `clients/acceptance/` drives the packed hApp through `@holochain/client` and the GraphQL adapter, over an ephemeral `hc sandbox` conductor it spawns itself. It doubles as an automated acceptance battery and a narrated demo (`yarn workspace hrea-acceptance run demo`).

- `clients/playground-e2e/` is a Playwright suite that drives the real demo UI in a browser against a live sandboxed conductor, proving the human-facing surface rather than the API.

See [Contributing](./contributing.md#testing) for how these fit alongside `tests/sweettest`.
