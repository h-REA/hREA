<script lang="ts">
  import { onMount } from 'svelte';
  import { type AppClient, AdminWebsocket, AppWebsocket } from '@holochain/client';
  // @ts-ignore
  import { createHolochainSchema } from '@valueflows/vf-graphql-holochain';
  import { ApolloClient, InMemoryCache } from "@apollo/client/core";
  import { SchemaLink } from '@apollo/client/link/schema';
  import { setClient, query, mutation } from "svelte-apollo";
  import { gql } from 'graphql-tag'
  import FormBuilder from './FormBuilder.svelte';
  import AddField from './AddField.svelte';
  import { printSchema, graphql, getIntrospectionQuery } from 'graphql';
  import { schema as customSchema } from './schema';
  import { get, type Writable } from 'svelte/store';
  import { capitalize } from './utils';
  import { GET_ALL_AGENTS } from './fetch';

  export let apolloClient: ApolloClient<any>;
  export let fetch;
  export let file: Writable<any>;

  let schemaType;
  let requiredFields = [];
  let optionalFields = [];
  let presentOptionalFields = [];
  let formData = {};
  let gqlCreateString = '';

  file.subscribe(value => {
    if (value["schemaType"] && !schemaType) {
      schemaType = value["schemaType"] || null;
      requiredFields = value["requiredFields"] || [];
      optionalFields = value["optionalFields"] || [];
      presentOptionalFields = value["presentOptionalFields"] || [];
      formData = value["formData"] || {};
      gqlCreateString = value["gqlCreateString"] || '';
    }
  });
  $: if (schemaType || requiredFields || optionalFields || presentOptionalFields || formData || gqlCreateString) {
    file.update(current => ({
      ...current,
      schemaType: schemaType,
      requiredFields: requiredFields,
      optionalFields: optionalFields,
      presentOptionalFields: presentOptionalFields,
      formData: formData,
      gqlCreateString: gqlCreateString
    }));
  }

  $: if (schemaType) {
    // const schema = customSchema[schemaType];
    console.log(gqlCreateString);
    gqlCreateString = `mutation create${capitalize(schemaType)}($${schemaType}: ${capitalize(schemaType)}CreateParams!)`;
    gqlCreateString += ` { create${capitalize(schemaType)}(${schemaType}: $${schemaType}) }`;
  }

  onMount(async () => {
  })
</script>
<div id="outer">
  <!-- choose schema type -->
  {#if !schemaType}
    <h1>Step 2: Select a form type</h1>
    {#each Object.keys(customSchema) as type}
        {#if !customSchema[type].hidden}
            <button on:click={() => {
                schemaType = type;
                requiredFields = Object.keys(customSchema[type].required);
                optionalFields = Object.keys(customSchema[type].optional);
                presentOptionalFields = [];
                formData = {};
            }}>
                {type.charAt(0).toUpperCase() + type.slice(1)}
            </button>
        {/if}
    {/each}
  {:else}
  <h1>Create {capitalize(schemaType)}</h1>
  <FormBuilder bind:presentOptionalFields bind:output={formData} {requiredFields} {customSchema} {schemaType} {apolloClient}/>
  <!-- <br> -->
  <AddField bind:presentOptionalFields {optionalFields} />
  <!-- <pre>{JSON.stringify(formData, null, 2)}</pre> -->
  <!-- <p>{JSON.stringify(gqlCreateString, null, 2)}</p> -->
  <!-- <p>{JSON.stringify(gqlSchema, null, 2)}</p> -->

    <div id="end-buttons">
      <button>
        Reset
      </button>
      <button on:click={async () => {
        const mutationData = {
          mutation: gql`${gqlCreateString}`,
          variables: {
            [schemaType]: formData
          }
        }
        console.log("mutationData", mutationData);
        const res = await apolloClient.mutate(mutationData);
        console.log("res", res);
        console.log("schemaType", schemaType, fetch);
        fetch[schemaType].refetch();
        console.log("fetch", fetch[schemaType]);
      }}>Submit</button>
    </div>
  {/if}
</div>

<style>
  #end-buttons {
    display: flex;
    flex-direction: row;
    width: 100%;
    gap: 0.4em;
  }
  h1 {
    min-height: 30px;
    color:  var(--text-color);
    font-size: 1.5em;
    text-align: center;
    margin: 0.4em;
  }
  button {
    background-color: rgb(95, 95, 95);
    background-color: rgb(18, 18, 18);
    color: rgb(95, 95, 95);
    border: none;
    cursor: pointer;
    padding: 0.5em;
    width: 100%;
    font-size: 1em;
  }
  button:hover {
    background-color: black
  }
  #outer {
    display: flex;
    flex-direction: column;
    gap: 0.4em;
    padding: 0.4em;
		/* max-width: 800px; */
    margin: auto;
    gap: 0.4em;
    overflow: auto;
    position: relative;
    z-index: 0;
  }
</style>