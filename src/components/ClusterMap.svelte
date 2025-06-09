<script lang="ts">
  import DendrogramCanvas from "./clusterMap/dendrogramCanvas.svelte";
  import { panZoom, type PanZoom } from "./clusterMap/panZoomStore.svelte";
  import type { D3Node } from "../types/clustermapTypes";
  import {
    loadTreeData,
    loadHeatmapData,
    type HeatmapData,
    type ClusterRes,
    jsonParser,
    loadHeatmap,
  } from "./clusterMap/data";
  import HeatmapCanvas from "./clusterMap/heatmapCanvas.svelte";
  import { getLeafOrder } from "./clusterMap/utils";
  import RowLabels from "./clusterMap/rowLabels.svelte";
  import ColumnLabels from "./clusterMap/columnLabels.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";

  const { containerHeight } = $props<{ containerHeight: number }>();

  // let containerHeight = $state<number>(window.innerHeight);
  let infoMsg = $state<string | null>("Waiting for data!");
  let clusterCompleteUnlisten: (() => void) | null = null;
  let waitingForCluster = $state<boolean>(false);

  let treeData = $state<D3Node | null>(null);
  let featTreeData = $state<D3Node | null>(null);
  let rawHeatmapData = $state<Record<string, Record<string, number>> | null>(
    null
  );
  let heatmapData = $state<HeatmapData | null>(null);
  let toolTips = $state<boolean>(true);

  // Clustering options
  let clusterRows = $state<boolean>(true);
  let clusterCols = $state<boolean>(true);

  // WARNING: Test URLS
  const tree: string = "/row_tree.json";
  const scores: string = "/scores.json";
  const featureTree: string = "/feat_tree.json";

  // Dimensions
  const TREE_WIDTH = 100;
  const TOP_TREE_HEIGHT = 100;
  const LABEL_WIDTH = 50;

  // panel dimensions
  let treeWidth = $state<number>(TREE_WIDTH);
  // let treeHeight = $state<number>(window.innerHeight);
  let treeHeight = $state<number>(0);
  let heatmapWidth = $state<number>(
    window.innerWidth - TREE_WIDTH - LABEL_WIDTH
  );
  let heatmapHeight = $state<number>(treeHeight);
  let labelW = $state<number>(LABEL_WIDTH);

  // headers
  let controlsE1 = $state<HTMLElement>();
  let contentE1 = $state<HTMLDivElement>();
  let topContentE1 = $state<HTMLDivElement>();

  // load data
  // $effect(() => {
  //   loadTreeData(tree).then((data) => (treeData = data));
  //   loadTreeData(featureTree).then((data) => (featTreeData = data));
  //   loadHeatmapData(scores).then((raw) => {
  //     rawHeatmapData = raw;
  //     heatmapData = loadHeatmap(raw);
  //   });
  // });

  // recalculate dimensions
  $effect(() => {
    const onR = () => {
      // treeHeight = window.innerHeight;
      const headerH =
        (controlsE1?.clientHeight ?? 0) + (topContentE1?.clientHeight ?? 0);
      const availH = containerHeight - headerH;

      treeHeight = availH;
      heatmapWidth = window.innerWidth - TREE_WIDTH - LABEL_WIDTH;
      heatmapHeight = treeHeight;
    };

    window.addEventListener("resize", onR);
    onR();
    return () => window.removeEventListener("resize", onR);
  });

  // bound effect
  // const contentW = $derived(() => {
  // 	return (TREE_WIDTH + LABEL_WIDTH + heatmapWidth) * $panZoom.scale;
  // });
  // const contentH = $derived(() => {
  // 	return Math.max(treeHeight, heatmapHeight) * $panZoom.scale;
  // });
  //
  // $effect(() => {
  // 	panZoom.setMinScale(1);
  // 	const id = setTimeout(() => {
  // 		panZoom.setBounds(heatmapWidth, contentH(), contentW(), contentH());
  // 	}, 0);
  //
  // 	return () => clearTimeout(id);
  // });

  $effect(() => {
    const viewW = contentE1?.clientWidth;
    const viewH = contentE1?.clientHeight;

    const contentW = heatmapWidth;
    const contentH = Math.max(treeHeight, heatmapHeight);

    panZoom.setMinScale(1);
    panZoom.setBounds(heatmapWidth, contentH, contentW, contentH);
    // panZoom.setBounds(contentW, contentH, viewW as number, viewH as number);
  });

  // WARNING: Temporary zoom controls
  const zoomIn = () => {
    panZoom.zoom(1.2);
  };
  const zoomOut = () => {
    panZoom.zoom(0.8);
  };
  const reset = () => {
    panZoom.reset();
  };

  // derive leaf ordering
  let rowOrder = $derived((): string[] => {
    return treeData ? getLeafOrder(treeData) : [];
  });
  // let colOrder = $derived((): string[] => (heatmapData ? heatmapData.cols : []));
  let colOrder = $derived((): string[] => {
    return featTreeData ? getLeafOrder(featTreeData) : [];
  });

  const cellH = $derived(() => {
    const rows = (rowOrder() as string[]).length;
    return rows ? heatmapHeight / rows : 0;
  });

  const cellW = $derived(() => {
    const cols = (colOrder() as string[]).length;
    return cols ? heatmapWidth / cols : 0;
  });
  // onMount(async () => {
  // 	treeData = await loadTreeData(tree);
  // 	rawHeatmapData = await loadHeatmapData(scores);
  // 	heatmapData = loadHeatmap(rawHeatmapData);
  // });

  function resetVars() {
    treeData = null;
    heatmapData = null;
    rawHeatmapData = null;
    featTreeData = null;
  }

  async function loadClusterData() {
    resetVars();
    if (waitingForCluster) return;
    infoMsg = "waiting for data!";
    try {
      let rawScores = await invoke<Record<
        string,
        Record<string, number>
      > | null>("get_hd_scores");
      rawScores = jsonParser<Record<string, Record<string, number>>>(rawScores);
      if (!rawScores) {
        infoMsg =
          "HeatMapData has not been calculated yet! Please run HistDiff first.";
        await invoke("terminal", { msg: infoMsg });
        heatmapData = null;
        return;
      }
      rawHeatmapData = rawScores;
      heatmapData = loadHeatmap(rawScores);

      let res = await invoke<ClusterRes | null>("get_cluster_res");
      res = jsonParser<ClusterRes>(res);
      if (!res) {
        infoMsg =
          "ClusterData has not been calculated yet! Calculating data now!";
        await invoke("terminal", { msg: infoMsg });
        waitingForCluster = true;

        if (!clusterCompleteUnlisten) {
          clusterCompleteUnlisten = await listen(
            "cluster-complete",
            async () => {
              waitingForCluster = false;
              infoMsg = "Clustering complete! Loading tree data...";
              await invoke("terminal", { msg: infoMsg });
              // Now fetch cluster results
              let finishedRes = await invoke<ClusterRes | null>(
                "get_cluster_res"
              );
              finishedRes = jsonParser<ClusterRes>(finishedRes);
              if (finishedRes) {
                treeData = jsonParser<D3Node>(finishedRes.row_cluster);
                featTreeData = jsonParser<D3Node>(finishedRes.col_cluster);
                await invoke("terminal", {
                  msg: "Got Tree Data!",
                });
              } else {
                infoMsg = "Cluster results still not found after completion!";
                await invoke("terminal", { msg: infoMsg });
              }
            }
          );
        }

        await invoke("cluster_hd", {
          matMetric: "Pearson",
          linkage: "Complete",
          features: true,
        });

        return;

        // res = await invoke<ClusterRes | null>("get_cluster_res");
      }

      treeData = jsonParser<D3Node>(res.row_cluster);
      featTreeData = jsonParser<D3Node>(res.col_cluster);

      if (treeData && featTreeData && heatmapData) {
        await invoke("terminal", { msg: "Got Data!" });
        console.log(res);
        // await invoke("terminal", {msg: treeData});
      }
    } catch (err) {
      infoMsg = "Error loading cluster data";
      await invoke("terminal", { msg: infoMsg });
    }
  }

  onDestroy(() => {
    if (clusterCompleteUnlisten) {
      clusterCompleteUnlisten();
      clusterCompleteUnlisten = null;
    }

    resetVars();
  });
