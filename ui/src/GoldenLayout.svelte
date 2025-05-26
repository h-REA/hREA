<script lang="ts">
  import { onMount } from 'svelte';
  import 'svelte-golden-layout/css/themes/goldenlayout-dark-theme.css';
  import GoldenLayout from 'svelte-golden-layout';
  import GqlChoice from './GqlChoice.svelte';
  import { LayoutConfig, type VirtualLayout } from 'golden-layout';
	// import { propertyStore } from 'svelte-writable-derived';

  export let apolloClient: any;

  let goldenLayout: VirtualLayout
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
    const layoutState = goldenLayout.saveLayout();
    localStorage.setItem('savedState', JSON.stringify(layoutState));


    // console.log(JSON.stringify(goldenLayout));
    
    // goldenLayout.on( 'stateChanged', function(){
    //   console.log('State changed');
    //   const state = JSON.stringify(goldenLayout.toConfig());
    //   localStorage.setItem('savedState', state);
    // });

    // goldenLayout?.registerComponent('PersistentComponent', components.PersistentComponent);
  }

  function addWindow() {
    const state = goldenLayout.saveLayout();
    layout = LayoutConfig.fromResolved(state!);
    console.log("here is the state", state)
    layout.root.content = [
      ...layout.root.content,
        {
					type: 'component',
					title: "name",
					componentType: 'Test',
					componentState: {
						name: "name",
						// file: propertyStore(files, name),
					},
        }
    ]
  }


  onMount(() => {
    // const savedState = localStorage.getItem('savedState');
    // layout = savedState ? JSON.parse(savedState) : {
    layout = {
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

<button on:click={() => {
  console.log(layout)
  // saveLayout();
  addWindow();
}}>+ Add window</button>

{#if layout}
  <div class="layout-container">
    <GoldenLayout bind:goldenLayout bind:config={layout} let:componentType let:componentState>
      <!-- {#if componentType === 'PersistentComponent'}
        <div>{JSON.stringify(componentState)}</div>
      {/if} -->
      <GqlChoice {apolloClient} />
    </GoldenLayout>
  </div>
{/if}

<style>
.layout-container {
  width: 100vw;
  height: calc(100vh - 54px);
  padding: 2px;
}
:global(body) {
  margin: 0;
  padding: 0;
  font-family: sans-serif;
}
</style>