<script lang="ts">
	import { panZoom } from "./panZoomStore.svelte";

	interface Props {
		colOrder: string[];
		cellWidth: number;
		width: number;
		height: number;
	}

	const { colOrder, cellWidth, width, height }: Props = $props();
	let labelDiv = $state<HTMLElement>();
</script>

<div
	class="overflow-hidden relative"
	style="width: {width}px; height: {height}px; "
>
	<div
		class="absolute top-0 left-0 flex items-end"
		style="transform: translateX({$panZoom.tx}px);"
		bind:this={labelDiv}
	>
		{#each colOrder as name}
			<div
				class="text-white text-xs text-center whitespace-nowrape"
				style="
					width: {cellWidth * $panZoom.scale}px;
					transform: rotate(-60deg) translateY(4px); 
					transform-origin: bottom left;
					"
			>
				{name}
			</div>
		{/each}
	</div>
</div>

<style>
	@reference 'tailwindcss';
</style>
