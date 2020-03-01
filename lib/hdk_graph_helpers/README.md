# Graph-like storage abstractions for the Holochain development kit

<!-- MarkdownTOC -->

- [Context](#context)
- [Theory](#theory)
- [Implementation](#implementation)
- [Status](#status)
- [License](#license)

<!-- /MarkdownTOC -->

## Context

**The short story:** in lots of distributed systems, graph architectures are highly favourable for a variety of reasons not worth detailing here. This is especially true of the highly referential dataset represented by ValueFlows; likely to due its roots as a semantic web ontology.

*Holochain is not a graph database.* Holochain hashchains and DHTs are managed via eventually-consistent [Entity-Attribute-Value](https://en.wikipedia.org/wiki/Entity%E2%80%93attribute%E2%80%93value_model) and [Content-Addressable-Store](https://en.wikipedia.org/wiki/Content-addressable_storage) abstractions. These are lower-level primitives that can be *combined* to create the fundamental architecture of a graph database, documented-oriented database, relational database, tuple store and probably many other patterns besides.

This library attempts to implement the simplest possible functional utility methods for running graph databases on Holochain. This includes non-native features like stable IDs (Holochain entry addresses mutate as the entry is updated) and inter-network linking architecture&mdash; all of which are necessary to describe interconnected graphs of data within and in-between fractally composable, disparate Holochain DNAs and zomes.



## Theory

The conceptual model of Holochain's storage engine is as follows:

- The available storage primitives are **entries** (CAS) and **links** (EAV).
- **entries** are identified by **addresses**.
- **links** are indexed via the **origin address** they link from, plus identifying **link types** and **link tags**. Link targets are referred to as **destination addresses**.
- Links provide a means of traversing the DHT graph. They can be queried from an **origin address** and filtered (via regex) by **link type** and **link tag** to determine the matching **destination addresses**.

On top of these primitives, `hdk_graph_helpers` provides this additional managed functionality:

**1.** Simple index types for identifying **entries** uniquely:

- **key indexes** are the most commonly used form of index. The data structures underpinning them enforce a separation between the actual entry content and its address, such that the address remains consistent even after updating. This is important for cross-DNA links, where shifting entry addresses make it harder to reason about remote entry identity. You can think of these like UUID primary keys in traditional database systems.
- **anchor indexes** are another form of index that links an identifier to an entry. These are uni-directional links where the entry stored at the anchoring address contains well-known content that can be used to easily determine a starting address to read from. You can think of these like unique keys in traditional database systems.

**2.** More complex index types that link *between* entries:

- **direct indexes** are composed of **key indexes** and **links** alone. In most use-cases these indexes are bidirectional and so are composed of two underlying **link** primitives that link between the **key index** entries of 2 related records. **direct indexes** are used where no additional metadata about the linkage between two **entries** is required.
- **indirect indexes** are composed of "joining" **entries** with **links** "between the seams". Essentially this creates compound keys which can be retrieved via their own ID by requesting the "joining" **entry** content. **indirect indexes** are used where additional metadata about the linkage is required: the "joining" **entry** contains fields referencing the linked **entries**, as well as additional fields describing the relationship. Note that it is also possible to link more than 2 **entries** together using this method by having 3 or more reference fields in the "joining" **entry**.
	- TODO: Indirect indexes have yet to be formalised. See `fulfillment` and `satisfaction` zomes for an understanding of the necessary behaviours.

**3.** Index types for linking between entries in foreign networks:

- **direct remote indexes** are also composed of **links** and **key indexes**, however **key indexes** on either side of the network boundary are left "dangling"- they do no have any underlying **entry** data kept locally.
	- In the origin DNA, the **key indexes** of the destination entry IDs dangle: they refer to records in the external DNA.
	- In the destination DNA, the **key index** of the origin entry ID dangles: it refers to the external entry linking in to the host network.
- **indirect remote indexes** are as above, but the "joining" entry is replicated in both networks in order that the linking context is readable by parties from either network who may only have access to data on their side of the membrane.
	- TODO: Indirect remote indexes should likely be developed as an [XDI link contract registration mixin zome](https://github.com/holo-rea/ecosystem/wiki/Modules-in-the-HoloREA-framework#links).

**4.** These indexing features together provide us with our ultimate graph-like abstraction:

- **records** are composed of sets of related **indexes** and **entries** which are reassembled at query time into a complete structure for representation to the world outside the zome API membrane.


## Implementation

These abstractions, particularly in regard to standard CRUD actions, require some additional logic and plumbing in order to facilitate an ergonomic and error-free development experience.

- Handling of undefined values in API calls is implemented with [Serde](https://serde.rs/) macros and a custom `MaybeUndefined` type. This provides for a standard request logic often encountered in JavaScript applications:
	- Omitting a field uses a default value when initialising a **record** (often `None`).
	- Omitting a field preserves the original value in an update operation.
	- In a create operation, assigning a field to `null` sets an initial value of `None` if there is no default or the default is some other value.
	- In an update operation, assigning a field to `null` explicitly erases the value.
	- Providing other values for fields either initialises them or updates them with the value provided.
- The rest of the API is split into areas of function:
	- `hdk_graph_helpers::links` contains methods for managing **indexes** between **entries**.
	- `hdk_graph_helpers::rpc` contains methods for managing communication between networks. This includes **remote index** functionality as well as general-purpose utilities for requesting and parsing **records** stored in other DNAs.
	- `hdk_graph_helpers::records` contains methods for managing CRUD operations for **entry** data.
		- `hdk_graph_helpers::record_interface` can be implemented for custom update operations where modification to one type of **record** effects data held in another (to view an example, see `/lib/vf_observation/src/economic_resource.rs` in this repository).

The goal is for the CRUD behaviours and other common logic to [eventually be wrapped up](https://github.com/holo-rea/holo-rea/issues/22) into proc macros in order to avoid the repetition and room for user error that is currently present in the WIP implementation.


## Status

This is currently an experiment and work in progress. There are [alternative architectural patterns to explore](https://github.com/holo-rea/holo-rea/issues/60) and we are aiming for a code review with the Holochain core & app developers before landing on a final methodology.

As such, all Holochain apps building on this library should only perform integration tests against the external zome API gateway, since it will remain a stable part of your system whilst the internals of the graph logic are in flux.


## License

Licensed under an Apache 2.0 license.
