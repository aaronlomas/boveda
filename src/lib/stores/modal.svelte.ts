// ─── Modal Manager (Svelte 5 Runes) ──────────────────────────────────────────
//
// Centralizes all modal state in a single reactive store, following the same
// pattern as toast.svelte.ts. Mount <ModalHost /> once in +layout.svelte and
// open any modal from anywhere with modal.open*() calls.
//
// Usage:
//   import { modal } from '$lib/stores/modal.svelte';
//
//   const added    = await modal.openAddCredential();
//   const confirmed = await modal.openConfirm({ title, message });
//   const assigned = await modal.openAssignGroup({ accountId: '...' });
//   const password = await modal.openExportPackage({ title, buttonText });
//   const result   = await modal.openImportPackage({ title, buttonText });
//   modal.close();

// ─── Descriptor types ─────────────────────────────────────────────────────────

export interface AddCredentialPayload {
  onadded?: () => void;
  oncancel?: () => void;
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
  itemType?: "account" | "pin";
  currentGroup?: string | null;
  onassigned?: () => void;
  oncancel?: () => void;
}

export interface ExportPackagePayload {
  onconfirm: (password: string) => void;
  oncancel?: () => void;
}

export interface ImportPackagePayload {
  onconfirm: (password: string, strategy: 'merge' | 'replace') => void;
  oncancel?: () => void;
}

export interface AddPinPayload {
  onadded?: () => void;
  oncancel?: () => void;
}

export interface VerifyMasterPayload {
  onconfirm: () => void;
  oncancel?: () => void;
}

type ModalDescriptor =
  | { kind: 'add-credential'; payload: AddCredentialPayload }
  | { kind: 'confirm'; payload: ConfirmPayload }
  | { kind: 'assign-group'; payload: AssignGroupPayload }
  | { kind: 'export-package'; payload: ExportPackagePayload }
  | { kind: 'import-package'; payload: ImportPackagePayload }
  | { kind: 'add-pin'; payload: AddPinPayload }
  | { kind: 'verify-master'; payload: VerifyMasterPayload };

// ─── Store ────────────────────────────────────────────────────────────────────

class ModalManager {
  current = $state<ModalDescriptor | null>(null);

  openAddCredential(): Promise<boolean> {
    return new Promise((resolve) => {
      this.current = { 
        kind: 'add-credential', 
        payload: {
          onadded: () => resolve(true),
          oncancel: () => resolve(false)
        } 
      };
    });
  }

  openAddPin(): Promise<boolean> {
    return new Promise((resolve) => {
      this.current = { 
        kind: 'add-pin', 
        payload: {
          onadded: () => resolve(true),
          oncancel: () => resolve(false)
        } 
      };
    });
  }

  openConfirm(payload: Omit<ConfirmPayload, 'onconfirm'|'oncancel'> = {}): Promise<boolean> {
    return new Promise((resolve) => {
      this.current = { 
        kind: 'confirm', 
        payload: {
          ...payload,
          onconfirm: () => resolve(true),
          oncancel: () => resolve(false)
        } 
      };
    });
  }


  openAssignGroup(payload: Omit<AssignGroupPayload, 'onassigned'|'oncancel'>): Promise<boolean> {
    return new Promise((resolve) => {
      this.current = { 
        kind: 'assign-group', 
        payload: {
          ...payload,
          onassigned: () => resolve(true),
          oncancel: () => resolve(false)
        } 
      };
    });
  }

  openExportPackage(): Promise<string | null> {
    return new Promise((resolve) => {
      this.current = { 
        kind: 'export-package', 
        payload: {
          onconfirm: (password) => resolve(password),
          oncancel: () => resolve(null)
        } 
      };
    });
  }

  openImportPackage(): Promise<{password: string, strategy: 'merge'|'replace'} | null> {
    return new Promise((resolve) => {
      this.current = { 
        kind: 'import-package', 
        payload: {
          onconfirm: (password, strategy) => resolve({password, strategy}),
          oncancel: () => resolve(null)
        } 
      };
    });
  }

  openVerifyMaster(): Promise<boolean> {
    return new Promise((resolve) => {
      this.current = {
        kind: 'verify-master',
        payload: {
          onconfirm: () => resolve(true),
          oncancel: () => resolve(false),
        },
      };
    });
  }

  close(): void {
    // If a modal is closed programmatically without triggering a callback, we should probably resolve the promise as cancelled.
    // However, the standard flow uses the specific callbacks inside ModalHost, which will call resolve and then modal.close().
    this.current = null;
  }
}

export const modal = new ModalManager();
