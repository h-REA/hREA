# ValueFlows GraphQL Client for hREA Backends

Binds Holochain cell connections for [hREA](https://github.com/holo-rea/holo-rea/) to a `GraphQLClient` interface for connecting to distributed, agent-centric [ValueFlows](http://valueflo.ws) coordination spaces.

<!-- MarkdownTOC -->

- [Usage](#usage)
- [Options](#options)
- [License](#license)

<!-- /MarkdownTOC -->

## Usage

Simply await the results of this asynchronous function to get a handle on a hREA collaboration space.

In a [Svelte](https://svelte.dev/) application, simple app initialisation logic for connecting to one collaboration space might look something like this:

```svelte
<script>
  import { setClient } from 'svelte-apollo'
  import initGraphQLClient from '@vf-ui/graphql-client-holochain'

  import App from './my-happ-ui'

  // init and manage GraphQL client connection
  let client = null
  let loading = true
  let error = null

  async function initConnection(opts) {
    try {
      client = await initGraphQLClient(opts)
    } catch (e) {
      error = e
    }
    loading = false
    error = null
  }

  // Omit these options for connecting via the Holochain Launcher.
  // During development, you can provide them as follows:
  initConnection({
	// A websocket URI to connect to the Holochain Conductor on:
  	// conductorUri,

	// Mapping of hREA module IDs to Holochain CellIds. If ommitted,
	// The client will attempt to sniff them by inspecting the names
	// of active app cells. Any cell with a known 'hrea_*_X' format
	// will be matched.
  	// dnaConfig,
  })

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

Options to the function exported by this module are the same as to [`@valueflows/vf-graphql-holochain`](../vf-graphql-holochain).


## License

Licensed under an Apache 2.0 license.
