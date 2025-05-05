<script lang="ts">
	import { panZoom } from "./panZoomStore.svelte";
	interface Props {
		rowOrder: string[];
		cellHeight: number;
		width: number;
		height: number;
	}

	const { rowOrder, cellHeight, width, height }: Props = $props();

	let labelDiv = $state<HTMLDivElement>();

	$effect(() => {
		const { ty, scale } = $panZoom;
		if (labelDiv) {
			// labelDiv.style.transformOrigin = "top left";
			labelDiv.style.transform = `translateY(${ty}px)`;
		}
	});
</script>

<div
	class="overflow-clip relative"
	style="width: {width}px; height: {height}px;"
>
	<div class="absolute top-0 left-0" bind:this={labelDiv}>
		{#each rowOrder as name}
			<div
				class="whitespace-nowrap pl-2 box-border"
				style="height: {cellHeight *
					$panZoom.scale}px; line-height: {cellHeight *
					$panZoom.scale}px;"
			>
				{name}
			</div>
		{/each}
	</div>
</div>
