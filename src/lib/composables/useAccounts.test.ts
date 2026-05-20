import { describe, it, expect, vi, beforeEach } from 'vitest';
import { useAccounts } from './useAccounts.svelte';
import { dataState } from '$lib/stores/stores.svelte';

// Mock dependencies
vi.mock('$lib/stores/stores.svelte', () => ({
  dataState: { accounts: [] }
}));

vi.mock('$lib/utils/tauri', () => ({
  getAccounts: vi.fn().mockResolvedValue([
    { id: '1', site: 'example.com', username: 'user1', password: 'pwd' }
  ]),
  deleteAccount: vi.fn().mockResolvedValue(true)
}));

vi.mock('$lib/stores/modal.svelte', () => ({
  modal: {
    openConfirm: vi.fn().mockResolvedValue(true)
  }
}));

vi.mock('$lib/stores/toast.svelte', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
  }
}));

describe('useAccounts composable', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    dataState.accounts = [];
  });

  it('should initialize with empty accounts', () => {
    const { accounts, loading } = useAccounts();
    expect(accounts).toEqual([]);
    expect(loading).toBe(false);
  });

  it('should load accounts when refresh is called', async () => {
    const state = useAccounts();
    
    // Call refresh but don't await immediately to check loading state
    const refreshPromise = state.refresh();
    
    // loading should be true immediately after calling refresh
    expect(state.loading).toBe(true);
    
    await refreshPromise;
    
    // After resolution, dataState should be populated and loading false
    expect(state.loading).toBe(false);
    expect(state.accounts).toHaveLength(1);
    expect(state.accounts[0].site).toBe('example.com');
  });
});
