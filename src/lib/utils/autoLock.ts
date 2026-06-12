/**
 * autoLock.ts
 *
 * Monitors user activity (mouse, keyboard, touch) and automatically locks
 * the vault after a configurable period of inactivity.
 *
 * Usage:
 *   startAutoLock({ onLock: () => ..., seconds: 60 });
 *   updateAutoLockSeconds(120); // live-reconfigure without restarting
 *   stopAutoLock();
 */

import { invoke } from "@tauri-apps/api/core";

/** Events that count as "user activity". */
const ACTIVITY_EVENTS: (keyof WindowEventMap)[] = [
  "mousemove",
  "mousedown",
  "keydown",
  "touchstart",
  "scroll",
];

// ─── Internal State ──────────────────────────────────────────────────────────

let timer: ReturnType<typeof setTimeout> | null = null;
let onLockCallback: (() => void) | null = null;
let _seconds = 0; // 0 means disabled

// ─── Public API ──────────────────────────────────────────────────────────────

export interface AutoLockOptions {
  /** Called when the inactivity timeout fires. */
  onLock: () => void;
  /** Timeout in seconds. 0 = disabled. */
  seconds?: number;
}

/** Begin monitoring inactivity. Safe to call multiple times — resets the timer. */
export function startAutoLock(options: AutoLockOptions): void {
  onLockCallback = options.onLock;
  _seconds = options.seconds ?? 0;

  ACTIVITY_EVENTS.forEach((event) => {
    window.removeEventListener(event, resetTimer);
    window.addEventListener(event, resetTimer, { passive: true });
  });

  scheduleTimer();
}

/** Update the timeout live without restarting listeners. */
export function updateAutoLockSeconds(seconds: number): void {
  _seconds = seconds;
  scheduleTimer(); // resets the countdown with the new value
}

/** Stop monitoring and clear the pending timer. */
export function stopAutoLock(): void {
  clearPendingTimer();
  ACTIVITY_EVENTS.forEach((event) => {
    window.removeEventListener(event, resetTimer);
  });
  onLockCallback = null;
  _seconds = 0;
}

// ─── Internal ────────────────────────────────────────────────────────────────

function resetTimer(): void {
  scheduleTimer();
}

function scheduleTimer(): void {
  clearPendingTimer();
  if (_seconds <= 0) return; // disabled
  timer = setTimeout(triggerLock, _seconds * 1000);
}

function clearPendingTimer(): void {
  if (timer !== null) {
    clearTimeout(timer);
    timer = null;
  }
}

async function triggerLock(): Promise<void> {
  try {
    await invoke("lock_vault");
  } catch (e) {
    console.error("[AutoLock] Failed to call lock_vault:", e);
  }
  onLockCallback?.();
  stopAutoLock();
}
