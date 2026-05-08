import { invoke } from "@tauri-apps/api/core";
import type { Account } from "../stores/stores.svelte";

export async function isVaultInitialized(): Promise<boolean> {
  return invoke<boolean>("is_vault_initialized");
}

export async function unlockVault(password: string): Promise<string> {
  return invoke<string>("unlock_vault", { password });
}

export async function lockVault(): Promise<void> {
  return invoke("lock_vault");
}

export async function addAccount(
  site: string,
  username: string,
  password: string,
  notes: string,
): Promise<string> {
  return invoke<string>("add_account", { site, username, password, notes });
}

export async function getAccounts(): Promise<Account[]> {
  return invoke<Account[]>("get_accounts");
}

export async function deleteAccount(id: string): Promise<void> {
  return invoke("delete_account", { id });
}

export async function generatePassword(
  length: number,
  useSymbols: boolean,
): Promise<string> {
  return invoke<string>("generate_password", { length, useSymbols });
}

// ─── Groups ───────────────────────────────────────────────────────────────────

export async function getGroups(): Promise<string[]> {
  const raw = await invoke<string | null>("get_preference", { key: "groups" });
  if (!raw) return [];
  try { return JSON.parse(raw) as string[]; } catch { return []; }
}

export async function saveGroups(groups: string[]): Promise<void> {
  return invoke("set_preference", { key: "groups", value: JSON.stringify(groups) });
}

export async function updateAccountGroup(
  id: string,
  groupName: string | null,
): Promise<void> {
  return invoke("update_account_group", { id, groupName });
}

export async function renameGroup(
  oldName: string,
  newName: string,
): Promise<void> {
  return invoke("rename_group", { oldName, newName });
}

export async function deleteGroup(name: string): Promise<void> {
  return invoke("delete_group", { name });
}
