# Economic Events in hREA

## Overview

**Economic Events** are the foundational elements in hREA for tracking all observable economic activities. They represent the actual movements, transformations, and exchanges of resources between agents. Economic events form the core of the REA accounting model's "Events" component and provide an immutable ledger of all economic activity.

## Economic Event Structure

### Core Definition

The `ReaEconomicEvent` struct captures all aspects of economic activities:

```rust
pub struct ReaEconomicEvent {
    pub id: Option<ActionHash>,                    // Unique identifier
    pub rea_action: String,                        // Type of economic action
    pub note: Option<String>,                      // Additional notes
    pub input_of: Option<ActionHash>,              // Process consuming the resource
    pub output_of: Option<ActionHash>,             // Process producing the resource
    pub provider: Option<ActionHash>,              // Agent providing the resource
    pub receiver: Option<ActionHash>,              // Agent receiving the resource
    pub resource_inventoried_as: Option<ActionHash>, // Resource being affected
    pub to_resource_inventoried_as: Option<ActionHash>, // Target resource for transfers
    pub resource_classified_as: Option<Vec<String>>,    // Resource classification
    pub resource_conforms_to: Option<ActionHash>,  // Resource specification
    pub resource_quantity: Option<QuantityValue>,  // Amount/quantity involved
    pub has_beginning: Option<Timestamp>,          // Event start time
    pub has_end: Option<Timestamp>,                // Event end time
    pub has_point_in_time: Option<Timestamp>,      // Specific moment timestamp
    pub at_location: Option<String>,               // Physical location
    pub agreed_in: Option<String>,                 // Agreement reference
    pub realization_of: Option<ActionHash>,        // Agreement being fulfilled
    pub in_scope_of: Option<Vec<ActionHash>>,      // Scope of economic activity
    pub triggered_by: Option<ActionHash>,          // Triggering event
    pub fulfills: Option<Vec<ActionHash>>,         // Commitments fulfilled
    pub satisfies: Option<Vec<ActionHash>>,        // Intentions satisfied
    pub corrects: Option<ActionHash>,              // Event being corrected
}
```

### Field Explanations

#### Essential Fields
| Field | Type | Description | Required |
|-------|------|-------------|----------|
| `rea_action` | `String` | Type of economic action (transfer, produce, consume, etc.) | ✅ |
| `provider` | `Option<ActionHash>` | Agent providing the resource | Recommended |
| `receiver` | `Option<ActionHash>` | Agent receiving the resource | Recommended |
| `resource_quantity` | `Option<QuantityValue>` | Amount of resource involved | Recommended |

#### Temporal Fields
| Field | Type | Description | Use Case |
|-------|------|-------------|----------|
| `has_point_in_time` | `Option<Timestamp>` | Exact moment of event | Instantaneous events |
| `has_beginning` | `Option<Timestamp>` | Event start time | Duration-based events |
| `has_end` | `Option<Timestamp>`` | Event end time | Duration-based events |

#### Process Context
| Field | Type | Description | Relationship |
|-------|------|-------------|-----------|
| `input_of` | `Option<ActionHash>` | Process consuming this event | Event → Process |
| `output_of` | `Option<ActionHash>` | Process producing this event | Process → Event |

#### Agreement Context
| Field | Type | Description | Purpose |
|-------|------|-------------|---------|
| `realization_of` | `Option<ActionHash>` | Agreement being implemented | Contract fulfillment |
| `agreed_in` | `Option<String>` | External agreement reference | Legal documentation |
| `fulfills` | `Option<Vec<ActionHash>>` | Commitments being satisfied | Promise completion |
| `satisfies` | `Option<Vec<ActionHash>>` | Intentions being satisfied | Need fulfillment |

## Economic Actions

### Core Action Types

#### Resource Movement Actions

**Transfer (`transfer`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "provider-agent-id"
    receiver: "receiver-agent-id"
    resourceInventoriedAs: "source-resource-id"
    toResourceInventoriedAs: "destination-resource-id"
    resourceQuantity: {
      hasNumericalValue: 100
      hasUnit: "kg"
    }
    hasPointInTime: "2024-03-15T10:00:00Z"
    atLocation: "Warehouse A, Loading Dock 3"
  }) {
    economicEvent {
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
```

