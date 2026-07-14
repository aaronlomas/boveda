import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { logStore, type LogCategory } from "$lib/stores/log.svelte";

let unlistenPersistent: UnlistenFn | null = null;

function handleAuditPayload(payload: any): void {
  if (payload.action === "clear_log") {
    logStore.clear();
    return;
  }

  if (payload.action === "remote_blocked") {
    logStore.add("SEC", "Vault unlock blocked: remote session detected (AnyDesk/VNC/RDP)");
    return;
  }

  if (payload.action === "custom" || payload.msg) {
    logStore.add(payload.category || "SYSTEM", payload.msg);
    return;
  }
}

export async function startPersistentLogListener(): Promise<void> {
  if (unlistenPersistent) return;

  unlistenPersistent = await listen<{ action: string; msg?: string; category?: LogCategory }>(
    "boveda://audit",
    ({ payload }) => handleAuditPayload(payload)
  );
}

export function stopPersistentLogListener(): void {
  unlistenPersistent?.();
  unlistenPersistent = null;
}


