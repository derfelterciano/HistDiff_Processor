<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { onMount, onDestroy, tick } from "svelte";
	import { get } from "svelte/store";
	import { invoke } from "@tauri-apps/api/core";

	let logs: string[] = $state([]);

	let unlisten: (() => void) | undefined;

	let logElement: HTMLDivElement;

	// $: if (logs.length > 0) {
	// 	scrollBottom(logElement);
	// }

	const scrollBottom = async (node: HTMLDivElement) => {
		node.scroll({ top: node.scrollHeight, behavior: "smooth" });
	};

	onMount(async () => {
		// invoke("test_log");
		const storedLogs: string[] = await invoke("get_logs");
		logs.push(...storedLogs);

		unlisten = await listen("rust-log", (e) => {
			logs = [...logs, e.payload as string];
		});
	});

	onDestroy(() => {
		if (unlisten) unlisten();
	});
</script>

<div class="log-window p-2">
	<h1 class="font-bold text-center text-xl my-2">Logs</h1>

	<div bind:this={logElement} class="logs rounded-md">
		{#each logs as line}
			<span class="m-2">{line}</span>
		{/each}
	</div>
</div>

<style>
	@reference 'tailwindcss';

	.log-window {
		@apply flex flex-col items-center h-screen;
	}

	.logs {
		@apply flex flex-col border-2 w-full h-full;
		overflow-y: auto;
	}
</style>
