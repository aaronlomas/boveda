import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

export type BackgroundType = "gradient" | "solid" | "image";
export type ColorScheme = "dark" | "light" | "system";

export interface ThemeState {
  activePresetId: string | null; // ID of the preset or null if custom
  colorScheme: ColorScheme;
  accentColor: string; // Hex color
  bgType: BackgroundType;
  bgValue: string; // hex color if solid, filename if image, empty if gradient
  textPrimary: string; // hex color
  textSecondary: string; // hex color
}

export interface ThemePreset {
  id: string;
  nameKey: string; // Translation key
  colorScheme: ColorScheme;
  accentColor: string;
  bgType: BackgroundType;
  bgValue: string;
  textPrimary: string;
  textSecondary: string;
}

// ─── Presets ──────────────────────────────────────────────────────────────────

export const PRESETS: ThemePreset[] = [
  {
    id: "midnight-violet",
    nameKey: "settings.theme.presets.midnight_violet",
    colorScheme: "dark",
    accentColor: "#7c3aed",
    bgType: "gradient",
    bgValue: "",
    textPrimary: "#ffffff",
    textSecondary: "#9999a3",
  },
  {
    id: "ocean-light",
    nameKey: "settings.theme.presets.ocean_light",
    colorScheme: "light",
    accentColor: "#0ea5e9",
    bgType: "solid",
    bgValue: "#f0f6fc",
    textPrimary: "#0f172a",
    textSecondary: "#475569",
  },
  {
    id: "matrix-dark",
    nameKey: "settings.theme.presets.matrix_dark",
    colorScheme: "dark",
    accentColor: "#10b981",
    bgType: "solid",
    bgValue: "#000000",
    textPrimary: "#22c55e",
    textSecondary: "#166534",
  },
  {
    id: "crimson-light",
    nameKey: "settings.theme.presets.crimson_light",
    colorScheme: "light",
    accentColor: "#e11d48",
    bgType: "solid",
    bgValue: "#fff1f2",
    textPrimary: "#4c0519",
    textSecondary: "#9f1239",
  }
];

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
  activePresetId: "midnight-violet",
  colorScheme: "dark",
  accentColor: "#7c3aed", // Violet default
  bgType: "gradient",
  bgValue: "",
  textPrimary: "#ffffff",
  textSecondary: "#9999a3",
};

