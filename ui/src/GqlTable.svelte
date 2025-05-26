<script lang="ts">
import { setClient, query, mutation, subscribe } from "svelte-apollo";
import { GET_ALL_AGENTS } from "./fetch";
import { onMount } from 'svelte';
// export let apolloClient;
// export let fetch;

export const agents = query(GET_ALL_AGENTS);

onMount(() => {
  agents.refetch();
});
</script>

<ul>
{#if $agents.loading}
  <li>Loading...</li>
{:else if $agents.error}
  <li>ERROR: {$agents.error.message}</li>
{:else}
  {@const agents = $agents.data?.agents?.edges?.map((a) => a.node).reverse() || []}
  Number of agents: {agents.length}
  {#each agents as agent (agent.id)}
    <li>
      {agent.name}
      - {agent.note}
      - {agent.classifiedAs}
    </li>
    {/each}
{/if}
</ul>

<style>
    ul {
        color: var(--text-color);
    }
</style>