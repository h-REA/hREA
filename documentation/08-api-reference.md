# hREA API Reference

## Overview

hREA provides a comprehensive API through GraphQL interfaces that enable interaction with all economic coordination features. The API is built on Holochain's distributed architecture and follows the ValueFlows specification for semantic interoperability.

## API Architecture

### Multi-Layer Interface

```
┌─────────────────────────────────────┐
│         Client Applications          │
├─────────────────────────────────────┤
│      GraphQL API Interface          │
├─────────────────────────────────────┤
│   hREA GraphQL Adapter Library      │
├─────────────────────────────────────┤
│      Holochain Zome Functions       │
├─────────────────────────────────────┤
│         Holochain Runtime           │
└─────────────────────────────────────┘
```

### Access Methods

1. **GraphQL API**: Primary interface for web and mobile applications
2. **Holochain Zomes**: Direct Rust/WASM functions for custom integrations
3. **JavaScript SDK**: TypeScript definitions and helper functions

## GraphQL Schema Overview

### Core Types

#### EconomicEvent
```graphql
type EconomicEvent {
  id: ID!
  action: String!
  note: String
  inputOf: Process
  outputOf: Process
  provider: Agent
  receiver: Agent
  resourceInventoriedAs: EconomicResource
  toResourceInventoriedAs: EconomicResource
  resourceClassifiedAs: [String!]
  resourceConformsTo: ResourceSpecification
  resourceQuantity: QuantityValue
  hasBeginning: DateTime
  hasEnd: DateTime
  hasPointInTime: DateTime
  atLocation: String
  agreedIn: String
  realizationOf: Agreement
  inScopeOf: [Agent!]
  triggeredBy: EconomicEvent
  fulfills: [Commitment!]
  satisfies: [Intent!]
  corrects: EconomicEvent
}
```

#### EconomicResource
```graphql
type EconomicResource {
  id: ID!
  note: String
  classifiedAs: [String!]
  conformsTo: ResourceSpecification
  trackingIdentifier: String
  currentQuantity: QuantityValue
  accountingQuantity: QuantityValue
  onhandQuantity: QuantityValue
  unitOfEffort: Unit
  stage: ProcessSpecification
  state: String
  location: String
  containedIn: EconomicResource
  primaryAccountable: Agent
  owner: Agent
  custodian: Agent
}
```

#### Agent
```graphql
type Agent {
  id: ID!
  name: String!
  agentType: String!
  image: String
  classifiedAs: [String!]
  note: String
  relationships: [AgentRelationship!]!
  economicEventsAsProvider: [EconomicEvent!]!
  economicEventsAsReceiver: [EconomicEvent!]!
  commitmentsAsProvider: [Commitment!]!
  commitmentsAsReceiver: [Commitment!]!
}
```

#### Process
```graphql
type Process {
  id: ID!
  name: String!
  note: String
  plannedStart: DateTime
  plannedEnd: DateTime
  hasBeginning: DateTime
  hasEnd: DateTime
  before: DateTime
  after: DateTime
  basedOn: ProcessSpecification
  finished: Boolean
  triggeredBy: [EconomicEvent!]!
  notices: [EconomicEvent!]!
  plannedInputs: [Commitment!]!
  plannedOutputs: [Commitment!]!
  inputs: [EconomicEvent!]!
  outputs: [EconomicEvent!]!
}
```

#### Commitment
```graphql
type Commitment {
  id: ID!
  action: String!
  note: String
  inputOf: Process
  outputOf: Process
  provider: Agent!
  receiver: Agent!
  resourceInventoriedAs: EconomicResource
  resourceClassifiedAs: [String!]
  resourceConformsTo: ResourceSpecification
  resourceQuantity: QuantityValue
  due: DateTime
  hasBeginning: DateTime
  hasEnd: DateTime
  hasPointInTime: DateTime
  agreedIn: String
  realizationOf: Agreement
  clauseOf: Agreement
  inScopeOf: [Agent!]
  fulfilledBy: [EconomicEvent!]!
  satisfies: [Intent!]
}
```

