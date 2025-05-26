<script lang="ts">
  export let presentOptionalFields: string[] = [];

  // All the fields the user could potentially add
  export let optionalFields;

  let selected = '';

  function addField() {
    if (selected && !presentOptionalFields.includes(selected)) {
      presentOptionalFields = [...presentOptionalFields, selected];
    }
    selected = '';
  }
</script>

<div class="field-selector">
    {#if optionalFields.length > presentOptionalFields.length}
    <select id="field-select" bind:value={selected} on:click={addField}>
        <option value="" disabled selected>+ add a field</option>
        {#each optionalFields as field}
        {#if !presentOptionalFields.includes(field)}
            <option value={field}>{field}</option>
        {/if}
        {/each}
    </select>
  {/if}
</div>

<style>
  .field-selector {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  select {
    /* padding: 0.25em; */
    background-color: rgb(95, 95, 95);
    background-color: transparent;
    border: 1px solid transparent;
    color: rgb(95, 95, 95);
    cursor: pointer;
    /* width: 100%; */
    margin: auto;
    text-align: center;
    padding: 0.5em;
  }
  select:hover {
    /* background-color: rgba(85, 85, 85, 0.163); */
    border: 1px solid rgb(95, 95, 95);
  }
</style>
