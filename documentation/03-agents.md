# Agents in hREA

## Overview

In hREA, **Agents** represent the economic actors who participate in resource flows, processes, and agreements. Agents are fundamental to the REA (Resources, Events, Agents) accounting model and serve as the primary subjects and objects in economic activities.

## Agent Definition

### Core Structure

The `ReaAgent` struct represents economic agents in the system:

```rust
pub struct ReaAgent {
    pub id: Option<ActionHash>,           // Unique identifier
    pub name: String,                     // Human-readable name
    pub agent_type: String,               // Type classification
    pub image: Option<String>,            // Optional profile image
    pub classified_as: Option<Vec<String>>, // Classification tags
    pub note: Option<String>,             // Additional notes
}
```

### Fields Explained

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `id` | `Option<ActionHash>` | Unique Holochain action hash identifier | `uhCAk...` |
| `name` | `String` | Human-readable display name | "Acme Corporation" |
| `agent_type` | `String` | Primary type classification | "Organization" |
| `image` | `Option<String>` | URL or reference to profile image | "https://example.com/logo.png" |
| `classified_as` | `Option<Vec<String>>` | Multiple classification tags | `["manufacturer", "certified"]` |
| `note` | `Option<String>`` | Free-form descriptive text | "Leading sustainable manufacturer" |

## Agent Types

### Standard Agent Classifications

#### 1. Individual Persons
```json
{
  "name": "Alice Johnson",
  "agent_type": "Person",
  "classified_as": ["farmer", "certified-organic"],
  "note": "Small-scale organic vegetable producer"
}
```

#### 2. Organizations
```json
{
  "name": "Green Foods Co-op",
  "agent_type": "Organization",
  "classified_as": ["cooperative", "food-distribution"],
  "image": "https://example.com/green-foods-logo.png"
}
```

#### 3. Groups and Teams
```json
{
  "name": "Production Team Alpha",
  "agent_type": "Group",
  "classified_as": ["production", "quality-assurance"],
  "note": "Responsible for Q3 production targets"
}
```

#### 4. Network Entities
```json
{
  "name": "Regional Supply Network",
  "agent_type": "Network",
  "classified_as": ["supply-chain", "regional"],
  "note": "Coordinates local producer distribution"
}
```

### Custom Agent Classifications

hREA supports flexible agent classification systems:

#### Role-Based Classifications
- `["producer", "consumer", "intermediary"]`
- `["supplier", "manufacturer", "distributor", "retailer"]`
- `["worker", "manager", "owner", "investor"]`

#### Economic Model Classifications
- `["for-profit", "non-profit", "cooperative", "mutual"]`
- `["gift-economy", "commons", "market"]`

#### Capability Classifications
- `["certified-organic", "fair-trade", "b-corp"]`
- `["skill-holder", "knowledge-provider", "service-provider"]`

## Agent Relationships

### Agent-to-Agent Links

hREA supports several types of relationships between agents:

#### Membership Relationships
```graphql
# Link group members to organizations
mutation {
  createLink(
    baseAgent: "team-alpha-id"
    targetAgent: "acme-corp-id"
    linkType: "member_of"
  )
}
```

#### Role Relationships
```graphql
# Define roles within organizations
mutation {
  createLink(
    baseAgent: "alice-id"
    targetAgent: "acme-corp-id"
    linkType: "employee"
  )
}
```

#### Collaboration Relationships
```graphql
# Create collaboration networks
mutation {
  createLink(
    baseAgent: "producer-union-id"
    targetAgent: "distributor-coop-id"
    linkType: "partner"
  )
}
```

## Agent Lifecycle

### Creating Agents

```graphql
mutation CreateAgent {
  createReaAgent(agent: {
    name: "New Sustainable Farm"
    agentType: "Organization"
    classifiedAs: ["organic-farm", "local-producer"]
    note: "Family farm practicing sustainable agriculture"
    image: "https://example.com/farm-logo.png"
  }) {
    reaAgent {
      id
      name
      agentType
      classifiedAs
      note
      image
    }
  }
}
```

### Updating Agents

```graphql
mutation UpdateAgent {
  updateReaAgent(
    revisionId: "agent-action-hash"
    agent: {
      name: "Updated Farm Name"
      note: "Updated description with new practices"
      classifiedAs: ["organic-farm", "local-producer", "biodynamic"]
    }
  ) {
    reaAgent {
      id
      name
      note
      classifiedAs
    }
  }
}
```

### Querying Agents

```graphql
query GetAgent {
  reaAgent(id: "agent-id") {
    id
    name
    agentType
    image
    classifiedAs
    note
  }
}
```

```graphql
query SearchAgents {
  reaAgents(
    filters: {
      agentType: "Organization"
      classifiedAs: ["organic-farm"]
    }
  ) {
    edges {
      node {
        id
        name
        agentType
        classifiedAs
      }
    }
  }
}
```

## Agent Networks

### All Agents Index

All agents are automatically indexed in the global `all_agents` collection:

```graphql
query {
  reaAgents {
    edges {
      node {
        id
        name
        agentType
        classifiedAs
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

### Agent Graph Traversal

Follow agent relationships through link queries:

```graphql
query AgentNetwork {
  reaAgent(id: "central-agent-id") {
    id
    name
    agentType
    agentRelationships {
      edges {
        node {
          relatedTo {
            id
            name
            agentType
          }
          relationship
        }
      }
    }
  }
}
```

## Trust and Reputation

### Agent Verification

While hREA doesn't implement a built-in reputation system, the agent structure supports external verification:

```json
{
  "name": "Verified Organic Farm",
  "agent_type": "Organization",
  "classified_as": [
    "certified-organic",
    "usda-verified",
    "third-party-audited"
  ],
  "note": "Certified by Organic Trade Association, certification #ORG-2024-001"
}
```

### Trust Networks

Agent relationships can be used to build trust networks:

```graphql
# Build web of trust connections
mutation {
  createLink(
    baseAgent: "verified-agent-id"
    targetAgent: "new-agent-id"
    linkType: "trusts"
  )
}
```

## Integration Patterns

### External Identity Systems

hREA agents can reference external identity systems:

```json
{
  "name": "John Doe",
  "agent_type": "Person",
  "classified_as": ["did-verified"],
  "note": "DID: did:holo:john-doe-2024"
}
```

### Organizational Hierarchies

Model complex organizational structures:

```graphql
# Multi-level organization
query {
  reaAgent(id: "parent-org-id") {
    id
    name
    agentType
    childOrganizations {
      edges {
        node {
          relatedTo {
            id
            name
            agentType
          }
          relationship
        }
      }
    }
  }
}
```

### Multi-Agent Processes

Agents participate in economic processes and events:

```graphql
query AgentEconomicActivity {
  reaAgent(id: "agent-id") {
    id
    name
    economicEventsAsProvider {
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
        }
      }
    }
    commitmentsAsProvider {
      edges {
        node {
          id
          action
          due
          resourceClassifiedAs
        }
      }
    }
  }
}
```

## Best Practices

### Agent Naming Conventions

1. **Descriptive Names**: Use clear, recognizable names
2. **Consistent Types**: Apply consistent agent_type classifications
3. **Rich Metadata**: Provide useful classification and note information
4. **Visual Identity**: Include appropriate images for organizational agents

### Classification Systems

1. **Standardized Vocabularies**: Use existing classification systems where possible
2. **Hierarchical Tags**: Structure classifications from general to specific
3. **Multi-Dimensional**: Use multiple classification dimensions
4. **Extensible Design**: Allow for future classification additions

### Relationship Management

1. **Clear Relationship Types**: Use consistent relationship terminology
2. **Avoid Circular References**: Prevent infinite relationship loops
3. **Document Relationships**: Include notes explaining complex connections
4. **Regular Audits**: Periodically review and clean up agent relationships

## Use Case Examples

### Supply Chain Traceability

```json
{
  "name": "Mountain View Dairy",
  "agent_type": "Organization",
  "classified_as": ["dairy-producer", "organic-certified", "local"],
  "note": "Family-owned dairy farm in Mountain Valley region"
}
```

### Collaborative Projects

```json
{
  "name": "Community Garden Team",
  "agent_type": "Group",
  "classified_as": ["volunteer", "urban-agriculture", "community-led"],
  "note": "Manages the downtown community garden project"
}
```

### Service Providers

```json
{
  "name": "Green Consulting LLC",
  "agent_type": "Organization",
  "classified_as": ["consulting", "sustainability", "b-corp"],
  "image": "https://example.com/consulting-logo.png"
}
```

## Data Validation

### Agent Creation Validation

The system enforces basic validation rules:

1. **Required Fields**: `name` and `agent_type` are mandatory
2. **Valid References**: Link targets must be valid agent records
3. **Update Constraints**: Agent updates maintain data integrity
4. **Link Immortality**: Agent update links cannot be deleted

### Integrity Constraints

```rust
// Example validation rules (simplified)
pub fn validate_create_rea_agent(
    _action: EntryCreationAction,
    rea_agent: ReaAgent,
) -> ExternResult<ValidateCallbackResult> {
    // Name cannot be empty
    if rea_agent.name.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            "Agent name cannot be empty".to_string(),
        ));
    }

    // Agent type is required
    if rea_agent.agent_type.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            "Agent type is required".to_string(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}
```

## Performance Considerations

### Query Optimization

1. **Indexing**: Agents are automatically indexed by type and classifications
2. **Pagination**: Use cursor-based pagination for large agent collections
3. **Filtering**: Apply filters at the database level when possible
4. **Caching**: Cache frequently accessed agent information

### Relationship Query Patterns

1. **Depth Limiting**: Limit relationship traversal depth
2. **Circular Detection**: Implement cycle detection in relationship graphs
3. **Lazy Loading**: Load relationships on-demand
4. **Batch Operations**: Use batch queries for multiple relationship lookups

## Future Enhancements

### Planned Features

1. **Reputation Systems**: Built-in reputation and scoring mechanisms
2. **Verification Services**: Integrated identity verification
3. **Role-Based Permissions**: Fine-grained access control
4. **Agent Migration**: Tools for agent data portability

### Extension Points

1. **Custom Validation**: Domain-specific validation rules
2. **External Integrations**: CRM and ERP system connections
3. **Notification Systems**: Agent-based event notifications
4. **Analytics**: Agent activity and network analysis tools