## Query Operations

### Basic Queries

#### Get Economic Event
```graphql
query GetEconomicEvent($id: ID!) {
  reaEconomicEvent(id: $id) {
    id
    action
    note
    provider {
      id
      name
      agentType
    }
    receiver {
      id
      name
      agentType
    }
    resourceQuantity {
      hasNumericalValue
      hasUnit {
        id
        label
        symbol
      }
    }
    resourceClassifiedAs
    hasPointInTime
    atLocation
  }
}
```

#### Get Agent
```graphql
query GetAgent($id: ID!) {
  reaAgent(id: $id) {
    id
    name
    agentType
    image
    classifiedAs
    note
    economicEventsAsProvider(first: 10) {
      edges {
        node {
          id
          action
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          hasPointInTime
        }
      }
    }
    commitmentsAsProvider(first: 5) {
      edges {
        node {
          id
          action
          due
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
        }
      }
    }
  }
}
```

#### Get Economic Resource
```graphql
query GetEconomicResource($id: ID!) {
  reaEconomicResource(id: $id) {
    id
    note
    classifiedAs
    conformsTo {
      id
      name
    }
    currentQuantity {
      hasNumericalValue
      hasUnit {
        label
        symbol
      }
    }
    accountingQuantity {
      hasNumericalValue
      hasUnit {
        label
        symbol
      }
    }
    trackingIdentifier
    location
    primaryAccountable {
      id
      name
    }
    owner {
      id
      name
    }
  }
}
```

### Advanced Queries

#### Filtered Economic Events
```graphql
query GetFilteredEconomicEvents(
  $action: String
  $provider: ID
  $receiver: ID
  $resourceClassifiedAs: [String!]
  $dateRangeStart: DateTime
  $dateRangeEnd: DateTime
  $first: Int
  $after: String
) {
  reaEconomicEvents(
    filters: {
      reaAction: $action
      provider: $provider
      receiver: $receiver
      resourceClassifiedAs: $resourceClassifiedAs
      dateRange: {
        start: $dateRangeStart
        end: $dateRangeEnd
      }
    }
    first: $first
    after: $after
  ) {
    edges {
      node {
        id
        action
        provider {
          id
          name
        }
        receiver {
          id
          name
        }
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
            symbol
          }
        }
        resourceClassifiedAs
        hasPointInTime
        note
      }
      cursor
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}
```

#### Agent Economic Summary
```graphql
query GetAgentEconomicSummary($agentId: ID!) {
  reaAgent(id: $agentId) {
    id
    name
    agentType
    economicEventsAsProvider {
      totalCount
      edges {
        node {
          id
          action
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          hasPointInTime
          receiver {
            id
            name
          }
        }
      }
    }
    economicEventsAsReceiver {
      totalCount
      edges {
        node {
          id
          action
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          hasPointInTime
          provider {
            id
            name
          }
        }
      }
    }
    commitmentsAsProvider {
      totalCount
      edges {
        node {
          id
          action
          due
          fulfilledBy {
            edges {
              node {
                id
                hasPointInTime
              }
            }
          }
        }
      }
    }
  }
}
```

#### Process Flow Query
```graphql
query GetProcessFlow($processId: ID!) {
  reaProcess(id: $processId) {
    id
    name
    plannedStart
    plannedEnd
    hasBeginning
    hasEnd
    finished
    basedOn {
      id
      name
    }
    inputs {
      edges {
        node {
          id
          action
          resourceInventoriedAs {
            id
            classifiedAs
          }
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          provider {
            id
            name
          }
          hasPointInTime
        }
      }
    }
    outputs {
      edges {
        node {
          id
          action
          resourceInventoriedAs {
            id
            classifiedAs
          }
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          receiver {
            id
            name
          }
          hasPointInTime
        }
      }
    }
    plannedInputs {
      edges {
        node {
          id
          action
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          due
          fulfilledBy {
            edges {
              node {
                id
                hasPointInTime
              }
            }
          }
        }
      }
    }
    plannedOutputs {
      edges {
        node {
          id
          action
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          due
          fulfilledBy {
            edges {
              node {
                id
                hasPointInTime
              }
            }
          }
        }
      }
    }
  }
}
```

