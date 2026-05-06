// ─── Performance Store (Svelte 5 Runes) ──────────────────────────────────────
//
// Manages user-facing performance settings persisted to localStorage.
// Import and use anywhere:
//   import { performanceStore } from '$lib/stores/performance.svelte';
//   performanceStore.massiveList  // boolean reactive state
//   performanceStore.toggle()     // toggle and persist

const STORAGE_KEY = 'boveda_massive_list';

class PerformanceStore {
  massiveList = $state(false);

  init(): void {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored !== null) {
        this.massiveList = stored === 'true';
      }
    } catch {
      // localStorage unavailable (e.g. during SSR) — keep default
    }
  }

  setMassiveList(value: boolean): void {
    this.massiveList = value;
    try {
      localStorage.setItem(STORAGE_KEY, String(value));
    } catch {
      // ignore write errors
    }
  }

  toggle(): void {
    this.setMassiveList(!this.massiveList);
  }
}

export const performanceStore = new PerformanceStore();
