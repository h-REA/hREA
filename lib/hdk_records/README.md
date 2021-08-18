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

Currently there are three distinct index types available, which the application developer must reason about when implementing. Generally, the pattern is to create/update/delete records first and continue with index updates afterward.

*Local* indexes create simple bidirectional links between two records in the same zome. It is expected that the `EntryHash` of both records exists, or these methods will error.

See `local_index_helpers.rs`.

### Remote record indexing

*Remote* indexes create bidirectional links between two records in different *DNAs*. In these cases, some wiring is necessary at the zome and DNA API layers.

The destination side of the index must have a [DNA Auth Resolver zome](https://github.com/holochain-open-dev/dna-auth-resolver/) present in its manifest.

The auth resolver must be configured with a permission ID bound to a zome API method which triggers the link update. For example, for "commitment" to trigger an update in the destination zome for "process":

```yaml
manifest_version: "1"
# ...
properties:
  remote_auth:
    permissions:
      - extern_id: index_process_input_commitments
        allowed_method: [process, index_input_commitments]
zomes:
  # ...
  - name: process
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_rea_process.wasm"
  - name: remote_auth
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_dna_auth_resolver.wasm"
```

```rust
const COMMITMENT_ENTRY_TYPE: &str = "vf_commitment";
const COMMITMENT_INPUT_OF_LINK_TAG: &str = "input_of";
const PROCESS_ENTRY_TYPE: &str = "vf_process";
const PROCESS_COMMITMENT_INPUTS_LINK_TAG: &str = "inputs";

use vf_attributes_hdk::{CommitmentAddress, ProcessAddress};
use hdk_records::{
    remote_indexes::{
        RemoteEntryLinkRequest,
        RemoteEntryLinkResponse,
        sync_remote_index,
    },
};

#[hdk_extern]
fn index_input_commitments(indexes: RemoteEntryLinkRequest<CommitmentAddress, ProcessAddress>) -> ExternResult<RemoteEntryLinkResponse> {
    let RemoteEntryLinkRequest { remote_entry, target_entries, removed_entries } = indexes;

    Ok(sync_remote_index(
        &COMMITMENT_ENTRY_TYPE, &remote_entry,
        &PROCESS_ENTRY_TYPE,
        target_entries.as_slice(),
        removed_entries.as_slice(),
        &COMMITMENT_INPUT_OF_LINK_TAG, &PROCESS_COMMITMENT_INPUTS_LINK_TAG,
    )?)
}
```

Once this is done, the destination zome is ready to handle index updates from remote networks. When calling the remote index helpers in the *origin* zome, the `remote_permission_id` provided to these methods must match the `extern_id` configured for the target DNA (in this example, `index_process_input_commitments`).

See `remote_index_helpers.rs`.

### Foreign record indexing

*Foreign* indexes are very similar to *remote* indexes, except that they link between records in different zomes within the *same* DNA. The only differences are in the toplevel logic for how connections are made- in the case of *foreign* indexes, configuration is made in the DNA manifest and the remote endpoint is accessed via `zome_name_from_config` and `foreign_fn_name`.

It is expected that foreign indexes are a temporary measure that will be deprecated once coherent APIs emerge from https://github.com/holochain/holochain/issues/743 and https://github.com/holochain/holochain/issues/563.

See `foreign_index_helpers.rs`.

### Inter-zome RPC

The lower-level RPC methods underpinning remote and foreign indexing logic are also useful abstractions for general-purpose communication between zomes and DNAs.

Some advanced uses of this include implementing record types which behave like "compound indexes" in an RDBMS, by having the origin DNA replicate shadowed records to its destination DNA and storing indexes at either side of the relationship. For an example of this, see the *satisfaction* and *fulfillment* zomes in the [hREA codebase](https://github.com/holo-rea/holo-rea/).

See `rpc_helpers.rs`.




## Status

This is currently an experiment and work in progress. There are [alternative architectural patterns to explore](https://github.com/holo-rea/holo-rea/issues/60) and we are aiming for a code review with the Holochain core & app developers before landing on a final methodology.

As such, all Holochain apps building on this library should only perform integration tests against the external zome API gateway, since it will remain a stable part of your system whilst the internals of the graph logic are in flux.


## License

Licensed under an Apache 2.0 license.
