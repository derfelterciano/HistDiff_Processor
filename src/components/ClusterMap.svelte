<script lang="ts">
	import * as d3 from "d3";
	import { onDestroy, onMount, tick } from "svelte";
	import type { D3Node } from "../types/clustermapTypes";
	import type { HierarchyPointNode, HierarchyPointLink } from "d3-hierarchy";

	const tree: string = "/tree.json";
	const scores: string = "/scores.json";

	let d3_tree: D3Node;
	let canvas: HTMLCanvasElement;

	let heatmapRaw: Record<string, Record<string, number>>;
	let heatmapData: {
		rows: string[];
		cols: string[];
		cells: { row: string; col: string; value: number }[];
	} = { rows: [], cols: [], cells: [] };

	async function fetch_json<T>(path: string): Promise<T> {
		const response = await fetch(path);
		return response.json() as T;
	}

	let treeWidth: number, cellWidth: number, cellHeight: number;
	let totalWidth: number, totalHeight: number;

	let contrast: { contMin: number; contMax: number } = {
		contMin: 0,
		contMax: 1,
	};

	let scale = 1;
	let translateX = 0;
	let translateY = 0;
	const MIN_SCALE = 0.2,
		MAX_SCALE = 10;

	onMount(async () => {
		// grab tree
		const rawTree = await fetch_json<D3Node>(tree);
		console.log(rawTree);

		d3_tree = rawTree;

		// grab heatmap data
		const rawHeatData =
			await fetch_json<Record<string, Record<string, number>>>(scores);
		heatmapRaw = rawHeatData;
		// console.log(heatmapRaw);

		//process heatmap data
		heatmapData.rows = Object.keys(heatmapRaw);
		if (heatmapData.rows.length > 0) {
			heatmapData.cols = Object.keys(heatmapRaw[heatmapData.rows[0]]);
		}
		heatmapData.rows.forEach((row: string) => {
			heatmapData.cols.forEach((col: string) => {
				heatmapData.cells.push({
					row,
					col,
					value: heatmapRaw[row][col],
				});
			});
		});

		console.log(heatmapData);

		const vals: number[] = heatmapData.cells.map((d) => d.value);
		contrast.contMin = d3.min(vals) ?? 0;
		contrast.contMax = d3.max(vals) ?? 1;

		// get dimensions
		// const heatmapWidth = heatmapData.cols.length * cellSize;
		// const heatmapHeight = heatmapData.rows.length * cellSize;
		// totalWidth = treeWidth + heatmapWidth;
		// totalHeight = Math.max(heatmapHeight, 600);

		await tick();
		window.addEventListener("resize", resizeCanvas);
		resizeCanvas();

		canvas.addEventListener("wheel", handleWheel, { passive: false });
	});

	onDestroy(() => {
		canvas.removeEventListener("wheel", handleWheel);
		window.removeEventListener("resize", resizeCanvas);
	});

	function resetView() {
		scale = 1;
		translateX = 0;
		translateY = 0;

		resizeCanvas();
	}

	function handleWheel(evt: WheelEvent) {
		evt.preventDefault();
		const zoomFactor = evt.deltaY < 0 ? 1.1 : 0.9;
		const rect = canvas.getBoundingClientRect();
		const mx = evt.clientX - rect.left;
		const my = evt.clientY - rect.top;
		const newScale = Math.min(
			MAX_SCALE,
			Math.max(MIN_SCALE, scale * zoomFactor),
		);
		const actualZoom = newScale / scale;
		scale = newScale;
		translateX = mx - actualZoom * (mx - translateX);
		translateY = my - actualZoom * (my - translateY);
		drawCanvas();
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

		drawCanvas();
	}

	// drawing canvas functions

	function drawCanvas() {
		if (!canvas) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		ctx.clearRect(0, 0, totalWidth, totalHeight);
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		ctx.save();
		ctx.translate(translateX, translateY);
		ctx.scale(scale, scale);

		const root = d3.hierarchy<D3Node>(d3_tree);
		const cluster = d3.cluster<D3Node>().size([totalHeight, treeWidth]);
		cluster(root);

		const leafOrder: string[] = root.leaves().map((n) => n.data.name!);

		// canvas.width = totalWidth;
		// canvas.height = totalHeight;

		drawTree(ctx, root);
		drawHeatmap(ctx, leafOrder);

		ctx.restore();
	}

	function drawTree(
		ctx: CanvasRenderingContext2D,
		root: d3.HierarchyNode<D3Node>,
	) {
		ctx.strokeStyle = "#ccc";
		root.links().forEach((link: d3.HierarchyLink<D3Node>) => {
			ctx.beginPath();
			ctx.moveTo(link.source.y as number, link.source.x as number);
			ctx.lineTo(link.source.y as number, link.target.x as number);
			ctx.lineTo(link.target.y as number, link.target.x as number);
			ctx.stroke();
		});

		// Draw nodes
		root.descendants().forEach((node) => {
			ctx.beginPath();
			ctx.arc(node.y as number, node.x as number, 2, 0, 2 * Math.PI);
			ctx.fillStyle = node.children ? "steelblue" : "orange";
			ctx.fill();
		});
	}

	function drawHeatmap(ctx: CanvasRenderingContext2D, rowOrder: string[]) {
		// const colorScale = d3
		// 	.scaleSequential(d3.interpolateViridis)
		// 	.domain([contrast.contMin, contrast.contMax]);

		const colorScale = d3
			.scaleSequential(d3.interpolateViridis)
			.domain([-0.005, 0.005]);

		// draw each cell
		rowOrder.forEach((row, r) => {
			heatmapData.cols.forEach((col, c) => {
				const value = heatmapRaw[row][col];
				const x = treeWidth + c * cellWidth;
				const y = r * cellHeight;
				ctx.fillStyle = colorScale(value);
				ctx.fillRect(x, y, cellWidth, cellHeight);
			});
		});
	}

	function drawSVG() {}
</script>

<h1>Cluster Map</h1>
<button
	type="button"
	class="bg-red-400 hover:bg-red-300 rounded p-1"
	onclick={resetView}>Reset View</button
>
<canvas bind:this={canvas}></canvas>
