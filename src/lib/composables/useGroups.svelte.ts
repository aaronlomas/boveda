import { dataState, uiState } from "$lib/stores/stores.svelte";
import { getGroups, saveGroups, renameGroup, deleteGroup, getGroupColors, saveGroupColors } from "$lib/utils/tauri";
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
      dataState.groupColors = await getGroupColors();
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
  async function add(name: string, color?: string | null): Promise<void> {
    const trimmed = name.trim();
    if (!trimmed || dataState.groups.includes(trimmed)) return;
    const updated = [...dataState.groups, trimmed];
    try {
      await saveGroups(updated);
      dataState.groups = updated;

      if (color) {
        const newColors = { ...dataState.groupColors, [trimmed]: color };
        await saveGroupColors(newColors);
        dataState.groupColors = newColors;
      }
    } catch (e) {
      console.error(e);
      toast.error("Error creating group");
    }
  }

  /**
   * Rename a group by updating the corresponding accounts in memory and database.
   */
  async function rename(oldName: string, newName: string, newColor?: string | null): Promise<void> {
    const trimmed = newName.trim();
    if (!trimmed) return;
    try {
      const nameChanged = trimmed !== oldName;
      
      if (nameChanged) {
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
      }

      // Update color key
      const newColors = { ...dataState.groupColors };
      let colorChanged = false;

      if (nameChanged) {
        delete newColors[oldName];
        colorChanged = true;
      }

      const finalColor = newColor !== undefined ? newColor : dataState.groupColors[oldName];
      if (finalColor) {
        if (newColors[trimmed] !== finalColor) {
          newColors[trimmed] = finalColor;
          colorChanged = true;
        }
      } else if (newColors[trimmed]) {
        delete newColors[trimmed];
        colorChanged = true;
      }

      if (colorChanged) {
        await saveGroupColors(newColors);
        dataState.groupColors = newColors;
      }

      if (nameChanged || colorChanged) {
        toast.success(t("groups.renamed_success"));
      }
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

      const newColors = { ...dataState.groupColors };
      if (newColors[name]) {
        delete newColors[name];
        await saveGroupColors(newColors);
        dataState.groupColors = newColors;
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