## Mutation Operations

### Economic Event Mutations

#### Create Economic Event
```graphql
mutation CreateEconomicEvent($event: EconomicEventCreateParams!) {
  createEconomicEvent(event: $event) {
    economicEvent {
      id
      action
      provider {
        id
        name
      }
      receiver {
        id
        name
      }
      resourceQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      resourceClassifiedAs
      hasPointInTime
      note
    }
  }
}
```

**Example Variables:**
```json
{
  "event": {
    "action": "transfer",
    "provider": "provider-agent-id",
    "receiver": "receiver-agent-id",
    "resourceInventoriedAs": "source-resource-id",
    "toResourceInventoriedAs": "destination-resource-id",
    "resourceQuantity": {
      "hasNumericalValue": 100,
      "hasUnit": "kg"
    },
    "hasPointInTime": "2024-03-15T10:00:00Z",
    "atLocation": "Warehouse A, Loading Dock 3",
    "note": "Weekly delivery"
  }
}
```

#### Update Economic Event
```graphql
mutation UpdateEconomicEvent(
  $revisionId: ID!
  $event: EconomicEventUpdateParams!
) {
  updateEconomicEvent(
    revisionId: $revisionId
    event: $event
  ) {
    economicEvent {
      id
      action
      note
      resourceQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
    }
  }
}
```

#### Delete Economic Event
```graphql
mutation DeleteEconomicEvent($revisionId: ID!) {
  deleteEconomicEvent(revisionId: $revisionId) {
    success
  }
}
```

### Agent Mutations

#### Create Agent
```graphql
mutation CreateAgent($agent: ReaAgentCreateParams!) {
  createReaAgent(agent: $agent) {
    reaAgent {
      id
      name
      agentType
      image
      classifiedAs
      note
    }
  }
}
```

**Example Variables:**
```json
{
  "agent": {
    "name": "Sustainable Foods Co-op",
    "agentType": "Organization",
    "classifiedAs": ["cooperative", "food-distribution", "local"],
    "note": "Community-owned organic food cooperative",
    "image": "https://example.com/coop-logo.png"
  }
}
```

#### Update Agent
```graphql
mutation UpdateAgent(
  $revisionId: ID!
  $agent: ReaAgentUpdateParams!
) {
  updateReaAgent(
    revisionId: $revisionId
    agent: $agent
  ) {
    reaAgent {
      id
      name
      agentType
      classifiedAs
      note
    }
  }
}
```

### Resource Mutations

#### Create Economic Resource
```graphql
mutation CreateEconomicResource($resource: EconomicResourceCreateParams!) {
  createEconomicResource(resource: $resource) {
    economicResource {
      id
      note
      classifiedAs
      conformsTo {
        id
        name
      }
      currentQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      trackingIdentifier
      primaryAccountable {
        id
        name
      }
      owner {
        id
        name
      }
    }
  }
}
```

**Example Variables:**
```json
{
  "resource": {
    "note": "Organic tomato harvest",
    "classifiedAs": ["vegetables", "organic", "tomatoes"],
    "currentQuantity": {
      "hasNumericalValue": 500,
      "hasUnit": "kg"
    },
    "trackingIdentifier": "TOM-2024-03-001",
    "primaryAccountable": "farm-agent-id",
    "owner": "farm-owner-id"
  }
}
```

#### Update Economic Resource
```graphql
mutation UpdateEconomicResource(
  $revisionId: ID!
  $resource: EconomicResourceUpdateParams!
) {
  updateEconomicResource(
    revisionId: $revisionId
    resource: $resource
  ) {
    economicResource {
      id
      currentQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      note
    }
  }
}
```

