/**
 * securityEvents.ts
 *
 * Listens to Tauri backend events emitted by the security layer:
 *   - `boveda://audit`          — Real-time audit event from boveda-core
 *   - `boveda://session-locked` — Vault was locked by an automatic mechanism (focus loss, etc.)
 *
 * Connects them to the visual log (logStore) and the session state.
 * Call `startSecurityListeners()` once when the vault is unlocked.
 * Call `stopSecurityListeners()` when it locks.
 */

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { logStore, type LogCategory } from "$lib/stores/log.svelte";

// Maps internal audit action IDs to human-readable log categories and messages
const AUDIT_MAP: Record<string, { category: LogCategory; msg: string }> = {
  vault_unlock:        { category: "AUTH",    msg: "Vault unlocked. Session initiated." },
  vault_lock:          { category: "AUTH",    msg: "Vault locked. Key cleared from memory." },
  vault_export:        { category: "EXPORT",  msg: "Full vault export requested." },
  vault_import:        { category: "IMPORT",  msg: "Vault import executed." },
  account_create:      { category: "WRITE",   msg: "New credential entry created." },
  account_delete:      { category: "WRITE",   msg: "Credential entry permanently deleted." },
  account_group_update:{ category: "WRITE",   msg: "Credential group assignment updated." },
  secret_access:       { category: "CIPHER",  msg: "Encrypted secret decrypted on demand." },
  totp_enabled:        { category: "AUTH",    msg: "TOTP 2FA enabled on vault." },
  totp_disabled:       { category: "WARN",    msg: "TOTP 2FA disabled. Single-factor only." },
  master_key_changed:  { category: "AUTH",    msg: "Master key rotated." },
  failed_login_attempt:{ category: "ERROR",   msg: "Failed vault unlock attempt detected." },
  pin_create:          { category: "WRITE",   msg: "New PIN entry created." },
  pin_delete:          { category: "WRITE",   msg: "PIN entry permanently deleted." },
  document_create:     { category: "WRITE",   msg: "Encrypted document created." },
  document_update:     { category: "WRITE",   msg: "Encrypted document updated." },
  document_delete:     { category: "WRITE",   msg: "Encrypted document deleted." },
};

let unlistenAudit: UnlistenFn | null = null;
let unlistenLocked: UnlistenFn | null = null;
let onSessionLock: (() => void) | null = null;

export async function startSecurityListeners(opts: {
  onLock: () => void;
}): Promise<void> {
  onSessionLock = opts.onLock;

  // Listen to audit events from Rust core
  unlistenAudit = await listen<{ action: string; trigger?: string; msg?: string }>(
    "boveda://audit",
    ({ payload }: { payload: any }) => {
      const isFocusLock = payload.trigger === "focus_lost";

      if (isFocusLock) {
        logStore.add("WARN", payload.msg ?? "Vault locked: window lost focus.");
        return;
      }

      if (payload.action === "clear_log") {
        logStore.clear();
        return;
      }

      if (payload.action === "custom") {
        logStore.add(payload.category, payload.msg);
        return;
      }

      const mapped = AUDIT_MAP[payload.action];
      if (mapped) {
        logStore.add(mapped.category, mapped.msg);
      } else {
        logStore.add("SYSTEM", payload.msg ?? `Core event: ${payload.action}`);
      }
    }
  );

  // Listen to the session-locked signal (auto-lock from focus loss or other triggers)
  unlistenLocked = await listen("boveda://session-locked", () => {
    onSessionLock?.();
  });
}

export function stopSecurityListeners(): void {
  unlistenAudit?.();
  unlistenLocked?.();
  unlistenAudit = null;
  unlistenLocked = null;
  onSessionLock = null;
}
