<script lang="ts">
	import DendrogramCanvas from "./clusterMap/dendrogramCanvas.svelte";
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
	import RowLabels from "./clusterMap/rowLabels.svelte";
	import ColumnLabels from "./clusterMap/columnLabels.svelte";

	const { containerHeight } = $props<{ containerHeight: number }>();

	// let containerHeight = $state<number>(window.innerHeight);

	let treeData = $state<D3Node | null>(null);
	let featTreeData = $state<D3Node | null>(null);
	let rawHeatmapData = $state<Record<string, Record<string, number>> | null>(
		null,
	);
	let heatmapData = $state<HeatmapData | null>(null);

	// WARNING: Test URLS
	const tree: string = "/row_tree.json";
	const scores: string = "/scores.json";
	const featureTree: string = "/feat_tree.json";

	// Dimensions
	const TREE_WIDTH = 200;
	const LABEL_WIDTH = 50;

	// panel dimensions
	let treeWidth = $state<number>(TREE_WIDTH);
	// let treeHeight = $state<number>(window.innerHeight);
	let treeHeight = $state<number>(0);
	let heatmapWidth = $state<number>(
		window.innerWidth - TREE_WIDTH - LABEL_WIDTH,
	);
	let heatmapHeight = $state<number>(treeHeight);
	let labelW = $state<number>(LABEL_WIDTH);

	// headers
	let controlsE1 = $state<HTMLElement>();
	let contentE1 = $state<HTMLDivElement>();

	// load data
	$effect(() => {
		loadTreeData(tree).then((data) => (treeData = data));
		loadTreeData(featureTree).then((data) => (featTreeData = data));
		loadHeatmapData(scores).then((raw) => {
			rawHeatmapData = raw;
			heatmapData = loadHeatmap(raw);
		});
	});

	// recalculate dimensions
	$effect(() => {
		const onR = () => {
			// treeHeight = window.innerHeight;
			const headerH = controlsE1?.clientHeight ?? 0;
			const availH = containerHeight - headerH;

			treeHeight = availH;
			heatmapWidth = window.innerWidth - TREE_WIDTH - LABEL_WIDTH;
			heatmapHeight = treeHeight;
		};

		window.addEventListener("resize", onR);
		onR();
		return () => window.removeEventListener("resize", onR);
	});

	// bound effect
	// const contentW = $derived(() => {
	// 	return (TREE_WIDTH + LABEL_WIDTH + heatmapWidth) * $panZoom.scale;
	// });
	// const contentH = $derived(() => {
	// 	return Math.max(treeHeight, heatmapHeight) * $panZoom.scale;
	// });
	//
	// $effect(() => {
	// 	panZoom.setMinScale(1);
	// 	const id = setTimeout(() => {
	// 		panZoom.setBounds(heatmapWidth, contentH(), contentW(), contentH());
	// 	}, 0);
	//
	// 	return () => clearTimeout(id);
	// });

	$effect(() => {
		const viewW = contentE1?.clientWidth;
		const viewH = contentE1?.clientHeight;

		const contentW = heatmapWidth;
		const contentH = Math.max(treeHeight, heatmapHeight);

		panZoom.setMinScale(1);
		panZoom.setBounds(heatmapWidth, contentH, contentW, contentH);
		// panZoom.setBounds(contentW, contentH, viewW as number, viewH as number);
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
	let rowOrder = $derived((): string[] => {
		return treeData ? getLeafOrder(treeData) : [];
	});
	// let colOrder = $derived((): string[] => (heatmapData ? heatmapData.cols : []));
	let colOrder = $derived((): string[] => {
		return featTreeData ? getLeafOrder(featTreeData) : [];
	});

	const cellH = $derived(() => {
		const rows = (rowOrder() as string[]).length;
		return rows ? heatmapHeight / rows : 0;
	});

	const cellW = $derived(() => {
		const cols = (colOrder() as string[]).length;
		return cols ? heatmapWidth / cols : 0;
	});
	// onMount(async () => {
	// 	treeData = await loadTreeData(tree);
	// 	rawHeatmapData = await loadHeatmapData(scores);
	// 	heatmapData = loadHeatmap(rawHeatmapData);
	// });
</script>

<div class="controls" bind:this={controlsE1}>
	<button
		class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
		type="button"
		onclick={zoomIn}>Zoom In</button
	>
	<button
		class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
		type="button"
		onclick={zoomOut}>Zoom Out</button
	>
	<button
		class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
		type="button"
		onclick={reset}>Reset</button
	>
</div>

{#if featTreeData}
	<div style="padding-left: {TREE_WIDTH}px; padding-right: {LABEL_WIDTH}px;">
		<DendrogramCanvas
			treeData={featTreeData}
			height={200}
			width={heatmapWidth}
			orientation="top"
		/>
	</div>
{/if}

{#if colOrder()}
	<div style="padding-left: {TREE_WIDTH}px; padding-right: {LABEL_WIDTH}px;">
		<ColumnLabels
			colOrder={colOrder()}
			height={100}
			width={heatmapWidth}
			cellWidth={cellW()}
		></ColumnLabels>
	</div>
{/if}

<div class="basic-clustermap flex overflow-hidden" bind:this={contentE1}>
	{#if treeData}
		<div class="flex-none">
			<DendrogramCanvas
				{treeData}
				orientation="left"
				width={treeWidth}
				height={treeHeight}
			/>
		</div>
	{:else}
		<p>Loading Dendrograms...</p>
	{/if}

	{#if heatmapData && rawHeatmapData && (rowOrder() as string[]).length}
		<!-- Heatmap -->
		<div class="flex-1">
			<HeatmapCanvas
				data={heatmapData}
				raw={rawHeatmapData}
				rowOrder={rowOrder() as string[]}
				colOrder={colOrder() as string[]}
				width={heatmapWidth}
				height={heatmapHeight}
			/>
		</div>
	{:else if treeData}
		<p>Loading heatmap…</p>
	{/if}

	{#if (rowOrder() as string[]).length}
		<div class="flex-none">
			<RowLabels
				rowOrder={rowOrder() as string[]}
				cellHeight={cellH()}
				width={labelW}
				height={treeHeight}
			/>
		</div>
	{/if}
</div>

<style>
	@reference 'tailwindcss';

	.basic-clustermap {
		@apply h-[100%];
	}
</style>
