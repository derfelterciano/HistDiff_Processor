import type { HeatmapData } from "./data.svelte.ts";
import type { TreeLayout } from "./layouts.svelte.ts";
import * as d3 from "d3";
import { drawTree } from "./draw.svelte.ts";

export interface Contrast {
	min: number;
	max: number;
}

export function createHeatmapBuff(
	data: HeatmapData,
	raw: Record<string, Record<string, number>>,
	rowOrder: string[],
	contrast: Contrast,
): HTMLCanvasElement {
	const buf = document.createElement("canvas");
	buf.width = data.cols.length;
	buf.height = data.rows.length;
	const ctx = buf.getContext("2d");
	if (!ctx) throw new Error("Could not create heatmap buffer context");

	const colorScale = d3
		.scaleSequential(d3.interpolateViridis)
		.domain([contrast.min, contrast.max]);

	rowOrder.forEach((row, r) =>
		data.cols.forEach((col, c) => {
			const v = raw[row][col];
			ctx.fillStyle = colorScale(v);
			ctx.fillRect(c, r, 1, 1);
		})
	);

	return buf;
}

export function createTreeBuff(
	layout: TreeLayout,
	treeWidth: number,
	totalHeight: number,
	dpr: number, // device pixel ration
): HTMLCanvasElement {
	const buf = document.createElement("canvas");
	// CSS size
	buf.style.width = `${treeWidth}px`;
	buf.style.height = `${totalHeight}px`;

	buf.width = treeWidth * dpr;
	buf.height = totalHeight * dpr;

	const ctx = buf.getContext("2d");
	if (!ctx) throw new Error("Could not create tree buffer context");
	ctx.imageSmoothingEnabled = false;
	ctx.scale(dpr, dpr);

	drawTree(ctx, layout.root, treeWidth);

	return buf;
}

export function blitBuffers(
	ctx: CanvasRenderingContext2D,
	treeBuf: HTMLCanvasElement,
	heatmapBuf: HTMLCanvasElement,
	treeWidth: number,
	cellWidth: number,
	cellHeight: number,
): void {
	ctx.imageSmoothingEnabled = false;
	ctx.imageSmoothingQuality = "low";

	//draw dendrogram
	ctx.drawImage(treeBuf, 0, 0);

	ctx.drawImage(
		heatmapBuf,
		0,
		0,
		heatmapBuf.width,
		heatmapBuf.height,
		treeWidth,
		0,
		heatmapBuf.width * cellWidth,
		heatmapBuf.height * cellHeight,
	);
}

