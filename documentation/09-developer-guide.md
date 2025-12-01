# hREA Developer Guide

## Overview

This guide provides comprehensive information for developers working with hREA, including setup, development workflows, testing, debugging, and contribution guidelines.

## Development Environment Setup

### Prerequisites

#### System Requirements
- **Operating System**: Linux, macOS, or Windows (with WSL2)
- **Memory**: 8GB RAM minimum, 16GB recommended
- **Storage**: 10GB free space
- **Network**: Internet connection for dependencies

#### Required Software

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustup update stable
   ```

2. **Node.js and npm**
   ```bash
   # Using nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   ```

3. **Holochain Development Tools**
   ```bash
   cargo install holochain
   cargo install hdk
   ```

4. **Git**
   ```bash
   # Ubuntu/Debian
   sudo apt-get install git

   # macOS
   brew install git

   # Windows
   # Download from https://git-scm.com/
   ```

### Project Setup

#### Clone Repository

```bash
git clone https://github.com/h-REA/hREA.git
cd hREA
```

#### Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies
npm install
```

#### Build Project

```bash
# Build all components
npm run build

# Or build specific components
npm run build:rust
npm run build:ui
npm run build:graphql
```

## Project Structure

### Directory Organization

```
hREA/
├── dnas/                    # Holochain DNA definitions
│   └── hrea/
│       ├── zomes/          # Zome modules
│       │   ├── integrity/   # Data integrity definitions
│       │   │   └── hrea/
│       │   └── coordinator/ # Business logic
│       │       └── hrea/
│       └── workdir/         # DNA build artifacts
├── modules/                # JavaScript/TypeScript modules
│   ├── vf-graphql-holochain/ # GraphQL adapter
│   └── ui/                  # User interface
├── tests/                  # Test suites
├── scripts/               # Build and deployment scripts
├── documentation/         # Project documentation
└── package.json           # Node.js project configuration
```

### Key Components

#### DNA (Distributed Network Application)
- **Location**: `dnas/hrea/`
- **Purpose**: Core Holochain application logic
- **Components**: Zomes for different functional areas

#### Zomes (Holochain Modules)
- **Integrity Zome**: `dnas/hrea/zomes/integrity/hrea/`
  - Data structures and validation rules
  - Entry definitions and link types
- **Coordinator Zome**: `dnas/hrea/zomes/coordinator/hrea/`
  - Business logic and external interfaces
  - CRUD operations and complex workflows

#### GraphQL Adapter
- **Location**: `modules/vf-graphql-holochain/`
- **Purpose**: GraphQL API layer
- **Features**: Schema generation, query execution

#### User Interface
- **Location**: `modules/ui/`
- **Technology**: Svelte/SvelteKit
- **Purpose**: Reference implementation UI

## Development Workflow

### 1. Making Changes

#### Rust/Zome Development

```bash
# Navigate to zome directory
cd dnas/hrea/zomes/integrity/hrea

# Make changes to source files
vim src/rea_economic_event.rs

# Build zome
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
```

#### GraphQL Schema Development

```bash
# Navigate to GraphQL module
cd modules/vf-graphql-holochain

# Make changes to schema or resolvers
vim src/schema/
vim src/resolvers/

# Build and test
npm run build
npm test
```

#### UI Development

```bash
# Navigate to UI directory
cd modules/ui

# Start development server
npm run dev

# Build for production
npm run build
```

### 2. Testing Your Changes

#### Unit Tests (Rust)

```bash
# Run all Rust tests
cargo test

# Run specific zome tests
cd dnas/hrea/zomes/integrity/hrea
cargo test

# Run tests with output
cargo test -- --nocapture
```

#### Integration Tests

```bash
# Run Holochain integration tests
npm run test:integration

# Run specific test suite
npm run test:api
npm run test:graphql
```

#### UI Tests

```bash
# Navigate to UI directory
cd modules/ui

