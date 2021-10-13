# (Simple) Semantic Indexing for Holochain apps

<!-- MarkdownTOC -->

- [High-level architecture](#high-level-architecture)
- [Usage](#usage)
	- [Defining an index](#defining-an-index)
	- [Managing an index](#managing-an-index)
	- ["Local" vs "Remote" indexes](#local-vs-remote-indexes)
	- [A word on `DnaAddressable` identifiers](#a-word-on-dnaaddressable-identifiers)
- [Status](#status)
	- [To-do](#to-do)
- [License](#license)

<!-- /MarkdownTOC -->

These crates provide functionality for quickly implementing specific, semantically-meaningful relationships between records; and performing queries against those relationships.

## High-level architecture

A "semantically meaningful" relationship is simply one which has been determined to have some meaning in your application's use-case. A simple example might be something like a multi-user blog, where `Posts` are `authored by` one or more `Writers` - and, conversely, `Writers` may have `contributed to` some `Posts`. An important thing to note about these relationships is that they are bi-directional. Therefore in most cases, updating a link between two records involves updating two different indexes.

In the architecture offered by these modules, *indexes live in their own zomes*. This enables different index types to be "pluggable", and for indexes to be easily swapped out for equivalent functionality as needed. Affordances also exist for index zomes to be pluggable alongside other indexing modules via higher-order "coordination" zomes; for example combining fulltext record searching with domain-specific semantic queries based on record relationships&mdash; all achievable with zero code changes.

As such, there are four crates comprising this module in its completeness:

- `hdk_semantic_indexes_zome_lib` contains code used *within each index zome*. Mostly you will not need to use these helpers directly, since-
- `hdk_semantic_indexes_zome_derive` offers a Rust proc macro which hides away all the boilerplate needed when defining an index zome.
- The "client" side of the logic runs within your host application zome, and uses the helpers within the `hdk_semantic_indexes_client_lib` module to trigger updates in the referenced index zomes.
- `hdk_semantic_indexes_zome_rpc` defines the interface structs needed for *client zomes* to communicate with companion *index zomes*.

## Usage

### Defining an index

You will need to declare two zome crates- one for each side of the index. In addition to these zome crates you also need to define some identifier types implementing `hdk_type_serialization_macros::DnaAddressable` and map them to a `QueryParams` struct which forms the external API.

In the example above, this might look as follows:

```rust
use hdk_semantic_indexes_zome_derive::index_zome;

//-- usually, you would define these shared identifier types in another crate
use hdk_type_serialization_macros::*;
addressable_identifier!(PostId => EntryHash);
//--

// This must be defined, and in scope.
// Usually this RPC structure would be declared in a separate crate so that consuming
// applications can import it to interact with the index zome's query API.
struct QueryParams {
	// all fields must be optional
	contributed_to: Option<PostId>,
}

// The same applies to whatever structure you define to represent each individual record in your system. 
// Ensure that a `ResponseData` is in scope.
use your_record_rpc_structs::{SomeAPIResponse as ResponseData};

#[index_zome]
struct Writer {
	contributed_to: Local<post, authored_by>,
}
```

```rust
use hdk_semantic_indexes_zome_derive::index_zome;

//-- usually, you would define these shared identifier types in another crate
use hdk_type_serialization_macros::*;
addressable_identifier!(AuthorId => AgentPubKey);
//--

struct QueryParams {
	authored_by: Option<AuthorId>,
}

#[index_zome]
struct Post {
	authored_by: Local<writer, contributed_to>,
}
```

In addition to this, you also need to associate the zomes in your DNA manifest so that they can communicate with each other.

```yaml
properties:
  posts:
    index_zome: posts_index
  posts_index:
    record_storage_zome: posts
```

The "client" zome (in this case, `posts`) **MUST** expose a method named `get_X` where X is the name of the record type. The parameters to this method must be defined as `hdk_semantic_indexes_zome_rpc::ByAddress`:

```rust
// ...somewhere in the "client" zome driving the indexing logic...

#[hdk_extern]
fn get_post(ByAddress { address }: ByAddress) -> ExternResult<PostData> {
    // ...logic to read a single post by its ID...
}
```

### Managing an index

In your "client" application zome (usually the same zome which manages record storage and CRUD operations), you can import the helper macros in `hdk_semantic_indexes_client_lib` to deal with updating the indexes.

For example, to create an indexed relationship between the author of a post and the post itself upon creation:

```rust
use hdk_semantic_indexes_client_lib::*;

// The structure of DnaConfigSlice would be application-specific
struct DnaConfigSlice {
	posts: PostZomeConfig,
}
struct PostZomeConfig {
	index_zome: String,
	writer_index_zome: String,
}

// Local indexes require appropriately formatted helper methods for accessing zome config to be in scope.
// The specific name of the index zome is decoupled via configuration (see 'Local vs Remote indexes', below).
// The format is simple: "read_{}_index_zome", substituting the record type as indicated in the macro call.
fn read_post_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.posts.index_zome)
}

fn read_writer_index_zome(conf: DnaConfigSlice) -> Option<String> {
    Some(conf.posts.writer_index_zome)
}

pub fn handle_post_authoring(/*...*/) -> ExternResult<SomeData> {
	// ...
	
	// Retrieve ID parameters for the index however is appropriate to your use-case.
	let agent_pubkey = DnaAddressable::new(zome_info()?.dna_hash, agent_info()?.agent_latest_pubkey);
	// :NOTE: simplified example presuming `entry_hash` from a previous write.
	// When using `hdk_records` methods, `DnaAddressable` identifiers are returned natively.
	let post_id = DnaAddressable::new(zome_info()?.dna_hash, entry_hash);

	// perform the index creation in related index zomes
	create_index!(Local(
		writer(&agent_pubkey).contributed_to ->	post(&post_id).authored_by
	))?;

	// ...
}
```


### "Local" vs "Remote" indexes

The zomes of "Local" index are both hosted in the local DNA, whereas in a "Remote" index one zome is hosted locally and the other is in a remote DNA.

Currently these are the two distinct index types available, which the application developer must reason about when implementing. Generally, the pattern is to create/update/delete records in the originating "client" zome first and continue with index updates afterward. You can always throw an error on any of the index update errors encountered to rollback the local record storage if your application deems it appropriate.

The distinction between "Local" and "Remote" necessitates some different lookup logic being needed to connect to the second zome comprising the index. When using the macros, you only need to replace "Local" with "Remote" to change where the second half of the index is located. (If using the helpers directly there are different methods prefixed with `_local_` and `_remote_`.)

You will, however, also need to update your DNA configuration.

"Local" indexes, which are to be driven by "client" zomes in the *same* DNA, should be configured for access via zome configuration attributes.

```yaml
manifest_version: "1"
properties:
  posts:
    # tells the `hc_zome_simple_posts` zome how to update the post 
    # side of the index, as per `read_post_index_zome`
    index_zome: posts_index
    # tells the `hc_zome_simple_posts` zome how to update the writer 
    # side of the index, as per `read_writer_index_zome`
    writer_index_zome: writers_index 
  posts_index:
    record_storage_zome: posts
  writers_index:
    record_storage_zome: writers
zomes:
  - name: posts
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_simple_posts.wasm"
  - name: posts_index
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_simple_posts_author_index.wasm"
  - name: writers
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_simple_authors.wasm"
  - name: writers_index
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_simple_authors_post_index.wasm"
```

"Remote" indexes, which are to be driven by "client" zomes in another DNA, should be configured for access via [DNA Auth Resolver](https://github.com/holochain-open-dev/dna-auth-resolver/). For example, if `Posts` were to be indexable by remote `Writers` from many networks:

```yaml
manifest_version: "1"
properties:
  post:
    # tells the `hc_zome_simple_posts` zome how to update the post 
    # side of the index, as per `read_post_index_zome`
    index_zome: post_index
  post_index:
    record_storage_zome: post
  remote_auth:
    permissions:
      # Exposes the `index_post_authored_by` API method (generated by #[index_zome] 
      # macro) via a permission of the same name.
      # 
      # The client zome macros use this method to connect if called with `Remote()` 
      # instead of `Local()`.
      - extern_id: index_post_authored_by
        allowed_method: [posts_index, index_post_authored_by]
zomes:
  - name: posts
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_simple_posts.wasm"
  - name: posts_index
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_simple_posts_author_index.wasm"
  - name: remote_auth
    bundled: "../../target/wasm32-unknown-unknown/release/hc_zome_dna_auth_resolver.wasm"
```

Note that you can do advanced things with the inclusion of multiple zomes. The `extern_id` that the client zome will call via is derived from parameters to the macros in `hdk_semantic_indexes_client_lib` - **this `extern_id` must match the format** `index_{}_{}` **, substituting the record type and relationship name given in the macro call**.

No other identifiers need match- in this example, the client zome need not have any awareness of the `posts_index` zome name since it is mapped transparently in the DNA configuration.




### A word on `DnaAddressable` identifiers

[`hdk_type_serialization_macros`](../hdk_type_serialization_macros) provides macros for wrapping "raw" (DNA-local) identifiers with an associated `DnaHash`, which makes them universally-unique between all cells in a running Holochain conductor.

Since **all indexes provided by this library manage many:many relationships between application cells** it is possible that links between records might reference foreign records in multiple different networks. Complicating this further, if UI applications are to be able to dynamically compose different network arrangements to create "agent-centric" views which interact with multiple communities simultaneously; then **the possibility exists for such multiple references to different networks to be created independently of the original design of each application**.

And so, all applications using these helpers should wrap values to include the `DnaHash` in all identifying information in order to maximise flexibility and future interoperability. This enables the appropriate cell to be referenced for *each individual* index operation, which is necessary if each individual relationship might refer to records kept in a different network space.

## Status

This is currently an experiment and work in progress. There are [alternative architectural patterns to explore](https://github.com/holo-rea/holo-rea/issues/60) and we are aiming for a code review with the Holochain core & app developers before landing on a final methodology.

As such, all Holochain apps building on this library should only perform integration tests against their external zome API gateway, since it will remain a stable part of your system whilst the internals of the graph logic are in flux.


### To-do

- Cleanup ergonomics- obviate need to provide particular scoped `QueryParams` / `ResponseData` etc symbols, pass them as arguments to the client & zome macros.
- Offer a macro which only updates a single DNA-local index, for indexes where only one side of the relationship is queryable.


## License

Licensed under an Apache 2.0 license.
