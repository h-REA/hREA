# GraphQL API & Integration

The primary way to build on hREA is the GraphQL adapter
[`@valueflows/vf-graphql-holochain`](https://www.npmjs.com/package/@valueflows/vf-graphql-holochain).
It produces an executable GraphQL schema whose resolvers call into the hREA coordinator zome,
so your application talks ValueFlows GraphQL and the adapter handles Holochain.

## Install

```bash
npm install @valueflows/vf-graphql-holochain @holochain/client graphql
# plus your GraphQL client, for example:
npm install @apollo/client
```

The package is published as an ES module (`"type": "module"`, entry `build/index.js`, types `build/index.d.ts`). It depends on `@holochain/client ^0.21.0`, so it needs a Holochain 0.7 conductor.

The version line tracks the DNA line: `0.700.x` pairs with the Holochain 0.7 releases, `0.600.x` with the 0.6 ones. `0.700.0-rc.0` is the adapter published alongside `happ-0.5.0-beta.1`.

## The public API

The module exposes a single function:

```typescript
function createHolochainSchema(params: {
  appWebSocket: any
  roleName: string
  cell?: any   // pass a pre-built cell instead of appWebSocket + roleName
}): GraphQLSchema
```

It builds the ValueFlows GraphQL schema (from `@valueflows/vf-graphql`) and binds resolvers
that call the hREA zomes via `appWebSocket.callZome({ role_name: roleName, ... })`. The
`roleName` must match the role declared in the hApp manifest, which for hREA is `hrea` (see
`workdir/happ.yaml`).

The following ValueFlows modules are enabled in the generated schema: `util`, `pagination`,
`history`, `agent`, `action`, `plan`, `commitment`, `proposal`, `recipe`, `process`,
`measurement`, `observation`, `process_specification`, `resource_specification`, `agreement`,
and `intent`.

## Wiring it into Apollo Client

The adapter only produces a schema. You bring your own GraphQL client. With Apollo, use
`SchemaLink` to execute operations directly against the in-process schema:

```typescript
import { AppWebsocket } from '@holochain/client'
import { createHolochainSchema } from '@valueflows/vf-graphql-holochain'
import { ApolloClient, InMemoryCache } from '@apollo/client'
import { SchemaLink } from '@apollo/client/link/schema'

// 1. Connect to the Holochain conductor.
const appWebSocket = await AppWebsocket.connect()

// 2. Build the executable schema bound to the `hrea` role.
const schema = createHolochainSchema({ appWebSocket, roleName: 'hrea' })

// 3. Wire the schema into Apollo.
const client = new ApolloClient({
  link: new SchemaLink({ schema }),
  cache: new InMemoryCache(),
})
```

From here you issue ordinary GraphQL operations with `client.query(...)` and
`client.mutate(...)`.

## What you can query and mutate

Queries come in singular and collection (paginated) forms. Mutations are **not** uniformly create / update / delete: the exceptions are listed below the table, and they are load-bearing if you are planning an integration.

- Agents: `agent`, `agents`, `person`, `people`, `organization`, `organizations`
- Observation: `economicEvent(s)`, `economicResource(s)`
- Planning: `commitment(s)`, `intent(s)`
- Coordination: `proposal(s)`, `agreement(s)`, `plan(s)`, `offers`, `requests`
- Process: `process(es)`, `processSpecification(s)`
- Specification: `resourceSpecification(s)`, `unit(s)`, `action(s)`
- Recipes: `recipeExchange(s)`, `recipeProcess(es)`, `recipeFlow(s)`
- VF 1.0 additions: `claim(s)`, `spatialThing(s)`, `agreementBundle(s)`

Where the CRUD set is incomplete, verified against `modules/vf-graphql-holochain/src/mutations/index.ts`:

| Type | Missing | Why it matters |
|---|---|---|
| `EconomicEvent` | no `deleteEconomicEvent` | events are an append-only record of what happened; correct one with a compensating event |
| `EconomicResource` | no `createEconomicResource`, no `deleteEconomicResource` | resources come into being through an event, via `newInventoriedResource` on `createEconomicEvent` |

`offers` and `requests` are hREA extensions, not base ValueFlows: they partition proposals by `Proposal.purpose` through `get_proposals_by_purpose`. `purpose` itself is an hREA addition to the schema (`ProposalPurpose`, values `offer` and `request`), alongside `ResourceSpecification.mediumOfExchange`, `EconomicEvent.reciprocalRealizationOf`, `EconomicEvent.settles` and `Commitment.reciprocalClauseOf`. All of them are declared in `modules/vf-graphql-holochain/index.ts`.

Two things the zome stores that the schema does not yet expose: `ReaResourceSpecification.substitutable`, and `agentRelationship` with its role types, which the base schema declares but this adapter does not implement. `clients/acceptance/README.md` tracks the latter as a known gap.

Field resolvers automatically follow relationships. For example, querying a commitment's
`provider` resolves the linked agent, and an economic event's `resourceInventoriedAs` resolves
the linked resource. The adapter keeps a small in-memory cache (with a short revision TTL) to
avoid redundant zome calls, and it converts between camelCase GraphQL fields and the zomes'
snake_case payloads, and between JavaScript dates and Holochain timestamps.

## Example query

Fetch a page of economic resources with their specification, custodian, and quantities:

```graphql
query GetEconomicResources {
  economicResources(first: 10) {
    edges {
      node {
        id
        name
        conformsTo { id name }
        primaryAccountable { id name }
        accountingQuantity { hasNumericalValue hasUnit { id symbol label } }
        onhandQuantity { hasNumericalValue hasUnit { id symbol } }
      }
    }
    pageInfo { startCursor endCursor hasNextPage }
  }
}
```

## Example mutation

Record an economic event. Because `produce` increments inventory, this creates or updates the
associated economic resource:

```graphql
mutation CreateEconomicEvent {
  createEconomicEvent(event: {
    action: "produce"
    provider: "<agent-id>"
    receiver: "<agent-id>"
    resourceConformsTo: "<resource-spec-id>"
    resourceQuantity: { hasNumericalValue: 5, hasUnit: "<unit-id>" }
    hasPointInTime: "2026-06-16T10:00:00Z"
    outputOf: "<process-id>"
    note: "Produced 5 widgets"
  }) {
    economicEvent {
      id
      action { id label }
      provider { id name }
      resourceQuantity { hasNumericalValue hasUnit { id symbol } }
    }
  }
}
```

The `action` value (`"produce"`, `"consume"`, `"transfer"`, and so on) must be one of the
built-in ValueFlows actions. See [Architecture](./architecture.md#the-action-system-vf_actions)
for the action set and how actions affect inventory.

## Gotcha: Action shape differs between Holochain 0.6 and 0.7

Holochain 0.7 splits an Action into `{ header, data }`: the fields every variant shares (`author`, `timestamp`, `action_seq`, `prev_action`) moved under `header`, and the variant-specific payload moved under `data`. On 0.6 those fields sat flat on the action's `content`. Anything reading `signed_action.hashed.content.timestamp` directly gets `undefined` on a 0.7 conductor.

The adapter handles this with an `actionTimestamp()` helper in `modules/vf-graphql-holochain/src/util.ts`, which reads the 0.7 path and falls back to the 0.6 one: `content?.header?.timestamp ?? content?.timestamp`. If you read raw action data yourself instead of going through the adapter's resolvers, use the same fallback rather than assuming either shape.

## Notes for app developers

- The adapter does not depend on Apollo; any GraphQL execution layer that accepts an executable
  schema works.
- You can pass a pre-constructed `cell` object (with a `callZome` method) via the `cell`
  parameter instead of `appWebSocket` + `roleName`, which is useful for testing.
- For a working example of the schema in use, see the demo app under `ui/` and the GraphQL acceptance suite under `clients/acceptance/` (see [Contributing](./contributing.md#testing)).
