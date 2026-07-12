/** All valid dashboard view identifiers. */
export type ViewId =
  | "general"
  | "accounts"
  | "documents"
  | "about"
  | "pin"
  | "tokens"
  | "settings"
  | "import";

export class UIState {
  sidebarCollapsed = $state(false);
  activeView = $state<ViewId>("general");
  /** The currently selected group filter; null means "All". */
  activeGroup = $state<string | null>(null);
  /** When true, all capsules are locked and cannot be expanded. */
  capsuleLocked = $state(false);
}

export const uiState = new UIState();