### Process Mutations

#### Create Process
```graphql
mutation CreateProcess($process: ProcessCreateParams!) {
  createProcess(process: $process) {
    process {
      id
      name
      note
      plannedStart
      plannedEnd
      basedOn {
        id
        name
      }
      finished
    }
  }
}
```

**Example Variables:**
```json
{
  "process": {
    "name": "Q2 Production Run",
    "note": "Second quarter manufacturing process for premium product line",
    "plannedStart": "2024-04-01T09:00:00Z",
    "plannedEnd": "2024-06-30T17:00:00Z",
    "basedOn": "production-spec-id"
  }
}
```

#### Update Process
```graphql
mutation UpdateProcess(
  $revisionId: ID!
  $process: ProcessUpdateParams!
) {
  updateProcess(
    revisionId: $revisionId
    process: $process
  ) {
    process {
      id
      name
      hasBeginning
      hasEnd
      finished
    }
  }
}
```

### Commitment Mutations

#### Create Commitment
```graphql
mutation CreateCommitment($commitment: CommitmentCreateParams!) {
  createCommitment(commitment: $commitment) {
    commitment {
      id
      action
      provider {
        id
        name
      }
      receiver {
        id
        name
      }
      resourceQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      due
      agreedIn
      note
    }
  }
}
```

**Example Variables:**
```json
{
  "commitment": {
    "action": "transfer",
    "provider": "supplier-agent-id",
    "receiver": "customer-agent-id",
    "resourceClassifiedAs": ["electronics", "smartphones"],
    "resourceQuantity": {
      "hasNumericalValue": 1000,
      "hasUnit": "units"
    },
    "due": "2024-04-15T17:00:00Z",
    "agreedIn": "PO-2024-001234",
    "note": "Q2 smartphone delivery commitment"
  }
}
```

## Subscription Operations

### Real-time Event Monitoring

#### Subscribe to Economic Events
```graphql
subscription SubscribeToEconomicEvents($filters: EconomicEventFilters) {
  reaEconomicEvents(filters: $filters) {
    economicEvent {
      id
      action
      provider {
        id
        name
      }
      receiver {
        id
        name
      }
      resourceQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      hasPointInTime
    }
  }
}
```

#### Subscribe to Agent Activity
```graphql
subscription SubscribeToAgentActivity($agentId: ID!) {
  reaAgentActivity(agentId: $agentId) {
    type
    timestamp
    data {
      ... on EconomicEvent {
        id
        action
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
      }
      ... on Commitment {
        id
        action
        due
      }
    }
  }
}
```

## Error Handling

### Error Response Format

```json
{
  "errors": [
    {
      "message": "Validation failed: Resource quantity is required for transfer events",
      "locations": [
        {
          "line": 5,
          "column": 3
        }
      ],
      "path": ["createEconomicEvent"],
      "extensions": {
        "code": "VALIDATION_ERROR",
        "field": "resourceQuantity",
        "details": "Transfer events must specify a valid resource quantity"
      }
    }
  ],
  "data": {
    "createEconomicEvent": null
  }
}
```

### Common Error Codes

| Code | Description | Resolution |
|------|-------------|------------|
| `VALIDATION_ERROR` | Input validation failed | Check required fields and data formats |
| `NOT_FOUND` | Referenced entity doesn't exist | Verify entity IDs exist |
| `PERMISSION_DENIED` | Insufficient permissions | Check agent authorization |
| `CONFLICT` | Data conflict detected | Resolve conflicting data state |
| `RATE_LIMITED` | Too many requests | Reduce request frequency |
| `INTERNAL_ERROR` | System error | Retry or contact support |

## Pagination

### Cursor-based Pagination

```graphql
query PaginatedEconomicEvents($first: Int!, $after: String) {
  reaEconomicEvents(first: $first, after: $after) {
    edges {
      node {
        id
        action
        provider {
          id
          name
        }
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
        hasPointInTime
      }
      cursor
    }
    pageInfo {
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }
    totalCount
  }
}
```

