# hREA: The Holochain/REA coordination framework

A suite of functionally independent building blocks affording most functionality commonly used in supply chain systems, project management software, logistics management and enterprise resource planning; as well as post-capitalist economic paradigms such as gift and contributory economies.

These and [other components](https://github.com/holochain-open-dev/ecosystem/issues?q=) can be used to create distributed social, economic & resource coordination applications of any kind; by plugging together [Holochain](https://holochain.org/) "zomes" (units of functionality) and "[DNA modules](https://developer.holochain.org/docs/concepts/2_application_architecture/)" (fully decentralised, agent-centric, networked application microservices).

<!-- MarkdownTOC -->

- [About](#about)
- [Documentation](#documentation)
- [hREA beyond Holochain](#hrea-beyond-holochain)
- [Repository structure](#repository-structure)
	- [Application layer](#application-layer)
	- [ValueFlows GraphQL \(protocol layer\)](#valueflows-graphql-protocol-layer)
	- [GraphQL interface \(JavaScript interface\)](#graphql-interface-javascript-interface)
	- [DNA modules \(outer Holochain layer\)](#dna-modules-outer-holochain-layer)
	- [Zome modules \(inner Holochain layer\)](#zome-modules-inner-holochain-layer)
		- [1. Interface struct crates \(Rust interface\)](#1-interface-struct-crates-rust-interface)
		- [2. Zome crates \(Holochain interface\)](#2-zome-crates-holochain-interface)
		- [3. Library crates \(hApp logic\)](#3-library-crates-happ-logic)
		- [4. Storage crates \(database layer\)](#4-storage-crates-database-layer)
		- [Storage constants \(database internals\)](#storage-constants-database-internals)
	- [Library modules](#library-modules)
		- [`hdk_records`](#hdk_records)
		- [`hdk_type_serialization_macros`](#hdk_type_serialization_macros)
		- [`serde_maybe_undefined`](#serde_maybe_undefined)
- [License](#license)

<!-- /MarkdownTOC -->






## About

What do we mean by "most functionality"?-

- **group management**: manage groups of collaborators and permission access between groups, sub-projects and across organisations
- **event ledger**: use the *observation* module to track the observed movements of resources, currencies and skills in any coordination space
- **coordination functions**: use *planning* modules to decide on future plans, manage agreements or coordinate actions with other participants
- **needs matching**: use *proposal* modules to group matched outcomes in order to describe bilateral and multilateral trade requests
- **knowledge sharing**: use the *recipe* module to share structured production knowledge and easily repeat well-understood processes

For a more detailed list of modules and features planned for development, see [modules in the hREA framework](https://github.com/holo-rea/ecosystem/wiki/Modules-in-the-HoloREA-framework).

A key aspect to note about these modules is that *they require no technical knowledge to remix or re-architect into different social organising patterns*. Different arrangements of network components can be used to address different use-cases in novel ways.

Most people making use of hREA will never have to delve into the software beyond this level. All modules in the suite have established APIs for interoperability and can be arranged into complex organisational patterns at runtime, like lego blocks. To see some concrete examples of this, see the scenarios orchestrated in [test/social-architectures](test/social-architectures).

Beyond this outer layer the system has been designed with flexibility, modularity and composability as core architectural concerns. The depth to which you will delve into the architecture depends on a project's needs; i.e. how much customisation is required. See [Repository Structure](#repository-structure) for a breakdown of how hREA fits together, how to customise it, and how to browse this repository.






## Documentation

**High-level documentation** for integrators, potential collaborators and entrepreneurs can be found in the project's [ecosystem wiki](https://github.com/holo-rea/ecosystem/wiki/). This includes information on hREA's organisational goals, strategic mission, design philosophy, cultural background and ideological positioning.

**Developer documentation** can be found in the [`docs/`](docs/README.md) directory. We keep it within the codebase instead of in the wiki so that all contributors retain the information necessary to understand, configure and run the system.

There is a [quick start guide](docs/README.md#quick-start) for those who want to spin up hREA locally for development or experimentation.






## hREA beyond Holochain

hREA is built to implement the [ValueFlows protocol](https://valueflo.ws/)&mdash; a set of common vocabularies based on [REA Accounting theory](https://en.wikipedia.org/wiki/Resources,_events,_agents_(accounting_model)) to describe flows of economic resources of all kinds within distributed economic ecosystems.

By building to align with the [ValueFlows GraphQL spec](#valueflows-graphql-protocol-layer), UI applications built for hREA are automatically compatible with other ValueFlows-compatible system backends like our partner project [Bonfire](https://bonfirenetworks.org/).

The goal is to enable radical code reuse and cross-project interoperability between next-gen distributed backend systems and traditional web infrastructure, and to allow user interfaces to span multiple disparate apps.






## Repository structure

This section details the layers of the system and where to find each set of components within this repository.


### Application layer

- [**`apps/`**](apps/) contains end-user applications built on the HoloREA framework.
	- [**`apps/holorea-graphql-explorer/`**](apps/holorea-graphql-explorer/) is a [GraphiQL](https://github.com/graphql/graphiql) interface to the system with some added [additions to assist with comprehension](https://github.com/OneGraph/graphiql-explorer-example). Wired up to the development DNAs by default&mdash; super handy for testing and getting to know the ValueFlows [data structure](https://github.com/valueflows/vf-graphql/).
- [**`test/`**](test/) contains integration tests for the application suite as a whole. Connections to the [GraphQL Interface](#graphql-interface-outer-layer) and Holochain application cells are managed in `init.js`.



### ValueFlows GraphQL (protocol layer)

The [ValueFlows GraphQL spec](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql) is an effort towards composable application architectures for distributed socioeconomic coordination applications.

Though it exists outside of this repository and is co-governed by many stakeholders, it is worth mentioning here that ValueFlows GraphQL and the core [ValueFlows RDF vocabulary](https://lab.allmende.io/valueflows/valueflows/) are open protocols which accept proposals for improvement. Contributions to the domain model from non-technical authors are welcomed and encouraged.

If you are implementing systems which you'd like to be compatible with our interfaces and client applications, the [ValueFlows GraphQL NodeJS module](https://www.npmjs.com/package/@valueflows/vf-graphql) can be leveraged to build and validate implementations, and to export raw schema formats for injection into other software.



### GraphQL interface (JavaScript interface)

[**`modules/vf-graphql-holochain`**](modules/vf-graphql-holochain) contains a NodeJS module which wraps the underlying Holochain 'cell' connections exposed by the [conductor API](https://www.npmjs.com/package/@holochain/conductor-api) with the [ValueFlows GraphQL schema](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql) to provide a simplified, coherent entrypoint to the system.

**You should develop against this interface if:**

- You are a UI app developer creating an application to run on top of standard hREA module functionality.
- You are a UI app developer extending hREA's functionality with custom business or domain-specific integrations (including web platforms, blockchain tech and other Holochain modules).
- You are a creator of distributed socioeconomic coordination applications and you would like your apps to run on hREA, Bonfire, [SSB-ValueFlows](https://github.com/open-app/economic-sentences-graphql) or other protocol-compatible storage backends.

Note that there is a [mock GraphQL client](https://www.npmjs.com/package/@vf-ui/graphql-client-mock) available for building ValueFlows apps against which does not require Holochain to be configured.

For more information on usage, see the module's readme.


### DNA modules (outer Holochain layer)

[**`happs/`**](happs/) contains a set of `*.yaml` files which configure assemblages of "[zomes](#zome-modules-inner-holochain-layer)" into Holochain DNAs.

DNAs are the highest-level units of functionality available in the system. One is available for each of the [modules in the hREA framework](https://github.com/holo-rea/ecosystem/wiki/Modules-in-the-HoloREA-framework). 

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

#### 2. Zome crates (Holochain interface)

**`rea_*/zome/`** defines the WASM externs and source chain data structures for building each module in its default configuration.

Third-party code using the [interface struct crates](#1-interface-struct-crates-rust-interface) is calling through the APIs exposed by `#[hdk_extern]` methods in these modules in the same way that hREA modules communicate with each other.

**You should create your own customised zome definitions if:**

- You wish to combine multiple separate hREA zomes in the same DNA and isolate the storage into different `entry_def` types.
- You wish to define private variants of ValueFlows record types or customise sharding and validation rules.
- You wish to add handling of bespoke organisational logic and related records that needs to be validated tightly against REA data or coordinated as a coherent unit of information.
	- [**`example/`**](example/) contains contrived implementations for particular use-cases and domain-specific applications which may be helpful in getting started with some of the more common advanced ValueFlows extensions.

#### 3. Library crates (hApp logic)

**`rea_*/lib/`** contains the bulk of the logic driving each ValueFlows record type. This layer is considered a "black box" to any outside systems, and forms the outermost guarantees of consistency provided by hREA.

You should consider the public API of these crates as the boundary of the REA system. Customisation of internal storage and link handling logic could lead to inconsistent database states and erroneous interpretations of ValueFlows records.

#### 4. Storage crates (database layer)

**`rea_*/storage`** crates define the data structures and logic used with the low-level HDK (Holochain Development Kit) functions. This includes DNA properties, struct definitions and data validation logic.

Each module exports an `EntryData` for the record information of relevance, and an `EntryStorage` which wraps this struct with standardised identifiers used to manage links between record updates.

In cases where records have standard CRUD features, `EntryData` is convertible `From<CreateRequest>` in its associated [interface struct crate](#1-interface-struct-crates-rust-interface); and implements `Updateable<UpdateRequest>` from the [`hdk_records`](#hdk_records) library. These traits are used by [library crates](#3-library-crates-happ-logic) to handle the underlying storage logic.

It is unlikely that there should be a need to create customised versions of these files. For maintenance reasons it is much better to compose additional fields and functionality onto the REA record types as *new* `entry_defs` in zome crates if adding additional fields is a requirement for your use-case.

#### Storage constants (database internals)

**`rea_*/storage_consts/`** are simple includes which provide the string constants used to identify and link records using the ValueFlows vocabulary. They are provided as separate crates for consistency, since multiple zomes often manage links to corresponding entries at either side of a network boundary.


### Library modules

The Rust crates in [**`lib/`**](lib/) provide some abstract functionality and type definitions used across the hREA suite. Of particular note:

#### `hdk_records`

Manages CRUD and indexing operations for entries, including DNA-local and remote-DNA indexing capabilities. Leverages [DNA Auth Resolver](https://github.com/holochain-open-dev/dna-auth-resolver/) to grant capabilities for cross-DNA API calls.

#### `hdk_type_serialization_macros`

Exports an `addressable_identifier!()` macro which wraps a primitive type implementing `Into<AnyDhtHash>` in a struct scoping it to a `DnaHash`; as well as `dna_scoped_string!()` which does the same for `String`.

These types are used as identifiers throughout all hREA record fields, allowing records to complexly reference each other in many:many relationships in different networks.

See the `vf_attributes_hdk` crate to see these macros in action.

#### `serde_maybe_undefined`

A helper type for record fields which acts similarly to `Option<T>` except that an explicit `null` is differentiated from omission of a field.

This provides an external API which is consistent with common developer expectations in the JavaScript community, where `null` is used to remove fields in update operations whilst omission indicates leaving a value unchanged.






## License

Licensed under an Apache 2.0 license.
