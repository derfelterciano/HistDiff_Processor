<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { panZoom } from "./panZoomStore.svelte";
	import * as d3 from "d3";
	import type { HeatmapData } from "./data";

	/**
	- data -> HeatMapData type with appropriate types filled
	- raw -> raw data for raw record lookup
	- rowOrder and colOrder array of keys for ordering
	- width / height css pixel dimensions of heatmap panel
	*/

	interface Props {
		data: HeatmapData;
		raw: Record<string, Record<string, number>>;
		rowOrder: string[];
		colOrder: string[];
		width: number;
		height: number;
	}

	const { data, raw, rowOrder, colOrder, width, height }: Props = $props();

	let canvas = $state<HTMLCanvasElement>();
	let buffer = $state<HTMLCanvasElement>();
	let dpr = $state<number>(1);

	let cellW = $derived(() => width / colOrder.length);
	let cellH = $derived(() => height / rowOrder.length);

	// min and max contrast vals
	let minVal = $derived(() => Math.min(...data.cells.map((c) => c.value)));
	let maxVal = $derived(() => Math.max(...data.cells.map((c) => c.value)));

	// create an offscreen buffer
	function createBuffer() {
		if (!canvas) return;

		dpr = window.devicePixelRatio || 1;

		const buf = document.createElement("canvas");
		buf.width = width * dpr;
		buf.height = height * dpr;
		const bctx = buf.getContext("2d");
		if (!bctx) return;

		// css buffer
		bctx.scale(dpr, dpr);
		bctx.imageSmoothingEnabled = false;

		// cells should be painted once
		const color = d3
			.scaleSequential(d3.interpolateViridis)
			.domain([-0.05, 0.05]); // TODO: Swap out with min and max vals

		rowOrder.forEach((rKey, i) => {
			colOrder.forEach((cKey, j) => {
				const v = raw[rKey][cKey];
				bctx.fillStyle = color(v);
				bctx.fillRect(j * cellW(), i * cellH(), cellW(), cellH());
			});
		});

		buffer = buf;
	}

	function draw() {}
	function resize() {}
</script>
