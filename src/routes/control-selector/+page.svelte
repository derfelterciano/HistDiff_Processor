<script lang="ts">
	import { emit } from "@tauri-apps/api/event";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	let selectedWells: string[] = [];

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
	<h3>Select Wells for Negative Controls</h3>

	<!-- 16x24 Well Plate Grid -->
	<div class="grid">
		{#each Array(16) as _, row}
			<div class="row">
				{#each Array(24) as _, col}
					<button
						class="well"
						class:selected={selectedWells.includes(`${row}-${col}`)}
						on:click={() => toggleWell(`${row}-${col}`)}
					>
						{row * 24 + col + 1}
					</button>
				{/each}
			</div>
		{/each}
	</div>

	<!-- Action Buttons -->
	<div class="actions">
		<button class="clear" on:click={clearSelection}>Clear</button>
		<button class="confirm" on:click={confirmSelection}>Confirm</button>
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
