<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	export let selectedWells: string[] = [];

	let plateFormat: number = 384;
	const openControlSelector = async (plateFormat: number): Promise<void> => {
		await invoke("open_control_selector_win", {
			plateFormat: plateFormat,
		});
	};

	listen("control-selection-complete", (e) => {
		const wells = e.payload;
		console.log("heard you loud and clear!", wells);
	});
</script>

<div class="control-selector">
	<button
		class="text-center bg-blue-500 hover:bg-blue-600 rounded border-2 border-white px-2 py-1"
		on:click={() => openControlSelector(plateFormat)}
		>Open Control Selector</button
	>
</div>
