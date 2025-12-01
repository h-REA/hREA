# Agreements and Commitments in hREA

## Overview

**Agreements** and **Commitments** form the contractual and promise-based foundation of economic coordination in hREA. These elements enable formal and informal economic relationships, from legally binding contracts to casual promises, providing the structure for trust-based economic interactions.

## Agreements

### Agreement Structure

The agreement system captures formal contracts, informal agreements, and mutual understandings between economic agents:

```rust
pub struct ReaAgreement {
    pub id: Option<ActionHash>,              // Unique identifier
    pub name: String,                        // Agreement name/identifier
    pub note: Option<String>,                 // Additional notes
    pub created: Option<Timestamp>,           // Creation timestamp
    pub agreed_in: Option<String>,             // External agreement reference
    pub classification: Option<String>,        // Agreement classification
    pub in_scope_of: Option<Vec<ActionHash>>, // Scope agents
}
```

### Agreement Types

#### Formal Legal Agreements

```json
{
  "name": "Supply Agreement 2024-001",
  "note": "Formal supply agreement between manufacturer and distributor",
  "created": "2024-01-15T10:00:00Z",
  "agreed_in": "LEGAL-CONTRACT-2024-001",
  "classification": "legal-contract",
  "in_scope_of": ["manufacturer-id", "distributor-id"]
}
```

#### Informal Agreements

```json
{
  "name": "Community Garden Collaboration",
  "note": "Informal agreement between neighbors for shared garden space",
  "created": "2024-03-01T18:00:00Z",
  "classification": "informal-agreement",
  "in_scope_of": ["neighbor-1-id", "neighbor-2-id", "neighbor-3-id"]
}
```

#### Service Level Agreements

```json
{
  "name": "IT Support SLA",
  "note": "Service level agreement for IT support services",
  "created": "2024-01-01T00:00:00Z",
  "agreed_in": "SLA-IT-2024-001",
  "classification": "service-level-agreement",
  "in_scope_of": ["service-provider-id", "client-id"]
}
```

## Commitments

### Commitment Structure

The `ReaCommitment` struct represents promises and obligations between agents:

```rust
pub struct ReaCommitment {
    pub id: Option<ActionHash>,                // Unique identifier
    pub rea_action: Option<String>,            // Type of economic action
    pub note: Option<String>,                  // Additional notes
    pub input_of: Option<ActionHash>,          // Process consuming the commitment
    pub output_of: Option<ActionHash>,         // Process producing from the commitment
    pub provider: Option<ActionHash>,          // Agent making the commitment
    pub receiver: Option<ActionHash>,          // Agent receiving the commitment
    pub resource_inventoried_as: Option<ActionHash>, // Specific resource
    pub resource_classified_as: Option<Vec<String>>, // Resource classification
    pub resource_conforms_to: Option<ActionHash>, // Resource specification
    pub resource_quantity: Option<QuantityValue>, // Committed quantity
    pub effort_quantity: Option<QuantityValue>, // Effort required
    pub has_beginning: Option<Timestamp>,      // Commitment start time
    pub has_end: Option<Timestamp>,            // Commitment end time
    pub has_point_in_time: Option<Timestamp>,  // Specific time point
    pub due: Option<Timestamp>,               // Due date/time
    pub at_location: Option<String>,           // Location
    pub agreed_in: Option<String>,             // External reference
    pub clause_of: Option<ActionHash>,         // Parent agreement
    pub planned_within: Option<ActionHash>,    // Containing plan
    pub independent_demand_of: Option<ActionHash>, // Demand being met
    pub finished: Option<bool>,                // Completion status
    pub in_scope_of: Option<Vec<ActionHash>>,  // Scope agents
    pub stage: Option<ActionHash>,            // Process stage
    pub satisfies: Option<ActionHash>,         // Intent being satisfied
}
```

### Field Explanations

| Field | Type | Description | Use Case |
|-------|------|-------------|----------|
| `rea_action` | `Option<String>` | Type of economic action | "transfer", "produce", "consume" |
| `provider` | `Option<ActionHash>` | Agent making the promise | Supplier, service provider |
| `receiver` | `Option<ActionHash>` | Agent receiving the promise | Customer, beneficiary |
| `resource_quantity` | `Option<QuantityValue>` | Amount being promised | "1000 units", "50 kg" |
| `due` | `Option<Timestamp>` | Deadline for fulfillment | "2024-03-15T17:00:00Z" |
| `clause_of` | `Option<ActionHash>` | Parent agreement | Legal contract reference |
| `satisfies` | `Option<ActionHash>` | Intent being satisfied | Need fulfillment |

