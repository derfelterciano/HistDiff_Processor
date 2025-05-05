<script lang="ts">
	import { panZoom } from "./panZoomStore.svelte";
	import * as d3 from "d3";
	import type { D3Node } from "../../types/clustermapTypes";

	// export let treeData: D3Node;
	// export let width: number;
	// export let height: number;
	// export let orientation: "left" | "top";

	const { treeData, width, height, orientation } = $props<{
		treeData: D3Node;
		width: number;
		height: number;
		orientation: "left" | "top";
	}>();

	let canvas = $state<HTMLCanvasElement>();
	let dpr = $state<number>(1);

	// $: $panZoom, treeData, draw();

	$effect(() => {
		$panZoom;
		treeData;
		width;
		height;
		resize();
		draw();
	});

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

		if (orientation === "left") {
			ctx.translate(0, ty);
			// ctx.scale(scale, 1);
		} else {
			ctx.translate(tx, 0);
			// ctx.scale(1, scale);
		}

		ctx.strokeStyle = "#ccc";

		// D3 hierarchy and clustering
		const root = d3.hierarchy(treeData);
		const layout = d3.cluster<d3.HierarchyPointNode<D3Node>>();

		const leafCount = root.leaves().length;
		const cellSize =
			orientation === "left"
				? (height * scale) / leafCount
				: (width * scale) / leafCount;

		const leafSpacing =
			orientation === "left" ? height * scale : width * scale;
		// const leafSpacing = cellSize * (leafCount - 1);
		const branchLen = orientation === "left" ? width : height;

		layout.size([leafSpacing, branchLen]);

		layout(root as any);

		// draw elbow links
		ctx.beginPath();
		root.links().forEach((link) => {
			const s: any = link.source,
				t: any = link.target;
			if (orientation === "left") {
				// vert tree = (y,x)
				ctx.moveTo(s.y, s.x);
				ctx.lineTo(s.y, t.x);
				ctx.lineTo(t.y, t.x);
			} else {
				// horizontal tree: (x,y)
				ctx.moveTo(s.x, s.y);
				ctx.lineTo(t.x, s.y);
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
			ctx.fillStyle = n.children ? "steelblue" : "orange";
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

	// === PANNING ===

	function handlePanning(): () => void {
		if (!canvas) return () => {};

		let dragging = false;
		let lastX = 0,
			lastY = 0;

		const onDown = (e: MouseEvent) => {
			dragging = true;
			lastX = e.clientX;
			lastY = e.clientY;
			e.preventDefault();
		};

		const onMove = (e: MouseEvent) => {
			if (!dragging) return;

			const dx = e.clientX - lastX;
			const dy = e.clientY - lastY;
			(lastX = e.clientX), (lastY = e.clientY);
			if (orientation === "left") {
				panZoom.pan(0, dy);
			} else {
				panZoom.pan(dx, 0);
			}
		};

		const onUp = () => {
			dragging = false;
		};

		canvas.addEventListener("mousedown", onDown);
		window.addEventListener("mousemove", onMove);
		window.addEventListener("mouseup", onUp);

		return () => {
			if (!canvas) return () => {};

			canvas.removeEventListener("mousedown", onDown);
			window.removeEventListener("mouseup", onUp);
			window.removeEventListener("mousemove", onMove);
		};
	}

	let cleanupPanning: () => void;
	$effect(() => {
		if (!canvas) return;
		cleanupPanning = handlePanning();

		return cleanupPanning;
	});
</script>

<canvas bind:this={canvas} on:resize={resize} style="display:block;"></canvas>