### Pagination Best Practices

1. **Page Size**: Use reasonable page sizes (10-100 items)
2. **Cursor Storage**: Store cursors for navigation, not page numbers
3. **Forward Navigation**: Prefer forward pagination over backward
4. **Data Consistency**: Handle potential data changes between requests

## Filtering and Search

### Advanced Filtering

```graphql
query ComplexFiltering {
  reaEconomicEvents(
    filters: {
      reaAction: "transfer"
      resourceClassifiedAs: ["organic", "vegetables"]
      dateRange: {
        start: "2024-01-01T00:00:00Z"
        end: "2024-12-31T23:59:59Z"
      }
      location: "California"
      agents: ["farm-1", "distributor-2"]
      minValue: 100
      maxValue: 10000
      units: ["kg", "lb"]
    }
  ) {
    edges {
      node {
        id
        action
        provider {
          name
        }
        receiver {
          name
        }
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
        resourceClassifiedAs
        hasPointInTime
        atLocation
      }
    }
  }
}
```

### Text Search

```graphql
query SearchAgents($query: String!) {
  searchAgents(query: $query) {
    edges {
      node {
        id
        name
        agentType
        classifiedAs
        note
      }
      score
    }
  }
}
```

## Performance Optimization

### Query Optimization Tips

1. **Field Selection**: Request only needed fields
2. **Pagination**: Use pagination for large datasets
3. **Filtering**: Apply server-side filters when possible
4. **Batch Operations**: Use batch queries for multiple requests
5. **Caching**: Implement appropriate caching strategies

### Efficient Query Patterns

```graphql
# Good: Specific field selection
query OptimizedEventQuery($id: ID!) {
  reaEconomicEvent(id: $id) {
    id
    action
    hasPointInTime
    resourceQuantity {
      hasNumericalValue
      hasUnit {
        label
      }
    }
  }
}

# Avoid: Excessive field selection
query NotOptimizedEventQuery($id: ID!) {
  reaEconomicEvent(id: $id) {
    id
    action
    note
    inputOf {
      id
      name
      note
      plannedStart
      plannedEnd
      basedOn {
        id
        name
        note
      }
    }
    # ... many more fields
  }
}
```

## Authentication and Authorization

### Authentication Headers

```http
Authorization: Bearer <holochain-app-token>
Content-Type: application/json
```

### Agent-based Authorization

Access is controlled based on the authenticated Holochain agent:

```graphql
# Query will only return events involving the authenticated agent
query MyEconomicEvents {
  reaEconomicEvents(first: 10) {
    edges {
      node {
        id
        action
        provider {
          id
          name
        }
        receiver {
          id
          name
        }
      }
    }
  }
}
```

## Development Tools

### GraphQL Playground

The API includes a GraphQL Playground for testing queries:

```http
GET /graphql/playground
```

### Schema Introspection

Query the schema for introspection:

```graphql
query IntrospectionQuery {
  __schema {
    types {
      name
      description
      fields {
        name
        type {
          name
        }
      }
    }
  }
}
```

### Type Documentation

```graphql
query GetTypeDocumentation {
  __type(name: "EconomicEvent") {
    name
    description
    fields {
      name
      description
      type {
        name
        kind
      }
    }
  }
}
```

## JavaScript/TypeScript SDK

### Installation

```bash
npm install @valueflows/vf-graphql-holochain
```

### Basic Usage

```typescript
import { VFGraphQLHolochain } from '@valueflows/vf-graphql-holochain';

// Initialize client
const client = new VFGraphQLHolochain({
  holochainClient: holochainClient,
  dnaConfig: {
    hrea: {
      zome_name: 'hrea'
    }
  }
});

// Create economic event
const result = await client.mutations.createEconomicEvent({
  event: {
    action: 'transfer',
    provider: 'provider-id',
    receiver: 'receiver-id',
    resourceQuantity: {
      hasNumericalValue: 100,
      hasUnit: 'kg'
    },
    hasPointInTime: new Date().toISOString()
  }
});

// Query economic events
const events = await client.queries.reaEconomicEvents({
  filters: {
    action: 'transfer'
  }
});
```

