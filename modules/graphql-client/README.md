# ValueFlows GraphQL Client for hREA Backends

Binds Holochain cell connections for [hREA](https://github.com/holo-rea/holo-rea/) to a `GraphQLClient` interface for connecting to distibuted, agent-centric [ValueFlows](http://valueflo.ws) coordination spaces.

Options to the function exported by this module are the same as to [`@valueflows/vf-graphql-holochain`](../vf-graphql-holochain).

In a [Svelte](https://svelte.dev/) application, your app initialisation logic might look something like this:

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
