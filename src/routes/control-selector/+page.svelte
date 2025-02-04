<script lang="ts">
	import { emit } from "@tauri-apps/api/event";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { page } from "$app/state";
	import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

	// Svelte 5 way of retrieving parameters
	const plateDims = $derived(page.url.searchParams.get("plate"));

	let selectedWells: string[] = [];
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

	// Toggle well selection
	const toggleWell = (well: string) => {
		if (selectedWells.includes(well)) {
			selectedWells = selectedWells.filter((w) => w !== well);
		} else {
			selectedWells = [...selectedWells, well];
		}
	};

	// Send the selected wells back to ControlSelector.svelte
	const confirmSelection = async () => {
		await emit("control-selection-complete", { selectedWells });
		getCurrentWindow().close();
	};

	// Clear selection
	const clearSelection = () => {
		selectedWells = [];
	};
</script>

<div class="control-selector">
	<h3>Select Wells for {plateDims}-well plate</h3>

	<!-- 16x24 Well Plate Grid -->
	<div class="grid flex flex-col item-center">
		{#each Array(16) as _, row}
			<div class="row">
				{#each Array(24) as _, col}
					<button
						class="well"
						class:selected={selectedWells.includes(`${row}-${col}`)}
						onclick={() => toggleWell(`${row}-${col}`)}
					>
						{row * 24 + col + 1}
					</button>
				{/each}
			</div>
		{/each}
	</div>

	<!-- Action Buttons -->
	<div class="actions">
		<button class="clear" onclick={clearSelection}>Clear</button>
		<button class="confirm" onclick={confirmSelection}>Confirm</button>
	</div>
</div>

<style>
	.control-selector {
		text-align: center;
		padding: 1rem;
	}
	.grid {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}
	.row {
		display: flex;
		gap: 0.5rem;
	}
	.well {
		width: 30px;
		height: 30px;
		background: lightgray;
		border: 1px solid black;
		cursor: pointer;
	}
	.well.selected {
		background: blue;
		color: white;
	}
	.actions {
		margin-top: 1rem;
	}
	.clear {
		background: red;
		color: white;
	}
	.confirm {
		background: green;
		color: white;
	}
</style>
