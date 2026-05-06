// ─── Modal Manager (Svelte 5 Runes) ──────────────────────────────────────────
//
// Centralizes all modal state in a single reactive store, following the same
// pattern as toast.svelte.ts. Mount <ModalHost /> once in +layout.svelte and
// open any modal from anywhere with modal.open*() calls.
//
// Usage:
//   import { modal } from '$lib/stores/modal.svelte';
//   modal.openAddCredential({ onadded: () => refresh() });
//   modal.openConfirm({ title, message, onconfirm: () => doSomething() });
//   modal.openPreferences();
//   modal.openAssignGroup({ accountId: '...', onassigned: () => refresh() });
//   modal.close();

// ─── Descriptor types ─────────────────────────────────────────────────────────

export interface AddCredentialPayload {
  onadded?: () => void;
}

export interface ConfirmPayload {
  title?: string;
  message?: string;
  confirmText?: string;
  cancelText?: string;
  type?: 'danger' | 'warning' | 'info';
  onconfirm?: () => void;
  oncancel?: () => void;
}

export interface AssignGroupPayload {
  accountId: string;
  currentGroup?: string | null;
  onassigned?: () => void;
}

type ModalDescriptor =
  | { kind: 'add-credential'; payload: AddCredentialPayload }
  | { kind: 'confirm'; payload: ConfirmPayload }
  | { kind: 'preferences' }
  | { kind: 'assign-group'; payload: AssignGroupPayload };

// ─── Store ────────────────────────────────────────────────────────────────────

class ModalManager {
  current = $state<ModalDescriptor | null>(null);

  openAddCredential(payload: AddCredentialPayload = {}): void {
    this.current = { kind: 'add-credential', payload };
  }

  openConfirm(payload: ConfirmPayload = {}): void {
    this.current = { kind: 'confirm', payload };
  }

  openPreferences(): void {
    this.current = { kind: 'preferences' };
  }

  openAssignGroup(payload: AssignGroupPayload): void {
    this.current = { kind: 'assign-group', payload };
  }

  close(): void {
    this.current = null;
  }
}

export const modal = new ModalManager();
