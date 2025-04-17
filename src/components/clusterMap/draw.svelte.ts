import type { D3Node } from "../../types/clustermapTypes.ts";
import type { HierarchyNode } from "d3-hierarchy";
import type { HeatmapData } from "./data.svelte.ts";
import * as d3 from "d3";

/**Draw denderogram tree with right angles*/
export function drawTree(
	ctx: CanvasRenderingContext2D,
	root: HierarchyNode<D3Node>,
	treeWidth: number,
) {
	ctx.strokeStyle = "#ccc";
	root.links().forEach((link: d3.HierarchyLink<D3Node>) => {
		ctx.beginPath();
		ctx.moveTo(link.source.y as number, link.source.x as number);
		ctx.lineTo(link.source.y as number, link.target.x as number);
		ctx.lineTo(link.target.y as number, link.target.x as number);
		ctx.stroke();
	});

	root.descendants().forEach((node) => {
		ctx.beginPath();
		ctx.arc(node.y as number, node.x as number, 2, 0, 2 * Math.PI);
		ctx.fillStyle = node.children ? "steelblue" : "orange";
		ctx.fill();
	});
}

/** Draw the heatmap cells on canvas */
export function drawHeatmap(
	ctx: CanvasRenderingContext2D,
	data: HeatmapData,
	rawData: Record<string, Record<string, number>>,
	rowOrder: string[],
	treeWidth: number,
	cellW: number,
	cellH: number,
	contrast: { min: number; max: number },
) {
	const color = d3
		.scaleSequential(d3.interpolateViridis)
		.domain([contrast.min, contrast.max]);

	rowOrder.forEach((row, r) =>
		data.cols.forEach((col, c) => {
			const x = treeWidth + c * cellW;
			const y = r * cellH;
			const v = rawData[row][col];
			ctx.fillStyle = color(v);
			ctx.fillRect(x, y, cellW, cellH);
		})
	);
}