# Run component tests
npm run test:unit

# Run E2E tests
npm run test:e2e
```

### 3. Building and Running

#### Development Build

```bash
# Build all components
npm run build:dev

# Start development environment
npm run dev
```

#### Production Build

```bash
# Build for production
npm run build:prod

# Package application
npm run package
```

#### Local Testing

```bash
# Start local Holochain conductor
holochain --conductor-port 8888

# Install hApp
hc admin --interface-port 8888 app install --path bundles/hrea.happ

# Start UI
cd modules/ui && npm run dev
```

## Code Style and Guidelines

### Rust Code Style

#### Formatting

```bash
# Format all Rust code
cargo fmt

# Check formatting
cargo fmt -- --check
```

#### Linting

```bash
# Run clippy lints
cargo clippy -- -D warnings

# Fix auto-fixable lints
cargo clippy --fix
```

#### Code Style Rules

1. **Naming Conventions**
   - Use `snake_case` for functions and variables
   - Use `PascalCase` for types and structs
   - Use `SCREAMING_SNAKE_CASE` for constants

2. **Error Handling**
   ```rust
   // Good: Use Result types
   fn create_event(event: EconomicEvent) -> ExternResult<ActionHash> {
       validate_event(&event)?;
       let action_hash = create_entry(&event)?;
       Ok(action_hash)
   }

   // Bad: Use unwrap() in production code
   fn create_event_bad(event: EconomicEvent) -> ActionHash {
       create_entry(&event).unwrap()  // Don't do this
   }
   ```

3. **Documentation**
   ```rust
   /// Creates a new economic event in the Holochain source chain
   ///
   /// # Arguments
   /// * `event` - The economic event to create
   ///
   /// # Returns
   /// * `Result<ActionHash, WasmError>` - The action hash or error
   ///
   /// # Examples
   /// ```
   /// let event = EconomicEvent::new("transfer".to_string());
   /// let hash = create_event(event)?;
   /// ```
   #[hdk_extern]
   pub fn create_economic_event(event: EconomicEvent) -> ExternResult<ActionHash> {
       // Implementation
   }
   ```

### JavaScript/TypeScript Style

#### ESLint Configuration

```bash
# Run linter
npm run lint

# Fix auto-fixable issues
npm run lint:fix
```

#### Code Style Rules

1. **TypeScript Usage**
   ```typescript
   // Good: Use explicit types
   interface EconomicEvent {
     id: string;
     action: string;
     resourceQuantity?: QuantityValue;
   }

   function createEvent(event: EconomicEvent): Promise<string> {
     return client.mutations.createEconomicEvent({ event });
   }

   // Bad: Use implicit types
   function createEventBad(event) {  // Missing type annotations
     return client.mutations.createEconomicEvent({ event });
   }
   ```

2. **Async/Await**
   ```typescript
   // Good: Use async/await
   async function fetchAgent(id: string): Promise<Agent> {
     const result = await client.queries.reaAgent({ id });
     return result.reaAgent;
   }

   // Avoid: Complex promise chains
   function fetchAgentBad(id: string): Promise<Agent> {
     return client.queries.reaAgent({ id })
       .then(result => result.reaAgent)
       .catch(err => { throw err; });
   }
   ```

## Testing Strategy

### Test Organization

#### Unit Tests
- **Purpose**: Test individual functions and modules
- **Location**: Co-located with source code
- **Framework**: Built-in Rust testing, Jest for JavaScript

#### Integration Tests
- **Purpose**: Test component interactions
- **Location**: `tests/` directory
- **Framework**: Holochain test harness

#### End-to-End Tests
- **Purpose**: Test complete user workflows
- **Location**: `tests/e2e/`
- **Framework**: Playwright, Cypress

### Writing Tests

#### Rust Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::test_agent;

    #[test]
    fn test_economic_event_creation() {
        let agent = test_agent();
        let event = EconomicEvent {
            action: "transfer".to_string(),
            provider: Some(agent.id),
            resource_quantity: Some(QuantityValue {
                has_numerical_value: 100.0,
                has_unit: "kg".to_string(),
            }),
            ..Default::default()
        };

        let result = validate_economic_event(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_event_rejection() {
        let event = EconomicEvent {
            action: "".to_string(),  // Invalid: empty action
            ..Default::default()
        };

        let result = validate_economic_event(&event);
        assert!(result.is_err());
    }
}
```

