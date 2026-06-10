import { dataState } from "$lib/stores/stores.svelte";
import { getPins, deletePin } from "$lib/utils/tauri";
import { toast } from "$lib/stores/toast.svelte";
import { modal } from "$lib/stores/modal.svelte";
import { _ } from "svelte-i18n";
import { get } from "svelte/store";

/**
 * @composable usePins
 * @description Composable reactive system for security PIN management.
 * Controls PIN loading, display, and disposal.
 */
export function usePins() {
  let loading = $state(false);
  let error = $state<string | null>(null);

  const t = get(_);

  /**
   * Load the PINs from the Tauri database and update the reactive dataState.
   */
  async function refresh(): Promise<void> {
    loading = true;
    error = null;
    try {
      dataState.pins = await getPins();
    } catch (e) {
      console.error("Failed to load pins:", e);
      error = t("pin_security.load_error") || "Error loading PINs";
    } finally {
      loading = false;
    }
  }

  /**
   * Request confirmation and securely delete a PIN.
   */
  async function remove(id: string): Promise<void> {
    const confirmed = await modal.openConfirm({
      title: t("pin_security.delete_confirm_pin") || "Delete PIN",
      message: t("pin_security.delete_confirm_message"),
      confirmText: t("actions.delete"),
      type: "danger",
    });

    if (!confirmed) return;

    try {
      await deletePin(id);
      await refresh();
      toast.success(t("pin_security.deleted_success"));
    } catch (e) {
      console.error("Delete pin failed:", e);
      toast.error(t("pin_security.deleted_error"));
    }
  }

  return {
    get pins() {
      return dataState.pins;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    refresh,
    delete: remove,
  };
}
