<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import ClusterMap from "../../components/ClusterMap.svelte";
	let headerE1 = $state<HTMLDivElement>();
	let containerE1 = $state<HTMLDivElement>();
	let availH = $state<number>(0);

	function updateAvailHeight() {
		const hh = headerE1?.clientHeight ?? 0;
		availH = window.innerHeight - hh;
	}

	onMount(() => {
		updateAvailHeight();
		window.addEventListener("resize", updateAvailHeight);
	});
	onDestroy(() => {
		window.removeEventListener("resize", updateAvailHeight);
	});
</script>

<div class="overflow-hidden">
	<div bind:this={headerE1} class="header">
		<h1 class="text-3xl">Analysis</h1>
	</div>

	<div bind:this={containerE1} class="overflow-hidden h-full flex flex-col">
		<ClusterMap containerHeight={availH} />
	</div>
</div>
