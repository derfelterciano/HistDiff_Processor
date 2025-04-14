<script lang="ts">
	import * as d3 from "d3";
	import { onMount } from "svelte";

	const tree: string = "/tree.json";

	let d3_tree: any;

	onMount(async () => {
		const response = await fetch(tree);
		const raw = await response.json();
		console.log(raw);
	});
</script>

<h1>Hello World!</h1>
