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
  group_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface Document {
  id: string;
  title: string;
  encrypted_description: string | null;
  encrypted_content: string;
  created_at: string;
  updated_at: string;
}

export class DataState {
  accounts = $state<Account[]>([]);
  pins = $state<Pin[]>([]);
  documents = $state<Document[]>([]);
  /** Persisted list of group names. */
  groups = $state<string[]>([]);
  /** Persisted group colors. */
  groupColors = $state<Record<string, string>>({});
}

export const dataState = new DataState();
