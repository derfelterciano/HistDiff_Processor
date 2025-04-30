<script lang="ts">
	import { panZoom } from "./panZoomStore.svelte";
	import * as d3 from "d3";
	import type { HeatmapData } from "./data";

	// ── 1) Props ─────────────────────────────────────────────────────────────
	interface Props {
		data: HeatmapData;
		raw: Record<string, Record<string, number>>;
		rowOrder: string[];
		colOrder: string[];
		width: number; // CSS px of heatmap panel
		height: number; // CSS px
	}
	const { data, raw, rowOrder, colOrder, width, height }: Props = $props();

	// ── 2) Local canvases ─────────────────────────────────────────────────────
	let canvas = $state<HTMLCanvasElement>();
	let buffer = $state<HTMLCanvasElement>();

	// ── 3) Deriveds ───────────────────────────────────────────────────────────
	// how many columns / rows
	const cols = $derived(() => colOrder.length);
	const rows = $derived(() => rowOrder.length);

	// each cell’s size (in CSS px)
	const cellW = $derived(() => width / cols());
	const cellH = $derived(() => height / rows());

	// ── 4) Build offscreen 1px-per-cell buffer ────────────────────────────────
	function buildBuffer() {
		if (!cols() || !rows() || !canvas) return;
		// 1) create offscreen at cols×rows
		const buf = document.createElement("canvas");
		buf.width = cols();
		buf.height = rows();
		const ctx = buf.getContext("2d")!;
		ctx.imageSmoothingEnabled = false;

		// 2) color scale
		const values = data.cells.map((c) => c.value);
		const colorFn = d3
			.scaleSequential(d3.interpolateViridis)
			.domain([-0.005, 0.005]) as any;

		// 3) paint one pixel per cell
		for (let i = 0; i < rows(); i++) {
			for (let j = 0; j < cols(); j++) {
				ctx.fillStyle = colorFn(raw[rowOrder[i]][colOrder[j]]);
				ctx.fillRect(j, i, 1, 1);
			}
		}

		buffer = buf;
	}

	// ── 5) Draw into onscreen canvas, scaling each pixel → cellW×cellH ────────
	function draw() {
		if (!canvas || !buffer) return;
		const ctx = canvas.getContext("2d")!;
		ctx.resetTransform();
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		// map CSS panel → backing-store
		canvas.style.width = `${width}px`;
		canvas.style.height = `${height}px`;
		canvas.width = Math.round(cols() * cellW());
		canvas.height = Math.round(rows() * cellH());

		ctx.imageSmoothingEnabled = false;

		// pan/zoom
		const { tx, ty, scale } = $panZoom;
		ctx.translate(tx, ty);
		ctx.scale(scale, scale);

		// single draw call:
		// source: buffer (cols×rows px)
		// dest:   [ treeWidth,0 ] then [cols*cellW,rows*cellH]
		ctx.drawImage(
			buffer,
			0,
			0,
			buffer.width,
			buffer.height,
			0,
			0,
			cols() * cellW(),
			rows() * cellH(),
		);
	}

	// ── 6) Reactivity ─────────────────────────────────────────────────────────
	// rebuild buffer whenever data or ordering changes
	$effect(() => {
		data;
		raw;
		rowOrder;
		colOrder;
		buildBuffer();
	});

	// draw whenever panel size OR panZoom OR buffer changes
	$effect(() => {
		width;
		height;
		$panZoom;
		buffer;
		draw();
	});
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

<canvas
	bind:this={canvas}
	style="
    display: block;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
    -ms-interpolation-mode: nearest-neighbor;
  "
></canvas>
