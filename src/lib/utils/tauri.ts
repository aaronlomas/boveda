import { invoke } from "@tauri-apps/api/core";
import { logStore } from "$lib/stores/log.svelte";
import type { Account, Pin } from "../stores/stores.svelte";

export async function isVaultInitialized(): Promise<boolean> {
  return invoke<boolean>("is_vault_initialized");
}

export async function unlockVault(password: string): Promise<string> {
  const t0 = performance.now();
  logStore.add("INIT", "Requesting vault unlock...");
  try {
    const res = await invoke<string>("unlock_vault", { password });
    const t1 = performance.now();
    logStore.add("SUCCESS", `Vault unlocked successfully [${Math.round(t1 - t0)}ms]`);
    return res;
  } catch (err: any) {
    // Sanitized
    logStore.add("ERROR", "Vault unlock failed. Check your master password.");
    throw err;
  }
}

export async function lockVault(): Promise<void> {
  logStore.add("MEM", "Clearing vault from memory. Locking...");
  return invoke("lock_vault");
}

export async function addAccount(
  site: string,
  username: string,
  password: string,
  recoveryCode: string,
  notes: string,
): Promise<string> {
  return invoke<string>("add_account", { site, username, password, recoveryCode, notes });
}

export async function readExternalFile(path: string): Promise<string> {
  return invoke<string>("read_external_file", { path });
}

export async function getAccounts(): Promise<Account[]> {
  const t0 = performance.now();
  logStore.add("NETWORK", "Fetching secure accounts payload...");
  try {
    const accounts = await invoke<Account[]>("get_accounts");
    const t1 = performance.now();
    // Sanitized
    logStore.add("SUCCESS", `Accounts payload decrypted and loaded [${Math.round(t1 - t0)}ms]`);
    return accounts;
  } catch (err: any) {
    logStore.add("ERROR", "Failed to load accounts payload.");
    throw err;
  }
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

// ─── PINs ───────────────────────────────────────────────────────────────────

export async function addPin(
  name: string,
  pin: string,
  notes: string,
): Promise<string> {
  return invoke<string>("add_pin", { name, pin, notes });
}

export async function getPins(): Promise<Pin[]> {
  return invoke<Pin[]>("get_pins");
}

export async function deletePin(id: string): Promise<void> {
  return invoke("delete_pin", { id });
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

// ─── Documents ───────────────────────────────────────────────────────────────

export async function addDocument(
  title: string,
  description: string | null,
  content: string,
): Promise<string> {
  return invoke<string>("add_document", { title, description, content });
}

export async function getDocuments(): Promise<import("../stores/stores.svelte").Document[]> {
  return invoke("get_documents");
}

export async function updateDocument(
  id: string,
  title: string,
  description: string | null,
  content: string,
): Promise<void> {
  return invoke("update_document", { id, title, description, content });
}

export async function deleteDocument(id: string): Promise<void> {
  return invoke("delete_document", { id });
}

export async function decryptDocumentContent(encryptedContent: string): Promise<string> {
  return invoke<string>("decrypt_document_content", { encryptedContent });
}

export async function decryptSecret(ciphertext: string): Promise<string> {
  const t0 = performance.now();
  logStore.add("DECRYPT", "Requesting payload decryption...");
  try {
    const cleartext = await invoke<string>("decrypt_secret", { ciphertext });
    const t1 = performance.now();
    logStore.add("CIPHER", `Payload decrypted. Integrity verified [${Math.round(t1 - t0)}ms]`);
    return cleartext;
  } catch (err: any) {
    logStore.add("ERROR", `Decryption failed: ${err}`);
    throw err;
  }
}

export async function getOsUsername(): Promise<string> {
  return invoke<string>("get_os_username");
}
