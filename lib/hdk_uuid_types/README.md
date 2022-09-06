# HDK Type Serialization Macros

This module provides macros for wrapping "raw" (DNA-local) `EntryHash`, `ActionHash`, `AgentPubKey`, `String` and other identifiers with an associated `DnaHash`, which is a necessity in implementing Holochain applications which span multiple DNAs in complex ways.

<!-- MarkdownTOC -->

- [Usage](#usage)
- [Status](#status)
	- [To-do](#to-do)
- [License](#license)

<!-- /MarkdownTOC -->


## Usage

```rust
use hdk_uuid_types::*;

// "Newtype struct" pattern, wraps values in different types to enforce compile-time distinctness.
// To access the raw wrapped value, use `.as_ref()`.
simple_alias!(AnyURL => String);
simple_alias!(XMLNamespaceURI => String);

// generate DNA-scoped identifiers which point to `AnyDhtHash` values within the DNA
addressable_identifier!(PostId => EntryHash);
addressable_identifier!(AuthorId => AgentPubKey);

// generate DNA-scoped identifier which wraps a `String` to uniquely identify it with the local DNA
dna_scoped_string!(AuthorAlias);
```

## Status

This is currently an experiment and work in progress. There are [alternative architectural patterns to explore](https://github.com/h-REA/hREA/issues/60) and we are aiming for a code review with the Holochain core & app developers before landing on a final methodology.

As such, all Holochain apps building on this library should only perform integration tests against their external zome API gateway, since it will remain a stable part of your system whilst the internals of the graph logic are in flux.


### To-do

- Experiment with more compact, less ambiguous packed-byte format for representing DNA-scoped identifiers. Discuss the possibility of a new byte prefix with HC Core team.
- Experiment with use of Base64 string identifiers & [a native URI format](https://github.com/h-REA/hREA/issues/49) throughout. Pending decisions by HC Core team as to whether to provide Base64 encoding features within the host.


## License

Licensed under an Apache 2.0 license.
