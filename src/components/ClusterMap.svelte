<script lang="ts">
	import DendrogramCanvas from "./clusterMap/dendrogramCanvas.svelte";
	import { onMount } from "svelte";
	import { panZoom, type PanZoom } from "./clusterMap/panZoomStore.svelte";
	import type { D3Node } from "../types/clustermapTypes";
	import {
		loadTreeData,
		loadHeatmapData,
		type HeatmapData,
		loadHeatmap,
	} from "./clusterMap/data";
	import HeatmapCanvas from "./clusterMap/heatmapCanvas.svelte";
	import { getLeafOrder } from "./clusterMap/utils";

	let treeData = $state<D3Node | null>(null);
	let rawHeatmapData = $state<Record<string, Record<string, number>> | null>(
		null,
	);
	let heatmapData = $state<HeatmapData | null>(null);

	// WARNING: Test URLS
	const tree: string = "/tree.json";
	const scores: string = "/scores.json";
	const TREE_WIDTH = 200;

	// panel dimensions
	let treeWidth = $state<number>(TREE_WIDTH);
	let treeHeight = $state<number>(window.innerHeight);
	let heatmapWidth = $state<number>(window.innerWidth - TREE_WIDTH);
	let heatmapHeight = $state<number>(treeHeight);

	// load data
	// $effect(async () => {
	// 	treeData = await loadTreeData(tree);
	// 	rawHeatmapData = await loadHeatmapData(scores);
	// 	heatmapData = loadHeatmap(rawHeatmapData!);
	// });

	$effect(() => {
		loadTreeData(tree).then((data) => (treeData = data));
		loadHeatmapData(scores).then((raw) => {
			rawHeatmapData = raw;
			heatmapData = loadHeatmap(raw);
		});
	});

	// recalculate dimensions
	$effect(() => {
		const onR = () => {
			treeHeight = window.innerHeight;
			heatmapWidth = window.innerWidth - TREE_WIDTH;
			heatmapHeight = treeHeight;
		};
		window.addEventListener("resize", onR);
		return () => window.removeEventListener("resize", onR);
	});

	// WARNING: Temporary zoom controls
	const zoomIn = () => {
		panZoom.zoom(1.2);
	};
	const zoomOut = () => {
		panZoom.zoom(0.8);
	};
	const reset = () => {
		panZoom.reset();
	};

	// derive leaf ordering
	let rowOrder = $derived(() => {
		if (!treeData) return;
		return getLeafOrder(treeData);
	});
	let colOrder = $derived(() => (heatmapData ? heatmapData.cols : []));

	// onMount(async () => {
	// 	treeData = await loadTreeData(tree);
	// 	rawHeatmapData = await loadHeatmapData(scores);
	// 	heatmapData = loadHeatmap(rawHeatmapData);
	// });
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

<div class="clustermap flex flex-1 overflow-hidden">
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

	{#if heatmapData && rawHeatmapData && rowOrder().length}
		<!-- Heatmap -->
		<div class="panel heatmap-panel">
			<HeatmapCanvas
				data={heatmapData}
				raw={rawHeatmapData}
				rowOrder={rowOrder()}
				colOrder={colOrder()}
				width={heatmapWidth}
				height={heatmapHeight}
			/>
		</div>
	{:else if treeData}
		<p>Loading heatmap…</p>
	{/if}
</div>
