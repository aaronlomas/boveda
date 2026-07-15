// vitest-setup.ts
import { vi } from 'vitest';
import '@testing-library/svelte/vitest';

// Mock Tauri API so components can render without errors
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

// Mock matchMedia if not present in jsdom
if (typeof window !== 'undefined') {
  Object.defineProperty(window, 'matchMedia', {
    writable: true,
    value: vi.fn().mockImplementation(query => ({
      matches: false,
      media: query,
      onchange: null,
      addListener: vi.fn(), // deprecated
      removeListener: vi.fn(), // deprecated
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      dispatchEvent: vi.fn(),
    })),
  });
}
