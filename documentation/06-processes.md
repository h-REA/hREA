# Processes and Workflows in hREA

## Overview

**Processes** in hREA represent the organized workflows and activities that transform resources, coordinate economic activities, and achieve specific outcomes. Processes are the dynamic components that bring together resources, agents, and commitments to accomplish economic goals.

## Process Definition

### Core Structure

The `ReaProcess` struct represents economic processes in the system:

```rust
pub struct ReaProcess {
    pub id: Option<ActionHash>,              // Unique identifier
    pub name: String,                        // Process name
    pub has_beginning: Option<Timestamp>,    // Actual start time
    pub has_end: Option<Timestamp>,          // Actual end time
    pub before: Option<Timestamp>,           // Planned start time
    pub after: Option<Timestamp>,            // Planned end time
    pub classified_as: Option<Vec<String>>,   // Process classifications
    pub based_on: Option<ActionHash>,        // Process specification
    pub planned_within: Option<ActionHash>,   // Containing plan
    pub finished: Option<bool>,               // Completion status
    pub in_scope_of: Option<Vec<ActionHash>>, // Scope agents
    pub note: Option<String>,                 // Additional notes
}
```

### Field Explanations

| Field | Type | Description | Use Case |
|-------|------|-------------|----------|
| `name` | `String` | Human-readable process name | "Tomato Canning Process" |
| `has_beginning` | `Option<Timestamp>` | Actual start time | When process actually started |
| `has_end` | `Option<Timestamp>` | Actual end time | When process actually finished |
| `before` | `Option<Timestamp>` | Planned start time | Scheduled start time |
| `after` | `Option<Timestamp>` | Planned end time | Scheduled end time |
| `based_on` | `Option<ActionHash>` | Process specification | Template/blueprint |
| `finished` | `Option<bool>` | Completion flag | Process completion tracking |
| `classified_as` | `Option<Vec<String>>` | Process categories | ["manufacturing", "food-processing"] |

## Process Types and Classifications

### Manufacturing Processes

#### Production Workflows
```json
{
  "name": "Electronic Assembly Line Process",
  "before": "2024-03-15T09:00:00Z",
  "after": "2024-03-15T17:00:00Z",
  "based_on": "electronic-assembly-spec-id",
  "classified_as": ["manufacturing", "assembly", "electronics"],
  "note": "Daily assembly line run for smartphone production"
}
```

#### Quality Control Processes
```json
{
  "name": "Product Quality Inspection Process",
  "before": "2024-03-15T14:00:00Z",
  "after": "2024-03-15T15:00:00Z",
  "based_on": "quality-inspection-spec-id",
  "classified_as": ["quality-control", "inspection", "manufacturing"],
  "note": "Quality inspection for batch LOT-2024-03-001"
}
```

### Agricultural Processes

#### Growing Processes
```json
{
  "name": "Organic Tomato Growing Season 2024",
  "before": "2024-03-01T00:00:00Z",
  "after": "2024-08-31T23:59:59Z",
  "based_on": "organic-tomato-growing-spec",
  "classified_as": ["agriculture", "growing", "organic", "vegetables"],
  "in_scope_of": ["farm-organization-id", "certifying-body-id"]
}
```

#### Harvest Processes
```json
{
  "name": "Summer Harvest Process",
  "before": "2024-08-15T06:00:00Z",
  "after": "2024-08-15T18:00:00Z",
  "based_on": "harvest-procedure-spec",
  "classified_as": ["agriculture", "harvest", "seasonal"],
  "note": "Peak summer harvest for multiple crops"
}
```

### Service Processes

#### Consulting Projects
```json
{
  "name": "Digital Transformation Consulting Project",
  "has_beginning": "2024-03-01T09:00:00Z",
  "has_end": "2024-06-30T17:00:00Z",
  "classified_as": ["consulting", "digital-transformation", "professional-services"],
  "in_scope_of": ["client-organization-id", "consulting-firm-id"],
  "finished": false
}
```

#### Maintenance Processes
```json
{
  "name": "Annual Equipment Maintenance Process",
  "before": "2024-03-15T08:00:00Z",
  "after": "2024-03-15T17:00:00Z",
  "based_on": "equipment-maintenance-spec",
  "classified_as": ["maintenance", "equipment", "scheduled"],
  "note": "Annual comprehensive maintenance for production equipment"
}
```

## Process Lifecycle Management

### Creating Processes

#### Planned Process Creation

