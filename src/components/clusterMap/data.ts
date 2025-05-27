import * as d3 from "d3";
import type { D3Node } from "../../types/clustermapTypes.ts";

export interface HeatmapData {
	rows: string[];
	cols: string[];
	cells: { row: string; col: string; value: number }[];
}

export interface ClusterRes {
	row_cluster: D3Node | null;
	col_cluster: D3Node | null;
}

/**JSON fetcher (not for production)**/
export async function fetchJSON<T>(path: string): Promise<T> {
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

/**Convert's rust string obj into specific types */
export function jsonParser<T>(data: T | string | null): T | null {
	if (data === null || data === undefined) return null;
	if (typeof data === "string") {
		console.log("STRING FOUND");
		try {
			return JSON.parse(data) as T;
		} catch (err) {
			console.warn("Could not parse string as JSON", err, data);
			return null;
		}
	}

	return data as T;
}