<script lang="ts">
  import Combobox from "./ComboBox.svelte";

  interface Props {
    arrayOptions: string[];
    title?: string;
    options: string[];
  }

  let {
    arrayOptions = $bindable([]),
    title = "",
    options = $bindable([]),
  }: Props = $props();

  let arrayRows: { id: number; value: string }[] = $state([]);
  let nextID = $state<number>(0);

  // $effect(() => {
  //   arrayRows = arrayOptions.map((val, i) => ({
  //     id: i + 100000 * Math.random(),
  //     value: val,
  //   }));
  //   nextID = arrayRows.length;
  // });

  $effect(() => {
    arrayOptions = arrayRows.map((row) => row.value);
  });

  function onAddMetaInfo() {
    arrayRows = [...arrayRows, { id: nextID++, value: "" }];
  }

  function onRemoveMetaInfo(id: number) {
    arrayRows = arrayRows.filter((row) => row.id !== id);
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
          onclick={() => onRemoveMetaInfo(row.id)}
          type="button">-</button
        >
      </div>
    {/each}
  </div>
  <button
    class="cursor-pointer w-1/2 mx-auto bg-blue-500 text-white hover:bg-blue-600 rounded text-center"
    onclick={onAddMetaInfo}
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
