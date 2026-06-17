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
| `tests/` | Integration test suite (Vitest + Tryorama). |
| `workdir/` | App-level manifests: `happ.yaml` and `web-happ.yaml`, plus the built bundles. |
| `scripts/` | Build and release automation helpers. |
| `Cargo.toml` | Rust workspace definition for the zome crates. |
| `package.json` | Root workspace and the canonical script entry points. |
| `flake.nix` | Nix dev environment (Holonix, pinned to `main-0.6`). |
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

The root `Cargo.toml` defines a workspace pinning the Holochain 0.6 SDK:

```toml
[workspace.dependencies]
hdi = "=0.7.0"   # Holochain Deterministic Integrity, used by the integrity zome
hdk = "=0.6.0"   # Holochain Development Kit, used by the coordinator zome
```

The crates are: `hrea_integrity` (integrity), `hrea` (coordinator), and `vf_actions` (a local
path dependency of the coordinator).

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

`ui/` is a Svelte + Vite application used to explore an hREA network during development. It
uses `@apollo/client` with `svelte-apollo` and visualizes flows with `@xyflow/svelte`. Run it
standalone with `yarn workspace ui start` (it serves on `$UI_PORT`, default 8888), or as part
of the full `yarn dev` flow.
