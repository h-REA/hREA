<script lang="ts">
import { setClient, query, mutation, subscribe } from "svelte-apollo";
import { GET_ALL_AGENTS } from "./fetch";
import { onMount } from 'svelte';
import { gql } from 'graphql-tag';
import { schema as customSchema } from './schema';
import { capitalize, pluralize, parseGraphQLFields } from './utils';
import { type Writable } from "svelte/store";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import { getIntrospectionQuery, buildClientSchema, printSchema } from 'graphql';
export let apolloClient;
export let file: Writable<any>;

const agents = query(GET_ALL_AGENTS);
let fetchAllSchema;
$: everyListType = fetchAllSchema ? Object.keys(fetchAllSchema) : [];
let schemaType = null;
let gqlFetchString;
let gqlFetch;

file.subscribe(value => {
  if (value["schemaType"] && !schemaType) {
    schemaType = value["schemaType"] || null;
    gqlFetchString = value["gqlFetchString"] || generateBasicFetch(schemaType);
  }
});
$: if (schemaType || gqlFetchString) {
  file.update(current => ({
    ...current,
    schemaType: schemaType,
    gqlFetchString: gqlFetchString
  }));
}

$: if (gqlFetchString) {
  gqlFetch = query(gql`
    ${gqlFetchString}
  `);
  gqlFetch.refetch();
}

function generateBasicFetch(type: string) {
  return `query {${type}(last: 100000) { edges { cursor node { 
      id
      revisionId
      name
      image
      note
      classifiedAs
  } } } }`;
}

async function fetchSchema() {
  const introspectionQuery = getIntrospectionQuery();

  const result = await apolloClient.query({
    query: gql(introspectionQuery),
  });

  const schema = buildClientSchema(result.data);
  const printedSchema = printSchema(schema);
  const subsection = printedSchema.split('type Query ')[1].split(`
"""
A boundary `)[0];
  const parsedFields = parseGraphQLFields(subsection);
  console.log("Parsed fields:", parsedFields);
  fetchAllSchema = parsedFields;
  console.log("fetchAllSchema:", fetchAllSchema);
  return printedSchema;
}

onMount(async () => {
  await fetchSchema();
});
</script>

<!-- choose schema type -->
{#if !schemaType}
  <h1>Step 2: Select a form type</h1>
  <div style="display: flex; flex-direction: column; align-items: center; margin: 10px;">
    {#each everyListType as type}
      {#if fetchAllSchema[type]?.args?.includes("first")}
      <button on:click={() => {
        schemaType = type;
        gqlFetchString = generateBasicFetch(type);
        console.log(gqlFetchString);
        console.log("compared to", GET_ALL_AGENTS.loc.source.body)
        file.update(current => ({ ...current, schemaType }));
      }}>
          {capitalize(type)}
        </button>
      {/if}
    {/each}
  </div>
{:else}
  <ul>
  {#if $gqlFetch.loading}
    <li>Loading...</li>
  {:else if $gqlFetch.error}
    <li>ERROR: {$gqlFetch.error.message}</li>
  {:else}
    {$gqlFetch?.data?.[schemaType]?.edges?.length
      ? `Number of ${schemaType}: ${$gqlFetch.data[schemaType].edges.length}`
      : `No ${schemaType} found.`}
    <!-- {#each $gqlFetch.data?.[pluralize(schemaType)]?.edges as edge (edge.node.id)}
      <li>
        {edge.node.id}
      </li>
    {/each} -->
    <!-- {@const agents = $agents.data?.agents?.edges?.map((a) => a.node).reverse() || []}
    Number of agents: {agents.length}
    {#each agents as agent (agent.id)}
      <li>
        {agent.name}
        - {agent.note}
        - {agent.classifiedAs}
      </li>
      {/each} -->
  {/if}
  </ul>
{/if}

<style>
    ul {
        color: var(--text-color);
    }
</style>