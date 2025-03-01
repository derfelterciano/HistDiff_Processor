import { get } from "svelte/store";
import { logStore } from "./logStorage.svelte.ts";
import { listen } from "@tauri-apps/api/event";

let unlisten: (() => void) | undefined;

/**
 * initialize listening to logs
 */
export async function initLoggerListener(): Promise<void> {
  // dont duplicate listeners
  if (unlisten) return;

  unlisten = await listen("rust-log", (e) => {
    // console.log("heard log!: ", e.payload as string);
    logStore.update((oldLogs) => [...oldLogs, e.payload as string]);
    // console.log("logInit: ", get(logStore));
  });
}

/**
 * Stop and destroy listener
 */
export function removeLogger() {
  unlisten?.();
  unlisten = undefined;
}
