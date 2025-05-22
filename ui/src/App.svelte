<script lang="ts">
  import { onMount } from 'svelte';
  import { type AppClient, AdminWebsocket, AppWebsocket } from '@holochain/client';
  // @ts-ignore
  import { createHolochainSchema } from '@valueflows/vf-graphql-holochain';
  import { ApolloClient, InMemoryCache } from "@apollo/client/core";
  import { SchemaLink } from '@apollo/client/link/schema';
  import { setClient, query, mutation } from "svelte-apollo";
  import { gql } from 'graphql-tag'

  const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'hrea'
  const roleName = 'hrea'
  const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
  const adminPort = import.meta.env.VITE_ADMIN_PORT
  const url = `ws://localhost:${appPort}`;
  let client: AppClient | undefined;
  let loading = true;

  const cache = new InMemoryCache();
  const apolloClient = new ApolloClient({
    cache
  });
  setClient(apolloClient);

  const AGENT_CORE_FIELDS = gql`
    fragment AgentCoreFields on Organization {
      id
      revisionId
      name
      image
      note
      classifiedAs
    }
  `

  const GET_ALL_AGENTS = gql`
  ${AGENT_CORE_FIELDS}
  query {
    agents(last: 100000) {
      edges {
        cursor
        node {
          ...AgentCoreFields
        }
      }
    }
  }
  `

  const agents = query(GET_ALL_AGENTS);

  $: agents.refetch();

  const ADD_AGENT = gql`
  ${AGENT_CORE_FIELDS},
  mutation($agent: OrganizationCreateParams!){
    createOrganization(organization: $agent) {
      agent {
        ...AgentCoreFields
      }
    }
  }
  `

  onMount(async () => {
    let tokenResp;
    if (adminPort) {
        const url = `ws://localhost:${adminPort}`;
        console.log("connecting to admin port at:", url);
        const adminWebsocket = await AdminWebsocket.connect({
          url: new URL(url)
        });
        console.log("issuing token");
        tokenResp = await adminWebsocket.issueAppAuthenticationToken({
          installed_app_id: appId,
        });
        console.log("token", tokenResp);
        const x = await adminWebsocket.listApps({});
        console.log("apps", x);
        const cellIds = await adminWebsocket.listCellIds();
        console.log("CELL IDS", cellIds);
        await adminWebsocket.authorizeSigningCredentials(cellIds[0]);
      }
      console.log("appPort and Id is", appPort, appId);
      const params: any = { url: new URL(url) };
      console.log("params", params);
      if (tokenResp) params.token = tokenResp.token;
      console.log("connecting to app port at:", params.url);
      client = await AppWebsocket.connect(params);
      console.log("client", client);
      console.log("get cell", await client.appInfo())

    // setup Apollo server
    const schema = createHolochainSchema(
      {
        appWebSocket: client,
        roleName: 'hrea'
      }
    );

    apolloClient.setLink(
      new SchemaLink(
        { schema }
      )
    );

    agents.refetch();
  })
</script>

<div id="header">
  <img src="/logo.jpeg" alt="Logo" style="height: 36px; margin-right: 20px;">
  <h1>hREA explorer</h1>
  <button on:click={async () => {
    console.log("add agent")
    const res = await apolloClient.mutate({
      mutation: ADD_AGENT,
      variables: {
        agent: {
          name: "agent" + Math.floor(Math.random() * 1000),
          image: "https://example.com/image.png",
          note: "This is a test agent",
          classifiedAs: "Test Agent",
        }
      }
    });
    console.log("res", res);
    await agents.refetch();
  }}>add agent</button>
  <button on:click={async () => {
    console.log("add agent")
    const res = await agents.refetch();
    console.log("res", res?.data?.agents?.edges?.map((a) => a.node));
  }}>get agents</button>
</div>

{#if $agents.loading}
  <li>Loading...</li>
{:else if $agents.error}
  <li>ERROR: {$agents.error.message}</li>
{:else}
  {@const agents = $agents.data?.agents?.edges?.map((a) => a.node).reverse() || []}
  Number of agents: {agents.length}
  <ul>
  {#each agents as agent (agent.id)}
    <li>
      {agent.name}
      - {agent.note}
      - {agent.classifiedAs}
    </li>
    {/each}
  </ul>
{/if}

<style>
#header {
  height: 50px !important;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
}
#header h1 {
  margin: 0;
  padding: 0;
  font-size: 24px;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  color: #444444;
}
#header button {
  margin-left: 20px;
  padding: 6px 12px;
  font-size: 16px;
  background-color: #38bdb9;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}
#header button:hover {
  background-color: #1d8a7d;
}
:global(body) {
  margin: 0;
  padding: 0;
  font-family: sans-serif;
}
</style>