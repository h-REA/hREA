<script lang="ts">
  import { onMount } from 'svelte';
  import { type AppClient, AdminWebsocket, AppWebsocket } from '@holochain/client';
  // @ts-ignore
  import { createHolochainSchema } from '@valueflows/vf-graphql-holochain';
  import { ApolloClient, InMemoryCache } from "@apollo/client/core";
  import { SchemaLink } from '@apollo/client/link/schema';
  import { setClient, query, mutation } from "svelte-apollo";
  import { gql } from 'graphql-tag'
  import { printSchema, graphql, getIntrospectionQuery } from 'graphql';
  import GqlForm from './GqlForm.svelte';
  // import GoldenLayout from './GoldenLayout.svelte';
  import GoldenLayout from './GoldenLayout.svelte';

  const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'hrea'
  const roleName = 'hrea'
  const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
  const adminPort = import.meta.env.VITE_ADMIN_PORT
  const url = `ws://localhost:${appPort}`;
  let client: AppClient | undefined;
  let loading = true;
  let goldenLayout;

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

  const GET_ALL_PLANS = gql`
  query {
    plans(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          name
          note
        }
      }
    }
  }
  `

  const GET_ALL_PROCESS_SPECIFICATIONS = gql`
  query {
    processSpecifications(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          name
          note
        }
      }
    }
  }
  `

  const GET_ALL_PROCESSES = gql`
  query {
    processes(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          name
          finished
        }
      }
    }
  }
  `

  const GET_ALL_PROPOSALS = gql`
  query {
    proposals(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          name
          note
          status
          publishes {
            id
            availableQuantity {
              hasNumericalValue
              hasUnit {
                label
                symbol
                omUnitIdentifier
              }
            }
          }
          reciprocal {
            id
          }
        }
      }
    }
  }
  `

  const GET_ALL_UNITS = gql`
  query {
    units(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          label
          symbol
          omUnitIdentifier
        }
      }
    }
  }`

  const GET_ALL_AGREEMENTS = gql`
  query {
    agreements(last: 100000) {
      edges {
        cursor
        node {
          id
          revisionId
          name
          note
          status
        }
      }
    }
  }
  `

  const fetch = {
    organization: query(GET_ALL_AGENTS),
    person: query(GET_ALL_AGENTS),
    agent: query(GET_ALL_AGENTS),
    plan: query(GET_ALL_PLANS),
    processSpecification: query(GET_ALL_PROCESS_SPECIFICATIONS),
    process: query(GET_ALL_PROCESSES),
    proposal: query(GET_ALL_PROPOSALS),
    agreement: query(GET_ALL_AGREEMENTS),
    unit: query(GET_ALL_UNITS),
  }

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
    // set a css var for the base color
    document.documentElement.style.setProperty('--text-color', 'rgb(130, 130, 130)');

    // Enhanced error handling with timeout management
    const CONNECTION_TIMEOUT = 15000; // 15 seconds

    try {
      let tokenResp;

      // Admin connection with timeout and error handling
      if (adminPort) {
        try {
          const url = `ws://localhost:${adminPort}`;
          console.log("connecting to admin port at:", url);

          // v0.6 compatible connection with timeout
          const connectPromise = AdminWebsocket.connect({
            url: new URL(url)
          });

          const timeoutPromise = new Promise((_, reject) =>
            setTimeout(() => reject(new Error('Admin connection timeout')), CONNECTION_TIMEOUT)
          );

          const adminWebsocket = await Promise.race([connectPromise, timeoutPromise]) as any;
          console.log("admin websocket connected successfully");

          // Token issuance with error handling
          try {
            tokenResp = await adminWebsocket.issueAppAuthenticationToken({
              installed_app_id: appId,
            });
            console.log("token issued successfully", tokenResp);
          } catch (tokenError) {
            console.error("Failed to issue token:", tokenError);
            throw new Error(`Authentication token issuance failed: ${tokenError.message}`);
          }

          // Verify app info (v0.6 compatibility check)
          try {
            const apps = await adminWebsocket.listApps({});
            console.log("available apps", apps);
            const cellIds = await adminWebsocket.listCellIds();
            console.log("CELL IDS", cellIds);
            await adminWebsocket.authorizeSigningCredentials(cellIds[0]);
          } catch (appInfoError) {
            console.warn("Could not fetch app info (non-critical):", appInfoError);
            // Continue without app info verification
          }

        } catch (adminError) {
          console.error("Admin connection failed:", adminError);
          throw new Error(`Admin connection failed: ${adminError.message}`);
        }
      }

      // App connection with enhanced error handling
      try {
        console.log("appPort and Id is", appPort, appId);
        const params: any = { url: new URL(url) };
        if (tokenResp) params.token = tokenResp.token;
        console.log("connecting to app port at:", params.url);

        // v0.6 compatible AppWebsocket connection with timeout
        const appConnectPromise = AppWebsocket.connect(params);
        const appTimeoutPromise = new Promise((_, reject) =>
          setTimeout(() => reject(new Error('App connection timeout')), CONNECTION_TIMEOUT)
        );

        client = await Promise.race([appConnectPromise, appTimeoutPromise]) as AppClient;
        console.log("app websocket connected successfully");

        // Verify connection with appInfo
        try {
          const appInfo = await client.appInfo();
          console.log("app info:", appInfo);
        } catch (appInfoError) {
          console.warn("Could not fetch app info:", appInfoError);
          // Connection established but appInfo failed
        }

      } catch (appConnectionError) {
        console.error("App connection failed:", appConnectionError);
        throw new Error(`App connection failed: ${appConnectionError.message}`);
      }

      // Setup Apollo server (v0.6 compatible)
      try {
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

        console.log("Apollo client configured successfully");
      } catch (schemaError) {
        console.error("Failed to setup Apollo client:", schemaError);
        throw new Error(`Apollo client setup failed: ${schemaError.message}`);
      }

      // Initial data fetch
      try {
        agents.refetch();
        const test = fetch.proposal.refetch();
        await new Promise(r => setTimeout(r, 1000));
        console.log("initial data refetched successfully", test);
      } catch (fetchError) {
        console.warn("Initial data fetch failed (will retry):", fetchError);
        // Don't throw - connection established, data can retry
      }

    } catch (error) {
      console.error("Critical connection error:", error);
      // Enhanced error handling for v0.6 specific scenarios
      const errorMessage = error instanceof Error ? error.message : 'Unknown connection error';

      // User-friendly error messages
      let userMessage = "Connection failed. Please check your Holochain conductor.";
      if (errorMessage.includes('timeout')) {
        userMessage = "Connection timeout. Please ensure your Holochain conductor is running and accessible.";
      } else if (errorMessage.includes('authentication')) {
        userMessage = "Authentication failed. Please check your app configuration.";
      } else if (errorMessage.includes('Admin connection')) {
        userMessage = "Could not connect to Holochain admin port. Please check conductor settings.";
      }

      alert(userMessage + "\n\nTechnical details: " + errorMessage);
      throw error; // Re-throw to prevent silent failures
    }
  })
