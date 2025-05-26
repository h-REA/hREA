<script lang="ts">
  import { schema } from './schema';
  import { gql } from 'graphql-tag'
  import { query } from 'svelte-apollo';
  import { actions } from './schema';

  // export let apolloClient;
  export let schemaType;
  export let selectedId; 

  let fetchAllString;
  let fetchQuery;
  let res;
  async function fetchData() {
    const requiredFields = schema[schemaType]?.required;
    if (requiredFields) {
      const firstField = Object.keys(requiredFields)[0];
      console.log("firstField", firstField);
      fetchAllString = `query { ${schemaType}s(last: 100000) { edges { cursor node { id ${firstField} } } } }`
      fetchQuery = query(gql`${fetchAllString}`)
      res = await fetchQuery.refetch();
    }
  }

  $: if (schemaType && schemaType != 'action') {
    fetchData();
  }

</script>
{#if schemaType == 'action'}
    {@const actionKeys = Object.keys(actions)}
    <select bind:value={selectedId}>
      <option value="" disabled selected>Select action</option>
      {#each actionKeys as id}
        <option value={id}>{actions[id].label}</option>
      {/each}
    </select>
{:else if $fetchQuery}
  {#if $fetchQuery.loading}
    <li>Loading...</li>
  {:else if $fetchQuery.error}
    <li>ERROR: {$fetchQuery.error.message}</li>
  {:else}
    {@const entries = $fetchQuery.data?.[`${schemaType}s`]?.edges?.map((a) => a.node).reverse() || []}
    <select bind:value={selectedId}>
      <option value="" disabled selected>Select {schemaType}</option>
      {#if schemaType}
        {#each entries as entry}
          <option value={entry.id}>{entry.name}</option>
        {/each}
      {/if}
    </select>
  {/if}
{/if}

<style>
  select {
    padding: 0.25em;
    background-color: rgb(95, 95, 95);
    border: none;
    cursor: pointer;
    width: 100%;
  }
  select:hover {
    background-color: var(--text-color);
  }
</style>