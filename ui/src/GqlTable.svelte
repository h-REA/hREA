<script lang="ts">
import { setClient, query, mutation, subscribe } from "svelte-apollo";
import { GET_ALL_AGENTS } from "./fetch";
import { onMount, onDestroy } from 'svelte';
import { gql } from 'graphql-tag';
import { schema as customSchema } from './schema';
import { capitalize, pluralize, parseGraphQLFields } from './utils';
import { type Writable } from "svelte/store";
import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import { getIntrospectionQuery, buildClientSchema, printSchema } from 'graphql';
export let apolloClient;
export let file: Writable<any>;
export let fetch: (query: string) => Promise<any>;

const agents = query(GET_ALL_AGENTS);
let fetchAllSchema;
$: everyListType = fetchAllSchema ? Object.keys(fetchAllSchema) : [];
let schemaType = null;
let gqlFetchString;
let gqlFetch;
let gqlTypes = {};

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

let intervalId: NodeJS.Timeout;
$: if (gqlFetchString) {
  gqlFetch = query(gql`
    ${gqlFetchString}
  `);
  console.log("gqlFetchString:", gqlFetchString);
  // gqlFetch.refetch();
  // clearInterval(intervalId);
  // intervalId = setInterval(() => {
  //   fetch[schemaType]?.refetch();
  // }, 10000);
}

function generateBasicFetch(type: string) {
    return `query {
    ${type}(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          name
          note
          image
          classifiedAs
        }
      }
    }
  }`;

  // if (!fetchAllSchema || !fetchAllSchema[type]) {
  //   return `query {${type}(last: 100000) { edges { cursor node { id } } } }`;
  // }
  // console.log("Generating fetch for type:", 
  //   gqlTypes[gqlTypes[gqlTypes[fetchAllSchema[type].returnType.split('!')[0]][0].type.ofType.ofType.ofType.name][0].type.ofType.name],
  //   // fetchAllSchema[type].returnType[0].type.ofType,
  //   // fetchAllSchema[type].returnType[0].type.ofType.ofType.ofType.name
  // );
  // let returnType = gqlTypes[gqlTypes[gqlTypes[fetchAllSchema[type].returnType.split('!')[0]][0].type.ofType.ofType.ofType.name][0].type.ofType.name]
  // console.log("returnType:", returnType, type.slice(0, -1));
  // // const fieldList = returnType?.map(field => field.name).join('\n');
  // const fieldList = fetch[type.slice(0, -1)]
  // console.log("fieldList:", fieldList);
  // return `query {
  //   ${type}(last: 100000) {
  //     edges {
  //       cursor
  //       node {
  //         ${fieldList || 'id'}
  //       }
  //     }
  //   }
  // }`;
}

async function fetchSchema() {
  const introspectionQuery = getIntrospectionQuery();

  const result = await apolloClient.query({
    query: gql(introspectionQuery),
  });

  console.log("Introspection result:", result);

  const rawGqlTypes = result.data.__schema.types
  for (const type of rawGqlTypes) {
    if (type.name.startsWith('__')) continue; // Skip introspection types
    gqlTypes[type.name] = type.fields
  }

  console.log("gqlTypes:", gqlTypes);
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

onDestroy(() => {
  clearInterval(intervalId);
});
</script>

<!-- choose schema type -->
{#if !schemaType}
<div id="info">
  <h1>Step 2: Select an entry type</h1>
  <div id="buttons">
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
</div>
{:else}
  <ul>
  {#if $gqlFetch.loading}
    <li>Loading...</li>
  {:else if $gqlFetch.error}
    <li>ERROR: {$gqlFetch.error.message}</li>
  {:else}
    <h2>
      {$gqlFetch?.data?.[schemaType]?.edges?.length
        ? `${$gqlFetch.data[schemaType].edges.length} ${schemaType}`
        : `No ${schemaType} found.`}
    </h2>
    {@const fields = Object.keys($gqlFetch?.data[schemaType]?.edges[0]?.node || {})}

    {JSON.stringify($gqlFetch.data[schemaType]?.edges[0]?.node, null, 2)}

    {#if fetchAllSchema && fetchAllSchema[schemaType]}
      <table>
      <thead>
        <tr>
        {#each fields as field}
          {#if field == '__typename'}
          {:else}
            <th>{capitalize(field)}</th>
          {/if}
        {/each}
        </tr>
      </thead>
      <tbody>
        {#each $gqlFetch.data?.[schemaType]?.edges as edge (edge.node.id)}
        <tr>
          {#each fields as field}
          {#if field == '__typename'}
          {:else if field == 'id'}
            <td>
              <span style="cursor: pointer;" title="click to copy" on:click={() => {
                navigator.clipboard.writeText(edge.node[field]);
                alert(`Copied ${field} to clipboard!`);
              }}>
                ✄ {edge.node[field].substring(0, 8)}...
            </span>
            </td>
          {:else if field.toLowerCase().includes('id')}
            <td>
              {edge.node[field].substring(0, 8)}...
            </td>
          {:else}
          <td>
            {edge.node[field]}
          </td>
          {/if}
          {/each}
        </tr>
        {/each}
      </tbody>
      </table>
    {/if}

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
    h2 {
        font-size: 1.4em;
        color: var(--primary-color);
        margin-left: 5px;
    }
    ul {
        color: var(--text-color);
    }
    table {
        width: 100%;
        border-collapse: collapse;
    }
    th, td {
        border: 1px solid var(--border-color);
        padding: 8px;
        text-align: left;
    }
    th {
        background-color: var(--header-bg-color);
        color: var(--header-text-color);
    }
    tr:nth-child(even) {
        background-color: var(--row-bg-color);
    }
    tr:hover {
        background-color: var(--row-hover-bg-color);
    }
    button {
        background-color: var(--button-bg-color);
        color: var(--button-text-color);
        border: none;
        padding: 10px 20px;
        cursor: pointer;
        margin: 5px;
    }
    button:hover {
        background-color: var(--button-hover-bg-color);
    }
    #info {
        display: flex;
        flex-direction: column;
        align-items: center;
        margin: 10px;
        color: var(--text-color);
        width: 100%;
    }
    #buttons {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    #buttons button {
        background-color: rgb(18, 18, 18);
        margin: 0;
        width: 100%;
    }

    #buttons button:hover {
        background-color: black;
    }
</style>