## Commitment Types

### Transfer Commitments

#### Product Delivery Commitments

```graphql
mutation CreateDeliveryCommitment {
  createCommitment(commitment: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "customer-id"
    resourceClassifiedAs: ["electronics", "smartphones"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    due: "2024-03-15T17:00:00Z"
    agreedIn: "PURCHASE-ORDER-2024-001"
    clauseOf: "supply-agreement-id"
    note: "Q1 smartphone delivery commitment"
  }) {
    commitment {
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
      due
      agreedIn
    }
  }
}
```

#### Service Delivery Commitments

```graphql
mutation CreateServiceCommitment {
  createCommitment(commitment: {
    action: "service"
    provider: "consultant-id"
    receiver: "client-id"
    resourceClassifiedAs: ["consulting", "digital-transformation"]
    effortQuantity: {
      hasNumericalValue: 40
      hasUnit: "hours"
    }
    due: "2024-04-30T17:00:00Z"
    clauseOf: "consulting-agreement-id"
    note: "Digital transformation consulting project"
  }) {
    commitment {
      id
      action
      effortQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      due
    }
  }
}
```

### Production Commitments

#### Manufacturing Commitments

```graphql
mutation CreateProductionCommitment {
  createCommitment(commitment: {
    action: "produce"
    provider: "manufacturer-id"
    receiver: "client-id"
    outputOf: "manufacturing-process-id"
    resourceClassifiedAs: ["electronics", "custom-components"]
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "units"
    }
    due: "2024-03-20T17:00:00Z"
    agreedIn: "MANUFACTURING-ORDER-2024-001"
    note: "Custom component manufacturing commitment"
  }) {
    commitment {
      id
      action
      outputOf {
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
}
```

#### Agricultural Production Commitments

```graphql
mutation CreateHarvestCommitment {
  createCommitment(commitment: {
    action: "raise"
    provider: "farm-id"
    receiver: "cooperative-id"
    resourceClassifiedAs: ["agricultural", "organic", "vegetables"]
    resourceQuantity: {
      hasNumericalValue: 2000
      hasUnit: "kg"
    }
    due: "2024-08-15T00:00:00Z"
    agreedIn: "FARM-CONTRACT-2024-001"
    clauseOf: "agricultural-supply-agreement"
    note: "Organic vegetable harvest commitment for summer season"
  }) {
    commitment {
      id
      action
      resourceQuantity {
        hasNumericalValue
        hasUnit {
          label
        }
      }
      due
    }
  }
}
```

### Resource Usage Commitments

#### Equipment Usage Commitments

```graphql
mutation CreateEquipmentCommitment {
  createCommitment(commitment: {
    action: "use"
    provider: "equipment-owner-id"
    receiver: "renter-id"
    resourceInventoriedAs: "heavy-machinery-id"
    hasBeginning: "2024-03-15T08:00:00Z"
    hasEnd: "2024-03-15T18:00:00Z"
    agreedIn: "RENTAL-AGREEMENT-2024-001"
    note: "Heavy machinery rental for construction project"
  }) {
    commitment {
      id
      action
      resourceInventoriedAs {
        name
      }
      hasBeginning
      hasEnd
    }
  }
}
```

## Commitment Lifecycle

### Creating Commitments

#### Standalone Commitments

```graphql
mutation CreateStandaloneCommitment {
  createCommitment(commitment: {
    action: "transfer"
    provider: "donor-id"
    receiver: "charity-id"
    resourceClassifiedAs: ["food", "donations", "emergency-relief"]
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "kg"
    }
    due: "2024-03-20T12:00:00Z"
    note: "Emergency food donation for disaster relief"
  }) {
    commitment {
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
    }
  }
}
```

#### Agreement-Based Commitments

```graphql
mutation CreateAgreementCommitment {
  createCommitment(commitment: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "retailer-id"
    resourceClassifiedAs: ["retail", "consumer-goods"]
    resourceQuantity: {
      hasNumericalValue: 10000
      hasUnit: "units"
    }
    due: "2024-03-25T17:00:00Z"
    clauseOf: "distribution-agreement-id"
    agreedIn: "DISTRIBUTION-CONTRACT-2024"
    note: "Monthly product delivery under distribution agreement"
  }) {
    commitment {
      id
      clauseOf {
        id
        name
      }
      agreedIn
    }
  }
}
```

### Commitment Fulfillment

#### Fulfilling Commitments with Economic Events