</script>

<div
  class="controls flex items-center justify-between px-2 py-1"
  bind:this={controlsE1}
>
  <div class="flex-none flex items-center space-x-2">
    <button
      class="bg-blue-400 hover:bg-blue-300 p-1 border-1 rounded"
      type="button"
      onclick={loadClusterData}
      disabled={waitingForCluster}>Load Data</button
    >
    <label class="flex items-center space-x-1 ml-2 cursor-pointer select-none">
      <input type="checkbox" bind:checked={toolTips} class="accent-blue-500" />
      <span class="text-xs text-white">Show tooltips</span>
    </label>

    <label class="flex items-center space-x-1 ml-2 cursor-pointer select-none">
      <input
        type="checkbox"
        bind:checked={clusterCols}
        class="accent-blue-500"
      />
      <span class="text-xs text-white">Cluster Features</span>
    </label>

    <label class="flex items-center space-x-1 ml-2 cursor-pointer select-none">
      <input
        type="checkbox"
        bind:checked={clusterRows}
        class="accent-blue-500"
      />
      <span class="text-xs text-white">Cluster Rows</span>
    </label>
  </div>
  <div class="flex flex-none space-x-2">
    <button
      class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
      type="button"
      onclick={zoomIn}>Zoom In</button
    >
    <button
      class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
      type="button"
      onclick={zoomOut}>Zoom Out</button
    >
    <button
      class="bg-red-500 hover:bg-red-400 p-1 border-1 rounded"
      type="button"
      onclick={reset}>Reset</button
    >
  </div>
  <div class="flex-none">
    <!-- Future buttons go here -->
  </div>
