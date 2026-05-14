<script lang="ts">
  import { modal } from '$lib/stores/modal.svelte';
  import AddCredentialModal from './forms/AddCredentialModal.svelte';
  import ConfirmModal from './confirmation/ConfirmModal.svelte';
  import AssignGroupModal from './forms/AssignGroupModal.svelte';
  import ExportPasswordModal from './forms/ExportPasswordModal.svelte';
  import ImportPackageModal from './forms/ImportPackageModal.svelte';
</script>

<!--
  ModalHost — mounted once in +layout.svelte (same as <Toast />).
  Reads modal.current and renders the active modal reactively.
  All modals share the same close path: modal.close().
-->

{#if modal.current?.kind === 'add-credential'}
  <AddCredentialModal
    onadded={() => {
      modal.current?.kind === 'add-credential' && modal.current.payload.onadded?.();
    }}
    onclose={() => modal.close()}
  />
{:else if modal.current?.kind === 'confirm'}
  <ConfirmModal
    title={modal.current.payload.title}
    message={modal.current.payload.message}
    confirmText={modal.current.payload.confirmText}
    cancelText={modal.current.payload.cancelText}
    type={modal.current.payload.type}
    onconfirm={() => {
      const payload = modal.current?.kind === 'confirm' ? modal.current.payload : null;
      modal.close();
      payload?.onconfirm?.();
    }}
    oncancel={() => {
      const payload = modal.current?.kind === 'confirm' ? modal.current.payload : null;
      modal.close();
      payload?.oncancel?.();
    }}
  />
{:else if modal.current?.kind === 'assign-group'}
  <AssignGroupModal
    accountId={modal.current.payload.accountId}
    currentGroup={modal.current.payload.currentGroup}
    onassigned={() => {
      modal.current?.kind === 'assign-group' && modal.current.payload.onassigned?.();
    }}
    onclose={() => modal.close()}
  />
{:else if modal.current?.kind === 'export-package'}
  <ExportPasswordModal
    title={modal.current.payload.title}
    desc={modal.current.payload.desc}
    buttonText={modal.current.payload.buttonText}
    onconfirm={(password) => {
      const payload = modal.current?.kind === 'export-package' ? modal.current.payload : null;
      modal.close();
      payload?.onconfirm(password);
    }}
    oncancel={() => {
      const payload = modal.current?.kind === 'export-package' ? modal.current.payload : null;
      modal.close();
      payload?.oncancel?.();
    }}
  />
{:else if modal.current?.kind === 'import-package'}
  <ImportPackageModal
    title={modal.current.payload.title}
    desc={modal.current.payload.desc}
    buttonText={modal.current.payload.buttonText}
    onconfirm={(password, strategy) => {
      const payload = modal.current?.kind === 'import-package' ? modal.current.payload : null;
      modal.close();
      payload?.onconfirm(password, strategy);
    }}
    oncancel={() => {
      const payload = modal.current?.kind === 'import-package' ? modal.current.payload : null;
      modal.close();
      payload?.oncancel?.();
    }}
  />
{/if}
