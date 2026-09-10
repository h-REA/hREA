# Getting Started

This guide takes you from a fresh clone to a running two-agent hREA network with the demo UI. It targets Holochain 0.7 (see [docs/README.md](./README.md) for exactly which release this describes).

## Prerequisites

The supported and reproducible way to get the full toolchain is **Nix**. The repository
ships a `flake.nix` that pins everything you need.

- [Nix](https://nixos.org/download.html) with flakes enabled.

- That is the only hard requirement. The Nix dev shell provides Holochain, the `hc` CLI, `lair-keystore`, the bootstrap service, the Rust toolchain (with the `wasm32-unknown-unknown` target), Node.js 24, `yarn`, and `binaryen`.

If you prefer to manage tools yourself instead of Nix, you will need: Holochain 0.7.x (`hc`, `holochain`), Rust with the `wasm32-unknown-unknown` target, Node.js 24+, and `binaryen`. Nix is strongly recommended.

## Enter the development shell

```bash
nix develop
```

This drops you into the Holonix shell (pinned to `holochain/holonix?ref=main-0.7`, locked in `flake.lock` at rev `ffcc7c63`) with the prompt `[holonix:...]`. Run every command below from inside this shell.

## Install dependencies

```bash
yarn install
```

The repository is an npm/yarn workspace with four members: `ui`, `modules/vf-graphql-holochain`, `clients/acceptance`, and `clients/playground-e2e`. The root scripts invoke `yarn`.

## Build

The build pipeline compiles the Rust zomes to WebAssembly, then packs them into a DNA and an
hApp bundle. See [Repository Structure](./repository-structure.md#build-artifact-flow) for the
full artifact flow.

```bash
# Compile both zomes to wasm (target/wasm32-unknown-unknown/release/*.wasm)
yarn run build:zomes

# Build the GraphQL adapter (TypeScript -> build/)
yarn run build:graphql:adapter

# Pack the DNA and hApp (workdir/hrea.happ)
yarn run build:happ

# Everything in one shot: zomes + adapter + happ
yarn run build
```

`build:zomes` runs `cargo build --release --target wasm32-unknown-unknown` with `RUSTFLAGS='--cfg getrandom_backend="custom"'`. That flag is load-bearing on Holochain 0.7: `hdi 0.8` pulls in `getrandom 0.3` for the `wasm32-unknown-unknown` target, and `getrandom 0.3` refuses to build for that target unless a backend is selected. `build:happ` runs `hc app pack workdir --recursive`.

## Run a local network

```bash
# Two-agent dev network with the demo UI on http://localhost:8888
yarn dev
```

`yarn dev` builds the zomes and adapter, cleans previous conductor state, packs the hApp, and
runs the UI, the conductor, and the local signal and bootstrap services together. The conductor
uses admin port 22994 and app port 29281 by default.

For a network with a custom number of agents plus the Holochain Playground introspection tool:

```bash
AGENTS=3 yarn network
```

To connect to the public Holo infrastructure instead of local services, use `yarn dev:online`.

## Run the tests

There is no single `yarn test` anymore. Tryorama is retired: the last published `@holochain/tryorama`, 0.19.2, depends on `@holochain/client ^0.20.4`, and no version of Tryorama targets Holochain 0.7 or client 0.21, so it was the thing blocking this upgrade. Two independent surfaces replace the suite it used to drive:

```bash
# Zome-boundary tests: a native Rust crate driving SweetConductor against the packed DNA
yarn run test:sweettest

# GraphQL adapter tests: spawns its own ephemeral hc sandbox conductor
yarn run test:acceptance
```

`test:sweettest` packs the DNA itself before running. `test:acceptance` expects `yarn run build:happ` to have already produced `workdir/hrea.happ`. See [Contributing](./contributing.md#testing) for what each surface covers, why the split exists, and a known limitation with local test conductors on 0.7.

## Next steps

- Building an app? Go to [GraphQL API & Integration](./graphql-api.md).
- Working on the zomes? Read [Architecture](./architecture.md).
