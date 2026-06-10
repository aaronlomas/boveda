import { dataState, uiState } from "$lib/stores/stores.svelte";
import { getGroups, saveGroups, renameGroup, deleteGroup } from "$lib/utils/tauri";
import { toast } from "$lib/stores/toast.svelte";
import { _ } from "svelte-i18n";
import { get } from "svelte/store";

/**
 * @composable useGroups
 * @description Composable reactive group management tool.
 * Controls the filtering, saving, renaming, and deletion of credential group tags.
 */
export function useGroups() {
  let loading = $state(false);
  let error = $state<string | null>(null);

  const t = get(_);

  /**
   * It loads the groups from Tauri's persistent storage and saves them in dataState..
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
   * Create a new group, making sure not to repeat existing names..
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
   * Rename a group by updating the corresponding accounts in memory and database.
   */
  async function rename(oldName: string, newName: string): Promise<void> {
    const trimmed = newName.trim();
    if (!trimmed || trimmed === oldName) return;
    try {
      await renameGroup(oldName, trimmed);

      // Update list in memory
      dataState.groups = dataState.groups.map((g) =>
        g === oldName ? trimmed : g,
      );

      // Update the assigned credentials
      dataState.accounts = dataState.accounts.map((a) =>
        a.group_name === oldName ? { ...a, group_name: trimmed } : a,
      );

      // Keeps the group filter active when renaming
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
   * Delete a group permanently.
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
    get groups() {
      return dataState.groups;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    refresh,
    add,
    rename,
    delete: remove,
  };
}
