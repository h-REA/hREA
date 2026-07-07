# hREA Acceptance Client

A scriptable GraphQL API client that doubles as an **automated acceptance surface** and a **live demo** of hREA correctness. It spawns an ephemeral Holochain conductor (tryorama), installs the packed hApp, and drives it through an Apollo Client over a `SchemaLink` — the same access path real consumer apps (e.g. Requests-and-Offers) use.

## What the scenario proves

| Step | VF 1.0 surface exercised |
|---|---|
| Two Person agents | `createPerson`, agent round-trip |
| ResourceSpecifications | `substitutable`, `mediumOfExchange` booleans (#398 / #407) |
| Offer proposal | `Proposal.purpose="offer"`, `publishes`, `reciprocal` intents (#322) |
| Re-read by id | field-level round-trip integrity |
| Request proposal | `Proposal.purpose="request"` |
| `offers` / `requests` queries | purpose DHT index partition, no cross-leak |
| Transfer event | two-agent VF `transfer` action |
| Claim + settlement | `EconomicEvent.settles` → `Claim.settledBy` reverse relation (VF 1.0 settlement shape) |
| purpose="banana" | ProposalPurpose guard rejects |
| action="banana" | `VF_BUILTIN_ACTIONS` vocabulary gate rejects |
| hasBeginning > hasEnd | VF temporal validation rejects |

Every step is asserted; any failure flips the process exit code — CI-ready.

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
