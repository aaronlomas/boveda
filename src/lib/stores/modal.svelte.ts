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

export interface ExportPackagePayload {
  title: string;
  desc: string;
  buttonText: string;
  onconfirm: (password: string) => void;
  oncancel?: () => void;
}

export interface ImportPackagePayload {
  title: string;
  desc: string;
  buttonText: string;
  onconfirm: (password: string, strategy: 'merge' | 'replace') => void;
  oncancel?: () => void;
}

type ModalDescriptor =
  | { kind: 'add-credential'; payload: AddCredentialPayload }
  | { kind: 'confirm'; payload: ConfirmPayload }
  | { kind: 'assign-group'; payload: AssignGroupPayload }
  | { kind: 'export-package'; payload: ExportPackagePayload }
  | { kind: 'import-package'; payload: ImportPackagePayload };

// ─── Store ────────────────────────────────────────────────────────────────────

class ModalManager {
  current = $state<ModalDescriptor | null>(null);

  openAddCredential(payload: AddCredentialPayload = {}): void {
    this.current = { kind: 'add-credential', payload };
  }

  openConfirm(payload: ConfirmPayload = {}): void {
    this.current = { kind: 'confirm', payload };
  }


  openAssignGroup(payload: AssignGroupPayload): void {
    this.current = { kind: 'assign-group', payload };
  }

  openExportPackage(payload: ExportPackagePayload): void {
    this.current = { kind: 'export-package', payload };
  }

  openImportPackage(payload: ImportPackagePayload): void {
    this.current = { kind: 'import-package', payload };
  }

  close(): void {
    this.current = null;
  }
}

export const modal = new ModalManager();
