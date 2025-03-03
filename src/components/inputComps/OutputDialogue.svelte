<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { save } from "@tauri-apps/plugin-dialog";
	let savePath: string = "";
	export let saveReady: boolean = false;

	const path = async (): Promise<string | null> => {
		return await save({
			filters: [{ name: "extension", extensions: ["csv"] }],
		});
	};

	const click = async (): Promise<void> => {
		let fp = await path();
		if (fp && typeof fp === "string") {
			savePath = fp;
		}

		await invoke("write_res", { outPath: savePath });
		// console.log("path: ", fp);
	};
</script>

<button type="button" class="save-btn" onclick={click} disabled={!saveReady}
	>Save HistDiff Results</button
>

<style>
	@reference 'tailwindcss';

	.save-btn {
		@apply font-bold text-center p-2 mt-4 bg-purple-500 hover:bg-purple-600 border rounded;
	}
</style>
