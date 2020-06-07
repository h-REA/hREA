# Holo-REA GraphQL schema binding

Binds Holochain DNA resolvers for Holo-REA to the ValueFlows protocol spec, thus creating a pluggable VF implementation on Holochain.

**Work in progress!**

<!-- MarkdownTOC -->

- [Usage](#usage)
	- [Multiple collaboration spaces](#multiple-collaboration-spaces)
	- [Partial schema generation](#partial-schema-generation)
	- [Schema extension and additional service integration](#schema-extension-and-additional-service-integration)
	- [Direct access to resolver callbacks](#direct-access-to-resolver-callbacks)
	- [Other examples](#other-examples)
- [Important files](#important-files)
- [Publishing to NPM](#publishing-to-npm)

<!-- /MarkdownTOC -->


## Usage

This module provides a configurable generator function which returns a runtime-configurable GraphQL schema for any configured arrangement of Holo-REA modules. You can think of the objects returned by this function as single "collaboration spaces", where groups of Holochain DNA modules with specific, coordinated purposes are arranged together to form one coherent logical "place" that people hang out in.

It is expected that you will most often use this module in a React / Apollo application. In the simplest case, your app initialisation logic will probably look something like this:

```javascript
import { ApolloClient } from 'apollo-client'
import { InMemoryCache } from 'apollo-cache-inmemory'
import { SchemaLink } from 'apollo-link-schema'

import bindSchema from '@valueflows/vf-graphql-holochain'

const client = new ApolloClient({
  link: new SchemaLink({ schema: bindSchema() }),
  cache: new InMemoryCache()
})
```

When executing `bindSchema` as above, an optional `options` argument can be provided to specify the underlying services to connect to. See `types.ts` for the canonical reference.

### Multiple collaboration spaces

The `dnaConfig` option allows the callee to specify custom DNA identifiers to bind GraphQL functions to. For each Holo-REA module ID (see the directory names under `/happs`), a specific runtime identifier can be specified as an instance of that hApp to bind to.

Multiple Holochain conductors can also be bound to by specifying `conductorUri` as an option.

This allows multiple "collaboration spaces" to be initialised for a single client application, in order that several GraphQL APIs can be interacted with via the standard ValueFlows specification.

For more examples of this, see the tests under `/test/social-architectures` in this repository.

### Partial schema generation

The `enabledVFModules` option, if specified, controls the subset of [ValueFlows modules](https://github.com/valueflows/vf-graphql/#generating-schemas) to bind when initialising the collaboration space. Not all economic networks require all features, and this option allows your network to shed unnecessary extra weight. Both the schema fields and resolvers for non-present modules will be omitted from the generated schema.

### Schema extension and additional service integration

The two optional parameters  and `extensionResolvers` allow the dynamic injection of other non-VF functionality into the collaboration space. For example: geolocation functionality, file uploads, commenting, tagging, chat, blogging&hellip;

Simply specify `extensionSchemas` as an array of GraphQL SDL schema strings, and `extensionResolvers` as an additional mapping of GraphQL resolver callbacks to include the additional functionality within the collaboration space's API bindings.

### Direct access to resolver callbacks

In some cases, tooling may require low-level access to the GraphQL resolver callbacks (for example, when tooling requires this option to be passed separately to an SDL schema string). You can use the provided `generateResolvers(options?: ResolverOptions)` method to create such functions, bound to the set of DNA modules specified via `options`.

```js
import { makeExecutableSchema } from '@graphql-tools/schema'

import buildSchema, { generateResolvers } from '@valueflows/vf-graphql-holochain'
const { buildSchema, printSchema } = require('@valueflows/vf-graphql')

const enabledVFModules = ['measurement', 'knowledge', 'observation']

const resolvers = generateResolvers({ enabledVFModules })

const schema = makeExecutableSchema({
	typeDefs: printSchema(buildSchema(enabledVFModules)),
	resolvers,
})
```

### Other examples

There are other use-cases and examples provided in the `example` folder at the root of [this repository](https://github.com/holo-rea/holo-rea).


## Important files

- `types.ts` contains implementations for the GraphQL scalar types defined in the VF spec. Any system connecting to a VF-compatible schema requires these handlers to be present.
- `connection.ts` is the Holochain conductor websocket connection handling logic.

Other files implement the query bindings between the linked Holo-REA app DNAs and GraphQL entity relationships:

- `queries/*.ts` implement the root-level queries exposed by the API.
- `mutations/*.ts` implement write operations for interacting with the app DNAs.
- `resolvers/*.ts` contains the logic for resolving links between different records.


## Publishing to NPM

- You will need to be given access to the [VF NPM org](https://www.npmjs.com/org/valueflows) in order to update the module on the registry. You can request access in https://gitter.im/valueflows/welcome
- Bump the version in `package.json` & commit to the repository
- Run `npm run build` from this directory or `npm run build:graphql-adapter` from the root of the Holo-REA repository
- Change to `./build` under this directory, where the new generated files are
- Run `npm publish --access public` from the `./build` directory
- Tag the current release in git and push the tag to `origin`
