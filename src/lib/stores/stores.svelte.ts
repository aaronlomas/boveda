// ─── Domain Types ─────────────────────────────────────────────────────────────

export interface Account {
  id: string;
  site: string;
  username: string;
  password_cipher: string;
  recovery_code_cipher: string | null;
  notes_cipher: string | null;
  favicon_url: string | null;
  group_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface Pin {
  id: string;
  name: string;
  encrypted_pin: string;
  encrypted_notes: string | null;
  created_at: string;
  updated_at: string;
}

/** All valid dashboard view identifiers. Adding a new view? Add it here first. */
export type ViewId =
  | "general"
  | "accounts"
  | "documents"
  | "about"
  | "pin"
  | "tokens"
  | "settings";

// ─── Global State (Svelte 5 Runes) ────────────────────────────────────────────

class AppState {
  isUnlocked = $state(false);
  accounts = $state<Account[]>([]);
  pins = $state<Pin[]>([]);
  sidebarCollapsed = $state(false);
  activeView = $state<ViewId>("general");
  /** The currently selected group filter; null means "All". */
  activeGroup = $state<string | null>(null);
  /** Persisted list of group names (loaded from preferences on unlock). */
  groups = $state<string[]>([]);
}

export const globalState = new AppState();