```graphql
mutation CreateProcess {
  createProcess(process: {
    name: "Q2 Manufacturing Run"
    before: "2024-04-01T09:00:00Z"
    after: "2024-06-30T17:00:00Z"
    basedOn: "manufacturing-process-spec-id"
    classifiedAs: ["manufacturing", "production-run", "q2-2024"]
    plannedWithin: "quarterly-production-plan-id"
    note: "Second quarter manufacturing run for premium product line"
  }) {
    process {
      id
      name
      before
      after
      basedOn {
        id
        name
      }
      plannedWithin {
        id
        name
      }
    }
  }
}
```

#### Ad-hoc Process Creation

```graphql
mutation CreateAdHocProcess {
  createProcess(process: {
    name: "Emergency Response Process"
    hasBeginning: "2024-03-15T10:30:00Z"
    classifiedAs: ["emergency", "response", "ad-hoc"]
    note: "Emergency response to supply chain disruption"
    inScopeOf: ["operations-team-id", "management-id"]
  }) {
    process {
      id
      name
      hasBeginning
      classifiedAs
      note
    }
  }
}
```

### Process Execution

#### Starting a Process

```graphql
mutation StartProcess {
  updateProcess(
    revisionId: "process-action-hash"
    process: {
      hasBeginning: "2024-03-15T09:00:00Z"
      note: "Process started as scheduled"
    }
  ) {
    process {
      id
      name
      hasBeginning
      note
    }
  }
}
```

#### Completing a Process

```graphql
mutation CompleteProcess {
  updateProcess(
    revisionId: "process-action-hash"
    process: {
      hasEnd: "2024-03-15T16:30:00Z"
      finished: true
      note: "Process completed successfully, all objectives met"
    }
  ) {
    process {
      id
      name
      hasEnd
      finished
      note
    }
  }
}
```

### Process Updates

#### Schedule Modifications

```graphql
mutation UpdateProcessSchedule {
  updateProcess(
    revisionId: "process-action-hash"
    process: {
      before: "2024-03-16T09:00:00Z"  # Delayed by 1 day
      after: "2024-03-16T17:00:00Z"
      note: "Schedule updated due to resource availability constraints"
    }
  ) {
    process {
      id
      name
      before
      after
      note
    }
  }
}
```

## Process Relationships and Dependencies

### Process Specifications

#### Template-based Processes

