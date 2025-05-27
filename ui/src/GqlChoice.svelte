<script lang="ts">
    import GqlForm from './GqlForm.svelte';
    import GqlTable from './GqlTable.svelte';
  import Graph from './Graph.svelte';
  import type { Writable } from 'svelte/store';
    export let apolloClient;
    export let fetch;
    export let file: Writable<any>;
    let selected;
    file.subscribe(value => {
        if (value) {
            selected = value["selected"];
        }
    });
    $: if (selected) {
        file.update(current => ({
          ...current,
          selected: selected
        }));
    }
</script>
<!-- <p>{name}: <input type="text" bind:value={$file} /></p> -->
<div class="root-flex-wrapper">
  <div class="root-flex-inner">
    <div class="root-flex">
      {#if selected}
          <div class="content-flex">
              {#if selected === 'form'}
                  <GqlForm {file} {apolloClient} {fetch} />
              {:else if selected === 'graph'}
                  <Graph />
              {:else if selected === 'table'}
                  <GqlTable {file} {apolloClient} {fetch} />
              {/if}
          </div>
      {:else}
          <h1>Step 1: Select a view type</h1>
          <div class="full-view">
              <button on:click={() => selected = 'form'}>Form</button>
              <button on:click={() => selected = 'table'}>Table</button>
              <button on:click={() => selected = 'graph'}>Graph</button>
          </div>
      {/if}
    </div>
  </div>
</div>

<style>
.root-flex-wrapper {
    container-type: inline-size;
    width: 100%;
    height: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: column;
}
.root-flex-inner {
    width: 100%;
    height: 100%;
    /* Default scale */
    transform: scale(1);
    transform-origin: top left;
    /* Prevent overflow from scaling */
    display: flex;
    flex-direction: column;
    /* flex: 1 1 0; */
    min-width: 0;
    min-height: 0;
    font-size: 1em;

}
/* @container (max-width: 600px) {
  .root-flex-inner {
    transform: translate(10%, 10%) scale(0.8);
  }
}
@container (max-width: 400px) {
  .root-flex-inner {
    transform: translate(20%, 20%) scale(0.6);
  }
}
@container (max-width: 200px) {
  .root-flex-inner {
    transform: translate(30%, 30%) scale(0.4);
  }
}
@container (max-width: 100px) {
  .root-flex-inner {
    transform: translate(40%, 40%) scale(0.2);
  }
} */
 @container (max-width: 600px) {
  .root-flex-inner {
    font-size: 0.8em;
  }
}
@container (max-width: 400px) {
  .root-flex-inner {
    font-size: 0.6em;
  }
}
@container (max-width: 200px) {
  .root-flex-inner {
    font-size: 0.4em;
  }
}
@container (max-width: 100px) {
  .root-flex-inner {
    font-size: 0.2em;
  }
}
.root-flex {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    min-width: 0;
    min-height: 0;
    /* flex: 1 1 0; */
    overflow: auto;
}
.content-flex {
    display: flex;
    /* flex: 1 1 0; */
    min-width: 0;
    min-height: 0;
    overflow: auto;
}
h1 {
    min-height: 30px;
    color:  var(--text-color);
    font-size: clamp(1em, 2vw, 1.5em);
    text-align: center;
    margin: 0.4em;
    /* flex-shrink: 1; */
}
.full-view {
    display: flex;
    flex-direction: row;
    height: 100%;
    width: 100%;
    justify-content: center;
    align-items: stretch;
    margin: 0;
    padding: 0;
    min-width: 0;
    min-height: 0;
    /* flex-shrink: 1; */
}
.full-view button {
    /* flex: 1 1 0; */
    min-width: 0;
    min-height: 0;
    width: 100%;
    /* font-size: clamp(1em, 2vw, 1.5rem); */
    font-size: 1em;
    cursor: pointer;
    color:  var(--text-color);
    background-color: rgb(28, 28, 28);
    border: 1px solid rgb(79, 79, 79);
    /* flex-shrink: 1; */
}
.full-view button:hover {
    background-color: rgb(16, 16, 16);
}
:global(.golden-layout-container), :global(.golden-layout-item) {
    min-width: 0;
    min-height: 0;
    /* flex-shrink: 1; */
    overflow: auto;
}
</style>