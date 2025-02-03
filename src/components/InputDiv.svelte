<script lang="ts">
	import FileSelect from "./inputComps/FileSelect.svelte";
	// import Combobox from "./inputComps/Combobox.svelte";
	import DynamicText from "./inputComps/DynamicText.svelte";
	import ControlSelector from "./inputComps/ControlSelector.svelte";

	let file_path: string = "";
	let well_col: string = "";
	let headers: string[] = [];
	let additionalMeta: string[] = [];

	const handleSubmit = (): void => {
		console.log("file: ", file_path, "well: ", well_col, headers);
		console.log("meta: ", additionalMeta);
	};
</script>

<form class="UserInput" on:submit={handleSubmit}>
	<h3
		class="justify-center text-center font-bold mt-2 mb-4 border-b-2 w-full"
	>
		User Configuration
	</h3>
	<label class="select-data flex items-center px-2 w-full">
		<span class="mr-2">Dataset:</span>
		<FileSelect bind:file_path bind:headers />
	</label>

	<div class="meta-information">
		<h6>Meta Information</h6>
		<label class="mb-3">
			<span class="mr-2">Well Name Column:</span>
			<!-- EXPERIMENTAL Combobox-->
			<!-- <Combobox items={headers} bind:selectedValue={well_col} /> -->
			<input
				class="bg-white text-black text-center border-2 border-white rounded-sm"
				type="text"
				placeholder="Enter column name"
				bind:value={well_col}
			/>
		</label>

		<DynamicText
			title="Additional Meta"
			bind:arrayOptions={additionalMeta}
		/>
	</div>

	<div class="negative-controls">
		<h6>Negative control selection</h6>
		<ControlSelector />
	</div>

	<button
		type="submit"
		class="justify-center mt-4 border-2 rounded-lg px-4 py-1 text-center bg-green-500 hover:bg-green-300"
		>Submit</button
	>
</form>

<style>
	@reference 'tailwindcss';

	.UserInput {
		@apply flex flex-col items-center border-2 rounded-md max-w-md pb-4;
	}

	.meta-information {
		@apply flex flex-col items-center border-2 rounded-sm p-2 mt-4 w-[95%];
	}
</style>
