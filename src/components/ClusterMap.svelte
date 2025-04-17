<script lang="ts">
	import * as d3 from "d3";
	import { onDestroy, onMount, tick } from "svelte";
	import type { D3Node } from "../types/clustermapTypes";
	import type { HierarchyPointNode, HierarchyPointLink } from "d3-hierarchy";
	import {
		loadTreeData,
		loadHeatmapData,
		loadHeatmap,
		computeContrast,
		type HeatmapData,
	} from "./clusterMap/data.svelte";
	import {
		computeTreeLayout,
		type TreeLayout,
	} from "./clusterMap/layouts.svelte";
	import { drawTree, drawHeatmap } from "./clusterMap/draw.svelte";
	import {
		initPanZoom,
		type PanZoomState,
	} from "./clusterMap/panZoom.svelte";
	import {
		createHeatmapBuff,
		createTreeBuff,
		blitBuffers,
	} from "./clusterMap/canvasBuffers.svelte";

	// WARNING: Test URLS
	const tree: string = "/tree.json";
	const scores: string = "/scores.json";

	// DOM
	let d3_tree: D3Node;
	let canvas: HTMLCanvasElement;

	// Data
	let heatmapRaw: Record<string, Record<string, number>>;
	let heatmapData: HeatmapData;
	let contrast: { min: number; max: number };

	// layout
	let treeWidth: number, cellWidth: number, cellHeight: number;
	let totalWidth: number, totalHeight: number;

	// pan + zoom
	const panZoom: PanZoomState = {
		scale: 1,
		tx: 0,
		ty: 0,
		minScale: 0.2,
		maxScale: 10,
	};
	let cleanUpPanZoom: () => void;

	//cached layout
	let layout: TreeLayout;
	let treeBuf: HTMLCanvasElement;
	let heatmapBuff: HTMLCanvasElement;

	onMount(async () => {
		// grab tree
		const rawTree = await loadTreeData(tree);
		console.log(rawTree);

		d3_tree = rawTree;

		// grab heatmap data
		const rawHeatData = await loadHeatmapData(scores);
		heatmapRaw = rawHeatData;
		heatmapData = loadHeatmap(rawHeatData);
		console.log(heatmapData);

		// contrast
		// contrast = computeContrast(heatmapData.cells);
		contrast = { min: -0.005, max: 0.005 };

		//set up canvas
		await tick();
		window.addEventListener("resize", resizeCanvas);
		resizeCanvas();
		cleanUpPanZoom = initPanZoom(canvas, panZoom, drawCanvas);
	});

	onDestroy(() => {
		window.removeEventListener("resize", resizeCanvas);
		cleanUpPanZoom();
	});

	function resetView() {
		panZoom.scale = 1;
		panZoom.tx = 0;
		panZoom.ty = 0;
		resizeCanvas();
	}

	function resizeCanvas() {
		if (!canvas) return;
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;

		totalWidth = canvas.width;
		totalHeight = canvas.height;

		const treeSpace: number = 0.1;
		treeWidth = Math.max(100, totalWidth * treeSpace);

		const heatmapCols = heatmapData.cols.length;
		const heatmapRows = heatmapData.rows.length;
		cellWidth = (totalWidth - treeWidth) / heatmapCols;
		cellHeight = totalHeight / heatmapRows;

		// drawCanvas();

		layout = computeTreeLayout(d3_tree, totalHeight, treeWidth);
		treeBuf = createTreeBuff(layout, treeWidth, totalHeight);
		heatmapBuff = createHeatmapBuff(
			heatmapData,
			heatmapRaw,
			layout.leafOrder,
			contrast,
		);
	}

	// drawing canvas functions

	function drawCanvas() {
		if (!canvas) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		// pan + zoom
		ctx.save();
		ctx.clearRect(0, 0, totalWidth, totalHeight);
		ctx.translate(panZoom.tx, panZoom.ty);
		ctx.scale(panZoom.scale, panZoom.scale);

		layout = computeTreeLayout(d3_tree, totalHeight, treeWidth);

		blitBuffers(
			ctx,
			treeBuf,
			heatmapBuff,
			treeWidth,
			cellWidth,
			cellHeight,
		);

		ctx.restore();
	}
</script>

<h1>Cluster Map</h1>
<button
	type="button"
	class="bg-red-400 hover:bg-red-300 rounded p-1"
	onclick={resetView}>Reset View</button
>
<canvas bind:this={canvas}></canvas>