**Raise (`raise`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "raise"
    provider: "farm-agent-id"
    resourceInventoriedAs: "new-harvest-resource-id"
    resourceClassifiedAs: ["vegetables", "organic-tomatoes"]
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "kg"
    }
    hasPointInTime: "2024-08-20T06:00:00Z"
    atLocation: "Greenhouse #3"
    note: "Summer tomato harvest"
  }) {
    economicEvent {
      id
      action
      resourceClassifiedAs
    }
  }
}
```

**Lower (`lower`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "lower"
    provider: "manufacturer-id"
    resourceInventoriedAs: "raw-materials-resource-id"
    resourceQuantity: {
      hasNumericalValue: 50
      hasUnit: "kg"
    }
    hasPointInTime: "2024-03-15T14:30:00Z"
    note: "Materials consumed in production run #1234"
  }) {
    economicEvent {
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
```

#### Process-Related Actions

**Produce (`produce`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "produce"
    outputOf: "manufacturing-process-id"
    provider: "factory-agent-id"
    resourceInventoriedAs: "finished-goods-id"
    resourceClassifiedAs: ["electronics", "smartphone"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T16:00:00Z"
    atLocation: "Production Line A"
  }) {
    economicEvent {
      id
      action
      outputOf {
        name
      }
    }
  }
}
```

**Consume (`consume`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "consume"
    inputOf: "assembly-process-id"
    provider: "component-supplier-id"
    resourceInventoriedAs: "components-inventory-id"
    resourceClassifiedAs: ["electronics", "circuit-boards"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T11:00:00Z"
  }) {
    economicEvent {
      id
      action
      inputOf {
        name
      }
    }
  }
}
```

#### Service Actions

