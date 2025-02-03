<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";

	export let selectedWells: string[] = [];

	const openControlSelector = async () => {
		await invoke("open_control_selector_win");
	};
</script>

<div class="control-selector">
	<button
		class="text-center bg-blue-500 hover:bg-blue-600 rounded border-2 border-white px-2 py-1"
		on:click={openControlSelector}>Open Control Selector</button
	>
</div>
