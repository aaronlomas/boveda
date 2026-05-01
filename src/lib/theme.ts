import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

export type BackgroundType = "gradient" | "solid" | "image";

export interface ThemeState {
  accentColor: string; // Hex color
  bgType: BackgroundType;
  bgValue: string; // hex color if solid, filename if image, empty if gradient
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/** Converts hex to rgba for CSS variables */
function hexToRgba(hex: string, alpha: number): string {
  let r = 0, g = 0, b = 0;
  // 3 digits
  if (hex.length === 4) {
    r = parseInt(hex[1] + hex[1], 16);
    g = parseInt(hex[2] + hex[2], 16);
    b = parseInt(hex[3] + hex[3], 16);
  }
  // 6 digits
  else if (hex.length === 7) {
    r = parseInt(hex.substring(1, 3), 16);
    g = parseInt(hex.substring(3, 5), 16);
    b = parseInt(hex.substring(5, 7), 16);
  }
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

/** Lightens a color for the "accent-light" variant */
function lightenColor(hex: string, percent: number): string {
  let r = parseInt(hex.substring(1, 3), 16);
  let g = parseInt(hex.substring(3, 5), 16);
  let b = parseInt(hex.substring(5, 7), 16);

  r = Math.min(255, Math.floor(r + (255 - r) * percent));
  g = Math.min(255, Math.floor(g + (255 - g) * percent));
  b = Math.min(255, Math.floor(b + (255 - b) * percent));

  const hr = r.toString(16).padStart(2, "0");
  const hg = g.toString(16).padStart(2, "0");
  const hb = b.toString(16).padStart(2, "0");

  return `#${hr}${hg}${hb}`;
}

// ─── Store ────────────────────────────────────────────────────────────────────

const defaultState: ThemeState = {
  accentColor: "#7c3aed", // Violet default
  bgType: "gradient",
  bgValue: "",
};

function createThemeStore() {
  const { subscribe, set, update } = writable<ThemeState>(defaultState);

  /** Apply the current theme to the DOM immediately */
  async function applyToDom(state: ThemeState) {
    if (typeof document === "undefined") return;

    const root = document.documentElement;

    // 1. Dynamic Accent Colors
    const accent = state.accentColor;
    root.style.setProperty("--color-accent", accent);
    root.style.setProperty("--color-accent-light", lightenColor(accent, 0.3));
    root.style.setProperty("--color-accent-glow", hexToRgba(accent, 0.35));
    root.style.setProperty("--color-accent-dim", hexToRgba(accent, 0.15));

    // 2. Background
    if (state.bgType === "gradient") {
      root.style.setProperty("--bg-image-layer", "none");
      root.style.setProperty("--bg-solid-color", "#07070e");
    } else if (state.bgType === "solid") {
      root.style.setProperty("--bg-image-layer", "none");
      root.style.setProperty("--bg-solid-color", state.bgValue || "#07070e");
    } else if (state.bgType === "image" && state.bgValue) {
      try {
        const dataUrl = await invoke<string>("get_background_data_url", { filename: state.bgValue });
        root.style.setProperty("--bg-image-layer", `url("${dataUrl}")`);
        root.style.setProperty("--bg-solid-color", "#07070e");
      } catch (e) {
        console.warn("Could not load background image:", e);
        root.style.setProperty("--bg-image-layer", "none");
      }
    }
  }

  /** Load preferences from SQLite and apply to DOM */
  async function init() {
    try {
      const [accent, bgType, bgValue] = await Promise.all([
        invoke<string | null>("get_preference", { key: "theme_accent_color" }),
        invoke<string | null>("get_preference", { key: "theme_bg_type" }),
        invoke<string | null>("get_preference", { key: "theme_bg_value" }),
      ]);

      const state: ThemeState = {
        accentColor: accent ?? "#7c3aed",
        bgType: (bgType as BackgroundType) ?? "gradient",
        bgValue: bgValue ?? "",
      };
      set(state);
      await applyToDom(state);
    } catch (e) {
      console.warn("Could not load theme preferences:", e);
      await applyToDom(defaultState);
    }
  }

  async function setAccentColor(color: string) {
    update((s) => {
      const next = { ...s, accentColor: color };
      applyToDom(next);
      return next;
    });
    try {
      await invoke("set_preference", { key: "theme_accent_color", value: color });
    } catch (e) {
      console.error("Failed to save accent preference:", e);
    }
  }

  async function setBackground(bgType: BackgroundType, bgValue: string) {
    update((s) => {
      const next = { ...s, bgType, bgValue };
      applyToDom(next);
      return next;
    });
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_bg_type", value: bgType }),
        invoke("set_preference", { key: "theme_bg_value", value: bgValue }),
      ]);
    } catch (e) {
      console.error("Failed to save background preference:", e);
    }
  }

  async function resetToDefaults() {
    set(defaultState);
    await applyToDom(defaultState);
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_accent_color", value: defaultState.accentColor }),
        invoke("set_preference", { key: "theme_bg_type", value: defaultState.bgType }),
        invoke("set_preference", { key: "theme_bg_value", value: defaultState.bgValue }),
      ]);
    } catch (e) {
      console.error("Failed to reset theme preferences:", e);
    }
  }

  return { subscribe, init, setAccentColor, setBackground, resetToDefaults };
}

export const themeStore = createThemeStore();
