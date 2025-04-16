<script lang="ts">
	import * as d3 from "d3";
	import { onMount } from "svelte";
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

	onMount(async () => {
		// grab tree
		const rawTree = await fetch_json<D3Node>(tree);
		console.log(rawTree);

		d3_tree = rawTree;
		// drawDendrogram();

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

		drawSVG();
	});

	function drawSVG() {}

	function drawDendrogram() {
		if (!d3_tree) return;

		svgDiv.innerHTML = "";

		const width = 600;
		const height = 1000;
		const margin = { top: 20, right: 90, bottom: 30, left: 90 };

		const svg = d3
			.select(svgDiv)
			.append("svg")
			.attr("width", width + margin.left + margin.right)
			.attr("height", height + margin.top + margin.bottom);

		const g = svg
			.append("g")
			.attr("transform", `translate(${margin.left},${margin.top})`);

		const root = d3.hierarchy<D3Node>(d3_tree);

		const cluster = d3
			.cluster<D3Node>()
			.size([
				height - margin.top - margin.bottom,
				width - margin.left - margin.right,
			]);

		cluster(root);

		function rightAnglePath(d: {
			source: HierarchyPointNode<D3Node>;
			target: HierarchyPointNode<D3Node>;
		}): string {
			// Move to the source point, draw a vertical line down/up to the target’s x value,
			// then draw a horizontal line to the target's y value.
			return `M${d.source.y},${d.source.x} V${d.target.x} H${d.target.y}`;
		}

		g.selectAll("path.link")
			.data(root.links() as HierarchyPointLink<D3Node>[])
			.enter()
			.append("path")
			.attr("class", "link")
			.attr("d", rightAnglePath)
			.attr("fill", "none")
			.attr("stroke", "#ccc");

		g.selectAll("circle.node")
			.data(root.descendants() as HierarchyPointNode<D3Node>[])
			.enter()
			.append("circle")
			.attr("class", "node")
			.attr("cx", (d: HierarchyPointNode<D3Node>) => d.y)
			.attr("cy", (d: HierarchyPointNode<D3Node>) => d.x)
			.attr("r", 3)
			.attr("fill", (d: HierarchyPointNode<D3Node>) =>
				d.children ? "steelblue" : "orange",
			);
	}
</script>

<h1>Hello World!</h1>
<div bind:this={canvas}></div>
