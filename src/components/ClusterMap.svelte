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

	const treeWidth = 100;
	const cellSize = 10;
	let totalWidth: number, totalHeight: number;

	let contrast: { contMin: number; contMax: number } = {
		contMin: 0,
		contMax: 1,
	};

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
		const heatmapWidth = heatmapData.cols.length * cellSize;
		const heatmapHeight = heatmapData.rows.length * cellSize;
		totalWidth = treeWidth + heatmapWidth;
		totalHeight = Math.max(heatmapHeight, 600);

		resizeCanvas();
		window.addEventListener("resize", resizeCanvas);
	});

	onDestroy(() => {
		window.removeEventListener("resize", resizeCanvas);
	});

	function resizeCanvas() {
		if (!canvas) return;
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;

		totalWidth = canvas.width;
		totalHeight = canvas.height;

		drawCanvas();
	}

	// drawing canvas functions

	function drawCanvas() {
		if (!canvas) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		const root = d3.hierarchy<D3Node>(d3_tree);
		const cluster = d3.cluster<D3Node>().size([totalHeight, treeWidth]);
		cluster(root);

		const leafOrder: string[] = root.leaves().map((n) => n.data.name!);

		canvas.width = totalWidth;
		canvas.height = totalHeight;
		ctx.clearRect(0, 0, totalWidth, totalHeight);

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
				const x = treeWidth + c * cellSize;
				const y = r * cellSize;
				ctx.fillStyle = colorScale(value);
				ctx.fillRect(x, y, cellSize, cellSize);
			});
		});
	}

	function drawSVG() {}

	// function drawDendrogram() {
	// 	if (!d3_tree) return;
	//
	// 	svgDiv.innerHTML = "";
	//
	// 	const width = 600;
	// 	const height = 1000;
	// 	const margin = { top: 20, right: 90, bottom: 30, left: 90 };
	//
	// 	const svg = d3
	// 		.select(svgDiv)
	// 		.append("svg")
	// 		.attr("width", width + margin.left + margin.right)
	// 		.attr("height", height + margin.top + margin.bottom);
	//
	// 	const g = svg
	// 		.append("g")
	// 		.attr("transform", `translate(${margin.left},${margin.top})`);
	//
	// 	const root = d3.hierarchy<D3Node>(d3_tree);
	//
	// 	const cluster = d3
	// 		.cluster<D3Node>()
	// 		.size([
	// 			height - margin.top - margin.bottom,
	// 			width - margin.left - margin.right,
	// 		]);
	//
	// 	cluster(root);
	//
	// 	function rightAnglePath(d: {
	// 		source: HierarchyPointNode<D3Node>;
	// 		target: HierarchyPointNode<D3Node>;
	// 	}): string {
	// 		// Move to the source point, draw a vertical line down/up to the target’s x value,
	// 		// then draw a horizontal line to the target's y value.
	// 		return `M${d.source.y},${d.source.x} V${d.target.x} H${d.target.y}`;
	// 	}
	//
	// 	g.selectAll("path.link")
	// 		.data(root.links() as HierarchyPointLink<D3Node>[])
	// 		.enter()
	// 		.append("path")
	// 		.attr("class", "link")
	// 		.attr("d", rightAnglePath)
	// 		.attr("fill", "none")
	// 		.attr("stroke", "#ccc");
	//
	// 	g.selectAll("circle.node")
	// 		.data(root.descendants() as HierarchyPointNode<D3Node>[])
	// 		.enter()
	// 		.append("circle")
	// 		.attr("class", "node")
	// 		.attr("cx", (d: HierarchyPointNode<D3Node>) => d.y)
	// 		.attr("cy", (d: HierarchyPointNode<D3Node>) => d.x)
	// 		.attr("r", 3)
	// 		.attr("fill", (d: HierarchyPointNode<D3Node>) =>
	// 			d.children ? "steelblue" : "orange",
	// 		);
	// }
</script>

<h1>Hello World!</h1>
<canvas bind:this={canvas}></canvas>
