# hREA Overview

## What is hREA?

hREA (Holochain/REA) is a comprehensive framework for building distributed economic coordination systems. It implements the **ValueFlows protocol**, which extends the well-established **REA (Resources, Events, Agents) accounting model** for modern distributed applications.

## Core Philosophy

### Economic Coordination for All

hREA provides building blocks for:
- **Supply Chain Management**: Track goods and services through production and distribution
- **Project Management**: Coordinate collaborative work and resource allocation
- **Enterprise Resource Planning**: Integrated business process management
- **Alternative Economies**: Gift economies, contributory systems, and post-capitalist models

### Composable by Design

Unlike monolithic ERP systems, hREA is built with **modularity** and **composability** as core principles:

- **No-Code Reconfiguration**: Business users can remix economic patterns without technical expertise
- **Runtime Flexibility**: System architectures can be reorganized dynamically
- **Cross-Project Interoperability**: Components work across different applications and organizations

## Key Capabilities

### Core Economic Functions

| Function | Description | Use Cases |
|----------|-------------|-----------|
| **People & Groups** | Identity management and trust networks | Team formation, reputation systems |
| **Scheduled Deliverables** | Process planning with defined outcomes | Project timelines, production schedules |
| **Agreements & Contracts** | Market exchanges and mutual obligations | Purchase orders, service agreements |
| **Event Ledger** | Immutable record of economic flows | Transaction history, audit trails |
| **Coordination Functions** | Collaborative decision-making | Consensus processes, voting systems |
| **Needs Matching** | Bilateral and multilateral trade matching | Marketplaces, resource sharing |
| **Knowledge Sharing** | Structured production knowledge | Best practices, process documentation |

### Data Types and Entities

#### Economic Agents
- **Individuals**: People participating in economic activities
- **Organizations**: Companies, non-profits, collectives
- **Groups**: Teams, departments, project groups

#### Resources
- **Tangible Goods**: Physical products, materials, equipment
- **Intangible Resources**: Skills, knowledge, digital assets
- **Services**: Labor, expertise, time-based contributions

#### Economic Events
- **Transfers**: Movement of resources between agents
- **Transformations**: Production processes and resource conversion
- **Consumption**: Resource utilization in processes

## Technical Architecture

### Holochain Foundation

hREA is built on **Holochain**, providing:
- **Distributed Storage**: Data integrity without centralized servers
- **Scalable Networking**: Peer-to-peer communication patterns
- **Cryptographic Security**: Tamper-evident data and identity verification
- **Agent-Centric Architecture**: User-controlled data and privacy

### ValueFlows Integration

The **ValueFlows protocol** provides:
- **Common Vocabulary**: Standardized economic terminology
- **Interoperability**: Cross-platform compatibility
- **Semantic Precision**: Unambiguous data representation
- **Extensibility**: Framework for custom economic concepts

### GraphQL Interface

hREA includes a **JavaScript/TypeScript GraphQL adapter** for:
- **REST-like APIs**: Familiar web service integration
- **Real-time Subscriptions**: Live data updates
- **Type Safety**: Auto-generated TypeScript definitions
- **Developer Experience**: Modern development tooling

## Use Case Examples

### Supply Chain Tracking
```graphql
# Track a product from raw material to final delivery
mutation {
  createEconomicEvent(event: {
    action: "produce"
    resourceInventoriedAs: "raw-material-id"
    outputOf: "manufacturing-process-id"
    resourceQuantity: { hasNumericalValue: 100, hasUnit: "items" }
  }) {
    economicEvent {
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
```

### Project Management
```graphql
# Create a commitment between team members
mutation {
  createCommitment(commitment: {
    action: "deliver"
    agreedIn: "project-agreement-id"
    provider: "team-member-1"
    receiver: "project-manager"
    resourceClassifiedAs: ["software-feature"]
    due: "2024-03-15T00:00:00Z"
  }) {
    commitment {
      id
      due
      resourceClassifiedAs
    }
  }
}
```

### Knowledge Sharing
```graphql
# Share a production process specification
mutation {
  createProcessSpecification(specification: {
    name: "Sustainable Farming Process"
    note: "Organic vegetable growing best practices"
    intendedUses: ["food-production", "education"]
  }) {
    processSpecification {
      id
      name
      intendedUses
    }
  }
}
```

## The ValueFlows Ecosystem

### Cross-Platform Compatibility

hREA implements the ValueFlows specification, enabling interoperability with:
- **Bonfire**: Alternative distributed backend platform
- **Traditional Systems**: ERP and CRM applications
- **Custom Applications**: Any ValueFlows-compatible implementation

### Semantic Interoperability

The ValueFlows vocabulary ensures consistent meaning across systems:
- **Standardized Terms**: Precise economic terminology
- **Data Models**: Consistent relationship definitions
- **API Patterns**: Standardized interface conventions

## Development Approach

### Incremental Adoption

Organizations can adopt hREA incrementally:
1. **Start Small**: Begin with specific use cases
2. **Scale Gradually**: Expand capabilities over time
3. **Integrate Existing**: Connect with current systems
4. **Customize Selectively**: Add specialized functionality as needed

### Open Source Development

hREA follows open development principles:
- **Community Contribution**: Collaborative development process
- **Transparent Governance**: Open decision-making
- **Extensible Architecture**: Plugin system for custom functionality
- **Documentation-First**: Comprehensive developer resources

## Next Steps

To continue exploring hREA:

1. **Architecture Overview**: Read about system design and components
2. **Developer Guide**: Set up development environment
3. **API Reference**: Explore integration possibilities
4. **Examples**: Study real-world implementation patterns

## Resources

- **Project Website**: [hrea.io](https://hrea.io)
- **ValueFlows Specification**: [valueflo.ws](https://valueflo.ws)
- **Holochain Documentation**: [docs.holochain.org](https://docs.holochain.org)
- **Community Support**: [GitHub Discussions](https://github.com/h-REA/hREA/discussions)