# `hdk_records`

> Graph-like record storage abstractions for the Holochain development kit (HDK).

<!-- MarkdownTOC -->

- [Context](#context)
- [Theory & Usage](#theory--usage)
	- [Time-ordered updates with conflict resolution](#time-ordered-updates-with-conflict-resolution)
	- [User-defined identifiers](#user-defined-identifiers)
	- [Record indexing](#record-indexing)
	- [Remote record indexing](#remote-record-indexing)
	- [Foreign record indexing](#foreign-record-indexing)
	- [Inter-zome RPC](#inter-zome-rpc)
- [Status](#status)
- [License](#license)

<!-- /MarkdownTOC -->

This crate exports a suite of functions and traits useful for managing entries and links in multiple Holochain DHTs similarly to records and edges in a centralised graph database.

## Context

**The short story:** in many modern large-scale data architectures and distributed systems, graph architectures are highly favourable for a variety of reasons not worth detailing here.

*Holochain is not a graph database.* Holochain hashchains and DHTs are managed via eventually-consistent [Entity-Attribute-Value](https://en.wikipedia.org/wiki/Entity%E2%80%93attribute%E2%80%93value_model) and [Content-Addressable-Store](https://en.wikipedia.org/wiki/Content-addressable_storage) abstractions. These are lower-level primitives that can be *combined* to create the fundamental architecture of a graph database, documented-oriented database, relational database, tuple store and many other patterns besides.

This library attempts to implement the simplest possible functional utility methods for running graph databases on Holochain. This includes non-native features like stable IDs (Holochain entry addresses mutate as the entry is updated) and inter-network linking architecture&mdash; all of which are necessary to describe interconnected graphs of data within and in-between fractally composable, disparate Holochain DNAs and zomes.



## Theory & Usage

The conceptual model of Holochain's storage engine is as follows:

- The available storage primitives are **entries** (CAS) and **links** (EAV).
- **entries** are identified by **addresses**.
- **links** are indexed via the **origin address** they link from, plus identifying **link types** and **link tags**. Link targets are referred to as **destination addresses**.
- Links provide a means of traversing the DHT graph. They can be queried from an **origin address** and filtered by **link type** and **link tag** to determine the matching **destination addresses**.

On top of these primitives, `hdk_records` provides this additional managed functionality:

### Time-ordered updates with conflict resolution

Efficient index types for identifying chains of updating **entries** uniquely as single records which evolve over time.

See `crate::record_interface::Identified` and the `generate_record_entry!` macro.

### User-defined identifiers

Static indexing for "pinning" records to well-known IDs rather than GUIDs.

See `crate::record_interface::UniquelyIdentifiable` and `crate::record_interface::UpdateableIdentifier`.

### Record indexing

These helpers store record data in a format that is compatible with the [`hdk_semantic_indexes`](../hdk_semantic_indexes) library crates, which can be used to manage semantically meaningful relationships between records. See the readme for these modules for more information.

### Inter-zome RPC

The lower-level RPC methods underpinning remote and foreign indexing logic are also useful abstractions for general-purpose communication between zomes and DNAs.

Some advanced uses of this include implementing record types which behave like "compound indexes" in an RDBMS, by having the origin DNA replicate shadowed records to its destination DNA and storing indexes at either side of the relationship. For an example of this, see the *satisfaction* and *fulfillment* zomes in the [hREA codebase](https://github.com/holo-rea/holo-rea/).

See `rpc_helpers.rs`.




## Status

This is currently an experiment and work in progress. There are [alternative architectural patterns to explore](https://github.com/holo-rea/holo-rea/issues/60) and we are aiming for a code review with the Holochain core & app developers before landing on a final methodology.

As such, all Holochain apps building on this library should only perform integration tests against the external zome API gateway, since it will remain a stable part of your system whilst the internals of the graph logic are in flux.


## License

Licensed under an Apache 2.0 license.
