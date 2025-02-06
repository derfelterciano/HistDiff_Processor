<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { onMount, onDestroy } from "svelte";
	import type { ControlDefinition } from "../../types/controlType";

	export let selectedWells: string[] = [];
	export let modifiable: boolean = false; // WARN: this is set a true remove this
	export let id: number;
	export let controlDefinition: ControlDefinition | null;

	export let plateFormat: number = 384;
	const openControlSelector = async (
		plateFormat: number,
		modify: boolean,
	): Promise<void> => {
		const initData = JSON.stringify({ wells: selectedWells });
		await invoke("open_control_selector_win", {
			plateFormat: plateFormat,
			modify: modify,
			id: id,
			initalWells: initData,
		});
	};

	// listen("control-selection-complete", (e) => {
	// 	const payload = e.payload;
	// 	console.log("heard you loud and clear!", payload);
	// });

	let unsub: (() => void) | undefined;
	onMount(async () => {
		unsub = await listen<ControlDefinition>(
			`control-selection-complete-id${id}`,
			(e) => {
				const payload = e.payload;
				const controlPayload: ControlDefinition = {
					wells: payload.wells,
					id: payload.id,
					title: payload.title,
				};
				console.log("controls-recieved: ", controlPayload);
			},
		);
	});

	onDestroy(() => {
		if (unsub) {
			unsub();
		}
	});
</script>

<div class="control-selector">
	<button
		class="text-center bg-blue-500 hover:bg-blue-600 rounded border-2 border-white px-2 py-1"
		on:click={() => openControlSelector(plateFormat, modifiable)}
		type="button">Open Control Selector</button
	>
</div>