</script>

<div id="header">
  <div id="title">
    <img src="/logo.jpeg" alt="Logo" style="height: 32px; margin-right: 20px;">
    <h1>hREA explorer</h1>
  </div>
  <button
    on:click={goldenLayout.addWindow()}
  >
    + Window
  </button>
  <!-- <button on:click={async () => {
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
  }}>get agents</button> -->
</div>

<!-- <GqlForm {apolloClient} /> -->

<!-- {#if $agents.loading}
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
{/if} -->

<GoldenLayout bind:this={goldenLayout} {apolloClient} {fetch} />

<style>
#title {
  display: flex;
  align-items: center;
}
#header {
  height: 50px !important;
  text-align: center;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-left: 6px;
  margin-right: 6px;
}
#header h1 {
  margin: 0;
  padding: 0;
  font-size: 2em;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  color: #444444;
  color: rgb(95, 95, 95);
;
}
#header button {
  margin-left: 20px;
  padding: 6px 12px;
  font-size: 1em;
  background-color: #38bdb9;
  background-color: rgb(33, 33, 33);
  color: rgb(130, 130, 130);
  border: none;
  border-radius: 5px;
  cursor: pointer;
}
#header button:hover {
  background-color: #1d8a7d;
  background-color: rgb(17, 17, 17);
}
:global(body) {
  margin: 0;
  padding: 0;
  font-family: sans-serif;
  /* background-color: rgb(38, 38, 38); */
  background-color: rgb(0, 0, 0);
}
:global(.lm_header .lm_tab.lm_active.lm_focused) {
  background-color: rgb(34, 34, 34);
}
:global(.lm_header .lm_tab.lm_active) {
  padding-bottom: 5px;
}
:global(.lm-header) {
  background-color: black;
}
</style>
