<script lang="ts">
  import Dropdown from "./Dropdown.svelte";
  import { schema } from "./schema";
  export let presentOptionalFields: string[] = [];
  export let requiredFields: string[] = [];
  export let output: { [key: string]: string } = {};
  export let customSchema: { [key: string]: any } = {};
  export let schemaType: string;
  export let apolloClient: any;

  let fieldTypes: { [key: string]: string } = {};
  $: if (schemaType) {
    const schema = customSchema[schemaType];
    fieldTypes = { ...schema.required, ...schema.optional };
  }

  $: combinedFields = [...requiredFields, ...presentOptionalFields];

  function handleInput(field: string, value: any) {
    console.log(`Handling input for field: ${field}, value: ${value}`);
    output[field] = value;
  }

  function capitalize(string) {
    if (!string) return '';
    return string.charAt(0).toUpperCase() + string.slice(1);
  }
</script>

<div class="form">
    {#each combinedFields as field (field)}
        {@const fieldType = fieldTypes[field]}
        <div class="form-field">
            <label for={field}>
                {#if presentOptionalFields.includes(field)}
                    <span class="remove"
                        on:mousedown={() => {
                            presentOptionalFields = presentOptionalFields.filter((f) => f !== field);
                            delete output[field];
                            output = { ...output }; // trigger reactivity
                        }}
                    >
                        ×
                    </span>
                {/if}
                {capitalize(field)}
            </label>
            {#if fieldType == 'text'}
                <input
                    id={field}
                    type="text"
                    bind:value={output[field]}
                    on:input={(e) => handleInput(field, e.target.value)}
                    required
                />
            {:else if fieldType == 'textarea'}
                <textarea
                    id={field}
                    bind:value={output[field]}
                    on:input={(e) => handleInput(field, e.target.value)}
                    required
                ></textarea>
            {:else if fieldType == 'number'}
                <input
                    id={field}
                    type="number"
                    bind:value={output[field]}
                    on:input={(e) => handleInput(field, e.target.value)}
                    required
                />
            {:else if fieldType == 'boolean'}
                <input
                    id={field}
                    type="checkbox"
                    checked={output[field] === "true"}
                    on:input={(e) => handleInput(field, (e.target).checked ? true : false)}
                />
            {:else if fieldType == 'date'}
                {@const dateString = output[field] ? new Date(output[field]).toISOString().split('T')[0] : ''}
                <!-- <input
                    id={field}
                    type="date"
                    value={dateString}
                    on:input={(e) => handleInput(field, new Date(e.target.value).getTime() * 1000)}
                    required
                /> -->
                <input
                    id={field}
                    type="date"
                    value={dateString}
                    on:input={(e) => handleInput(field, new Date(e.target.value))}
                    required
                />
            {:else if fieldType.includes('[]')}
              {@const realFieldType = fieldType.replace('[]', '')}
              {#if !Array.isArray(output[field])}
                {output[field] = [null]}
              {/if}
              {#each output[field] as selectedId, idx (idx)}
                <!-- <Dropdown
                  {apolloClient}
                  schemaType={realFieldType}
                  bind:selectedId={output[field][idx]}
                  on:change={(e) => {
                    output[field][idx] = e.detail;
                    output = { ...output }; // trigger reactivity
                  }}
                /> -->
                <div
                  class="form-field"
                  style="display: flex; flex-direction: row; align-items: center; gap: 0.5em"
                >
                  <input
                  type="text"
                  bind:value={output[field][idx]}
                  on:input={(e) => {
                    output[field][idx] = e.target.value;
                    output = { ...output }; // trigger reactivity
                  }}
                  placeholder={`Enter ${realFieldType} ID`}
                  />
                  <button
                  type="button"
                  style="background-color: rgb(95, 95, 95); color: #15141A; border: none; padding: 0.2em 0.5em; cursor: pointer;"
                  on:click={() => {
                    output[field].splice(idx, 1);
                    output = { ...output };
                  }}
                  >×</button>
                </div>
              {/each}
              <button
                type="button"
                on:click={() => {
                  output[field] = [...output[field], null];
                  output = { ...output };
                }}
                style="background-color: rgb(95, 95, 95); color: #15141A; border: none; padding: 0.2em 0.5em; cursor: pointer; color: #15141A;"
              >+ add id field</button>
            {:else}
                <Dropdown
                    {apolloClient}
                    schemaType={fieldType}
                    bind:selectedId={output[field]}
                />
            {/if}
        </div>
    {/each}
  <!-- {#each presentOptionalFields as field}
    <div class="form-field">
        <div style="display: flex; align-items: center; cursor: pointer;">
            <label for={field}>{field}</label>
                <span
                    on:mousedown={() => {
                        presentOptionalFields = presentOptionalFields.filter((f) => f !== field);
                        delete output[field];
                        output = { ...output }; // trigger reactivity
                    }}
                >
                    Remove
                </span>
        </div>
        <input
            id={field}
            type="text"
            bind:value={output[field]}
            on:input={(e) => handleInput(field, e.target.value)}
        />
    </div>
  {/each} -->
</div>

<style>
  .remove {
    cursor: pointer;
  }
  .form {
    display: flex;
    flex-direction: column;
    gap: 0.4em;
  }
  .form-field {
    display: flex;
    flex-direction: column;
    gap: 0.4em;
  }
  input, textarea {
    background-color: rgb(95, 95, 95);
    color: #15141A;
    border: none;
    outline: none;
    padding: 0.5em;
    width: calc(100% - 0.8em);
  }
  /* input {
    text-align: center;
  } */
  textarea {
    height: 40px;
  }
  label {
    font-weight: bold;
    color: rgb(95, 95, 95);
    display: flex;
  }
  label {
    text-align: center;
    gap: 2px;
  }
  label > span {
    font-size: 1em;
    border: 1px solid #454545;
    height: fit-content;
    padding: 1px 3px !important;
    margin-right: 0.4em;
  }
</style>