### TypeScript Definitions

```typescript
interface EconomicEventCreateParams {
  action: string;
  provider?: string;
  receiver?: string;
  resourceInventoriedAs?: string;
  toResourceInventoriedAs?: string;
  resourceClassifiedAs?: string[];
  resourceConformsTo?: string;
  resourceQuantity?: QuantityValue;
  hasPointInTime?: string;
  hasBeginning?: string;
  hasEnd?: string;
  atLocation?: string;
  agreedIn?: string;
  note?: string;
}

interface QuantityValue {
  hasNumericalValue: number;
  hasUnit: string;
}
```

## Testing

### Unit Testing Queries

```javascript
import { gql } from 'apollo-server-express';

const GET_ECONOMIC_EVENT = gql`
  query GetEconomicEvent($id: ID!) {
    reaEconomicEvent(id: $id) {
      id
      action
      provider {
        id
        name
      }
      receiver {
        id
        name
      }
      resourceQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
    }
  }
`;

test('should fetch economic event by ID', async () => {
  const result = await server.executeOperation({
    query: GET_ECONOMIC_EVENT,
    variables: { id: 'test-event-id' }
  });

  expect(result.errors).toBeUndefined();
  expect(result.data.reaEconomicEvent).toBeDefined();
  expect(result.data.reaEconomicEvent.id).toBe('test-event-id');
});
```

### Integration Testing

```javascript
const { VFGraphQLHolochain } = require('@valueflows/vf-graphql-holochain');

describe('Economic Event Integration', () => {
  let client;

  beforeEach(async () => {
    client = new VFGraphQLHolochain(testConfig);
    await client.connect();
  });

  test('should create and retrieve economic event', async () => {
    // Create event
    const createResult = await client.mutations.createEconomicEvent({
      event: {
        action: 'transfer',
        provider: 'test-provider',
        receiver: 'test-receiver',
        resourceQuantity: {
          hasNumericalValue: 10,
          hasUnit: 'units'
        }
      }
    });

    // Retrieve event
    const getResult = await client.queries.reaEconomicEvent({
      id: createResult.economicEvent.id
    });

    expect(getResult.economicEvent.action).toBe('transfer');
    expect(getResult.economicEvent.resourceQuantity.hasNumericalValue).toBe(10);
  });
});
```

## Rate Limiting

### Rate Limit Headers

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

### Best Practices

1. **Batch Operations**: Combine multiple operations in single requests
2. **Efficient Polling**: Use subscriptions instead of polling when possible
3. **Cache Results**: Cache frequently accessed data
4. **Exponential Backoff**: Implement backoff for failed requests

## Migration and Versioning

### API Versioning

```http
# Current stable version
POST /graphql/v1

# Beta features
POST /graphql/v2beta
```

### Schema Changes

Breaking changes will be versioned. Backwards-compatible changes add new fields and types without removing existing ones.

### Migration Guide

See the [Migration Guide](./11-migration.md) for detailed information on upgrading between API versions.

## Troubleshooting

### Common Issues

1. **Connection Errors**: Verify Holochain conductor is running
2. **Authorization Failures**: Check agent credentials and permissions
3. **Validation Errors**: Review required fields and data formats
4. **Performance Issues**: Optimize queries and use pagination
5. **Data Inconsistencies**: Verify data integrity constraints

### Debug Mode

Enable debug logging for development:

```javascript
const client = new VFGraphQLHolochain({
  debug: true,
  logLevel: 'debug'
});
```

### Health Checks

Monitor API health:

```http
GET /health
```

Response:
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "holochainConnected": true,
  "timestamp": "2024-03-15T10:00:00Z"
}
```