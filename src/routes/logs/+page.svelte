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

<h1>Logs</h1>

<div class="logs">
	{#each logs as line}
		<span>{line}</span>
	{/each}
</div>
