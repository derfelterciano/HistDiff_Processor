<script lang="ts">
	import FileSelect from "./inputComps/FileSelect.svelte";
	// import Combobox from "./inputComps/Combobox.svelte";
	import DynamicText from "./inputComps/DynamicText.svelte";
	import ControlSelector from "./inputComps/ControlSelector.svelte";
	import { type ControlDefinition } from "../types/controlType";
	import MultiControls from "./inputComps/MultiControls.svelte";
	import type {
		SvelteConfig,
		ControlSelection,
	} from "../types/configInterface";
	import { invoke } from "@tauri-apps/api/core";
	import { onMount, onDestroy } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import OutputDialogue from "./inputComps/OutputDialogue.svelte";

	let file_path: string = $state("");
	let well_col: string = $state("");
	let headers: string[] = $state([]);
	let additionalMeta: string[] = $state([]);
	let plateFormat: number = $state(384);

	let negativeControls: ControlDefinition | null = $state(null);

	let controls: ControlDefinition[] = $state([]);
	let errorMessage: string = $state("");

	const checkInput = () => {
		errorMessage = "";

		if (!file_path) {
			errorMessage = "A path to the cell data is required!";
			return false;
		}

		if (!well_col) {
			errorMessage = "Well column must be specified.";
			return false;
		}

		if (negativeControls === null) {
			errorMessage = "You must specify your negative controls.";
			return false;
		}

		return true;
	};

	let isProcessing = $state(false);
	let saveReady = $state(false);
	let unlisten: () => void;
	onMount(async () => {
		unlisten = await listen("hd-completed", () => {
			console.log("hd-completed!");
			invoke("clear_logs");
			isProcessing = false;
			saveReady = true;
			invoke("open_analysis");
			invoke("cluster_hd", {
				matMetric: "Pearson",
				linkage: "Complete",
			});
		});
	});

	onDestroy(() => {
		if (unlisten) unlisten();
	});

	const openLogs = async (): Promise<void> => {
		await invoke("open_logging_window");
	};

	const handleSubmit = async (): Promise<void> => {
		invoke("clear_logs");
		if (!checkInput()) return;
		isProcessing = true;
		saveReady = false;

		let formattedNegativeCntrls: ControlSelection = {
			wells: negativeControls ? negativeControls.wells : [],
			name: "REFERENCE",
		};

		let additional_contrls: ControlSelection[] = [];
		controls.forEach((cntrl) => {
			additional_contrls.push({
				wells: cntrl ? cntrl.wells : [],
				name: cntrl.title ? cntrl.title : "",
			});
		});

		let userConfig: SvelteConfig = {
			dataset_path: file_path,
			plate_format: plateFormat,
			well_name: well_col,
			add_meta_cols: additionalMeta.length === 0 ? null : additionalMeta,
			negative_control: formattedNegativeCntrls,
			add_controls: additional_contrls,
		};
		await invoke("process_hd", { config: userConfig });
		await invoke("open_logging_window");

		// console.log("file: ", file_path, "well: ", well_col, headers);
		// console.log("meta: ", additionalMeta);
		// console.log(`n-controls: ${JSON.stringify(negativeControls)}`);
		// console.log(`other controls: ${JSON.stringify(controls)}`);
	};
</script>

<form class="UserInput" onsubmit={handleSubmit}>
	<h3
		class="justify-center text-center font-bold mt-2 mb-4 border-b-2 w-full"
	>
		User Configuration
	</h3>
	<label class="select-data flex items-center px-2 w-full">
		<span class="mr-2">Dataset:</span>
		<FileSelect bind:file_path bind:headers />
	</label>

	<label class="mt-4 w-full px-2">
		<span class="mr-2">Plate format:</span>
		<select
			class="border-2 rounded-sm text-black hover:bg-gray-500 appearance-auto px-2"
			bind:value={plateFormat}
		>
			<option value={384}>384-Well</option>
			<option value={96}>96-Well</option>
		</select>
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

	<div class="negative-controls flex flex-col items-center mt-2">
		<h6 class="font-bold mb-2">Negative control selection</h6>
		<ControlSelector
			bind:plateFormat
			id={0}
			bind:controlDefinition={negativeControls}
		/>
	</div>

	<MultiControls bind:controls bind:plateFormat />

	<button
		type="submit"
		class="justify-center mt-4 border-2 rounded-lg px-4 py-1 text-center bg-green-500 hover:bg-green-600"
		disabled={isProcessing}
		>{isProcessing ? "Processing..." : "Submit"}</button
	>

	{#if saveReady}
		<OutputDialogue bind:saveReady />
	{/if}

	{#if isProcessing}
		<button
			type="button"
			class="justify-center mt-2 border-2 rounded-sm text-sm bg-fuchsia-800 p-1 text-center"
			onclick={openLogs}>Open Logs</button
		>
	{/if}

	{#if errorMessage !== ""}
		<span class="mt-5 text-md text-red-400">{errorMessage}</span>
	{/if}
</form>

<style>
	@reference 'tailwindcss';

	.UserInput {
		@apply flex flex-col items-center border-2 rounded-md max-w-md pb-4 w-[500px];
	}

	.meta-information {
		@apply flex flex-col items-center border-2 rounded-sm p-2 mt-4 w-[95%];
	}
</style>
