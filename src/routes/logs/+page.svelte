<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { onMount, onDestroy, tick } from "svelte";
	import { get } from "svelte/store";
	import { logStore } from "../../logs/logStorage.svelte";
	import { invoke } from "@tauri-apps/api/core";

	let logs: string[] = $state([]);

	setTimeout(() => {
		logs = get(logStore);
		console.log(get(logStore));
	}, 1000);
	let unlisten: (() => void) | undefined;
	let unsub: () => void;

	let logElement: HTMLDivElement;

	// $: if (logs.length > 0) {
	// 	scrollBottom(logElement);
	// }

	const scrollBottom = async (node: HTMLDivElement) => {
		node.scroll({ top: node.scrollHeight, behavior: "smooth" });
	};

	onMount(async () => {
		console.log("mounted");
		invoke("test_log");
		setTimeout(() => {
			unsub = logStore.subscribe((updatedLogs) => {
				logs = updatedLogs;
			});
		}, 1000);
	});

	onDestroy(() => {
		if (unlisten) unlisten();
		unsub?.();
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