```graphql
mutation FulfillCommitment {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "provider-id"
    receiver: "receiver-id"
    resourceInventoriedAs: "product-resource-id"
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T14:30:00Z"
    fulfills: ["commitment-action-hash"]
    note: "Fulfilling commitment CMT-2024-001"
  }) {
    economicEvent {
      id
      fulfills {
        id
        action
        due
      }
    }
  }
}
```

#### Partial Fulfillment

```graphql
mutation PartialFulfillment {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "provider-id"
    receiver: "receiver-id"
    resourceInventoriedAs: "resource-id"
    resourceQuantity: {
      hasNumericalValue: 500  # Partial fulfillment
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T14:30:00Z"
    fulfills: ["commitment-action-hash"]
    note: "Partial fulfillment - 500 of 1000 units delivered"
  }) {
    economicEvent {
      id
      fulfills {
        id
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
```

### Commitment Management

#### Updating Commitments

```graphql
mutation UpdateCommitment {
  updateCommitment(
    revisionId: "commitment-action-hash"
    commitment: {
      due: "2024-03-20T17:00:00Z"  # Extended due date
      note: "Due date extended due to production delay"
    }
  ) {
    commitment {
      id
      due
      note
    }
  }
}
```

#### Canceling Commitments

```graphql
mutation CancelCommitment {
  updateCommitment(
    revisionId: "commitment-action-hash"
    commitment: {
      finished: true
      note: "Commitment cancelled due to circumstances beyond control"
    }
  ) {
    commitment {
      id
      finished
      note
    }
  }
}
```

## Agreement Management

### Creating Agreements

#### Formal Contract Creation

```graphql
mutation CreateAgreement {
  createAgreement(agreement: {
    name: "Supply Agreement 2024-Q1"
    note: "Supply agreement for Q1 2024 between manufacturer and distributor"
    agreedIn: "CONTRACT-SUPPLY-2024-Q1"
    classification: "legal-contract"
    inScopeOf: ["manufacturer-id", "distributor-id"]
  }) {
    agreement {
      id
      name
      agreedIn
      classification
      inScopeOf {
        id
        name
      }
    }
  }
}
```

#### Informal Agreement Creation

```graphql
mutation CreateInformalAgreement {
  createAgreement(agreement: {
    name: "Community Tool Sharing Agreement"
    note: "Informal agreement for sharing tools among community members"
    classification: "informal-agreement"
    inScopeOf: ["member-1-id", "member-2-id", "member-3-id", "member-4-id"]
  }) {
    agreement {
      id
      name
      classification
    }
  }
}
```

### Agreement Clauses

#### Adding Commitments to Agreements

```graphql
mutation AddCommitmentToAgreement {
  createCommitment(commitment: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "buyer-id"
    resourceClassifiedAs: ["electronics", "components"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    due: "2024-03-15T17:00:00Z"
    clauseOf: "supply-agreement-id"
    agreedIn: "SUPPLY-CONTRACT-2024"
  }) {
    commitment {
      id
      clauseOf {
        id
        name
      }
      agreedIn
    }
  }
}
```

#### Querying Agreement Commitments

```graphql
query AgreementWithCommitments {
  reaAgreement(id: "agreement-id") {
    id
    name
    classification
    agreedIn
    commitments {
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
          due
          finished
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
    inScopeOf {
      id
      name
      agentType
    }
  }
}
```

## Commitment Analytics

### Commitment Tracking

#### Overdue Commitments

```graphql
query OverdueCommitments {
  reaCommitments(
    filters: {
      dueBefore: "2024-03-15T00:00:00Z"
      finished: false
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
        due
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
        agreedIn
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
    totalCount
  }
}
```

#### Commitment Performance

```graphql
query CommitmentPerformance($providerId: ID!) {
  reaCommitments(
    filters: {
      provider: $providerId
      dateRange: {
        start: "2024-01-01T00:00:00Z"
        end: "2024-03-31T23:59:59Z"
      }
    }
  ) {
    edges {
      node {
        id
        action
        due
        finished
        fulfilledBy {
          edges {
            node {
              id
              hasPointInTime
            }
          }
        }
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
      }
    }
    totalCount
  }
}
```

#### Commitment Fulfillment Rate

```graphql
query CommitmentFulfillmentMetrics {
  reaCommitments {
    totalCount
    edges {
      node {
        id
        finished
        fulfilledBy {
          totalCount
        }
      }
    }
  }
}
```

## Advanced Commitment Patterns

### Multi-Agent Commitments

#### Chain Commitments

