<script lang="ts">
  import Combobox from "./ComboBox.svelte";

  export let arrayOptions: string[];
  export let title: string = "";
  export let options: string[] = [];

  let arrayRows: { id: number; value: string }[] = [];
  let nextID = 0;
  let lastArrayOptions: string[] = [];

  // Only update arrayRows when parent prop changes (and NOT from our own UI)
  $: if (arrayOptions !== lastArrayOptions) {
    arrayRows = arrayOptions.map((value, i) => ({
      id: i + 100000 * Math.random(),
      value,
    }));
    nextID = arrayRows.length;
    lastArrayOptions = arrayOptions;
  }

  function syncToParent() {
    arrayOptions = arrayRows.map((row) => row.value);
    lastArrayOptions = arrayOptions; // update guard
  }

  function onAddMetaInfo(): void {
    arrayRows = [...arrayRows, { id: nextID++, value: "" }];
    syncToParent();
  }

  function onRemoveMetaInfo(id: number): void {
    arrayRows = arrayRows.filter((row) => row.id !== id);
    syncToParent();
  }

  function onUpdateValue(index: number, val: string): void {
    arrayRows[index].value = val;
    syncToParent();
  }
</script>

<div class="dynamic-array">
  <h6 class="font-bold mb-2 text-center">{title}</h6>
  <div class="scroll-area mb-2 px-4">
    {#each arrayRows as row, index (row.id)}
      <div class="flex items-center space-x-2 mb-2">
        <Combobox bind:value={arrayRows[index].value} bind:options />
        <!-- <input
          class="bg-white text-black text-center rounded border w-full"
          type="text"
          value={option}
          placeholder="Enter information"
          on:input={(e) =>
            onUpdateMetaInfo(index, (e.target as HTMLInputElement).value)}
        /> -->
        <button
          class="bg-red-500 hover:bg-red-600 text-white font-bold py-1 px-3 rounded"
          on:click={() => onRemoveMetaInfo(row.id)}
          type="button">-</button
        >
      </div>
    {/each}
  </div>
  <button
    class="cursor-pointer w-1/2 mx-auto bg-blue-500 text-white hover:bg-blue-600 rounded text-center"
    on:click={onAddMetaInfo}
    type="button">+ Add Meta Info</button
  >
</div>

<style>
  @reference 'tailwindcss';

  .dynamic-array {
    @apply flex flex-col items-center border rounded-md w-full p-2;
    max-height: 150px;
    overflow-y: auto;
  }
  .scroll-area {
    @apply w-full;
    /* max-height: 150px;
    overflow-y: auto; */
  }
</style>
