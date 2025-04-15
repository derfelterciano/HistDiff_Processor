export interface D3Node {
	cid: number;
	name?: string;
	dist: number;
	children: D3Node[];
}
