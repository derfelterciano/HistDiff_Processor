<script lang="ts">
	import ControlSelector from "./ControlSelector.svelte";
	import {
		type ControlDefinition,
		EMPTY_CONTROL,
	} from "../../types/controlType";

	export let controls: ControlDefinition[] = [];
	export let plateFormat: number;

	const addControl = () => {
		controls = [...controls, EMPTY_CONTROL];
	};

	// const updateControl = (id: number, item: ControlDefinition) => {
	// 	controls[id] = item;
	// 	controls = controls;
	// };

	const deleteControl = (id: number) => {
		controls = controls.filter((_, i) => i !== id);
	};
</script>

<div class="multi-control">
	<h6 class="font-bold mb-2 text-center">Additional Controls</h6>

	<div class="control-list mb-2 px-4 w-full">
		{#each controls as _, index}
			<div class="control-row flex items-center gap-x-1 mb-2">
				<ControlSelector
					id={index + 1}
					bind:controlDefinition={controls[index]}
					modifiable={true}
					{plateFormat}
				/>
				<div
					class="flex-1 min-w-0 bg-gray-500 text-white rounded-md ml-4 mr-2 text-center"
				>
					{#if controls[index].title !== null}
						<span class="w-full">{controls[index].title}</span>
					{:else}
						<span class="w-full">No Title Selected</span>
					{/if}
				</div>

				<button
					type="button"
					class="flex-none justify-self-start w-auto bg-red-500 hover:bg-red-600 text-white font-bold py-1 px-3 rounded"
					on:click={() => deleteControl(index)}>-</button
				>
			</div>
		{/each}
	</div>

	<button
		type="button"
		class="bg-blue-500 hover:bg-blue-600 text-center px-4 rounded-md"
		on:click={addControl}>+ Add Control Type</button
	>
</div>

<style>
	@reference 'tailwindcss';
	.multi-control {
		@apply flex flex-col items-center w-[95%] border rounded mt-2 p-2;
	}

	.control-list {
		max-height: 200px;
		overflow-y: auto;
	}
	.control-row {
		@apply bg-purple-200 border rounded-md;
	}
</style>