```graphql
mutation CreateChainCommitment {
  createCommitment(commitment: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "manufacturer-id"
    resourceClassifiedAs: ["materials", "electronics"]
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "kg"
    }
    due: "2024-03-20T12:00:00Z"
    satisfies: "production-demand-id"
    note: "Raw materials for manufacturing process"
  }) {
    commitment {
      id
      satisfies {
        id
        name
      }
    }
  }
}
```

#### Conditional Commitments

```graphql
mutation CreateConditionalCommitment {
  createCommitment(commitment: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "customer-id"
    resourceClassifiedAs: ["products", "premium"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    due: "2024-04-01T17:00:00Z"
    agreedIn: "CONDITIONAL-AGREEMENT-001"
    note: "Conditional delivery subject to quality approval"
  }) {
    commitment {
      id
      agreedIn
      note
    }
  }
}
```

### Commitment Bundles

#### Service Package Commitments

```json
{
  "name": "Complete IT Support Package",
  "commitments": [
    {
      "action": "service",
      "resource_classified_as": ["it-support", "maintenance"],
      "effort_quantity": {"has_numerical_value": 40, "has_unit": "hours"},
      "due": "2024-03-31T17:00:00Z"
    },
    {
      "action": "use",
      "resource_classified_as": ["equipment", "server"],
      "has_beginning": "2024-01-01T00:00:00Z",
      "has_end": "2024-12-31T23:59:59Z"
    },
    {
      "action": "service",
      "resource_classified_as": ["training", "it-support"],
      "effort_quantity": {"has_numerical_value": 8, "has_unit": "hours"},
      "due": "2024-02-15T17:00:00Z"
    }
  ]
}
```

## Intent Satisfaction

### Creating Intentions

```graphql
mutation CreateIntention {
  createIntent(intention: {
    action: "transfer"
    receiver: "organization-id"
    resourceClassifiedAs: ["supplies", "office"]
    resourceQuantity: {
      hasNumericalValue: 100
      hasUnit: "units"
    }
    note: "Need office supplies for Q2 operations"
  }) {
    intent {
      id
      action
      receiver {
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
}
```

### Satisfying Intentions with Commitments

```graphql
mutation SatisfyIntent {
  createCommitment(commitment: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "organization-id"
    resourceClassifiedAs: ["supplies", "office"]
    resourceQuantity: {
      hasNumericalValue: 100
      hasUnit: "units"
    }
    due: "2024-04-15T17:00:00Z"
    satisfies: "intent-action-hash"
    note: "Office supplies to meet Q2 operational needs"
  }) {
    commitment {
      id
      satisfies {
        id
        action
        resourceQuantity {
          hasNumericalValue
        }
      }
    }
  }
}
```

## Smart Agreement Integration

### Blockchain-Based Agreements

```typescript
// Integration with smart contracts for automated enforcement
class SmartAgreementIntegration {
  async createSmartCommitment(commitment: CommitmentData) {
    // Create commitment in hREA
    const hreaCommitment = await this.holochainClient.createCommitment(commitment);

    // Create corresponding smart contract on blockchain
    const smartContract = await this.blockchainClient.createContract({
      parties: [commitment.provider, commitment.receiver],
      obligations: [{
        type: commitment.action,
        quantity: commitment.resourceQuantity,
        dueDate: commitment.due,
        enforcement: "automatic"
      }],
      agreementId: hreaCommitment.id
    });

    return {
      hreaCommitment,
      smartContract
    };
  }

  async enforceCommitment(commitmentId: string) {
    const commitment = await this.holochainClient.queryCommitment(commitmentId);
    const smartContract = await this.blockchainClient.getContract(commitmentId);

    // Check if commitment is fulfilled
    if (!commitment.finished && new Date() > new Date(commitment.due)) {
      // Trigger automatic enforcement
      await this.blockchainClient.enforceContract(smartContract.id);
    }
  }
}
```

## Performance Optimization

### Commitment Indexing Strategy

```rust
// Optimized commitment structure for efficient querying
#[hdk_entry_helper]
#[serde(rename_all = "camelCase")]
pub struct ReaCommitment {
    // Primary indices
    pub id: Option<ActionHash>,
    pub provider: Option<ActionHash>,    // Provider-based queries
    pub receiver: Option<ActionHash>,    // Receiver-based queries
    pub clause_of: Option<ActionHash>,    // Agreement-based queries

    // Status indices
    pub finished: Option<bool>,           // Completion status queries
    pub due: Option<Timestamp>,           // Due date queries

    // Time-based indices
    pub has_beginning: Option<Timestamp>, // Active commitment queries
    pub has_end: Option<Timestamp>,       // Time-bound queries

    // Classification indices
    pub resource_classified_as: Option<Vec<String>>, // Resource type queries
    pub rea_action: Option<String>,      // Action type queries

    // Link indices
    pub satisfies: Option<ActionHash>,    // Intent satisfaction queries
    pub input_of: Option<ActionHash>,     // Process input queries
    pub output_of: Option<ActionHash>,    // Process output queries
}
```

