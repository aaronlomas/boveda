/**
 * @composable useDocuments
 * @description Reactive composable for managing notes and encrypted documents.
 * Coordinates the current view, editor state, and persistence in Boveda Core.
 */
import { dataState } from "$lib/stores/stores.svelte";
import {
  addDocument,
  getDocuments,
  updateDocument,
  deleteDocument,
  decryptDocumentContent,
} from "$lib/utils/tauri";
import { toast } from "$lib/stores/toast.svelte";
import { modal } from "$lib/stores/modal.svelte";
import { _ } from "svelte-i18n";
import { get } from "svelte/store";

export function useDocuments() {
  const t = get(_);

  // ── View state ──────────────────────────────────────────────────────────────
  let content = $state("");
  let currentView = $state<"selection" | "editor" | "import" | "list">("selection");
  let showSaveModal = $state(false);

  // ── Edit mode – when opening an existing document ───────────────────────────
  let editingDocumentId = $state<string | null>(null);

  // ── Loading ─────────────────────────────────────────────────────────────────
  let loading = $state(false);

  // ── Navigation ──────────────────────────────────────────────────────────────
  function startNew() {
    content = "";
    editingDocumentId = null;
    currentView = "editor";
  }

  function startExisting() {
    currentView = "import";
  }

  function startImported(text: string) {
    content = text;
    editingDocumentId = null;
    currentView = "editor";
  }

  function startList() {
    refresh();
    currentView = "list";
  }

  function startSave() {
    showSaveModal = true;
  }

  function goBack() {
    editingDocumentId = null;
    content = "";
    currentView = "selection";
  }

  // ── Data ────────────────────────────────────────────────────────────────────

  /** Reloads the document list from the backend. */
  async function refresh(): Promise<void> {
    loading = true;
    try {
      dataState.documents = await getDocuments();
    } catch (e) {
      console.error("Failed to load documents:", e);
      toast.error(t("documents.load_error") || "Error al cargar documentos");
    } finally {
      loading = false;
    }
  }

  /**
   * Called from SaveNoteModal – creates a new document OR updates an existing one.
   */
  async function handleSave(title: string, description: string): Promise<void> {
    const desc = description.trim() || null;
    try {
      if (editingDocumentId) {
        await updateDocument(editingDocumentId, title, desc, content);
        toast.success(t("documents.save_success") || "Documento actualizado");
      } else {
        await addDocument(title, desc, content);
        toast.success(t("documents.save_success") || "Documento guardado y cifrado");
      }
      showSaveModal = false;
      editingDocumentId = null;
      content = "";
      currentView = "selection";
      // Refresh list in the background so it's ready for next visit
      getDocuments().then((docs) => { dataState.documents = docs; }).catch(() => {});
    } catch (e) {
      console.error("Save failed:", e);
      toast.error(t("documents.save_error") || "Error al guardar el documento");
    }
  }

  /**
   * Opens an existing document in the editor by decrypting its content on demand.
   */
  async function openDocument(id: string): Promise<void> {
    const doc = dataState.documents.find((d) => d.id === id);
    if (!doc) return;
    loading = true;
    try {
      const decrypted = await decryptDocumentContent(doc.encrypted_content);
      content = decrypted;
      editingDocumentId = id;
      currentView = "editor";
    } catch (e) {
      console.error("Failed to decrypt document:", e);
      toast.error(t("documents.decrypt_error") || "Error al descifrar el documento");
    } finally {
      loading = false;
    }
  }

  /**
   * Asks for confirmation then permanently deletes a document.
   */
  async function removeDocument(id: string): Promise<void> {
    const confirmed = await modal.openConfirm({
      title: t("documents.delete_confirm_title") || "¿Eliminar documento?",
      message: t("documents.delete_confirm_message") || "Esta acción no se puede deshacer.",
      confirmText: t("actions.delete") || "Eliminar",
      type: "danger",
    });
    if (!confirmed) return;
    try {
      await deleteDocument(id);
      dataState.documents = dataState.documents.filter((d) => d.id !== id);
      toast.success(t("documents.delete_success") || "Documento eliminado");
    } catch (e) {
      console.error("Delete failed:", e);
      toast.error(t("documents.delete_error") || "Error al eliminar el documento");
    }
  }

  return {
    get content() { return content; },
    set content(val) { content = val; },
    get currentView() { return currentView; },
    get showSaveModal() { return showSaveModal; },
    set showSaveModal(val) { showSaveModal = val; },
    get editingDocumentId() { return editingDocumentId; },
    get loading() { return loading; },
    get documents() { return dataState.documents; },
    startNew,
    startExisting,
    startImported,
    startList,
    startSave,
    handleSave,
    goBack,
    refresh,
    openDocument,
    removeDocument,
  };
}
