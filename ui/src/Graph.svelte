<script lang="ts">
  import { writable } from 'svelte/store';
  import { SvelteFlow, Controls, Background, BackgroundVariant, MiniMap } from '@xyflow/svelte';

  // you need to import the styles for Svelte Flow to work
  // if you just want to load the basic styleds, you can import '@xyflow/svelte/dist/base.css'
  import '@xyflow/svelte/dist/style.css';

  // We are using writables for the nodes and edges to sync them easily. When a user drags a node for example, Svelte Flow updates its position. This also makes it easier to update nodes in user land.
  const nodes = writable([
    {
      id: '1',
      type: 'input',
      data: { label: 'Process A' },
      position: { x: 0, y: 0 }
    },
    {
      id: 'inputs',
      type: 'custom',
      data: { label: 'Inputs' },
      position: { x: 0, y: 50 }
    },
    {
      id: '2',
      type: 'custom',
      data: { label: 'Resource A' },
      position: { x: 0, y: 150 }
    },
    {
      id: '3',
      type: 'custom',
      data: { label: 'Resource B' },
      position: { x: 0, y: 250 }
    },
    {
      id: '4',
      type: 'custom',
      data: { label: 'Resource C' },
      position: { x: 0, y: 350 }
    },
    {
      id: 'outputs',
      type: 'custom',
      data: { label: 'Outputs' },
      position: { x: 0, y: 450 }
    },
    {
      id: '5',
      type: 'custom',
      data: { label: 'Output Resource A' },
      position: { x: 0, y: 550 }
    }
  ]);

  // same for edges
  const edges = writable([
    {
      id: '1-2',
      type: 'default',
      source: '1',
      target: 'inputs',
      // label: 'Edge Text'
    },
    {
      id: '1-3',
      type: 'default',
      source: 'inputs',
      target: '2',
      // label: 'Edge Text'
    },
    {
      id: '1-inputs',
      type: 'default',
      source: '1',
      target: 'inputs',
      // label: 'Edge Text'
    },
    {
      id: '2-3',
      type: 'default',
      source: 'inputs',
      target: '3',
      // label: 'Edge Text'
    },
    {
      id: '2-4',
      type: 'default',
      source: 'inputs',
      target: '4',
      // label: 'Edge Text'
    },
    {
      id: '1-outputs',
      type: 'default',
      source: '1',
      target: 'outputs',
      // label: 'Edge Text'
    },
    {
      id: 'outputs-5',
      type: 'default',
      source: 'outputs',
      target: '5',
      // label: 'Edge Text'
    }
  ]);
</script>

<div id="graph-container">
  <SvelteFlow {nodes} {edges} fitView on:nodeclick={(event) => console.log('on node click', event)}>
    <Controls />
    <Background variant={BackgroundVariant.Dots} />
    <MiniMap />
  </SvelteFlow>
</div>

<style>
  #graph-container {
    width: 100%;
    height: 600px;
    position: relative;
  }
</style>