// ─── Types ────────────────────────────────────────────────────────────────────

export type ToastKind = "success" | "error" | "info" | "warning";

export interface Toast {
  id: number;
  kind: ToastKind;
  message: string;
  /** Duration in ms before auto-dismiss. 0 = persistent. */
  duration: number;
}

// ─── Defaults ─────────────────────────────────────────────────────────────────

const DEFAULT_DURATION: Record<ToastKind, number> = {
  success: 3500,
  info: 3000,
  warning: 4500,
  error: 5000,
};

// ─── Store ────────────────────────────────────────────────────────────────────

let _nextId = 0;

class ToastStore {
  items = $state<Toast[]>([]);

  add(kind: ToastKind, message: string, duration?: number): void {
    const id = ++_nextId;
    const ms = duration ?? DEFAULT_DURATION[kind];

    this.items.push({ id, kind, message, duration: ms });
  }

  dismiss(id: number): void {
    this.items = this.items.filter((t) => t.id !== id);
  }

  success(message: string, duration?: number) {
    this.add("success", message, duration);
  }
  error(message: string, duration?: number) {
    this.add("error", message, duration);
  }
  info(message: string, duration?: number) {
    this.add("info", message, duration);
  }
  warning(message: string, duration?: number) {
    this.add("warning", message, duration);
  }
}

export const toast = new ToastStore();
