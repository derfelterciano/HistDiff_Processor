<script lang="ts">
	import DendrogramCanvas from "./clusterMap/dendrogramCanvas.svelte";
	import { onMount } from "svelte";
	import { panZoom, type PanZoom } from "./clusterMap/panZoomStore.svelte";
	import type { D3Node } from "../types/clustermapTypes";
	import { loadTreeData, loadHeatmapData } from "./clusterMap/data";

	let treeData: D3Node | null = null;

	// row-dendrogram dimensions
	const treeWidth = 200;
	const treeHeight = 1100;

	// WARNING: Test URLS
	const tree: string = "/tree.json";
	const scores: string = "/scores.json";

	const zoomIn = () => {
		panZoom.zoom(1.2);
	};
	const zoomOut = () => {
		panZoom.zoom(0.8);
	};
	const reset = () => {
		panZoom.reset();
	};

	onMount(async () => {
		treeData = await loadTreeData(tree);
	});
</script>

<div class="controls">
	<button
		class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
		type="button"
		on:click={zoomIn}>Zoom In</button
	>
	<button
		class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
		type="button"
		on:click={zoomOut}>Zoom Out</button
	>
	<button
		class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
		type="button"
		on:click={reset}>Reset</button
	>
</div>

{#if treeData}
	<DendrogramCanvas
		{treeData}
		orientation="left"
		width={treeWidth}
		height={treeHeight}
	/>
{:else}
	<p>Loading Dendrograms...</p>
{/if}
