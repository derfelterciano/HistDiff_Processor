<script lang="ts">
	import ControlSelector from "./ControlSelector.svelte";
	import {
		type ControlDefinition,
		EMPTY_CONTROL,
	} from "../../types/controlType";

	export let controls: ControlDefinition[] = [];

	const addControl = () => {
		controls = [...controls, EMPTY_CONTROL];
	};

	// const updateControl = (id: number, item: ControlDefinition) => {
	// 	controls[id] = item;
	// 	controls = controls;
	// };

	const deleteControl = (id: number) => {
		controls = controls.filter((_, i) => i !== id);
		console.log(controls);
	};
</script>

<div class="multi-control">
	<h6>Additional Controls</h6>

	<div class="control-list mb-2 px-4">
		{#each controls as _, index}
			<div class="flex items-center gap-x-2 mb-2">
				<ControlSelector
					id={index + 1}
					bind:controlDefinition={controls[index]}
					modifiable={true}
				/>
				<button
					type="button"
					class="bg-red-500 hover:bg-red-600 text-white font-bold py-1 px-3 rounded"
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
</style>