```graphql
query ProcessWithSpecification {
  reaProcess(id: "process-id") {
    id
    name
    hasBeginning
    hasEnd
    finished
    basedOn {
      id
      name
      note
      plannedInputs {
        resourceClassifiedAs
        resourceQuantity {
          hasNumericalValue
          hasUnit {
            label
          }
        }
      }
      plannedOutputs {
        resourceClassifiedAs
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

#### Creating Process from Specification

```graphql
mutation CreateProcessFromSpecification {
  createProcess(process: {
    name: "Weekly Production Run"
    before: "2024-03-15T09:00:00Z"
    after: "2024-03-15T17:00:00Z"
    basedOn: "production-process-spec-id"
    classifiedAs: ["manufacturing", "production", "weekly"]
  }) {
    process {
      id
      name
      basedOn {
        id
        name
        plannedInputs {
          edges {
            node {
              id
              action
              resourceClassifiedAs
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
              action
              resourceClassifiedAs
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
  }
}
```

### Process Hierarchies

#### Parent-Child Processes

```graphql
query ProcessHierarchy {
  reaProcess(id: "parent-process-id") {
    id
    name
    hasBeginning
    hasEnd
    finished
    childProcesses {
      edges {
        node {
          id
          name
          hasBeginning
          hasEnd
          finished
        }
      }
    }
    parentProcess {
      id
      name
    }
  }
}
```

#### Creating Subprocesses

```graphql
mutation CreateSubprocess {
  createProcess(process: {
    name: "Quality Inspection Subprocess"
    before: "2024-03-15T14:00:00Z"
    after: "2024-03-15T15:00:00Z"
    basedOn: "quality-inspection-spec-id"
    classifiedAs: ["quality-control", "subprocess"]
    plannedWithin: "parent-process-id"
    note: "Quality inspection subprocess of main production process"
  }) {
    process {
      id
      name
      plannedWithin {
        id
        name
      }
    }
  }
}
```

## Process Inputs and Outputs

### Input Resources

#### Consuming Resources in Processes

```graphql
mutation CreateProcessInput {
  createCommitment(commitment: {
    action: "consume"
    inputOf: "manufacturing-process-id"
    provider: "supplier-agent-id"
    receiver: "manufacturer-agent-id"
    resourceClassifiedAs: ["materials", "electronics", "components"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    due: "2024-03-15T08:30:00Z"
    agreedIn: "SUPPLY-AGREEMENT-001"
    note: "Electronic components for manufacturing process"
  }) {
    commitment {
      id
      inputOf {
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
    }
  }
}
```

#### Querying Process Inputs

```graphql
query ProcessInputs {
  reaProcess(id: "process-id") {
    id
    name
    plannedInputs {
      edges {
        node {
          id
          action
          resourceClassifiedAs
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          due
          provider {
            name
          }
          fulfilledBy {
            edges {
              node {
                id
                hasPointInTime
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
    }
    inputs {
      edges {
        node {
          id
          action
          resourceInventoriedAs {
            name
          }
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

### Output Resources

#### Producing Resources from Processes

```graphql
mutation CreateProcessOutput {
  createEconomicEvent(event: {
    action: "produce"
    outputOf: "manufacturing-process-id"
    provider: "manufacturer-agent-id"
    resourceInventoriedAs: "new-product-resource-id"
    resourceClassifiedAs: ["electronics", "smartphones", "finished-goods"]
    resourceQuantity: {
      hasNumericalValue: 1000
      hasUnit: "units"
    }
    hasPointInTime: "2024-03-15T16:00:00Z"
    note: "Finished smartphones from manufacturing process"
  }) {
    economicEvent {
      id
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

#### Querying Process Outputs

```graphql
query ProcessOutputs {
  reaProcess(id: "process-id") {
    id
    name
    plannedOutputs {
      edges {
        node {
          id
          action
          resourceClassifiedAs
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          due
          receiver {
            name
          }
          fulfilledBy {
            edges {
              node {
                id
                hasPointInTime
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
    }
    outputs {
      edges {
        node {
          id
          action
          resourceInventoriedAs {
            name
          }
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
  }
}
```

## Process Planning and Scheduling

### Process Planning

#### Process Templates

```json
{
  "name": "Standard Manufacturing Process Template",
  "process_type": "manufacturing",
  "duration_hours": 8,
  "required_resources": [
    {
      "resource_type": "materials",
      "classification": ["electronics", "components"],
      "quantity": {
        "has_numerical_value": 1000,
        "has_unit": "units"
      }
    },
    {
      "resource_type": "equipment",
      "classification": ["machinery", "assembly-line"],
      "quantity": {
        "has_numerical_value": 1,
        "has_unit": "line"
      }
    }
  ],
  "required_personnel": [
    {
      "role": "operator",
      "quantity": 3,
      "skills": ["equipment-operation", "quality-control"]
    },
    {
      "role": "supervisor",
      "quantity": 1,
      "skills": ["process-management", "safety"]
    }
  ],
  "expected_outputs": [
    {
      "classification": ["electronics", "finished-goods"],
      "quantity": {
        "has_numerical_value": 1000,
        "has_unit": "units"
      }
    }
  ]
}
```

#### Creating Processes from Plans

```graphql
mutation CreateProcessFromPlan {
  createProcess(process: {
    name: "Q1 Production Run"
    before: "2024-01-15T09:00:00Z"
    after: "2024-03-31T17:00:00Z"
    basedOn: "standard-manufacturing-spec-id"
    plannedWithin: "q1-production-plan-id"
    classifiedAs: ["manufacturing", "production", "quarterly"]
  }) {
    process {
      id
      name
      plannedWithin {
        id
        name
      }
    }
  }
}
```

### Process Scheduling

#### Resource-Constrained Scheduling

```graphql
query ScheduleableProcesses {
  reaProcesses(
    filters: {
      finished: false
      dateRange: {
        start: "2024-03-15T00:00:00Z"
        end: "2024-03-22T23:59:59Z"
      }
    }
  ) {
    edges {
      node {
        id
        name
        before
        after
        basedOn {
          id
          name
          plannedInputs {
            edges {
              node {
                resourceClassifiedAs
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
    }
  }
}
```

#### Process Conflict Detection

```graphql
query ProcessConflicts {
  reaProcesses(
    filters: {
      before: "2024-03-16T09:00:00Z"
      after: "2024-03-16T17:00:00Z"
      classifiedAs: ["manufacturing"]
    }
  ) {
    edges {
      node {
        id
        name
        basedOn {
          plannedInputs {
            edges {
              node {
                resourceClassifiedAs
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
    }
  }
}
```

## Process Monitoring and Control

### Process Status Tracking

#### Real-time Process Monitoring

```graphql
subscription ProcessStatusUpdates($processId: ID!) {
  reaProcessUpdates(processId: $processId) {
    type
    timestamp
    process {
      id
      name
      hasBeginning
      hasEnd
      finished
      inputs {
        totalCount
        edges {
          node {
            id
            hasPointInTime
            resourceQuantity {
              hasNumericalValue
            }
          }
        }
      }
      outputs {
        totalCount
        edges {
          node {
            id
            hasPointInTime
            resourceQuantity {
              hasNumericalValue
            }
          }
        }
      }
    }
  }
}
```

#### Process Analytics

```graphql
query ProcessAnalytics($processId: ID!) {
  reaProcess(id: $processId) {
    id
    name
    hasBeginning
    hasEnd
    finished
    efficiencyMetrics {
      plannedDuration
      actualDuration
      inputEfficiency
      outputEfficiency
      resourceUtilization
    }
    costAnalysis {
      plannedCost
      actualCost
      variance
      costPerUnit
    }
    qualityMetrics {
      inputQualityScore
      outputQualityScore
      defectRate
      reworkRate
    }
  }
}
```

### Process Control

#### Process Intervention

```graphql
mutation PauseProcess {
  updateProcess(
    revisionId: "process-action-hash"
    process: {
      note: "Process paused due to equipment malfunction"
    }
  ) {
    process {
      id
      name
      note
    }
  }
}
```

#### Process Cancellation

```graphql
mutation CancelProcess {
  updateProcess(
    revisionId: "process-action-hash"
    process: {
      hasEnd: "2024-03-15T11:30:00Z"
      finished: true
      note: "Process cancelled due to supplier issue"
    }
  ) {
    process {
      id
      name
      hasEnd
      finished
      note
    }
  }
}
```

## Process Optimization

### Performance Analysis

#### Process Efficiency Calculation

```graphql
query ProcessEfficiency($processId: ID!) {
  reaProcess(id: $processId) {
    id
    name
    plannedInputs {
      edges {
        node {
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          resourceClassifiedAs
        }
      }
    }
    plannedOutputs {
      edges {
        node {
          resourceQuantity {
            hasNumericalValue
            hasUnit {
              label
            }
          }
          resourceClassifiedAs
        }
      }
    }
    inputs {
      edges {
        node {
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
    outputs {
      edges {
        node {
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
  }
}
```

### Bottleneck Identification

```graphql
query ProcessBottlenecks {
  reaProcesses(
    filters: {
      finished: false
      classifiedAs: ["manufacturing"]
    }
  ) {
    edges {
      node {
        id
        name
        inputs {
          totalCount
          fulfilledCount
        }
        outputs {
          totalCount
          fulfilledCount
        }
        bottleneckResources {
          resourceClassifiedAs
          demand
          available
          shortage
        }
      }
    }
  }
}
```

## Process Collaboration

### Multi-Agent Processes

#### Cross-Organization Processes

```json
{
  "name": "Supply Chain Collaboration Process",
  "before": "2024-03-15T09:00:00Z",
  "after": "2024-03-15T18:00:00Z",
  "classified_as": ["supply-chain", "collaboration", "cross-organization"],
  "in_scope_of": [
    "manufacturer-id",
    "distributor-id",
    "retailer-id",
    "logistics-provider-id"
  ],
  "note": "Collaborative process involving multiple supply chain partners"
}
```

#### Process Permissions

```graphql
query ProcessPermissions {
  reaProcess(id: "process-id") {
    id
    name
    inScopeOf {
      id
      name
      agentType
    }
    processPermissions {
      agent {
        id
        name
      }
      permission
      scope
    }
  }
}
```

## Process Templates and Reusability

### Creating Process Templates

```graphql
mutation CreateProcessTemplate {
  createProcessSpecification(specification: {
    name: "Standard Food Canning Process"
    note: "Template for food canning processes with safety and quality requirements"
    intendedUses: ["manufacturing", "food-processing", "canning"]
    plannedInputs: [
      {
        action: "consume"
        resourceClassifiedAs: ["food", "raw-materials"]
        resourceQuantity: {
          hasNumericalValue: 1000
          hasUnit: "kg"
        }
        hasCost: {
          hasNumericalValue: 2000
          hasUnit: "USD"
        }
      }
    ]
    plannedOutputs: [
      {
        action: "produce"
        resourceClassifiedAs: ["food", "canned-goods"]
        resourceQuantity: {
          hasNumericalValue: 950
          hasUnit: "kg"
        }
      }
    ]
  }) {
    processSpecification {
      id
      name
      intendedUses
      plannedInputs {
        edges {
          node {
            action
            resourceClassifiedAs
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
            action
            resourceClassifiedAs
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
}
```

## Process Integration

### Integration with External Systems

#### ERP System Integration

```typescript
// Process synchronization with ERP system
class ProcessERPIntegration {
  async syncProcessToERP(processId: string) {
    const process = await this.holochainClient.queryProcess(processId);

    const erpProcess = {
      name: process.name,
      plannedStart: process.before,
      plannedEnd: process.after,
      actualStart: process.hasBeginning,
      actualEnd: process.hasEnd,
      status: process.finished ? 'completed' : 'in-progress',
      specification: process.basedOn?.id,
      outputs: process.outputs.edges.map(edge => ({
        productId: edge.node.resourceInventoriedAs?.id,
        quantity: edge.node.resourceQuantity.hasNumericalValue,
        unit: edge.node.resourceQuantity.hasUnit.label
      }))
    };

    await this.erpClient.updateProcess(processId, erpProcess);
  }

  async syncFromERP(erpProcessId: string) {
    const erpProcess = await this.erpClient.getProcess(erpProcessId);

    const hreaProcess = {
      name: erpProcess.name,
      before: erpProcess.plannedStart,
      after: erpProcess.plannedEnd,
      hasBeginning: erpProcess.actualStart,
      hasEnd: erpProcess.actualEnd,
      finished: erpProcess.status === 'completed',
      basedOn: erpProcess.specificationId
    };

    await this.holochainClient.updateProcess(erpProcessId, hreaProcess);
  }
}
```

#### IoT Device Integration

```typescript
// Real-time process monitoring via IoT sensors
class ProcessIOTMonitoring {
  async monitorProcessWithSensors(processId: string) {
    const process = await this.holochainClient.queryProcess(processId);

    // Connect to IoT sensors for process equipment
    const sensors = await this.iotClient.getSensorsForProcess(processId);

    sensors.forEach(sensor => {
      sensor.on('reading', async (data) => {
        await this.handleSensorData(processId, sensor.id, data);
      });
    });
  }

  async handleSensorData(processId: string, sensorId: string, data: any) {
    // Process sensor data and create events if needed
    if (data.temperature > this.maxTemperature) {
      await this.createProcessEvent(processId, {
        type: 'temperature_alert',
        sensorId,
        value: data.temperature,
        timestamp: new Date().toISOString()
      });
    }
  }
}
```

## Best Practices

### Process Design

1. **Clear Objectives**: Define clear, measurable process objectives
2. **Appropriate Granularity**: Balance detail level with manageability
3. **Standardized Templates**: Use process specifications for consistency
4. **Resource Planning**: Account for all required resources and constraints
5. **Quality Gates**: Include quality control checkpoints

### Process Management

1. **Real-time Monitoring**: Track process progress and performance
2. **Exception Handling**: Plan for and manage process exceptions
3. **Continuous Improvement**: Regularly review and optimize processes
4. **Documentation**: Maintain comprehensive process documentation
5. **Stakeholder Communication**: Keep all stakeholders informed

### Process Integration

1. **System Compatibility**: Ensure compatibility with existing systems
2. **Data Consistency**: Maintain data consistency across integrations
3. **Performance Optimization**: Monitor and optimize integration performance
4. **Error Handling**: Implement robust error handling and recovery
5. **Security**: Maintain appropriate security controls

## Use Case Examples

### Manufacturing Process

```json
{
  "name": "Smartphone Manufacturing Process",
  "before": "2024-03-15T09:00:00Z",
  "after": "2024-03-15T17:00:00Z",
  "based_on": "smartphone-manufacturing-spec",
  "classified_as": ["manufacturing", "electronics", "assembly"],
  "note": "Daily smartphone assembly line with quality checkpoints"
}
```

### Agricultural Process

```json
{
  "name": "Organic Farm Season 2024",
  "before": "2024-03-01T00:00:00Z",
  "after": "2024-11-30T23:59:59Z",
  "based_on": "organic-farming-spec",
  "classified_as": ["agriculture", "organic", "seasonal"],
  "in_scope_of": ["farm-organization", "certification-body"]
}
```

### Service Process

```json
{
  "name": "Software Development Sprint",
  "has_beginning": "2024-03-15T09:00:00Z",
  "has_end": "2024-03-29T17:00:00Z",
  "classified_as": ["software-development", "agile", "sprint"],
  "finished": false,
  "note": "Two-week sprint for feature development and testing"
}
```

This process management system provides comprehensive tools for designing, executing, monitoring, and optimizing economic processes across diverse industries and use cases.