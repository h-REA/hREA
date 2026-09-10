# hREA Acceptance Client

A scriptable GraphQL API client that doubles as an **automated acceptance surface** and a **live demo** of hREA correctness. It spawns an ephemeral `hc sandbox` conductor, installs the packed hApp, and drives it through an Apollo Client over a `SchemaLink`, connected with `@holochain/client` — the same access path real consumer apps (e.g. Requests-and-Offers) use.

## Where a test belongs

This suite owns the GraphQL adapter: resolvers, field shapes, collections,
pagination. Integrity rules belong one layer down, in `tests/sweettest`, where
the rejection message can be asserted. The full three-suite contract is in
`tests/sweettest/README.md`.

## The battery (4 modules)

**`core`** — the VF 1.0 headline surface: Person agents, resource-specification booleans (#398/#407), offer/request proposals with `purpose` (#322) incl. immutability rejection, offers/requests index partition, transfer events, Claim settlement (`settles` → `settledBy`), `Process.intendedOutputs`, action-vocabulary parity (21 ids) and rejection guards (enum, action gate, temporal rule).

**`rea-flows`** — the REA engine in depth: an EconomicResource born from a `produce` event (with `conformsTo` inheritance), accounting arithmetic across produce/raise/lower/consume sequences, process `observedInputs`/`observedOutputs`, commitment→`fulfilledBy`, intent→`satisfiedBy`, agreement reverse fields, plan wiring (`plannedWithin`, `independentDemandOf`).

**`crud`** — depth and edge cases: full Unit lifecycle (create/partial update/delete), people/organizations/agents partition, sparse partial updates across entities, delete visibility, cursor pagination without duplicates, SpatialThing WGS84 bounds, AgreementBundle membership, millisecond temporal round-trips, invalid actions on Commitment/Claim, and a **tripwire** documenting that agentRelationship mutations are schema-only (unimplemented in the DNA — the step starts failing the day someone implements them).

Every step is asserted; any failure flips the process exit code — CI-ready. Run one module with `tsx src/run.ts --only=rea-flows`.

## Usage

```bash
# prerequisites (repo root): the packed hApp must exist
nix develop
yarn build:happ          # or: yarn test (packs it as a side effect)

# from the repo root
yarn workspace hrea-acceptance run check   # headless: compact output, exit code 0/1
yarn workspace hrea-acceptance run demo    # narrative demo of the same scenario

# or via the root shortcuts
yarn test:acceptance
yarn demo:acceptance
```

`FATAL` + exit code 2 means the harness itself failed (missing hApp bundle, conductor spawn failure) rather than an assertion.

## Extending scenarios

Add steps in `src/scenario.ts` using the `step(name, fn)` helper — throw inside the callback to fail the step, return a one-line evidence string to pass it. Use `expectRejection` for negative paths. The Apollo client is plain `@apollo/client/core`; any query/mutation the `@valueflows/vf-graphql-holochain` schema exposes can be exercised.

Notes:
- The GraphQL schema requires `provider`/`receiver` on `EconomicEvent`, so the integrity zome's transfer-two-agent rule can only be hit by direct zome calls — it is not (and cannot be) covered here.
- The `purpose="banana"` rejection is caught by the GraphQL enum guard; the equivalent integrity-zome rule additionally protects against direct zome calls.
- After editing the adapter, rebuild it (`yarn build:graphql:adapter`) — the `fresh` pre-step in `check`/`demo` clears yarn's nested copy so the client always resolves the live `modules/vf-graphql-holochain/build`. After editing zomes, rebuild + repack (`yarn build:happ`).
