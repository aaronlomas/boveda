/**
 * logEvents.ts
 *
 * Connects them to the visual log (logStore) and the session state.
 * Call `startLogListeners()` once when the vault is unlocked.
 * Call `stopLogListeners()` when it locks.
 */

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { logStore, type LogCategory } from "$lib/stores/log.svelte";

let unlistenAudit: UnlistenFn | null = null;

export async function startLogListeners(): Promise<void> {
  if (unlistenAudit) {
    unlistenAudit();
    unlistenAudit = null;
  }

  // Listen to audit events from Rust core
  unlistenAudit = await listen<{ action: string; msg?: string; category?: LogCategory }>(
    "boveda://audit",
    ({ payload }: { payload: any }) => {
      if (payload.action === "clear_log") {
        logStore.clear();
        return;
      }

      if (payload.action === "custom" || payload.msg) {
        logStore.add(payload.category || "SYSTEM", payload.msg);
        return;
      }
    }
  );
}

export function stopLogListeners(): void {
  unlistenAudit?.();
  unlistenAudit = null;
}