**Service (`service`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "service"
    provider: "consultant-id"
    receiver: "client-id"
    resourceClassifiedAs: ["consulting", "sustainability-advisory"]
    hasPointInTime: "2024-03-15T13:00:00Z"
    hasEnd: "2024-03-15T17:00:00Z"
    agreedIn: "CONSULT-2024-001"
    realizationOf: "consulting-agreement-id"
    note: "Sustainability assessment and recommendations"
  }) {
    economicEvent {
      id
      action
      hasBeginning
      hasEnd
    }
  }
}
```

#### Use Action

**Use (`use`)**
```graphql
mutation {
  createEconomicEvent(event: {
    action: "use"
    provider: "equipment-owner-id"
    resourceInventoriedAs: "machinery-id"
    resourceClassifiedAs: ["equipment", "industrial-mixer"]
    hasPointInTime: "2024-03-15T09:00:00Z"
    hasEnd: "2024-03-15T12:00:00Z"
    note: "Equipment used for batch #567 production"
  }) {
    economicEvent {
      id
      action
      resourceClassifiedAs
    }
  }
}
```

## Quantity and Measurement

### QuantityValue Structure

```rust
pub struct QuantityValue {
    pub has_numerical_value: f64,
    pub has_unit: UnitId,
}
```

### Common Units and Measurements

#### Weight Units
- `kg` - Kilograms
- `g` - Grams
- `t` - Metric tons
- `lb` - Pounds

#### Volume Units
- `L` - Liters
- `mL` - Milliliters
- `m3` - Cubic meters
- `gal` - Gallons

#### Count Units
- `units` - Individual items
- `pairs` - Paired items
- `sets` - Complete sets
- `dozens` - Groups of 12

#### Time Units
- `hours` - Hours
- `days` - Days
- `months` - Months
- `years` - Years

#### Area Units
- `m2` - Square meters
- `ha` - Hectares
- `acres` - Acres
- `km2` - Square kilometers

## Resource Classification

### Classification Hierarchies

#### Product Classifications
```json
{
  "resource_classified_as": [
    "food",
    "vegetables",
    "organic-produce",
    "leafy-greens"
  ]
}
```

#### Service Classifications
```json
{
  "resource_classified_as": [
    "professional-services",
    "consulting",
    "sustainability-advisory"
  ]
}
```

#### Material Classifications
```json
{
  "resource_classified_as": [
    "materials",
    "metals",
    "aluminum",
    "recycled-content"
  ]
}
```

## Process Integration

### Input Events

Events that provide resources to processes:

```graphql
# Raw material input to manufacturing
mutation {
  createEconomicEvent(event: {
    action: "consume"
    inputOf: "battery-manufacturing-id"
    provider: "materials-supplier-id"
    resourceInventoriedAs: "lithium-inventory-id"
    resourceClassifiedAs: ["materials", "lithium", "battery-grade"]
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "kg"
    }
    hasPointInTime: "2024-03-15T08:00:00Z"
  }) {
    economicEvent {
      id
      inputOf {
        id
        name
      }
    }
  }
}
```

### Output Events

Events that result from processes:

```graphql
# Finished product from manufacturing
mutation {
  createEconomicEvent(event: {
    action: "produce"
    outputOf: "battery-manufacturing-id"
    provider: "manufacturer-id"
    resourceInventoriedAs: "battery-inventory-id"
    resourceClassifiedAs: ["electronics", "batteries", "lithium-ion"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T18:00:00Z"
  }) {
    economicEvent {
      id
      outputOf {
        id
        name
      }
    }
  }
}
```

## Agreement Fulfillment

### Commitment Fulfillment

```graphql
# Fulfill a delivery commitment
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "supplier-id"
    receiver: "customer-id"
    resourceInventoriedAs: "product-inventory-id"
    resourceQuantity: {
      hasNumericalValue: 100
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T10:00:00Z"
    fulfills: ["delivery-commitment-action-hash"]
    realizationOf: "purchase-order-agreement-id"
    agreedIn: "PO-2024-001234"
  }) {
    economicEvent {
      id
      fulfills {
        id
      }
      realizationOf {
        id
        name
      }
    }
  }
}
```

### Intention Satisfaction

```graphql
# Satisfy a resource need
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "donor-id"
    receiver: "charity-id"
    resourceInventoriedAs: "food-supplies-id"
    resourceClassifiedAs: ["food", "canned-goods"]
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "kg"
    }
    hasPointInTime: "2024-03-15T14:00:00Z"
    satisfies: ["food-need-intention-action-hash"]
    note: "Emergency food donation"
  }) {
    economicEvent {
      id
      satisfies {
        id
      }
    }
  }
}
```

## Location Tracking

### Physical Locations

```graphql
# Event with detailed location
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "warehouse-agent-id"
    receiver: "retail-store-id"
    resourceInventoriedAs: "inventory-id"
    resourceQuantity: {
      hasNumericalValue: 50
      hasUnit: "cases"
    }
    hasPointInTime: "2024-03-15T09:00:00Z"
    atLocation: "Main Warehouse, Aisle 12, Section B, Position 3"
  }) {
    economicEvent {
      id
      atLocation
    }
  }
}
```

### Multi-Location Events

```graphql
# For events spanning multiple locations
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "farm-id"
    receiver: "processing-facility-id"
    resourceInventoriedAs: "harvest-id"
    resourceQuantity: {
      hasNumericalValue: 2000
      hasUnit: "kg"
    }
    hasPointInTime: "2024-03-15T06:00:00Z"
    note: "Transport from Farm Location A (GPS: 40.7128,-74.0060) to Processing Facility B (GPS: 40.7589,-73.9851)"
  }) {
    economicEvent {
      id
      note
    }
  }
}
```

## Event Sequencing and Triggers

### Triggered Events

```graphql
# Event triggered by another event
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "quality-control-id"
    receiver: "shipping-id"
    resourceInventoriedAs: "approved-products-id"
    resourceQuantity: {
      hasNumericalValue: 500
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T16:00:00Z"
    triggeredBy: "quality-inspection-event-action-hash"
    note: "Products approved for shipment after quality inspection"
  }) {
    economicEvent {
      id
      triggeredBy {
        id
        reaAction
      }
    }
  }
}
```

### Event Corrections

```graphql
# Correct a previous event
mutation {
  createEconomicEvent(event: {
    action: "transfer"
    provider: "warehouse-id"
    receiver: "customer-id"
    resourceInventoriedAs: "product-id"
    resourceQuantity: {
      hasNumericalValue: 95  # Corrected from 100
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T11:30:00Z"
    corrects: "original-shipment-event-action-hash"
    note: "Correction: Initial count was incorrect, actual quantity was 95 units"
  }) {
    economicEvent {
      id
      corrects {
        id
      }
    }
  }
}
```

## Querying Economic Events

### Basic Event Queries

```graphql
query GetEconomicEvent {
  reaEconomicEvent(id: "event-id") {
    id
    reaAction
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
```

### Advanced Filtering

```graphql
query FilteredEconomicEvents {
  reaEconomicEvents(
    filters: {
      reaAction: "transfer"
      resourceClassifiedAs: ["organic", "vegetables"]
      dateRange: {
        start: "2024-03-01T00:00:00Z"
        end: "2024-03-31T23:59:59Z"
      }
      provider: "supplier-agent-id"
    }
  ) {
    edges {
      node {
        id
        reaAction
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
        hasPointInTime
        provider {
          name
        }
        receiver {
          name
        }
      }
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}
```

### Process-Related Queries

```graphql
query ProcessEconomicEvents {
  reaProcess(id: "process-id") {
    id
    name
    plannedInputs {
      edges {
        node {
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
    plannedOutputs {
      edges {
        node {
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
}
```

## Agent Activity History

```graphql
query AgentEconomicHistory {
  reaAgent(id: "agent-id") {
    id
    name
    economicEventsAsProvider {
      edges {
        node {
          id
          reaAction
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          hasPointInTime
          receiver {
            name
          }
        }
      }
    }
    economicEventsAsReceiver {
      edges {
        node {
          id
          reaAction
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          hasPointInTime
          provider {
            name
          }
        }
      }
    }
  }
}
```

## Data Integrity and Validation

### Validation Rules

The system enforces comprehensive validation for economic events:

1. **Required Fields**: Essential fields must be provided
2. **Reference Integrity**: All referenced entities must exist
3. **Logical Consistency**: Event data must be logically consistent
4. **Temporal Validity**: Time-based data must be valid

### Example Validation Logic

```rust
pub fn validate_create_rea_economic_event(
    _action: EntryCreationAction,
    rea_economic_event: ReaEconomicEvent,
) -> ExternResult<ValidateCallbackResult> {
    // Validate action type
    if rea_economic_event.rea_action.is_empty() {
        return Ok(ValidateCallbackResult::Invalid(
            "REA action is required".to_string(),
        ));
    }

    // Validate provider exists if specified
    if let Some(action_hash) = rea_economic_event.provider.clone() {
        let record = must_get_valid_record(action_hash)?;
        let _rea_agent: crate::ReaAgent = record
            .entry()
            .to_app_option()
            .map_err(|e| wasm_error!(e))?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Provider must reference a valid agent"
            ))))?;
    }

    // Validate temporal consistency
    if let (Some(beginning), Some(end)) = (
        rea_economic_event.has_beginning,
        rea_economic_event.has_end,
    ) {
        if beginning > end {
            return Ok(ValidateCallbackResult::Invalid(
                "Event beginning cannot be after event end".to_string(),
            ));
        }
    }

    Ok(ValidateCallbackResult::Valid)
}
```

## Performance Considerations

### Query Optimization

1. **Indexing**: Events are indexed by action type, agents, time, and resources
2. **Pagination**: Use cursor-based pagination for large event collections
3. **Temporal Filtering**: Apply time-based filters at the database level
4. **Batch Operations**: Use batch queries for multiple event retrievals

### Data Management

1. **Event Pruning**: Archive old events while preserving audit trails
2. **Compression**: Optimize storage for similar event types
3. **Caching**: Cache frequently accessed event patterns
4. **Precomputation**: Precompute common aggregations and summaries

## Use Case Examples

### Supply Chain Tracking

```json
{
  "rea_action": "transfer",
  "provider": "farm-id",
  "receiver": "distribution-center-id",
  "resource_inventoried_as": "organic-tomatoes-id",
  "resource_classified_as": ["produce", "organic", "tomatoes"],
  "resource_quantity": {
    "has_numerical_value": 500,
    "has_unit": "kg"
  },
  "has_point_in_time": "2024-03-15T08:00:00Z",
  "at_location": "Farm Gate Loading Dock",
  "note": "Weekly harvest delivery to regional distributor"
}
```

### Service Delivery

```json
{
  "rea_action": "service",
  "provider": "consultant-id",
  "receiver": "client-id",
  "resource_classified_as": ["consulting", "sustainability"],
  "has_beginning": "2024-03-15T09:00:00Z",
  "has_end": "2024-03-15T17:00:00Z",
  "realization_of": "consulting-agreement-id",
  "agreed_in": "CONSULT-2024-001",
  "note": "Sustainability assessment and strategy development"
}
```

### Resource Transformation

```json
{
  "rea_action": "produce",
  "output_of": "manufacturing-process-id",
  "provider": "factory-id",
  "resource_inventoried_as": "finished-goods-id",
  "resource_classified_as": ["electronics", "smartphone"],
  "resource_quantity": {
    "has_numerical_value": 1000,
    "has_unit": "units"
  },
  "has_point_in_time": "2024-03-15T16:00:00Z",
  "at_location": "Production Line A"
}
```

## Future Enhancements

### Planned Features

1. **Event Templates**: Predefined event patterns for common scenarios
2. **Automatic Validation**: Rule-based validation for specific industries
3. **Event Composition**: Composite events for complex transactions
4. **Real-time Notifications**: Event-based alerting and notifications

### Integration Points

1. **IoT Integration**: Automatic event generation from sensor data
2. **Blockchain Bridges**: Cross-system event synchronization
3. **ERP Integration**: Legacy system event import/export
4. **Analytics Integration**: Real-time business intelligence