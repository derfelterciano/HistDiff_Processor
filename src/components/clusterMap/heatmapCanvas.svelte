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
			.domain([-0.005, 0.005]); // TODO: Swap out with min and max vals

		rowOrder.forEach((rKey, i) => {
			colOrder.forEach((cKey, j) => {
				const v = raw[rKey][cKey];
				bctx.fillStyle = color(v);
				bctx.fillRect(j * cellW(), i * cellH(), cellW(), cellH());
			});
		});

		buffer = buf;
	}

	function draw() {
		if (!canvas || !buffer) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		// clear and map css buffer
		ctx.resetTransform();
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		ctx.scale(dpr, dpr);

		// pan and zoom
		const { tx, ty, scale } = $panZoom;
		ctx.translate(tx, ty);
		ctx.scale(scale, scale);
		ctx.imageSmoothingEnabled = false;

		// blit heatmap
		ctx.drawImage(
			buffer,
			0,
			0,
			buffer.width,
			buffer.height,
			0,
			0,
			width,
			height,
		);
	}
	function resize() {
		if (!canvas) return;

		dpr = window.devicePixelRatio || 1;
		canvas.style.width = `${width}px`;
		canvas.style.height = `${height}px`;
		canvas.width = width * dpr;
		canvas.height = height * dpr;
	}

	// rebuild buffer reactivly
	$effect(() => {
		data;
		raw;
		rowOrder;
		colOrder;
		width;
		height;
		cellW();
		cellH();
		dpr;
		createBuffer();
	});

	// draw reactivly
	$effect(() => {
		resize();
		draw();
	});

	// resize effect
	$effect(() => {
		const onR = () => {
			resize();
			draw();
		};
		window.addEventListener("resize", onR);

		return () => {
			window.removeEventListener("resize", onR);
		};
	});

	// panning effect
	$effect(() => {
		if (!canvas) return;

		let dragging = false,
			lastX = 0,
			lastY = 0;
		const down = (e: MouseEvent) => {
			dragging = true;
			lastX = e.clientX;
			lastY = e.clientY;

			e.preventDefault();
		};

		const move = (e: MouseEvent) => {
			if (!dragging) return;
			const dx = e.clientX - lastX,
				dy = e.clientY - lastY;
			lastX = e.clientX;
			lastY = e.clientY;

			panZoom.pan(dx, dy);
		};

		const up = () => {
			dragging = false;
		};

		canvas.addEventListener("mousedown", down);
		window.addEventListener("mousemove", move);
		window.addEventListener("mouseup", up);

		return () => {
			if (!canvas) return;
			canvas.removeEventListener("mousedown", down);
			window.removeEventListener("mousemove", move);
			window.removeEventListener("mouseup", up);
		};
	});
</script>

<canvas bind:this={canvas} style="display:block"></canvas>
