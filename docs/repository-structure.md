# Repository structure

This document details the layers of the system and where to find each set of components within this repository.

Jump to:
- [Application layer](#application-layer)
- [ValueFlows GraphQL \(protocol layer\)](#valueflows-graphql-protocol-layer)
- [GraphQL interface \(JavaScript interface\)](#graphql-interface-javascript-interface)
- [DNA modules \(outer Holochain layer\)](#dna-modules-outer-holochain-layer)
- [Zome modules \(inner Holochain layer\)](#zome-modules-inner-holochain-layer)
	- [1. Interface struct crates \(Rust interface\)](#1-interface-struct-crates-rust-interface)
	- [2. Coordinator Zome crates \(WASM interface\)](#2-coordinator-zome-crates-wasm-interface)
	- [3. Library crates \(system core\)](#3-library-crates-system-core)
	- [4. Integrity Zome crates \(WASM interface\)](#4-integrity-zome-crates-wasm-interface)
	- [5. Storage crates \(database layer\)](#4-storage-crates-database-layer)
	- [6. Storage constants \(database internals\)](#6-storage-constants-database-internals)
- [Library modules](#library-modules)
	- [`hdk_records`](#hdk_records)
	- [`hdk_uuid_types`](#hdk_uuid_types)
	- [`serde_maybe_undefined`](#serde_maybe_undefined)


### Application layer

- [**`apps/`**](apps/) contains end-user applications built on the hREA framework.
	- [**`apps/hrea-graphql-explorer/`**](apps/hrea-graphql-explorer/) is a [GraphiQL](https://github.com/graphql/graphiql) interface to the system with some added [additions to assist with comprehension](https://github.com/OneGraph/graphiql-explorer-example). Wired up to the development DNAs by default&mdash; super handy for testing and getting to know the ValueFlows data structure.
- [**`test/`**](test/) contains integration tests for the application suite as a whole. Connections to the [GraphQL Interface](#graphql-interface-outer-layer) and Holochain application cells are managed in `init.js`.



### ValueFlows GraphQL (protocol layer)

The [ValueFlows GraphQL spec](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql) is an effort towards composable application architectures for distributed socioeconomic coordination applications.

Though it exists outside of this repository and is co-governed by many stakeholders, it is worth mentioning here that ValueFlows GraphQL and the core [ValueFlows RDF vocabulary](https://lab.allmende.io/valueflows/valueflows/) are open protocols which accept proposals for improvement. Contributions to the domain model from non-technical authors are welcomed and encouraged.

If you are implementing systems which you'd like to be compatible with our interfaces and client applications, the [ValueFlows GraphQL NodeJS module](https://www.npmjs.com/package/@valueflows/vf-graphql) can be leveraged to build and validate implementations, and to export raw schema formats for injection into other software.



### GraphQL interface (JavaScript interface)

[**`modules/graphql-client`**](modules/graphql-client) contains a NodeJS module exporting a configurable `GraphQLClient` generator function that provides provide a simplified and coherent entrypoint to the system. It contains some features which introspect the connected Holochain conductor to automatically determine the appropriate running DNA 'cells' to connect to under normal operations.

**You should develop against this interface if:**

- You are a UI app developer creating an application to run on top of standard hREA module functionality.
- You are a UI app developer extending hREA's functionality with custom business or domain-specific integrations (including web platforms, blockchain tech and other Holochain modules).
- You are a creator of distributed socioeconomic coordination applications and you would like your apps to run on hREA, Bonfire or other protocol-compatible storage backends.
- You are a UI app developer implementing a multi-agent UI (Holochain or otherwise), where the arrangements of hREA modules within a collaboration space do not diverge from standard configurations.

Note that there is a [mock GraphQL client](https://www.npmjs.com/package/@vf-ui/graphql-client-mock) available for building ValueFlows apps against which does not require Holochain to be configured.

[**`modules/vf-graphql-holochain`**](modules/vf-graphql-holochain) is one layer deeper, at the level of GraphQL schemas and resolvers. It contains helper methods which wraps the underlying Holochain 'cell' connections exposed by the [conductor API](https://www.npmjs.com/package/@holochain/conductor-api) with the [ValueFlows GraphQL schema](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql) and allows for combining and remixing collaboration spaces.

**You should develop against this interface if:**

- You are a UI app developer implementing a multi-agent UI (Holochain or otherwise) for a multi-stakeholder environment where hREA modules are complexly arranged across collaboration spaces. For example, two organisations with separate internal event logs (*observation* DNAs) collaborating in a shared *planning* DNA where some people might be members of both organisations.

For more information on usage and options, see the [GraphQL Client](modules/graphql-client) and [GraphQL Schema](modules/vf-graphql-holochain) module directories.


### DNA modules (outer Holochain layer)

There are a few sets of `*.yaml` configuration files used by Holochain in its build processes.

[**`bundles/`**](bundles/) contains configuration files for:

- `dna_templates`s which group assemblages of "[zomes](#zome-modules-inner-holochain-layer)" (compiled WASM files) into Holochain DNAs.
- `app_templates`s which group Holochain 'DNA' modules into 'hApp bundles'. A *hApp bundle* contains all backend components accessible by a single UI application; and
- `web-app`s which bind a 'hApp bundle' with a (zipped) JavaScript single-page web application that talks to the Holochain backend.

These bundles are used by the project scripts to run the application locally from this repository, and to build for release. The `*_templates` are first copied to non-`_template` locations and some substitutions made- see `scripts/package-dnas.sh`. In development mode, zome WASMs are simple path references; in release mode everything is bundled together into a much larger file.

If you aren't developing hREA yourself the bundled release is a much easier way to setup the app&mdash; simply download the `*.webhapp` file from the [releases page](https://github.com/h-rea/hrea/releases) and open it with the [Holochain Launcher](https://github.com/holochain/launcher).

DNAs are the highest-level units of functionality available in the system.

The architecture of hREA is designed to be as flexible as possible between components. We aim to separate the overall hREA app 'suite' into sensible logical services which allow for composition and pluggability. For example, you might swap an external project management tool for the `planning` DNA; or bring in your own agreement handling functionality.

The configurations in this directory are for "standard" module deployments where the combination of functionality and separation of concerns follows established patterns in ValueFlows networks. You can use them as guides for creating configurations of your own.

**You should customise at this layer if:**

- You wish to bundle hREA modules differently- for example, including them adjacent to other third-party zomes which extend their behaviour; or keeping *observation* and *planning* records within the same shared DHT space.
- You wish to include components of hREA as embedded logic within bespoke context-specific DNAs.

Note that as Holochain matures it is likely that this kind of configuration will be wrapped up into various administration interfaces, or even hREA-specific orchestration wizards.


### Zome modules (inner Holochain layer)

[**`zomes/`**](zomes/) is where the majority of the application logic resides. Each zome implements a modular set of functionality needed to support a single feature, and comes with consistency guarantees about its internal operations. In technical terms they are DHT storage + app logic + shared data validations + node-to-node messaging. These are implemented as Rust crates.

There are some simple registration zomes which only exist to link to remote Cargo dependencies.

**`rea_*` zomes** implement the core ValueFlows model. Each directory is typically broken down into at least five modules. From the outside inward, they are:

#### 1. Interface struct crates (Rust interface)

**`rea_*/rpc/`** defines the structs needed to de/serialize data at the WASM API boundary for this zome.

**You should develop against this interface if:**

- You are developing higher-order or companion zomes designed to work with hREA.
- You are building Rust client applications on top of hREA.

#### 2. Coordinator Zome crates (WASM interface)

**`rea_*/zome/`** defines the WASM externs (callable Zome functions) for building each module in its default configuration. Being a ["Coordinator Zome"](https://docs.rs/hdk/latest/hdk/#coordinator-zomes-), this code can be changed without the consequence of causing a necessity to migrate active user source chains.

Third-party code using the [interface struct crates](#1-interface-struct-crates-rust-interface) is calling through the APIs exposed by `#[hdk_extern]` methods in these modules in the same way that hREA modules communicate with each other.

**You should create your own customised zome definitions if:**

- You wish to combine multiple separate hREA zomes in the same DNA and isolate the storage into different `entry_def` types.
- You wish to define private variants of ValueFlows record types or customise sharding and validation rules.
- You wish to add handling of bespoke organisational logic and related records that needs to be validated tightly against REA data or coordinated as a coherent unit of information.

#### 3. Library crates (system core)

**`rea_*/lib/`** contains the bulk of the logic driving each ValueFlows record type. This layer is considered a "black box" to any outside systems, and forms the outermost guarantees of consistency provided by hREA.

Building against these APIs is the method by which one may create custom zome crates (as above). Methods for managing records most often follow the pattern `pub fn handle_{create|get|update|delete}_rec(...)`, where `rec` is the type of record.

You should consider the public API of these crates as the boundary of the REA system. Customisation of internal storage and link handling logic could lead to inconsistent database states and erroneous interpretations of ValueFlows records.

#### 4. Integrity Zome crates (WASM interface)

**`rea_*/integrity_zome/`** defines the WASM externs which are not Zome functions, but are technically required by Holochain to introspect the internal data structures (links types and entry types) of the Zome. Being an ["Integrity Zome"](https://docs.rs/hdk/latest/hdk/#integrity-zomes-), changing this code, or any of the code or crates that it depends on (notably Storage crates) will necessitate migrating any active user source chains.

#### 5. Storage crates (database layer)

**`rea_*/storage`** crates define the data structures and logic used with the low-level [HDI (Holochain Data Integrity) functions](https://docs.rs/hdi/latest/hdi/). This includes DNA properties, struct definitions and data validation logic. This crate will be imported both in Coordinator Zomes and also Integrity Zomes, but of those Integrity Zomes are most important, as any changes to this code will cause breaking changes to active user source chains, and require migrations. For this reason this code should be kept as minimal as possible and with as few dependencies as possible.

Each module exports an `EntryData` for the record information of relevance, and an `EntryStorage` which wraps this struct with standardised identifiers used to manage links between record updates.

In cases where records have standard CRUD features, `EntryData` is convertible `From<CreateRequest>` in its associated [interface struct crate](#1-interface-struct-crates-rust-interface); and implements `Updateable<UpdateRequest>` from the [`hdk_records`](#hdk_records) library. These traits are used by [library crates](#3-library-crates-system-core) to handle the underlying storage logic.

It is unlikely that there should be a need to create customised versions of these files. For maintenance reasons it is much better to compose additional fields and functionality onto the REA record types as *new* `entry_defs` in zome crates if adding additional fields is a requirement for your use-case.

#### 6. Storage constants (database internals)

**`rea_*/storage_consts/`** are simple includes which provide the string constants used to identify and link records using the ValueFlows vocabulary. They are provided as separate crates for consistency, since multiple zomes often manage links to corresponding entries at either side of a network boundary.


### Library modules

The Rust crates in [**`lib/`**](lib/) provide some abstract functionality and type definitions used across the hREA suite. Of particular note:

#### `hdk_records`

Manages CRUD and indexing operations for entries, including DNA-local and remote-DNA indexing capabilities. Leverages [DNA Auth Resolver](https://github.com/holochain-open-dev/dna-auth-resolver/) to grant capabilities for cross-DNA API calls.

#### `hdk_uuid_types`

Exports an `addressable_identifier!()` macro which wraps a primitive type implementing `Into<AnyDhtHash>` in a struct scoping it to a `DnaHash`; as well as `dna_scoped_string!()` which does the same for `String`.

These types are used as identifiers throughout all hREA record fields, allowing records to complexly reference each other in many:many relationships in different networks.

See the `vf_attributes_hdk` crate to see these macros in action.

#### `serde_maybe_undefined`

A helper type for record fields which acts similarly to `Option<T>` except that an explicit `null` is differentiated from omission of a field.

This provides an external API which is consistent with common developer expectations in the JavaScript community, where `null` is used to remove fields in update operations whilst omission indicates leaving a value unchanged.