#### GraphQL Tests

```javascript
import { gql } from 'apollo-server-express';
import { createTestClient } from '@apollo/client/testing';

const CREATE_ECONOMIC_EVENT = gql`
  mutation CreateEconomicEvent($event: EconomicEventCreateParams!) {
    createEconomicEvent(event: $event) {
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
`;

describe('Economic Event Mutations', () => {
  test('should create valid economic event', async () => {
    const mockClient = createTestClient({
      // Mock setup
    });

    const variables = {
      event: {
        action: 'transfer',
        resourceQuantity: {
          hasNumericalValue: 100,
          hasUnit: 'kg'
        }
      }
    };

    const result = await mockClient.mutate({
      mutation: CREATE_ECONOMIC_EVENT,
      variables
    });

    expect(result.data.createEconomicEvent).toBeDefined();
    expect(result.data.createEconomicEvent.economicEvent.action).toBe('transfer');
  });
});
```

### Test Fixtures

#### Rust Fixtures

```rust
// fixtures/mod.rs
pub fn test_agent() -> ReaAgent {
    ReaAgent {
        id: None,
        name: "Test Agent".to_string(),
        agent_type: "Person".to_string(),
        ..Default::default()
    }
}

pub fn test_economic_event() -> ReaEconomicEvent {
    ReaEconomicEvent {
        id: None,
        rea_action: "transfer".to_string(),
        provider: Some(ActionHash::from_raw_36(test_hash())),
        ..Default::default()
    }
}
```

### Test Data Management

#### Holochain Test Scenarios

```rust
use holochain::test_utils::conductor_setup::ConductorBuilder;

#[tokio::test]
async fn test_economic_event_workflow() {
    let mut conductor = ConductorBuilder::new().build().await;

    // Install app
    let app_id = conductor.setup_app("hrea", &[&dna_path]).await;

    // Create agent
    let agent_hash = conductor.call::<String, ActionHash>(
        &app_id,
        "zome_name",
        "create_agent",
        agent_data,
    ).await;

    // Create economic event
    let event_data = EconomicEventData {
        action: "transfer",
        provider: agent_hash,
        resource_quantity: QuantityValue {
            has_numerical_value: 100.0,
            has_unit: "kg".to_string(),
        },
    };

    let event_hash = conductor.call::<EconomicEventData, ActionHash>(
        &app_id,
        "zome_name",
        "create_economic_event",
        event_data,
    ).await;

    // Verify event was created
    let event = conductor.call::<ActionHash, EconomicEvent>(
        &app_id,
        "zome_name",
        "get_economic_event",
        event_hash,
    ).await;

    assert_eq!(event.rea_action, "transfer");
}
```

## Debugging

### Rust Debugging

#### Logging

```rust
use holochain::prelude::*;

#[hdk_extern]
pub fn create_economic_event(event: EconomicEvent) -> ExternResult<ActionHash> {
    debug!("Creating economic event: {:?}", event);

    match validate_event(&event) {
        Ok(()) => {
            let action_hash = create_entry(&event)?;
            info!("Created economic event with hash: {:?}", action_hash);
            Ok(action_hash)
        }
        Err(e) => {
            error!("Failed to validate economic event: {:?}", e);
            Err(e)
        }
    }
}
```

#### Unit Test Debugging

```rust
#[test]
fn test_complex_validation() {
    let event = create_complex_event();

    // Enable detailed output
    env_logger::init();

    debug!("Validating event: {:#?}", event);

    let result = validate_economic_event(&event);

    match result {
        Ok(()) => println!("✓ Validation passed"),
        Err(e) => {
            println!("✗ Validation failed: {:#?}", e);
            println!("Event details: {:#?}", event);
        }
    }
}
```

### GraphQL Debugging

#### Query Inspection

```typescript
// Add debug middleware
import { ApolloServer } from 'apollo-server-express';

const server = new ApolloServer({
  typeDefs,
  resolvers,
  plugins: [{
    requestDidStart() {
      return {
        didResolveOperation(requestContext) {
          console.log('GraphQL Query:', JSON.stringify(requestContext.request.query, null, 2));
          console.log('Variables:', JSON.stringify(requestContext.request.variables, null, 2));
        },
        didEncounterErrors(requestContext) {
          console.error('GraphQL Errors:', requestContext.errors);
        }
      };
    }
  }]
});
```

#### Error Handling

```typescript
// Custom error handling
export const errorHandler = {
  formatError: (error: GraphQLError) => {
    console.error('GraphQL Error:', {
      message: error.message,
      locations: error.locations,
      path: error.path,
      extensions: error.extensions
    });

    return {
      ...error,
      extensions: {
        ...error.extensions,
        timestamp: new Date().toISOString(),
        requestId: generateRequestId()
      }
    };
  }
};
```

### Frontend Debugging

#### Svelte DevTools

```typescript
// Enable debugging in development
import { setContext } from 'svelte';

if (process.env.NODE_ENV === 'development') {
  setContext('debug', true);
}
```

#### Network Inspection

```typescript
// Add request interceptors for debugging
import { ApolloClient, InMemoryCache } from '@apollo/client';

const client = new ApolloClient({
  uri: '/graphql',
  cache: new InMemoryCache(),
  defaultOptions: {
    watchQuery: {
      errorPolicy: 'all',
    },
  },
});

// Debug network requests
client.query({
  query: GET_ECONOMIC_EVENTS,
  errorPolicy: 'all',
  fetchPolicy: 'network-only'
}).then(result => {
  if (result.errors) {
    console.error('Query errors:', result.errors);
  }
  console.log('Query result:', result.data);
});
```

## Performance Optimization

### Rust Optimization

#### Wasm Binary Size

```toml
# Cargo.toml optimizations
[profile.release]
opt-level = "s"
lto = true
panic = "abort"
```

#### Memory Management

```rust
// Use efficient data structures
use std::collections::HashMap;

// Good: Pre-allocate capacity
let mut events = HashMap::with_capacity(expected_size);

// Avoid unnecessary cloning
fn process_event(event: &EconomicEvent) -> Result<()> {
    // Use references instead of cloning
    let resource = &event.resource_quantity;
    Ok(())
}
```

### GraphQL Optimization

#### Query Optimization

```typescript
// Efficient query design
const OPTIMIZED_QUERY = gql`
  query EconomicEvents($filters: EconomicEventFilters!) {
    reaEconomicEvents(filters: $filters, first: 50) {
      edges {
        node {
          id
          action  # Only request needed fields
          hasPointInTime
          resourceQuantity {
            hasNumericalValue
            hasUnit { label }
          }
        }
      }
      pageInfo {
        hasNextPage
        endCursor
      }
    }
  }
`;
```

#### Caching Strategy

```typescript
// Implement caching
const client = new ApolloClient({
  uri: '/graphql',
  cache: new InMemoryCache({
    typePolicies: {
      Query: {
        fields: {
          reaEconomicEvents: {
            keyArgs: ['filters'],
            merge: (existing, incoming) => {
              return {
                ...existing,
                ...incoming,
                edges: [...(existing?.edges || []), ...incoming.edges],
              };
            },
          },
        },
      },
    },
  }),
});
```

## Security Considerations

### Input Validation

```rust
// Comprehensive validation
pub fn validate_agent_input(agent: &ReaAgent) -> ExternResult<()> {
    // Check required fields
    if agent.name.is_empty() {
        return Err(wasm_error!("Agent name cannot be empty"));
    }

    // Sanitize input
    if agent.name.len() > 1000 {
        return Err(wasm_error!("Agent name too long"));
    }

    // Validate format
    if !is_valid_name(&agent.name) {
        return Err(wasm_error!("Invalid agent name format"));
    }

    Ok(())
}
```

### Permission Checks

```rust
// Agent-based authorization
#[hdk_extern]
pub fn update_economic_event(
    revision_id: ActionHash,
    event: ReaEconomicEvent,
) -> ExternResult<ActionHash> {
    let agent_info = agent_info()?;

    // Check if agent created original event
    let original_record = must_get_valid_record(revision_id)?;
    if !can_modify_record(&agent_info, &original_record) {
        return Err(wasm_error!("Permission denied"));
    }

    // Proceed with update
    update_entry(revision_id, &event)
}
```

### Data Privacy

```rust
// Sensitive data handling
pub struct PrivateEconomicEvent {
    pub id: ActionHash,
    pub action: String,
    pub resource_quantity: QuantityValue,
    // Private fields only accessible by specific agents
    private_details: Option<PrivateDetails>,
}

fn can_access_private_data(
    agent: &AgentInfo,
    event: &PrivateEconomicEvent,
) -> bool {
    // Implement access control logic
    is_participant(agent, event) || is_auditor(agent)
}
```

## Contribution Guidelines

### Development Workflow

#### 1. Create Feature Branch

```bash
git checkout -b feature/economic-event-enhancements
```

#### 2. Make Changes

- Follow coding standards
- Add comprehensive tests
- Update documentation

#### 3. Test Changes

```bash
# Run full test suite
npm run test:all

# Run linting
npm run lint:all

# Build project
npm run build:prod
```

#### 4. Submit Pull Request

- Clear commit messages
- Description of changes
- Reference relevant issues

### Code Review Process

#### Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are comprehensive
- [ ] Documentation is updated
- [ ] Performance impact is considered
- [ ] Security implications are addressed
- [ ] Backward compatibility is maintained

#### Review Guidelines

1. **Constructive Feedback**: Provide specific, actionable feedback
2. **Code Quality**: Focus on maintainability and readability
3. **Testing**: Ensure adequate test coverage
4. **Documentation**: Verify documentation accuracy

### Release Process

#### Version Management

```bash
# Update version numbers
npm version patch  # or minor, major

# Update changelog
git add CHANGELOG.md
git commit -m "Update changelog for v1.2.3"

# Tag release
git tag v1.2.3
git push origin v1.2.3
```

#### Release Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] Version numbers updated
- [ ] Changelog updated
- [ ] Security review completed
- [ ] Performance testing completed

## Troubleshooting

### Common Issues

#### Build Failures

```bash
# Clean build artifacts
cargo clean
rm -rf node_modules/
npm install
npm run build
```

#### Test Failures

```bash
# Run tests with verbose output
RUST_LOG=debug cargo test -- --nocapture

# Run specific test
cargo test test_economic_event_creation -- --exact
```

#### Holochain Issues

```bash
# Reset conductor
hc admin --interface-port 8888 conductor clean

# Check conductor status
hc admin --interface-port 8888 conductor status
```

### Getting Help

#### Community Resources

1. **GitHub Issues**: Report bugs and feature requests
2. **Discord Community**: Real-time discussion
3. **Documentation**: Check existing guides
4. **Examples**: Review sample implementations

#### Debug Information Collection

```bash
# Collect system information
npm run info

# Generate debug report
npm run debug-report
```

This developer guide provides a comprehensive foundation for contributing to hREA. For specific implementation details, refer to the relevant sections in the other documentation files.