</div>

<div class="top-cluster" bind:this={topContentE1}>
  {#if featTreeData && clusterCols}
    <div style="padding-left: {TREE_WIDTH}px; padding-right: {LABEL_WIDTH}px;">
      <DendrogramCanvas
        treeData={featTreeData}
        height={TOP_TREE_HEIGHT}
        width={heatmapWidth}
        orientation="top"
      />
    </div>
  {/if}

  {#if colOrder()}
    <div style="padding-left: {TREE_WIDTH}px; padding-right: {LABEL_WIDTH}px;">
      <ColumnLabels
        colOrder={clusterCols
          ? colOrder()
          : heatmapData
            ? heatmapData.cols
            : []}
        height={80}
        width={heatmapWidth}
        cellWidth={cellW()}
      ></ColumnLabels>
    </div>
  {/if}
</div>

<div class="basic-clustermap flex overflow-hidden" bind:this={contentE1}>
  {#if treeData && clusterRows}
    <div class="flex-none">
      <DendrogramCanvas
        {treeData}
        orientation="left"
        width={treeWidth}
        height={treeHeight}
      />
    </div>
  {:else}
    <!-- <p>Loading Dendrograms...</p> -->
    <p>{infoMsg}</p>
  {/if}

  {#if heatmapData && rawHeatmapData && (rowOrder() as string[]).length}
    <!-- Heatmap -->
    <div class="flex-1">
      <HeatmapCanvas
        data={heatmapData}
        raw={rawHeatmapData}
        rowOrder={clusterRows
          ? (rowOrder() as string[])
          : heatmapData
            ? heatmapData.rows
            : []}
        colOrder={clusterCols
          ? (colOrder() as string[])
          : heatmapData
            ? heatmapData.cols
            : []}
        width={heatmapWidth}
        height={heatmapHeight}
        showToolTips={toolTips}
      />
    </div>
  {:else if treeData}
    <!-- <p>Loading heatmap…</p> -->
    <p>{infoMsg}</p>
  {/if}

  {#if (rowOrder() as string[]).length}
    <div class="flex-none">
      <RowLabels
        rowOrder={clusterRows
          ? (rowOrder() as string[])
          : heatmapData
            ? heatmapData.rows
            : []}
        cellHeight={cellH()}
        width={labelW}
        height={treeHeight}
      />
    </div>
  {/if}
</div>

<style>
  @reference 'tailwindcss';

  .basic-clustermap {
    @apply h-[100%];
  }
</style>
