<script lang="ts">
	import { onMount } from "svelte";
	import { panZoom } from "./panZoomStore.svelte";
	import * as d3 from "d3";
	import type { D3Node } from "../../types/clustermapTypes";

	export let treeData: D3Node;
	export let width: number;
	export let height: number;
	export let orientation: "left" | "top";

	let canvas: HTMLCanvasElement;
	let dpr = 1;

	$: $panZoom, treeData, draw();

	function draw() {
		if (!canvas || !treeData) return;
		const ctx = canvas.getContext("2d");
		if (!ctx) return;

		// clear + reset transformations
		ctx.resetTransform();
		ctx.clearRect(0, 0, canvas.width, canvas.height);

		// css px to device px
		ctx.scale(dpr, dpr);

		// pan/zoom
		const { scale, tx, ty } = $panZoom;
		ctx.translate(tx, ty);
		ctx.scale(scale, scale);

		ctx.strokeStyle = "#555";
		ctx.fillStyle = "#444";

		// D3 hierarchy and clustering
		const root = d3.hierarchy(treeData);
		const layout = d3
			.cluster<d3.HierarchyPointNode<D3Node>>()
			.size(orientation === "left" ? [height, width] : [width, height]);
		layout(root as any);

		// draw elbow links
		ctx.beginPath();
		root.links().forEach((link) => {
			const s: any = link.source,
				t: any = link.target;
			if (orientation === "left") {
				// hori tree = (y,x)
				ctx.moveTo(s.y, s.x);
				ctx.lineTo(t.y, s.x);
				ctx.lineTo(t.y, t.x);
			} else {
				// vert tree: (x,y)
				ctx.moveTo(s.x, s.y);
				ctx.lineTo(s.x, t.y);
				ctx.lineTo(t.x, t.y);
			}
		});
		ctx.stroke();

		// draw nodes
		root.descendants().forEach((n) => {
			const x = orientation === "left" ? n.y : n.x;
			const y = orientation === "left" ? n.x : n.y;
			ctx.beginPath();
			ctx.arc(x as number, y as number, 3, 0, Math.PI * 2);
			ctx.fill();
		});
	}

	function resize() {
		if (!canvas) return;

		// css size
		canvas.style.width = `${width}px`;
		canvas.style.height = `${height}px`;

		// backing store
		canvas.width = width * dpr;
		canvas.height = height * dpr;

		draw();
	}

	onMount(() => {
		dpr = window.devicePixelRatio || 1;
		resize();
	});
</script>

<canvas bind:this={canvas} on:resize={resize} style="display:block;"></canvas>