function createThemeStore() {
  const { subscribe, set, update } = writable<ThemeState>(defaultState);

  /** Apply the current theme to the DOM immediately */
  async function applyToDom(state: ThemeState) {
    if (typeof document === "undefined") return;

    const root = document.documentElement;

    // 1. Color Scheme Mode
    const isSystemDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    const isDark = state.colorScheme === "dark" || (state.colorScheme === "system" && isSystemDark);
    
    if (isDark) {
      root.classList.remove("light-mode");
    } else {
      root.classList.add("light-mode");
    }

    // 2. Dynamic Accent Colors
    const accent = state.accentColor;
    root.style.setProperty("--color-accent", accent);
    root.style.setProperty("--color-accent-light", lightenColor(accent, 0.3));
    root.style.setProperty("--color-accent-glow", hexToRgba(accent, 0.35));
    root.style.setProperty("--color-accent-dim", hexToRgba(accent, 0.15));

    // 3. Text Colors
    root.style.setProperty("--text-primary", state.textPrimary);
    root.style.setProperty("--text-secondary", state.textSecondary);

    // 4. Background
    const defaultSolid = isDark ? "#07070e" : "#f3f4f6";
    
    if (state.bgType === "gradient") {
      root.style.setProperty("--bg-image-layer", "none");
      root.style.setProperty("--bg-solid-color", defaultSolid);
    } else if (state.bgType === "solid") {
      root.style.setProperty("--bg-image-layer", "none");
      root.style.setProperty("--bg-solid-color", state.bgValue || defaultSolid);
    } else if (state.bgType === "image" && state.bgValue) {
      try {
        const dataUrl = await invoke<string>("get_background_data_url", { filename: state.bgValue });
        root.style.setProperty("--bg-image-layer", `url("${dataUrl}")`);
        root.style.setProperty("--bg-solid-color", defaultSolid);
      } catch (e) {
        console.warn("Could not load background image:", e);
        root.style.setProperty("--bg-image-layer", "none");
      }
    }
  }

  /** Load preferences from SQLite and apply to DOM */
  async function init() {
    try {
      const [activePresetId, colorScheme, accent, bgType, bgValue, textPrimary, textSecondary] = await Promise.all([
        invoke<string | null>("get_preference", { key: "theme_active_preset_id" }),
        invoke<string | null>("get_preference", { key: "theme_color_scheme" }),
        invoke<string | null>("get_preference", { key: "theme_accent_color" }),
        invoke<string | null>("get_preference", { key: "theme_bg_type" }),
        invoke<string | null>("get_preference", { key: "theme_bg_value" }),
        invoke<string | null>("get_preference", { key: "theme_text_primary" }),
        invoke<string | null>("get_preference", { key: "theme_text_secondary" }),
      ]);

      const state: ThemeState = {
        activePresetId: activePresetId ?? defaultState.activePresetId,
        colorScheme: (colorScheme as ColorScheme) ?? defaultState.colorScheme,
        accentColor: accent ?? defaultState.accentColor,
        bgType: (bgType as BackgroundType) ?? defaultState.bgType,
        bgValue: bgValue ?? defaultState.bgValue,
        textPrimary: textPrimary ?? defaultState.textPrimary,
        textSecondary: textSecondary ?? defaultState.textSecondary,
      };
      set(state);
      await applyToDom(state);
      
      // Listen to system changes if in system mode
      if (window.matchMedia) {
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
          update(s => {
            if (s.colorScheme === "system") applyToDom(s);
            return s;
          });
        });
      }
      
    } catch (e) {
      console.warn("Could not load theme preferences:", e);
      await applyToDom(defaultState);
    }
  }

  async function setColorScheme(scheme: ColorScheme) {
    let nextState: ThemeState | null = null;

    update((s) => {
      let nextTextPrimary = s.textPrimary;
      let nextTextSecondary = s.textSecondary;

      if (s.colorScheme === "dark" && scheme === "light") {
        if (s.textPrimary === "#ffffff") nextTextPrimary = "#0f172a";
        if (s.textSecondary === "#9999a3") nextTextSecondary = "#475569";
      } 
      else if (s.colorScheme === "light" && scheme === "dark") {
        if (s.textPrimary === "#0f172a") nextTextPrimary = "#ffffff";
        if (s.textSecondary === "#475569") nextTextSecondary = "#9999a3";
      }

      nextState = { ...s, colorScheme: scheme, textPrimary: nextTextPrimary, textSecondary: nextTextSecondary, activePresetId: null };
      applyToDom(nextState);
      return nextState;
    });

    if (nextState) {
      try {
        const n = nextState as ThemeState;
        await Promise.all([
          invoke("set_preference", { key: "theme_active_preset_id", value: null }),
          invoke("set_preference", { key: "theme_color_scheme", value: scheme }),
          invoke("set_preference", { key: "theme_text_primary", value: n.textPrimary }),
          invoke("set_preference", { key: "theme_text_secondary", value: n.textSecondary }),
        ]);
      } catch (e) {
        console.error("Failed to save color scheme preference:", e);
      }
    }
  }

  async function setAccentColor(color: string) {
    update((s) => {
      const next = { ...s, accentColor: color, activePresetId: null };
      applyToDom(next);
      return next;
    });
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_active_preset_id", value: null }),
        invoke("set_preference", { key: "theme_accent_color", value: color }),
      ]);
    } catch (e) {
      console.error("Failed to save accent preference:", e);
    }
  }

  async function setTextColors(primary: string, secondary: string) {
    update((s) => {
      const next = { ...s, textPrimary: primary, textSecondary: secondary, activePresetId: null };
      applyToDom(next);
      return next;
    });
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_active_preset_id", value: null }),
        invoke("set_preference", { key: "theme_text_primary", value: primary }),
        invoke("set_preference", { key: "theme_text_secondary", value: secondary }),
      ]);
    } catch (e) {
      console.error("Failed to save text color preferences:", e);
    }
  }

  async function setBackground(bgType: BackgroundType, bgValue: string) {
    update((s) => {
      const next = { ...s, bgType, bgValue, activePresetId: null };
      applyToDom(next);
      return next;
    });
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_active_preset_id", value: null }),
        invoke("set_preference", { key: "theme_bg_type", value: bgType }),
        invoke("set_preference", { key: "theme_bg_value", value: bgValue }),
      ]);
    } catch (e) {
      console.error("Failed to save background preference:", e);
    }
  }
  
  async function applyPreset(preset: ThemePreset) {
    update((s) => {
      const next: ThemeState = {
        activePresetId: preset.id,
        colorScheme: preset.colorScheme,
        accentColor: preset.accentColor,
        bgType: preset.bgType,
        bgValue: preset.bgValue,
        textPrimary: preset.textPrimary,
        textSecondary: preset.textSecondary
      };
      applyToDom(next);
      return next;
    });
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_active_preset_id", value: preset.id }),
        invoke("set_preference", { key: "theme_color_scheme", value: preset.colorScheme }),
        invoke("set_preference", { key: "theme_accent_color", value: preset.accentColor }),
        invoke("set_preference", { key: "theme_bg_type", value: preset.bgType }),
        invoke("set_preference", { key: "theme_bg_value", value: preset.bgValue }),
        invoke("set_preference", { key: "theme_text_primary", value: preset.textPrimary }),
        invoke("set_preference", { key: "theme_text_secondary", value: preset.textSecondary }),
      ]);
    } catch (e) {
      console.error("Failed to apply preset preferences:", e);
    }
  }

  async function resetToDefaults() {
    set(defaultState);
    await applyToDom(defaultState);
    try {
      await Promise.all([
        invoke("set_preference", { key: "theme_active_preset_id", value: defaultState.activePresetId }),
        invoke("set_preference", { key: "theme_color_scheme", value: defaultState.colorScheme }),
        invoke("set_preference", { key: "theme_accent_color", value: defaultState.accentColor }),
        invoke("set_preference", { key: "theme_bg_type", value: defaultState.bgType }),
        invoke("set_preference", { key: "theme_bg_value", value: defaultState.bgValue }),
        invoke("set_preference", { key: "theme_text_primary", value: defaultState.textPrimary }),
        invoke("set_preference", { key: "theme_text_secondary", value: defaultState.textSecondary }),
      ]);
    } catch (e) {
      console.error("Failed to reset theme preferences:", e);
    }
  }

  return { subscribe, init, setColorScheme, setAccentColor, setBackground, setTextColors, applyPreset, resetToDefaults };
}

export const themeStore = createThemeStore();
