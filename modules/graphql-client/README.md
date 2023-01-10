# ValueFlows GraphQL Client (Apollo)
## for Calling hREA Backend Systems

Binds Holochain cell connections for [hREA](https://github.com/h-REA/hREA/) to a `GraphQLClient` interface for connecting to distributed, agent-centric [ValueFlows](http://valueflo.ws) coordination spaces.

<!-- MarkdownTOC -->

- [Usage](#usage)
- [Options](#options)
- [License](#license)

<!-- /MarkdownTOC -->

## Usage

Simply await the results of this asynchronous function (the default export) to get a handle on a hREA collaboration space.
With the resulting `ApolloClient` you can integrate it into many different user interface frameworks, or no framework. It will
error if it cannot establish a websocket connection with a running holochain service which itself has a running instance of a
valid `hREA` hApp.

In a [Svelte](https://svelte.dev/) application, simple app initialisation logic for connecting to one collaboration space might look something like this:

```svelte
<script>
  import { setClient } from 'svelte-apollo'
  import graphqlClientHolochain from '@vf-ui/graphql-client-holochain'

  import App from './my-happ-ui'

  // init and manage GraphQL client connection
  let client = null
  let loading = true
  let error = null

  async function initConnection() {
    try {
      // it can be called with no direct options passed
      // but if doing so, be aware that certain values may need
      // to be set by environment variables alternatively
      client = await graphqlClientHolochain()
    } catch (e) {
      error = e
    }
    loading = false
    error = null
  }

  initConnection()

  // workaround to set the context outside of init action
  $: {
    if (client) {
      setClient(client)
    }
  }
</script>

<main>
  {#if loading}
    <h1>Loading...</h1>
  {:else if error}
    <h1>Cannot connect to Holochain</h1>
    <p>{error.message}</p>
  {:else}
    <App />
  {/if}
</main>
```

Note that you can connect to multiple conductors and sets of Holochain DNAs in order to naively connect to multiple collaboration spaces; and you can also connect to other non-Holochain ValueFlows-compatible GraphQL client APIs in order to manage data across contexts. In reactive UI applications built with frameworks like React, Svelte etc this means that you can simply swap out the active `GraphQLClient` with another by wrapping UI elements in a different connection provider in order to target different networks.

TODO: provide an example of this

## Options

It is possible to omit any or all of these options, and even to leave the options object undefined. Below the type
definition are descriptions of each.

```typescript
interface ClientOptions {
  dnaConfig?: DNAIdMappings
  conductorUri?: string
  adminConductorUri?: string
  appID?: string
  enabledVFModules?: VfModule[]
  extensionSchemas?: string[]
  extensionResolvers?: IResolvers
  traceAppSignals?: AppSignalCb
}
```

`dnaConfig`
  Mapping of hREA module IDs to Holochain CellIds. If omitted,
	the client will attempt to sniff them by inspecting the names
	of active app cells. Any Cell with a known 'hrea_*_X' format
	will be matched.

`conductorUri`
  A websocket URI to connect to a running `holochain` service which has websocket ports open.
  An example is "ws://localhost:4000".
  There are two main circumstances that define what to pass here:
  1. when running in the Holochain Launcher context, the `conductorUri` will be auto-discovered, and can thus be omitted
  2. when NOT running in the Holochain Launcher context, the value must be explicit, and can be provided one of two ways:
    a. via the REACT_APP_HC_CONN_URL environment variable
    b. via this config option, which would override the environment variable value, if set

`adminConductorUri`
  A websocket URI to connect to a running `holochain` "admin" service which has websocket ports open.
  An example is "ws://localhost:4000".
  There are two main circumstances that define what to pass here:
  1. when running in the Holochain Launcher context, the `adminConductorUri` will be auto-discovered, and can thus be omitted
  2. when NOT running in the Holochain Launcher context, the value must be explicit, and can be provided one of two ways:
    a. via the REACT_APP_HC_ADMIN_CONN_URL environment variable
    b. via this config option, which would override the environment variable value, if set

`appID`
  When a hApp is installed to `holochain`, an `app_id` value is always provided. There are two main circumstances that define what to pass here:
  1. when running in the Holochain Launcher context, the `app_id` will be auto-discovered, and can thus be omitted
  2. when NOT running in the Holochain Launcher context, the value must be explicit, and can be provided one of two ways:
    a. via the REACT_APP_HC_APP_ID environment variable
    b. via this config option, which would override the environment variable value, if set

`enabledVFModules`
  This defines which Valueflows Modules or `VfModule`s are enabled within your hApp. This will actually trim the schema and resolvers down to only include the scope of the modules you enable, and requests outside of those modules will result in graphql
  schema errors being thrown. It is optional because it will by default take on the value of the "full set" of all `VfModule`s
  which have been developed so far.

`extensionSchemas` TODO
`extensionResolvers` TODO

`traceAppSignals`
  As of this writing, the hREA hApp backends have not been configured to emit "signals" (which are like events that your connected client can subscribe to), and so there is no point in setting this value. In the future there may be, and this is how you would listen for signals from `holochain` in your client.

## Building and publishing to NPM

- You will need to be given access to the [VF NPM org](https://www.npmjs.com/org/valueflows) in order to update the module on the registry. You can request access in [Discord](https://discord.gg/um4UsxdFDk)
- Bump the version in `package.json` & commit to the repository
- Run `pnpm run build` from this directory or `pnpm run build:graphql:client` from the root of the hREA repository
- Run `pnpm publish --access public` from this directory
- Tag the current release in git and push the tag to `origin`


## License

Licensed under an Apache 2.0 license.
