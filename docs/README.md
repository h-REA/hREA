# hREA Developer Documentation

In-repository documentation for developers working **on** hREA and developers building
applications **with** hREA. For the conceptual overview and ecosystem context, see the
top-level [README](../README.md), [hrea.io](https://hrea.io), and the ValueFlows ontology
at [valueflo.ws](https://www.valueflo.ws).

This documentation tracks the `main-0.6` line (Holochain 0.6, HDK 0.6.0, HDI 0.7.0).

## What is hREA?

hREA (Holochain Resource-Event-Agent) is a suite of Holochain DNA modules implementing the
[ValueFlows](https://valueflo.ws) protocol, a vocabulary based on REA accounting theory for
describing flows of economic resources in distributed networks. The primary way to interface
with hREA is the GraphQL adapter library
[`@valueflows/vf-graphql-holochain`](https://www.npmjs.com/package/@valueflows/vf-graphql-holochain).

## Navigation

| Document | Audience | What it covers |
|----------|----------|----------------|
| [Getting Started](./getting-started.md) | Everyone | Prerequisites, Nix dev shell, building, running a dev network, running tests |
| [Repository Structure](./repository-structure.md) | Contributors | Monorepo layout, every top-level directory, the build artifact pipeline |
| [Architecture](./architecture.md) | Contributors | The REA/ValueFlows model, the DNA, integrity vs coordinator zomes, record types, the action system |
| [GraphQL API & Integration](./graphql-api.md) | App developers | Installing the adapter, creating the schema, Apollo wiring, example queries and mutations |
| [Contributing](./contributing.md) | Contributors | Dev workflow, testing, CI, the branch and PR model, releases |

## Two reading paths

**Building an app on hREA?** Start with [Getting Started](./getting-started.md) to spin up a
local network, then go straight to [GraphQL API & Integration](./graphql-api.md). Skim
[Architecture](./architecture.md) for the record types you will query.

**Contributing to hREA itself?** Read [Getting Started](./getting-started.md), then
[Repository Structure](./repository-structure.md) and [Architecture](./architecture.md) to
understand the zomes, and finally [Contributing](./contributing.md) for the workflow.

## License

hREA is licensed under [Apache 2.0](../LICENSE).
