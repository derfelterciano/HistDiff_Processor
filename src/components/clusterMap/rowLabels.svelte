<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { panZoom } from "./panZoomStore.svelte";
  import { jsonParser } from "./data";
  import { onMount } from "svelte";
  interface Props {
    rowOrder: string[];
    cellHeight: number;
    width: number;
    height: number;
  }

  const { rowOrder, cellHeight, width, height }: Props = $props();

  let labelDiv = $state<HTMLDivElement>();

  let negControls = $state<string[]>([]);

  onMount(async () => {
    let resp = await invoke<string[] | null>("get_neg_controls");
    negControls = jsonParser<string[]>(resp) ?? [];
  });

  $effect(() => {
    const { ty, scale } = $panZoom;
    if (labelDiv) {
      // labelDiv.style.transformOrigin = "top left";
      labelDiv.style.transform = `translateY(${ty}px)`;
    }
  });
</script>

<div
  class="overflow-hidden relative"
  style="width: {width}px; height: {height}px;"
>
  <div class="absolute top-0 left-0" bind:this={labelDiv}>
    {#each rowOrder as name}
      {#if negControls.includes(name)}
        <div
          class="whitespace-nowrap pl-2 box-border bg-green-300 border-1"
          style="height: {cellHeight *
            $panZoom.scale}px; line-height: {cellHeight * $panZoom.scale}px;"
        >
          <span class="text-red-600">{name}</span>
        </div>
      {:else}
        <div
          class="whitespace-nowrap pl-2 box-border"
          style="height: {cellHeight *
            $panZoom.scale}px; line-height: {cellHeight * $panZoom.scale}px;"
        >
          <span>{name}</span>
        </div>
      {/if}
    {/each}
  </div>
</div>
