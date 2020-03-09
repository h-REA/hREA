# Holo-REA GraphQL schema binding

**Work in progress!**

Binds Holochain DNA resolvers for Holo-REA to the ValueFlows protocol spec, thus creating a pluggable VF implementation on Holochain.


## Usage

This module provides a raw GraphQL schema object and can be used in a variety of ways. However, it is expected that you will most often use it module in a React / Apollo application. In this case, your app initialisation logic will probably include something like this:

```javascript
import { ApolloClient } from 'apollo-client'
import { InMemoryCache } from 'apollo-cache-inmemory'
import { SchemaLink } from 'apollo-link-schema'

import schema from '@valueflows/vf-graphql-holochain'

const client = new ApolloClient({
  link: new SchemaLink({ schema }),
  cache: new InMemoryCache()
})
```

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
