import { writable } from "svelte/store";

export interface Account {
  id: string;
  site: string;
  username: string;
  password: string;
  notes: string;
  favicon_url: string | null;
  created_at: string;
  updated_at: string;
}

export const isUnlocked = writable<boolean>(false);
export const accounts = writable<Account[]>([]);
export const sidebarCollapsed = writable<boolean>(false);
export const showAddModal = writable<boolean>(false);
export const activeView = writable<string>('general');
