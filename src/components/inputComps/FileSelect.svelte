<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { readTextFileLines } from "@tauri-apps/plugin-fs";

  export let label: string = "Select File";
  // export let onFileSelect: (file: string) => void;
  export let file_path: string;
  export let headers: string[] = [];

  function truncatePath(path: string, max_len: number = 20): string {
    if (path.length <= max_len) return path;

    const ellipsis = "...";
    const keepEachSide = Math.floor((max_len - ellipsis.length) / 2);

    return path.slice(0, keepEachSide) + ellipsis + path.slice(-keepEachSide);
  }

  // TODO: extract headers
  async function extractHeaders(file: string): Promise<void> {
    try {
      const lines = await readTextFileLines(file);

      let header_line = (await lines.next()).value;

      if (header_line && typeof header_line === "string") {
        const delimiter = header_line.includes("\t") ? "\t" : ",";

        headers = header_line
          .replace(/[\r\n\u0000]/g, "")
          .split(delimiter)
          .map((header) => header.trim());
      }
    } catch (error) {
      console.error("Error reading:", error);
    }
  }

  let disp_text: string = "No File Selected";
  async function selectFile(): Promise<void> {
    console.log("Clicked");
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "delimited file", extensions: ["csv", "tsv", "txt"] }],
    });

    if (file && typeof file === "string") {
      //onFileSelect(file);
      file_path = file;
      disp_text = file;
      extractHeaders(file);
    }
  }
</script>

<div class="file-select">
  <button
    class="cursor-pointer"
    dir="ltr"
    id="file-select"
    on:click={selectFile}
    type="button">{label}</button
  >
  <span id="file-text">{truncatePath(disp_text, 30)}</span>
</div>

<style>
  @reference 'tailwindcss';

  .file-select {
    @apply flex items-center bg-slate-700 border-2 rounded-md w-full;
  }

  #file-select {
    @apply justify-center rounded-s-sm bg-sky-500 hover:bg-sky-800 px-4 h-full;
  }

  #file-text {
    @apply mx-2;
  }
</style>
