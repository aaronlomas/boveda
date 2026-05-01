/**
 * autoLock.ts
 *
 * Monitors user activity (mouse, keyboard, touch) and automatically locks
 * the vault after a configurable period of inactivity.
 *
 * Usage:
 *   import { startAutoLock, stopAutoLock } from '$lib/autoLock';
 *
 *   // Start watching when the vault unlocks (inside your +page.svelte onMount)
 *   startAutoLock({ onLock: () => isUnlocked.set(false) });
 *
 *   // Stop watching when the vault locks manually or the component unmounts
 *   stopAutoLock();
 */

import { invoke } from '@tauri-apps/api/core';

// ─── Configuration ───────────────────────────────────────────────────────────

/** How many minutes of inactivity before the vault auto-locks. */
const AUTO_LOCK_MINUTES = 5;

/** Events that count as "user activity". */
const ACTIVITY_EVENTS: (keyof WindowEventMap)[] = [
  'mousemove',
  'mousedown',
  'keydown',
  'touchstart',
  'scroll',
];

// ─── Internal State ──────────────────────────────────────────────────────────

let timer: ReturnType<typeof setTimeout> | null = null;
let onLockCallback: (() => void) | null = null;

// ─── Public API ──────────────────────────────────────────────────────────────

export interface AutoLockOptions {
  /** Called when the inactivity timeout fires. Use this to update your UI store. */
  onLock: () => void;
  /** Override the default timeout in minutes (default: AUTO_LOCK_MINUTES). */
  minutes?: number;
}

/** Begin monitoring inactivity. Safe to call multiple times — resets the timer. */
export function startAutoLock(options: AutoLockOptions): void {
  onLockCallback = options.onLock;
  const minutes = options.minutes ?? AUTO_LOCK_MINUTES;

  ACTIVITY_EVENTS.forEach((event) => {
    window.addEventListener(event, resetTimer, { passive: true });
  });

  scheduleTimer(minutes);
}

/** Stop monitoring and clear the pending timer. */
export function stopAutoLock(): void {
  clearPendingTimer();
  ACTIVITY_EVENTS.forEach((event) => {
    window.removeEventListener(event, resetTimer);
  });
  onLockCallback = null;
}

// ─── Internal ────────────────────────────────────────────────────────────────

let _minutes = AUTO_LOCK_MINUTES;

function resetTimer(): void {
  scheduleTimer(_minutes);
}

function scheduleTimer(minutes: number): void {
  _minutes = minutes;
  clearPendingTimer();
  timer = setTimeout(triggerLock, minutes * 60 * 1000);
}

function clearPendingTimer(): void {
  if (timer !== null) {
    clearTimeout(timer);
    timer = null;
  }
}

async function triggerLock(): Promise<void> {
  try {
    await invoke('lock_vault');
  } catch (e) {
    console.error('[AutoLock] Failed to call lock_vault:', e);
  }
  onLockCallback?.();
  stopAutoLock();
}
