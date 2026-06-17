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

The package is published as an ES module (`"type": "module"`, entry `build/index.js`, types
`build/index.d.ts`). The version on the `main-0.6` line is `0.600.0-rc.0`.

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

The schema covers the full hREA record set. Queries come in singular and collection
(paginated) forms, and mutations provide create / update / delete for each type:

- Agents: `agent`, `agents`, `person`, `people`, `organization`, `organizations`
- Observation: `economicEvent(s)`, `economicResource(s)`
- Planning: `commitment(s)`, `intent(s)`
- Coordination: `proposal(s)`, `agreement(s)`, `plan(s)`
- Process: `process(es)`, `processSpecification(s)`
- Specification: `resourceSpecification(s)`, `unit(s)`, `action(s)`
- Recipes: `recipeExchange(s)`, `recipeProcess(es)`, `recipeFlow(s)`

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

## Notes for app developers

- The adapter does not depend on Apollo; any GraphQL execution layer that accepts an executable
  schema works.
- You can pass a pre-constructed `cell` object (with a `callZome` method) via the `cell`
  parameter instead of `appWebSocket` + `roleName`, which is useful for testing.
- For a working example of the schema in use, see the demo app under `ui/` and the integration
  tests under `tests/`.
