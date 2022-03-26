# hREA GraphQL schema binding

Binds Holochain cell connections for [hREA](https://github.com/holo-rea/holo-rea/) to the ValueFlows protocol spec, thus creating a pluggable and extensible [ValueFlows](http://valueflo.ws) implementation backed by multiple distributed & interconnected Holochain networks.

**Work in progress!**

<!-- MarkdownTOC -->

- [Usage](#usage)
	- [Required options](#required-options)
	- [Schema extension and additional service integration](#schema-extension-and-additional-service-integration)
	- [Partial schema generation](#partial-schema-generation)
	- [Multiple collaboration spaces](#multiple-collaboration-spaces)
	- [Direct access to resolver callbacks](#direct-access-to-resolver-callbacks)
- [Repository structure](#repository-structure)
- [Building and publishing to NPM](#building-and-publishing-to-npm)
- [License](#license)

<!-- /MarkdownTOC -->


## Usage

This module provides a runtime-configurable generator function which returns a GraphQL schema for any configured arrangement of hREA modules. You can think of the objects returned by this function as single "collaboration spaces", where groups of Holochain DNA modules with specific, coordinated purposes are arranged together to form one coherent logical "place" that people hang out in.

**1 instance of `bindSchema()` = 1 agent acting within 1 collaboration space**. It is important to remember this in an agent-centric environment, especially when composing multiple hREA collaboration spaces together to form "cross-membrane" interfaces.

It is expected that you will most often use this module in an [Apollo](https://apollographql.com/)-based application. In the simplest case, your app initialisation logic will probably look something like the `@vf-ui/graphql-client-holochain` module (also available in this repository), which wraps the `@valueflows/vf-graphql-holochain` schema in a `GraphQLClient` interface compatible with `@vf-ui/graphql-client-mock`.

In most cases, you should be able to use `@vf-ui/graphql-client-holochain` directly from NPM. This module is for advanced usage in client applications seeking to overlay and interweave collaboration spaces in specific and nuanced ways.

### Required options

- `conductorUri` specifies the websocket URI for connecting to the Holochain conductor for this set of DNA connections. Usually this is `ws://localhost:4001`. Note that you can create multiple instances connected to multiple `conductorUri`s in order to simulate disparate hosts in multi-agent testing or usage.
- `dnaConfig` takes a mapping of pre-specified string module identifiers and associates them with the actual Holochain `CellId` to connect to. There are different ways of determining appropriate `CellId`s depending on the executing context.

See [`types.ts`](./types.ts) for a complete reference of configuration options.

### Schema extension and additional service integration

The two optional parameters `extensionSchemas` and `extensionResolvers` allow the dynamic injection of other non-VF functionality into the collaboration space. For example: geolocation functionality, file uploads, commenting, tagging, chat, blogging&hellip;

Simply specify `extensionSchemas` as an array of GraphQL SDL schema strings, and `extensionResolvers` as an additional mapping of GraphQL resolver callbacks to include the additional functionality within the collaboration space's API bindings.

### Partial schema generation

The `enabledVFModules` option, if specified, [controls the subset of ValueFlows modules](https://lab.allmende.io/valueflows/vf-schemas/vf-graphql#generating-schemas) to bind when initialising the collaboration space. Not all economic networks require all features, and this option allows your network to shed unnecessary extra weight. Both the schema fields and resolvers for non-present modules will be omitted from the generated schema.

### Multiple collaboration spaces

The `dnaConfig` option allows the callee to specify custom DNA identifiers to bind GraphQL functions to. For each hREA module ID (see the directory names under `/bundles/dna` in this repository), a runtime `CellId` must be provided as an instance of that DNA to bind to.

By targeting multiple sets of DNAs, multiple "collaboration spaces" can be initialised for a single client application. Several GraphQL APIs can be interacted with via the standard ValueFlows specification. User interfaces should make explicit the scope of data and destination networks to perform query and mutation operations against.

TODO: locate or author combinators for composing collaboration spaces. See https://github.com/holo-rea/holo-rea/issues/159

For more examples of scenarios involving complexly overlapping collaboration spaces, see the tests under `/test/social-architectures` in the hREA repository.

### Direct access to resolver callbacks

In some cases, tooling may require low-level access to the GraphQL resolver callbacks (for example when requiring resolvers to be passed separately to an SDL schema string). You can use the provided `generateResolvers(options?: ResolverOptions)` method to create such functions, bound to the set of DNA modules specified via `options.dnaConfig`.

```js
import { makeExecutableSchema } from '@graphql-tools/schema'

import { generateResolvers } from '@valueflows/vf-graphql-holochain'
const { buildSchema, printSchema } = require('@valueflows/vf-graphql')

const enabledVFModules = ['measurement', 'knowledge', 'observation']

const resolvers = generateResolvers({ enabledVFModules })

const schema = makeExecutableSchema({
	typeDefs: printSchema(buildSchema(enabledVFModules)),
	resolvers,
})
```

Note that the IDs of ValueFlows modules in `enabledVFModules` above do not map exactly 1:1 with the hREA DNA identifiers in `dnaConfig`. For example, the "knowledge" VF module determines the presence of the `ResourceSpecification` and `ProcessSpecification` resolvers, which actually map to an hREA *specification* DNA.


## Repository structure

- `types.ts` contains implementations for the GraphQL scalar types defined in the VF spec. Any system connecting to a VF-compatible schema requires these scalar types to be defined.
- `connection.ts` is the Holochain conductor websocket connection handling logic.

Other files implement the query bindings between the linked hREA app DNAs and GraphQL entity relationships:

- `queries/*.ts` implement the root-level queries exposed by the API.
- `mutations/*.ts` implement write operations for interacting with the app DNAs.
- `resolvers/*.ts` contains the logic for resolving links between different records.


## Building and publishing to NPM

- You will need to be given access to the [VF NPM org](https://www.npmjs.com/org/valueflows) in order to update the module on the registry. You can request access in https://gitter.im/valueflows/welcome
- Bump the version in `package.json` & commit to the repository
- Run `pnpm run build` from this directory or `pnpm run build:graphql-adapter` from the root of the hREA repository
- Change to `./build` under this directory, where the new generated files are
- Run `npm publish --access public` from the `./build` directory
- Tag the current release in git and push the tag to `origin`


## License

Licensed under an Apache 2.0 license.
