import { invoke } from "@tauri-apps/api/core";
import type { Account } from "./stores";

export async function isVaultInitialized(): Promise<boolean> {
  return invoke<boolean>("is_vault_initialized");
}

export async function unlockVault(password: string): Promise<boolean> {
  return invoke<boolean>("unlock_vault", { password });
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
