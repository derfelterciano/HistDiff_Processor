<script lang="ts">
  import Combobox from "./ComboBox.svelte";

  export let arrayOptions: string[];
  export let title: string = "";
  export let options: string[] = [];

  function onAddMetaInfo(): void {
    arrayOptions = [...arrayOptions, ""];
  }

  function onUpdateMetaInfo(index: number, value: string): void {
    arrayOptions[index] = value;
    arrayOptions = arrayOptions;
  }

  function onRemoveMetaInfo(index: number): void {
    arrayOptions = arrayOptions.filter((_, i) => i !== index);
  }
</script>

<div class="dynamic-array">
  <h6 class="font-bold mb-2 text-center">{title}</h6>
  <div class="scroll-area mb-2 px-4">
    {#each arrayOptions as option, index}
      <div class="flex items-center space-x-2 mb-2">
        <Combobox bind:value={arrayOptions[index]} bind:options />
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
          on:click={() => onRemoveMetaInfo(index)}
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
