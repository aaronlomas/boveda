import { dataState, uiState } from "$lib/stores/stores.svelte";
import { getGroups, saveGroups, renameGroup, deleteGroup } from "$lib/utils/tauri";
import { toast } from "$lib/stores/toast.svelte";
import { _ } from "svelte-i18n";
import { get } from "svelte/store";

/**
 * @composable useGroups
 * @description Composable reactivo para la gestión de grupos.
 * Controla el filtrado, guardado, renombrado y borrado de etiquetas de grupo de credenciales.
 */
export function useGroups() {
  let loading = $state(false);
  let error = $state<string | null>(null);

  const t = get(_);

  /**
   * Carga los grupos desde el almacenamiento persistente de Tauri y los guarda en dataState.
   */
  async function refresh(): Promise<void> {
    loading = true;
    error = null;
    try {
      dataState.groups = await getGroups();
    } catch (e) {
      console.error("Failed to load groups:", e);
      error = "Error loading groups";
    } finally {
      loading = false;
    }
  }

  /**
   * Crea un nuevo grupo asegurándose de no repetir nombres existentes.
   */
  async function add(name: string): Promise<void> {
    const trimmed = name.trim();
    if (!trimmed || dataState.groups.includes(trimmed)) return;
    const updated = [...dataState.groups, trimmed];
    try {
      await saveGroups(updated);
      dataState.groups = updated;
    } catch (e) {
      console.error(e);
      toast.error("Error creating group");
    }
  }

  /**
   * Renombra un grupo actualizando los accounts correspondientes en memoria y base de datos.
   */
  async function rename(oldName: string, newName: string): Promise<void> {
    const trimmed = newName.trim();
    if (!trimmed || trimmed === oldName) return;
    try {
      await renameGroup(oldName, trimmed);
      
      // Actualiza lista en memoria
      dataState.groups = dataState.groups.map((g) =>
        g === oldName ? trimmed : g,
      );
      
      // Actualiza las credenciales asignadas
      dataState.accounts = dataState.accounts.map((a) =>
        a.group_name === oldName ? { ...a, group_name: trimmed } : a,
      );
      
      // Mantiene el filtro de grupo activo al renombrar
      if (uiState.activeGroup === oldName) {
        uiState.activeGroup = trimmed;
      }
      
      toast.success(t("groups.renamed_success"));
    } catch (e) {
      console.error("Rename group failed:", e);
      toast.error(t("groups.renamed_error"));
    }
  }

  /**
   * Borra un grupo permanentemente.
   */
  async function remove(name: string): Promise<void> {
    try {
      await deleteGroup(name);
      dataState.groups = dataState.groups.filter((g) => g !== name);
      
      if (uiState.activeGroup === name) {
        uiState.activeGroup = null;
      }
      
      toast.success(t("groups.deleted_success"));
    } catch (e: any) {
      console.error("Delete group failed:", e);
      toast.error(e?.toString() ?? t("groups.deleted_error"));
    }
  }

  return {
    get groups() { return dataState.groups; },
    get loading() { return loading; },
    get error() { return error; },
    refresh,
    add,
    rename,
    delete: remove,
  };
}
