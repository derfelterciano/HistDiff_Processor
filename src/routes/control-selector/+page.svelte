<script lang="ts">
	import { emit } from "@tauri-apps/api/event";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { page } from "$app/state";
	import type { ControlDefinition } from "../../types/controlType";

	// Svelte 5 way of retrieving parameters

	//get plate dimensions
	const plateDims: number = $derived(
		Number(page.url.searchParams.get("plate") || 384),
	);

	const modify: boolean = $derived(
		page.url.searchParams.get("modify") === "true",
	);

	const id: number = $derived(Number(page.url.searchParams.get("id") || 0));
	const initialDataRaw = $derived(() => {
		const param = page.url.searchParams.get("initialWells") || "{}";
		try {
			return decodeURIComponent(param);
		} catch (e) {
			console.log("failed to parse init: ", e);
			return "{}";
		}
	});

	function parsejson(): { wells: string[] } {
		let parsed = { wells: [] };
		try {
			console.log(initialDataRaw);
			parsed = JSON.parse(initialDataRaw());
		} catch (err) {
			console.error("Failed to parse initial data", err);
		}
		return parsed;
	}

	let selectedWells = $state(new Set<string>(parsejson().wells || []));
	let controlName: string | null = $state(null);

	const genPlateConfig = (plate: number): [row: number, col: number] => {
		switch (plate) {
			case 384:
				return [24, 16];
			case 96:
				return [12, 8];
			default:
				return [0, 0];
		}
	};

	const plateGrid = (): [number, number] => {
		return genPlateConfig(plateDims);
	};
	const [rowSize, colSize] = plateGrid();

	const rows: string[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
		.slice(0, colSize)
		.split("");
	const cols: string[] = Array.from({ length: rowSize }, (_, i) =>
		(i + 1).toString().padStart(2, "0"),
	);

	const toggleWell = (id: string): void => {
		if (selectedWells.has(id)) {
			selectedWells.delete(id);
		} else {
			selectedWells.add(id);
		}

		selectedWells = new Set(selectedWells);
		console.log("selected: ", selectedWells);
	};

	// Send the selected wells back to ControlSelector.svelte
	const confirmSelection = async () => {
		const selectedPayload = Array.from(selectedWells);
		const payload: ControlDefinition = {
			wells: selectedPayload,
			id: id,
			title: controlName,
		};
		await emit(`control-selection-complete-id${id}`, payload);
		getCurrentWindow().close();
	};

	// Clear selection
	const clearSelection = () => {
		selectedWells = new Set<string>();
	};
</script>

<div class="control-selector">
	<h3 class="text-xl font-bold mb-4">
		Select Wells for {plateDims}-well plate
	</h3>

	<div class="plate-title">
		{#if modify}
			<label class="flex items-center gap-4 mb-2">
				<span>Control Set Name:</span>
				<input
					type="text"
					placeholder="control name"
					class="text-black bg-white text-center border rounded"
					bind:value={controlName}
				/>
			</label>
		{/if}
	</div>

	<div class="well-grid">
		<div class="grid" style="grid-template-rows: repeat({colSize}, auto);">
			{#each rows as row}
				<div
					class="grid"
					style="grid-template-columns: repeat({rowSize}, 1fr);"
				>
					{#each cols as col}
						<button
							class="text-center border-2 bg-white text-black text-sm rounded-md hover:bg-gray-400 p-1"
							class:selected={selectedWells.has(`${row}${col}`)}
							onclick={() => toggleWell(`${row}${col}`)}
							>{`${row}${col}`}</button
						>
					{/each}
				</div>
			{/each}
		</div>
	</div>

	<!-- Action Buttons -->
	<div class="actions flex gap-4 items-center m-4">
		<button class="clear" onclick={clearSelection}>Clear</button>
		<button class="confirm" onclick={confirmSelection}>Confirm</button>
	</div>
</div>

<style>
	@reference 'tailwindcss';
	.control-selector {
		@apply flex flex-col items-center p-2;
	}

	.well-grid {
		@apply p-2;
	}

	.selected {
		@apply bg-blue-500 text-black hover:bg-blue-600;
	}

	.clear {
		@apply bg-red-400 hover:bg-red-500 rounded px-4 py-2;
	}
	.confirm {
		@apply bg-green-600 hover:bg-green-700 rounded px-4 py-2;
	}
</style>
