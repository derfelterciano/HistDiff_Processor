import * as d3 from "d3";
import type { D3Node } from "../../types/clustermapTypes.ts";
import type { HierarchyNode } from "d3-hierarchy";

export interface TreeLayout {
	root: HierarchyNode<D3Node>;
	leafOrder: string[];
}

/** Cluster nodes via D3 */
export function computeTreeLayout(
	tree: D3Node,
	height: number,
	width: number,
): TreeLayout {
	const root = d3.hierarchy<D3Node>(tree);

	// cluster nodes
	d3.cluster<D3Node>().size([height, width])(root);
	const leafOrder = root.leaves().map((n) => n.data.name!);
	return { root, leafOrder };
}
