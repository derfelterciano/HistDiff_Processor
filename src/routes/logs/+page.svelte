<script lang="ts">
	import { listen } from "@tauri-apps/api/event";
	import { onMount, onDestroy } from "svelte";

	let logs: string[] = [];
	let unlisten: (() => void) | undefined;

	onMount(async () => {
		unlisten = await listen("rust-log", (e) => {
			logs = [...logs, e.payload as string];
			console.log("Heard log!");
		});
	});

	onDestroy(() => {
		if (unlisten) unlisten();
	});
</script>

<div class="log-window p-2 h-screen">
	<h1 class="font-bold text-center text-xl my-2">Logs</h1>

	<div class="logs rounded-md">
		{#each logs as line}
			<span class="m-2">{line}</span>
		{/each}
	</div>
</div>

<style>
	@reference tailwindcss;

	.log-window {
		@apply flex flex-col items-center;
	}

	.logs {
		@apply flex flex-col border-2 w-full h-full;
		overflow-y: auto;
	}
</style>
