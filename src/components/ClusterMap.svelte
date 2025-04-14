<script lang="ts">
	import * as d3 from "d3";
	import { onMount } from "svelte";

	const tree: string = "/tree.json";

	let d3_tree: any;
	let svgDiv: HTMLDivElement;

	onMount(async () => {
		const response = await fetch(tree);
		const raw = await response.json();
		console.log(raw);

		d3_tree = raw;
		drawDendrogram();
	});

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

		const root = d3.hierarchy(d3_tree);

		const cluster = d3
			.cluster()
			.size([
				height - margin.top - margin.bottom,
				width - margin.left - margin.right,
			]);

		cluster(root);
	}
</script>

<h1>Hello World!</h1>
<div bind:this={svgDiv}></div>
