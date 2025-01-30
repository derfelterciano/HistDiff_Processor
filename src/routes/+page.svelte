<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import InputDiv from "../components/InputDiv.svelte";

	let name = $state("");
	let greetMsg = $state("");

	async function greet(event: Event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		greetMsg = await invoke("greet", { name });
	}
</script>

<main
	class="main-container flex flex-col items-center justify-center w-full p-2"
>
	<h1 class="text-center text-3xl font-bold mx-auto">
		HistDiff Pipeline Processor
	</h1>
	<div class="main-div flex flex-col px-2 w-[100%]">
		<InputDiv />
	</div>
</main>

<style>
	/* THIS IS HOW TO USE APPLY IN TAILWIND V4*/
	/* @reference 'tailwindcss'; */
	/**/
	/* .container { */
	/* 	@apply text-red-500; */
	/* } */
</style>
