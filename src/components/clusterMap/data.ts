import * as d3 from "d3";
import type { D3Node } from "../../types/clustermapTypes.ts";

export interface HeatmapData {
	rows: string[];
	cols: string[];
	cells: { row: string; col: string; value: number }[];
}

/**JSON fetcher (not for production)**/
async function fetchJSON<T>(path: string): Promise<T> {
	const response = await fetch(path);
	return response.json() as T;
}

/**Fetches Tree json file**/
export function loadTreeData(url: string): Promise<D3Node> {
	return fetchJSON<D3Node>(url);
}

/**Fetches Heatmap data json file**/
export function loadHeatmapData(
	url: string,
): Promise<Record<string, Record<string, number>>> {
	return fetchJSON<Record<string, Record<string, number>>>(url);
}

/**Converts raw heatmap record into useable data***/
export function loadHeatmap(
	rawHeatmap: Record<string, Record<string, number>>,
): HeatmapData {
	const rows: string[] = Object.keys(rawHeatmap);
	const cols: string[] = rows.length ? Object.keys(rawHeatmap[rows[0]]) : [];
	const cells: HeatmapData["cells"] = [];

	rows.forEach((r) =>
		cols.forEach((c) =>
			cells.push({ row: r, col: c, value: rawHeatmap[r][c] })
		)
	);

	return { rows, cols, cells };
}

/**Computes contrast for cells**/
export function computeContrast(cells: HeatmapData["cells"]) {
	const vals = cells.map((d) => d.value);
	return { min: d3.min(vals) ?? 0, max: d3.max(vals) ?? 1 };
}
