export * from './ui.svelte';
export * from './data.svelte';
export * from './session.svelte';

import { uiState } from './ui.svelte';
import { dataState } from './data.svelte';
import { sessionState } from './session.svelte';

/** 
 * @deprecated Use uiState, dataState or sessionState instead for specialized concerns.
 * This object maintains backward compatibility during the transition period.
 */
export const globalState = {
  get isUnlocked() { return sessionState.isUnlocked; },
  set isUnlocked(v) { sessionState.isUnlocked = v; },
  get accounts() { return dataState.accounts; },
  set accounts(v) { dataState.accounts = v; },
  get pins() { return dataState.pins; },
  set pins(v) { dataState.pins = v; },
  get sidebarCollapsed() { return uiState.sidebarCollapsed; },
  set sidebarCollapsed(v) { uiState.sidebarCollapsed = v; },
  get activeView() { return uiState.activeView; },
  set activeView(v) { uiState.activeView = v; },
  get activeGroup() { return uiState.activeGroup; },
  set activeGroup(v) { uiState.activeGroup = v; },
  get groups() { return dataState.groups; },
  set groups(v) { dataState.groups = v; }
};
