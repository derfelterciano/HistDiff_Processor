<script lang="ts">
  import { panZoom } from "./panZoomStore.svelte";

  interface Props {
    colOrder: string[];
    cellWidth: number;
    width: number;
    height: number;
  }

  const { colOrder, cellWidth, width, height }: Props = $props();
  let labelDiv = $state<HTMLElement>();
</script>

<div
  class="overflow-hidden relative"
  style="width: {width}px; height: {height}px; "
>
  <div
    class="absolute top-0 left-0 flex items-end h-full overflow-x-auto"
    style="transform: translateX({$panZoom.tx}px);"
    bind:this={labelDiv}
  >
    {#each colOrder as name}
      <div
        class="flex flex-col items-center"
        style="
			width: {Math.max(cellWidth * $panZoom.scale, 1)}px; height: 100%;
			"
      >
        <span
          class="text-white text-[10px] whitespace-nowrap"
          style="display: inline-block;
            transform: rotate(-90deg);
            transform-origin: bottom center;
            margin-bottom: 2px;">{name}</span
        >
      </div>
    {/each}
  </div>
</div>

<style>
  @reference 'tailwindcss';
</style>
