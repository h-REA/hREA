<script lang="ts">
	import type { ComponentType } from 'svelte';
	import type { Writable } from 'svelte/store';
	import { writable } from 'svelte/store';
	import { propertyStore } from 'svelte-writable-derived';

	import type { JsonValue, ResolvedLayoutConfig, VirtualLayout } from 'golden-layout';
	import { LayoutConfig } from 'golden-layout';
    import 'svelte-golden-layout/css/themes/goldenlayout-dark-theme.css';
    import GoldenLayout from 'svelte-golden-layout';
	import Test from './GqlChoice.svelte';

    export let apolloClient;
	export let fetch;

	const names = ['foo', 'bar', 'baz'];
	const files = writable<{ [name: string]: string }>({
		foo: '',
		bar: '',
		baz: '',
	});

	function fileStore(name: string): Writable<string> {
		return propertyStore(files, name);
	}

	let display = true;
	let rows = 1;

	const components: Record<string, ComponentType> = { Test };

	// let layout: LayoutConfig;
	let saved: ResolvedLayoutConfig | void = undefined;

	let goldenLayout: VirtualLayout;

	// $: layout = {
	// 	root: {
	// 		type: 'column',
	// 		content: Array.from({ length: rows }, (_, i) => {
	// 			const name = names[i % names.length];

	// 			return {
	// 				type: 'component',
	// 				title: name,
	// 				componentType: 'Test',
	// 				componentState: {
	// 					name,
	// 					file: propertyStore(files, name),
	// 				},
	// 			};
	// 		}),
	// 	},
	// };

    let layout: LayoutConfig = {
        root: {
			type: 'column',
			content: Array.from({ length: rows }, (_, i) => {
				const name = names[i % names.length];

				return {
					type: 'component',
					title: name,
					componentType: 'Test',
					componentState: {
						name,
						file: fileStore(name),
					},
				};
			}),
		},
	}

	function handleSave() {
		saved = goldenLayout.saveLayout();
	}

	function handleRestore() {
		layout = LayoutConfig.fromResolved(saved!);
	}
	
	export function addWindow() {
		handleSave();
		saved["root"]["content"].push(saved["root"]["content"][0])
		handleRestore();
	}

	// work around limitation that there's No TS in markup
	// see https://svelte.dev/docs/typescript#limitations
	function castComponentState(componentState: JsonValue | undefined): object | undefined {
		return componentState as object | undefined;
	}
</script>

<!-- <button
	on:click={() => {
		handleSave();
		saved["root"]["content"].push(saved["root"]["content"][0])
		handleRestore();
	}}
>
	Add Component
</button> -->

<!-- <main>
	<div>
		<p>
			<input type="checkbox" bind:checked={display} /> display
			<input type="number" min="0" max="10" bind:value={rows} /> columns
			<button
				on:click={() => {
					handleSave();
					saved["root"]["content"].push(saved["root"]["content"][0])
					handleRestore();
				}}
			>
				Add Component
			</button>
			<button on:click={handleSave}>Save Layout</button>
			<button on:click={handleRestore} disabled={saved === undefined}>Restore Layout</button>
		</p>
		{#each Object.entries($files) as [name, content]}
			<p>{name}: {content}</p>
		{/each}
		<h2>Saved Layout</h2>
		{#if saved !== undefined}
			<pre>{JSON.stringify(saved, undefined, 2)}</pre>
		{:else}
			<p>(none)</p>
		{/if}
	</div>
</main> -->
<div class="layout-container">
	{#if display}
		<GoldenLayout config={layout} bind:goldenLayout let:componentType let:componentState>
			<svelte:component
				{apolloClient}
				{fetch}
				this={components[componentType]}
				{...castComponentState(componentState)}
			/>
		</GoldenLayout>
	{/if}
</div>

<style>
	.layout-container {
		height: calc(100vh - 54px);
		border-top: 1px solid rgb(79, 79, 79);
	}
</style>