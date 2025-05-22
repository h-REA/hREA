<script lang="ts">
  import { onMount } from 'svelte';
  import 'svelte-golden-layout/css/themes/goldenlayout-light-theme.css';
  import GoldenLayout from 'svelte-golden-layout';
  import { LayoutConfig } from 'golden-layout';

  let goldenLayoutInstance;
  let layout: LayoutConfig;

  const components = {
    "Entry": "Hello entry",
    "PersistentComponent": (container, state) => {
      const input = document.createElement('input');
      input.type = 'text';
      input.value = state.label || '';
      input.addEventListener('change', () => {
        container.setState({ label: input.value });
      });
      container.getElement().appendChild(input);
    }
  };

  function saveLayout() {
    // const layoutState = goldenLayoutInstance.toConfig();
    // localStorage.setItem('savedState', JSON.stringify(layoutState));


    // console.log(JSON.stringify(goldenLayoutInstance));
    // goldenLayoutInstance.on('stateChanged', () => {
    //   const state = JSON.stringify(goldenLayoutInstance.toConfig());
    //   localStorage.setItem('savedState', state);
    // });

    // goldenLayoutInstance?.registerComponent('PersistentComponent', components.PersistentComponent);
  }

  onMount(() => {
    const savedState = localStorage.getItem('savedState');
    layout = savedState ? JSON.parse(savedState) : {
      root: {
        type: 'row',
        content: [
          {
            type: 'component',
            componentType: 'PersistentComponent',
            componentState: { label: 'A' },
          },
          {
            type: 'column',
            content: [
              {
                type: 'component',
                componentType: 'PersistentComponent',
                componentState: { label: 'B' },
              },
              {
                type: 'component',
                componentType: 'PersistentComponent',
                componentState: { label: 'C' },
              },
            ],
          },
        ],
      },
    };
  });
</script>

<div id="header">
  <img src="/logo.jpeg" alt="Logo" style="height: 36px; margin-right: 20px;">
  <h1>hREA explorer</h1>
  <button on:click={() => {
    console.log(layout)
    saveLayout();
  }}>+ Add window</button>
</div>

{#if layout}
<div class="layout-container">
  <GoldenLayout bind:this={goldenLayoutInstance} bind:config={layout} let:componentType let:componentState>
    {#if componentType === 'PersistentComponent'}
      <div>{JSON.stringify(componentState)}</div>
    {/if}
  </GoldenLayout>
</div>
{/if}

<style>
.layout-container {
  width: calc(100vw - 4px);
  height: calc(100vh - 54px);
  padding: 2px;
}
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