### Caching Strategy

```typescript
// Intelligent commitment caching
class CommitmentCache {
  private cache = new Map<string, CommitmentData>();
  private fulfillmentCache = new Map<string, EconomicEvent[]>();
  private ttl = 600000; // 10 minutes

  async getCommitment(id: string): Promise<CommitmentData> {
    const cached = this.cache.get(id);
    if (cached && Date.now() - cached.timestamp < this.ttl) {
      return cached.data;
    }

    const commitment = await this.holochainClient.queryCommitment(id);
    this.cache.set(id, {
      data: commitment,
      timestamp: Date.now()
    });

    return commitment;
  }

  async getFulfillments(commitmentId: string): Promise<EconomicEvent[]> {
    const cached = this.fulfillmentCache.get(commitmentId);
    if (cached && Date.now() - cached[0]?.timestamp < this.ttl) {
      return cached.map(item => item.data);
    }

    const fulfillments = await this.holochainClient.queryFulfillments(commitmentId);
    this.fulfillmentCache.set(commitmentId, fulfillments.map(event => ({
      data: event,
      timestamp: Date.now()
    })));

    return fulfillments;
  }

  invalidateCommitment(id: string) {
    this.cache.delete(id);
    this.fulfillmentCache.delete(id);
  }
}
```

## Best Practices

### Commitment Design

1. **Clear Terms**: Define clear, measurable commitment terms
2. **Realistic Deadlines**: Set achievable due dates
3. **Resource Specification**: Clearly specify resources and quantities
4. **Legal References**: Link commitments to appropriate legal agreements
5. **Performance Tracking**: Monitor commitment fulfillment rates

### Agreement Management

1. **Formal Documentation**: Use proper legal documentation for formal agreements
2. **Clear Scope**: Define agreement scope and limitations
3. **Dispute Resolution**: Include dispute resolution mechanisms
4. **Version Control**: Maintain version history for agreements
5. **Compliance**: Ensure compliance with relevant regulations

### Risk Management

1. **Commitment Risk Assessment**: Evaluate risk for significant commitments
2. **Insurance**: Consider insurance for high-value commitments
3. **Contingency Planning**: Plan for commitment fulfillment failures
4. **Monitoring**: Implement continuous commitment monitoring
5. **Escalation**: Define escalation procedures for missed commitments

## Use Case Examples

### Supply Chain Management

```json
{
  "agreement": {
    "name": "Supply Chain Partnership Agreement 2024",
    "classification": "strategic-partnership",
    "in_scope_of": ["manufacturer-id", "distributor-id", "retailer-id"]
  },
  "commitments": [
    {
      "action": "transfer",
      "provider": "manufacturer-id",
      "receiver": "distributor-id",
      "resource_classified_as": ["electronics", "smartphones"],
      "resource_quantity": {"has_numerical_value": 10000, "has_unit": "units"},
      "due": "2024-03-15T17:00:00Z",
      "clause_of": "partnership-agreement-id"
    }
  ]
}
```

### Service Level Agreements

```json
{
  "agreement": {
    "name": "IT Services SLA 2024",
    "classification": "service-level-agreement",
    "agreed_in": "SLA-IT-2024-001"
  },
  "commitments": [
    {
      "action": "service",
      "provider": "it-service-provider-id",
      "receiver": "client-id",
      "resource_classified_as": ["it-support", "technical"],
      "effort_quantity": {"has_numerical_value": 160, "has_unit": "hours"},
      "due": "2024-03-31T17:00:00Z"
    }
  ]
}
```

### Agricultural Contracts

```json
{
  "agreement": {
    "name": "Farm to Table Agreement 2024",
    "classification": "farmers-market-agreement",
    "in_scope_of": ["farm-id", "restaurant-id", "cooperative-id"]
  },
  "commitments": [
    {
      "action": "raise",
      "provider": "farm-id",
      "receiver": "restaurant-id",
      "resource_classified_as": ["agricultural", "organic", "vegetables"],
      "resource_quantity": {"has_numerical_value": 500, "has_unit": "kg"},
      "due": "2024-06-30T00:00:00Z"
    }
  ]
}
```

This agreements and commitments system provides a robust foundation for managing economic promises, contracts, and obligations across diverse industries and use cases, enabling trust-based economic coordination.