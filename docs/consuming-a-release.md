# Consuming an hREA release

For people building a hApp on top of hREA rather than working on hREA itself. If you are here because you pin `happ-0.4.0-beta` and want to move to `happ-0.5.0-beta`, start at [Upgrading from happ-0.4.0-beta](#upgrading-from-happ-040-beta).

## What a release ships

Two files, attached to the GitHub release:

| Artifact | What it is | Use it when |
|---|---|---|
| `hrea.dna` | the hREA DNA alone | you are composing hREA into your own hApp alongside your own DNAs |
| `hrea.happ` | a one-role hApp wrapping that DNA | you want hREA running on its own, as a separate installed app |

`happ-0.4.0-beta` shipped the same two names, so a script that fetches by name keeps working.

Nothing else is published from the release workflow. In particular the npm adapter is **not** published by it: see [Version pinning](#version-pinning) below, because getting this pair wrong is the most common way an integration fails.

## Composing the DNA into your own hApp

The DNA declares one integrity zome and one coordinator zome:

```yaml
# dnas/hrea/workdir/dna.yaml, as built
name: hrea
integrity:
  zomes:
    - name: hrea_integrity
coordinator:
  zomes:
    - name: hrea
      dependencies:
        - name: hrea_integrity
```

In your own `happ.yaml`, add a role pointing at the downloaded `hrea.dna`:

```yaml
roles:
  - name: hrea
    provisioning:
      strategy: create
      deferred: false
    dna:
      path: './hrea.dna'
      modifiers:
        network_seed: null
      clone_limit: 0
```

The zome name your client calls is `hrea`, and the role name is yours to choose. The GraphQL adapter takes both:

```ts
import { createHolochainSchema } from '@valueflows/vf-graphql-holochain'

const schema = createHolochainSchema({ appWebSocket, roleName: 'hrea' })
```

`roleName` must match the role name in your `happ.yaml`, not the DNA's internal name. If you install `hrea.happ` as its own app instead of composing the DNA, the role name is `hrea`.

## Version pinning

Three versions have to agree, and only two of them are in your `package.json`, which is why this is the section to read twice.

| Component | `happ-0.5.0-beta` | Where you set it |
|---|---|---|
| Holochain conductor | 0.7.x | your Nix shell or your Holochain install |
| `@holochain/client` | `^0.21.0` | your `package.json` |
| `@valueflows/vf-graphql-holochain` | see below | your `package.json` |

**The client and the conductor must move together.** `@holochain/client` 0.20 speaks to a 0.6 conductor and 0.21 speaks to a 0.7 one. Mixing them does not fail with a clear message.

**The adapter version line tracks the DNA line.** `0.700.x` pairs with the Holochain 0.7 releases and `0.600.x` with the 0.6 ones, so `@valueflows/vf-graphql-holochain@0.700.0-rc.0` is the adapter for `happ-0.5.0-beta.1`. That correspondence is a deliberate change: `happ-0.4.0-beta` shipped a DNA whose npm adapter was never republished, so the published version sat on the 0.6 line while the DNA had moved on. If you are pinning an older release, check the release notes rather than assuming, because the correspondence only holds from `0.700.0-rc.0` forward.

The adapter also pins `@valueflows/vf-graphql` at `^0.9.1-alpha.5`, which is the ValueFlows 1.0 schema. If you import VF types yourself, use the same line.

## What changed under you: Holochain 0.6 to 0.7

Two breaking changes will reach your code. Neither is hREA's doing; both are things hREA had to absorb, and the fixes are worth copying.

### Validation callback signatures

Every `validate_*` function in an integrity zome changes shape. The action argument is now a `TypedAction<T>`, with `T` naming what kind of action it is:

```rust
// Holochain 0.6
pub fn validate_create_rea_agreement(
    _action: EntryCreationAction,
    _rea_agreement: ReaAgreement,
) -> ExternResult<ValidateCallbackResult>

// Holochain 0.7
pub fn validate_create_rea_agreement(
    _action: TypedAction<EntryCreationData>,
    _rea_agreement: ReaAgreement,
) -> ExternResult<ValidateCallbackResult>
```

The type parameter follows the callback: `EntryCreationData` for create, `UpdateData` for update, `DeleteData` for delete, `CreateLinkData` and `DeleteLinkData` for links. `CreateLink` becomes `TypedAction<CreateLinkData>`.

At the dispatch sites in `lib.rs`, an `EntryCreationAction::Create(action)` or `::Update(action)` arm now hands the inner action on with `action.into()`. Link dispatch changes too: `create_link.target_address` and `.tag` need cloning where they did not before.

This is mechanical but wide. In hREA it touched 22 files and about 546 lines, and the compiler finds every site, so the safe way through is to let it: change the `hdk`/`hdi` pins first and work the error list down. Do not skip a module because it looked untouched. The three entity modules added most recently in hREA were missed on the first pass and produced 73 compile errors on their own.

### The Action wire format split

An Action is now `{ header, data }`. The fields every variant shares, `author`, `timestamp`, `action_seq`, `prev_action`, moved under `header`; the variant payload moved under `data`. On 0.6 they sat flat on the action's `content`.

Anything reading `signed_action.hashed.content.timestamp` gets `undefined` on a 0.7 conductor, silently. hREA's adapter handles it in `modules/vf-graphql-holochain/src/util.ts`:

```ts
export function actionTimestamp(content: any): any {
  return content?.header?.timestamp ?? content?.timestamp
}
```

If you go through the adapter's resolvers you get this for free. If you read raw action data, use the same fallback rather than committing to either shape.

### Building zomes for wasm

`hdi 0.8` pulls in `getrandom 0.3`, which refuses to build for `wasm32-unknown-unknown` unless a backend is chosen. Your build command needs the flag hREA's does:

```bash
RUSTFLAGS='--cfg getrandom_backend="custom"' cargo build --release --target wasm32-unknown-unknown
```

Without it the build fails with a `getrandom` error that does not mention your code.

## Upgrading from happ-0.4.0-beta

**The DNA hash changes.** Zome wasm changed, so this is a different DNA, and a `0.5.0-beta` network is a different network from a `0.4.0-beta` one. Agents on the two cannot see each other, and there is no in-place data migration: existing `0.4.0-beta` data does not carry over. Treat it as a new network, which for a beta consumer usually means re-seeding test data.

**`happ-0.4.0-beta` is not deleted, moved, or re-tagged.** If you need to stay on 0.6 for a while, it remains available.

**Schema additions since 0.4.0-beta**, all additive:

- `Proposal.purpose`, with the `ProposalPurpose` enum (`offer`, `request`), and the `offers` and `requests` queries that partition on it
- `Claim` with `settles` and `Claim.settledBy`
- `SpatialThing`
- `AgreementBundle`
- `ResourceSpecification.mediumOfExchange`
- `EconomicEvent.reciprocalRealizationOf`, `Commitment.reciprocalClauseOf`

**Behaviour that changed rather than being added:** ValueFlows 1.0 removed `ProposedIntent`, `Satisfaction` and `Fulfillment` as separate classes. If your integration modelled them as entities, they are now direct fields: `EconomicEvent.fulfills` and `.satisfies`, with `Commitment.fulfilledBy` and `Intent.satisfiedBy` as the reverse edges. VF 1.0's `Agreement` also has no parties; bind participation through the commitments and events that reference the agreement.

**Integrity validation is real now.** `0.4.0-beta` shipped stub validators on most entities. This release enforces the ValueFlows rules: required strings, temporal ordering, quantity bounds, the action vocabulary, collection bounds, immutable-on-update fields. Writes that a `0.4.0-beta` conductor accepted may now be rejected, with a message naming the rule. That is the point of the release, and it is the change most likely to surprise an existing integration.

## Where to go next

- [Getting started](./getting-started.md) if you want to run hREA locally rather than consume a build
- [GraphQL API](./graphql-api.md) for the schema, including which mutations do not exist
- [Architecture](./architecture.md) for the entry types and coordinator shape
- [Contributing](./contributing.md) for the test surfaces, if you are reporting a bug and want to reproduce it
