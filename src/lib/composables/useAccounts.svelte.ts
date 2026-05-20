import { dataState } from "$lib/stores/stores.svelte";
import { getAccounts, deleteAccount } from "$lib/utils/tauri";
import { toast } from "$lib/stores/toast.svelte";
import { modal } from "$lib/stores/modal.svelte";
import { _ } from "svelte-i18n";
import { get } from "svelte/store";

/**
 * @composable useAccounts
 * @description Composable reactivo para la gestión de credenciales (Accounts).
 * Encapsula la lógica de obtención de datos, estados de carga, manejo de errores y eliminación.
 */
export function useAccounts() {
  let loading = $state(false);
  let error = $state<string | null>(null);

  const t = get(_);

  /**
   * Refresca la lista de cuentas desde la base de datos local y actualiza dataState.
   */
  async function refresh(): Promise<void> {
    loading = true;
    error = null;
    try {
      dataState.accounts = await getAccounts();
    } catch (e) {
      console.error("Failed to load accounts:", e);
      error = t("accounts.load_error") || "Error loading credentials";
    } finally {
      loading = false;
    }
  }

  /**
   * Solicita confirmación visual y elimina una cuenta por ID.
   */
  async function remove(id: string): Promise<void> {
    const confirmed = await modal.openConfirm({
      title: t("accounts.delete_confirm_title"),
      message: t("accounts.delete_confirm_message"),
      confirmText: t("actions.delete"),
      type: "danger",
    });

    if (!confirmed) return;

    try {
      await deleteAccount(id);
      await refresh();
      toast.success(t("accounts.delete_success"));
    } catch (e) {
      console.error("Delete failed:", e);
      toast.error(t("accounts.delete_error"));
    }
  }

  return {
    get accounts() { return dataState.accounts; },
    get loading() { return loading; },
    get error() { return error; },
    refresh,
    delete: remove,
  };
}
