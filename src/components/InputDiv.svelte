<script lang="ts">
	import FileSelect from "./inputComps/FileSelect.svelte";

	let file_path: string = "";

	const handleSubmit = (): void => {
		console.log("file: ", file_path);
	};
</script>

<form class="UserInput" on:submit={handleSubmit}>
	<h3
		class="justify-center text-center font-bold mt-2 mb-4 border-b-2 w-full"
	>
		User Configuration
	</h3>
	<label class="select-data flex items-center w-full">
		<span class="mr-2">Dataset:</span>
		<FileSelect bind:file_path />
	</label>

	<button
		type="submit"
		class="justify-center mt-4 border-2 rounded-lg px-4 py-1 text-center bg-green-500 hover:bg-green-300"
		>Submit</button
	>
</form>

<style>
	@reference 'tailwindcss';

	.UserInput {
		@apply flex flex-col items-center border-2 rounded-md max-w-md pb-4 px-2;
	}
</style>
