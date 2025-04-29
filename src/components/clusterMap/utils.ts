import type { D3Node } from "../../types/clustermapTypes.ts";

/**
 * given a tree of D3Node
 * return an array of the leaf names (in-order)
 */
export function getLeafOrder(root: D3Node): string[] {
	const order: string[] = [];

	const recurse = (node: D3Node) => {
		if (!node.children || node.children.length == 0) {
			order.push(node.name ?? String(node.cid));
		} else {
			node.children.forEach(recurse);
		}
	};

	recurse(root);
	return order;
}
