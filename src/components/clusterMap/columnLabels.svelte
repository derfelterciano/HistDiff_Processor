<script lang="ts">
  import { panZoom } from "./panZoomStore.svelte";

  interface Props {
    colOrder: string[];
    cellWidth: number;
    width: number;
    height: number;
  }

  const { colOrder, cellWidth, width, height }: Props = $props();
  // let labelDiv = $state<HTMLElement>();

  let scaledCellWidth = $derived(cellWidth * $panZoom.scale);
  let absWidth = $derived(colOrder.length * scaledCellWidth);
</script>

<div
  class="relative overflow-hidden"
  style="width: {width}px; height: {height}px;"
>
  <div
    class="flex items-end"
    style="
    width: {absWidth}px; 
    height: {height}px;
    transform: translateX({$panZoom.tx}px);
    "
  >
    {#each colOrder as name, num}
      <div
        class="absolute bottom-0 flex items-end justify-center"
        style="
        left: {Math.round(num * scaledCellWidth)}px;
        width: {Math.round(scaledCellWidth)}px;
        height: {height}px;
        "
      >
        <span class="label-item-span text-white text-[10px] whitespace-nowrap"
          >{name}</span
        >
      </div>
    {/each}
  </div>
</div>

<style>
  .label-item-span {
    display: inline-block;
    /* transform-origin: left bottom;
    transform: rotate(-90deg); */
    transform-origin: bottom center;
    transform: rotate(-90deg) translateY(50%) translateX(50%);
  }
</style>
