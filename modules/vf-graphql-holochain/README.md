# HoloREA GraphQL schema binding

**Work in progress!**

Binds Holochain DNA resolvers for HoloREA to the ValueFlows protocol spec, thus creating a pluggable VF implementation on Holochain.

## Usage

This module provides a raw GraphQL schema object and can be used in a variety of ways. However, it is expected that you will most often use it module in a React / Apollo application. In this case, your app initialisation logic will probably include something like this:

```javascript
import { ApolloClient } from 'apollo-client'
import { InMemoryCache } from 'apollo-cache-inmemory'
import { SchemaLink } from 'apollo-link-schema'

import { schema } from '@valueflows/vf-graphql-holochain'

const client = new ApolloClient({
  link: new SchemaLink({ schema }),
  cache: new InMemoryCache()
})
```

There are other use-cases and examples provided in the `example` folder at the root of [this repository](https://github.com/holo-rea/holo-rea).

## Important files

- `types.ts` contains implementations for the GraphQL scalar types defined in the VF spec. Any system connecting to a VF-compatible schema requires these handlers to be present.
- `connection.ts` is the Holochain conductor websocket connection handling logic.

Other files implement the query bindings between the linked HoloREA app DNAs and GraphQL entity relationships:

- `queries/*.ts` implement the root-level queries exposed by the API.
- `mutations/*.ts` implement write operations for interacting with the app DNAs.
- `resolvers/*.ts` contains the logic for resolving links between different records.
