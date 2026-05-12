import { invoke } from "@tauri-apps/api/core";

// ─── Types ────────────────────────────────────────────────────────────────────

export type BackgroundType = "gradient" | "solid" | "image";
export type ColorScheme = "dark" | "light" | "system";

/** A single appearance variant (colors for one mode) */
export interface ThemeVariant {
  accentColor: string;
  bgType: BackgroundType;
  bgValue: string; // hex if solid, filename if image, empty if gradient
  textPrimary: string;
  textSecondary: string;
}

/** A preset declares how it looks in BOTH dark and light modes */
export interface ThemePreset {
  id: string;
  nameKey: string; // i18n key
  defaultScheme: ColorScheme; // which mode to use when first applied
  dark: ThemeVariant;
  light: ThemeVariant;
}

/** Runtime state stored in the Svelte store */
export interface ThemeState {
  activePresetId: string | null;
  colorScheme: ColorScheme;
  accentColor: string;
  bgType: BackgroundType;
  bgValue: string;
  textPrimary: string;
  textSecondary: string;
}

// ─── Presets ──────────────────────────────────────────────────────────────────
// To add a new preset: just add an object here with `dark` and `light` variants.
// No logic changes needed anywhere else.

export const PRESETS: ThemePreset[] = [
  {
    id: "midnight-violet",
    nameKey: "settings.theme.presets.midnight_violet",
    defaultScheme: "dark",
    dark: {
      accentColor: "#7c3aed",
      bgType: "gradient",
      bgValue: "",
      textPrimary: "#ffffff",
      textSecondary: "#9999a3",
    },
    light: {
      accentColor: "#7c3aed",
      bgType: "solid",
      bgValue: "#f3f4f6",
      textPrimary: "#0f172a",
      textSecondary: "#475569",
    },
  },
  {
    id: "ocean-light",
    nameKey: "settings.theme.presets.ocean_light",
    defaultScheme: "light",
    dark: {
      accentColor: "#0ea5e9",
      bgType: "solid",
      bgValue: "#0c1a2e",
      textPrimary: "#e2f0ff",
      textSecondary: "#7db8e8",
    },
    light: {
      accentColor: "#0ea5e9",
      bgType: "solid",
      bgValue: "#f0f6fc",
      textPrimary: "#0f172a",
      textSecondary: "#475569",
    },
  },
  {
    id: "matrix-dark",
    nameKey: "settings.theme.presets.matrix_dark",
    defaultScheme: "dark",
    dark: {
      accentColor: "#22c55e",
      bgType: "solid",
      bgValue: "#151715",
      textPrimary: "#22c55e",
      textSecondary: "#9999a3",
    },
    light: {
      accentColor: "#16a34a",
      bgType: "solid",
      bgValue: "#f3f4f6",
      textPrimary: "#0f172a",
      textSecondary: "#475569",
    },
  },
  {
    id: "crimson-light",
    nameKey: "settings.theme.presets.crimson_light",
    defaultScheme: "light",
    dark: {
      accentColor: "#e11d48",
      bgType: "solid",
      bgValue: "#1a0008",
      textPrimary: "#fecdd3",
      textSecondary: "#9f1239",
    },
    light: {
      accentColor: "#e11d48",
      bgType: "solid",
      bgValue: "#fff1f2",
      textPrimary: "#4c0519",
      textSecondary: "#9f1239",
    },
  },
];

// ─── Helpers ──────────────────────────────────────────────────────────────────

/** Converts hex to rgba string */
function hexToRgba(hex: string, alpha: number): string {
  let r = 0,
    g = 0,
    b = 0;
  if (hex.length === 4) {
    r = parseInt(hex[1] + hex[1], 16);
    g = parseInt(hex[2] + hex[2], 16);
    b = parseInt(hex[3] + hex[3], 16);
  } else if (hex.length === 7) {
    r = parseInt(hex.substring(1, 3), 16);
    g = parseInt(hex.substring(3, 5), 16);
    b = parseInt(hex.substring(5, 7), 16);
  }
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

/** Lightens a hex color by a percentage (0–1) */
function lightenColor(hex: string, percent: number): string {
  let r = parseInt(hex.substring(1, 3), 16);
  let g = parseInt(hex.substring(3, 5), 16);
  let b = parseInt(hex.substring(5, 7), 16);
  r = Math.min(255, Math.floor(r + (255 - r) * percent));
  g = Math.min(255, Math.floor(g + (255 - g) * percent));
  b = Math.min(255, Math.floor(b + (255 - b) * percent));
  return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
}

/** Returns the variant (dark/light) for a given resolved color scheme */
function getPresetVariant(preset: ThemePreset, isDark: boolean): ThemeVariant {
  return isDark ? preset.dark : preset.light;
}

/** Resolves whether the effective mode is dark, considering "system" */
function resolveIsDark(scheme: ColorScheme): boolean {
  if (scheme === "system") {
    return window.matchMedia?.("(prefers-color-scheme: dark)").matches ?? true;
  }
  return scheme === "dark";
}

// ─── Store ────────────────────────────────────────────────────────────────────

const defaultState: ThemeState = {
  activePresetId: "midnight-violet",
  colorScheme: "dark",
  accentColor: "#7c3aed",
  bgType: "gradient",
  bgValue: "",
  textPrimary: "#ffffff",
  textSecondary: "#9999a3",
};

/** Keys used to persist state in SQLite */
const PREF_KEYS = {
  activePresetId: "theme_active_preset_id",
  colorScheme: "theme_color_scheme",
  accentColor: "theme_accent_color",
  bgType: "theme_bg_type",
  bgValue: "theme_bg_value",
  textPrimary: "theme_text_primary",
  textSecondary: "theme_text_secondary",
} as const;

/** Persist all mutable theme fields to SQLite */
async function persistState(s: ThemeState): Promise<void> {
  await Promise.all([
    invoke("set_preference", {
      key: PREF_KEYS.activePresetId,
      value: s.activePresetId, // Can be null (deletes preference)
    }),
    invoke("set_preference", {
      key: PREF_KEYS.colorScheme,
      value: s.colorScheme,
    }),
    invoke("set_preference", {
      key: PREF_KEYS.accentColor,
      value: s.accentColor,
    }),
    invoke("set_preference", { key: PREF_KEYS.bgType, value: s.bgType }),
    invoke("set_preference", { key: PREF_KEYS.bgValue, value: s.bgValue }),
    invoke("set_preference", {
      key: PREF_KEYS.textPrimary,
      value: s.textPrimary,
    }),
    invoke("set_preference", {
      key: PREF_KEYS.textSecondary,
      value: s.textSecondary,
    }),
  ]);
}

class ThemeStore {
  activePresetId = $state<string | null>(defaultState.activePresetId);
  colorScheme = $state<ColorScheme>(defaultState.colorScheme);
  accentColor = $state<string>(defaultState.accentColor);
  bgType = $state<BackgroundType>(defaultState.bgType);
  bgValue = $state<string>(defaultState.bgValue);
  textPrimary = $state<string>(defaultState.textPrimary);
  textSecondary = $state<string>(defaultState.textSecondary);

  // Helper to read current state as an object
  private getState(): ThemeState {
    return {
      activePresetId: this.activePresetId,
      colorScheme: this.colorScheme,
      accentColor: this.accentColor,
      bgType: this.bgType,
      bgValue: this.bgValue,
      textPrimary: this.textPrimary,
      textSecondary: this.textSecondary,
    };
  }

  // Helper to write state from an object
  private setState(s: Partial<ThemeState>) {
    if (s.activePresetId !== undefined) this.activePresetId = s.activePresetId;
    if (s.colorScheme !== undefined) this.colorScheme = s.colorScheme;
    if (s.accentColor !== undefined) this.accentColor = s.accentColor;
    if (s.bgType !== undefined) this.bgType = s.bgType;
    if (s.bgValue !== undefined) this.bgValue = s.bgValue;
    if (s.textPrimary !== undefined) this.textPrimary = s.textPrimary;
    if (s.textSecondary !== undefined) this.textSecondary = s.textSecondary;
  }

  async applyToDom(): Promise<void> {
    if (typeof document === "undefined") return;

    const root = document.documentElement;
    const isDark = resolveIsDark(this.colorScheme);

    root.classList.toggle("light-mode", !isDark);

    const accent = this.accentColor;
    root.style.setProperty("--color-accent", accent);
    root.style.setProperty("--color-accent-light", lightenColor(accent, 0.3));
    root.style.setProperty("--color-accent-glow", hexToRgba(accent, 0.35));
    root.style.setProperty("--color-accent-dim", hexToRgba(accent, 0.15));

    root.style.setProperty("--text-primary", this.textPrimary);
    root.style.setProperty("--text-secondary", this.textSecondary);

    const fallbackBg = isDark ? "#141418" : "#f3f4f6";
    if (this.bgType === "gradient") {
      root.style.setProperty("--bg-image-layer", "none");
      root.style.setProperty("--bg-solid-color", fallbackBg);
    } else if (this.bgType === "solid") {
      root.style.setProperty("--bg-image-layer", "none");
      root.style.setProperty("--bg-solid-color", this.bgValue || fallbackBg);
    } else if (this.bgType === "image" && this.bgValue) {
      try {
        const dataUrl = await invoke<string>("get_background_data_url", {
          filename: this.bgValue,
        });
        root.style.setProperty("--bg-image-layer", `url("${dataUrl}")`);
        root.style.setProperty("--bg-solid-color", fallbackBg);
      } catch (e) {
        console.warn("Could not load background image:", e);
        root.style.setProperty("--bg-image-layer", "none");
      }
    }
  }

  async init(): Promise<void> {
    try {
      const [
        activePresetId,
        colorScheme,
        accentColor,
        bgType,
        bgValue,
        textPrimary,
        textSecondary,
      ] = await Promise.all([
        invoke<string | null>("get_preference", {
          key: PREF_KEYS.activePresetId,
        }),
        invoke<string | null>("get_preference", { key: PREF_KEYS.colorScheme }),
        invoke<string | null>("get_preference", { key: PREF_KEYS.accentColor }),
        invoke<string | null>("get_preference", { key: PREF_KEYS.bgType }),
        invoke<string | null>("get_preference", { key: PREF_KEYS.bgValue }),
        invoke<string | null>("get_preference", { key: PREF_KEYS.textPrimary }),
        invoke<string | null>("get_preference", {
          key: PREF_KEYS.textSecondary,
        }),
      ]);

      this.setState({
        activePresetId: activePresetId ?? defaultState.activePresetId,
        colorScheme: (colorScheme as ColorScheme) ?? defaultState.colorScheme,
        accentColor: accentColor ?? defaultState.accentColor,
        bgType: (bgType as BackgroundType) ?? defaultState.bgType,
        bgValue: bgValue ?? defaultState.bgValue,
        textPrimary: textPrimary ?? defaultState.textPrimary,
        textSecondary: textSecondary ?? defaultState.textSecondary,
      });

      await this.applyToDom();

      if (window.matchMedia) {
        window
          .matchMedia("(prefers-color-scheme: dark)")
          .addEventListener("change", () => {
            if (this.colorScheme === "system") this.applyToDom();
          });
      }
    } catch (e) {
      console.warn("Could not load theme preferences:", e);
      this.setState(defaultState);
      await this.applyToDom();
    }
  }

  async setColorScheme(scheme: ColorScheme): Promise<void> {
    const isDark = resolveIsDark(scheme);
    const preset = PRESETS.find((p) => p.id === this.activePresetId);

    if (preset) {
      const variant = getPresetVariant(preset, isDark);
      this.setState({
        colorScheme: scheme,
        activePresetId: preset.id,
        ...variant,
      });
    } else {
      this.setState({ colorScheme: scheme });
    }

    this.applyToDom();

    try {
      await persistState(this.getState());
    } catch (e) {
      console.error("Failed to save color scheme preference:", e);
    }
  }

  async applyPreset(preset: ThemePreset): Promise<void> {
    const isDark = resolveIsDark(preset.defaultScheme);
    const variant = getPresetVariant(preset, isDark);
    this.setState({
      activePresetId: preset.id,
      colorScheme: preset.defaultScheme,
      ...variant,
    });
    this.applyToDom();

    try {
      await persistState(this.getState());
    } catch (e) {
      console.error("Failed to apply preset preferences:", e);
    }
  }

  async setAccentColor(color: string): Promise<void> {
    this.setState({ accentColor: color, activePresetId: null });
    this.applyToDom();
    try {
      await Promise.all([
        invoke("set_preference", {
          key: PREF_KEYS.activePresetId,
          value: null, // Explicitly delete activePresetId when customizing
        }),
        invoke("set_preference", { key: PREF_KEYS.accentColor, value: color }),
      ]);
    } catch (e) {
      console.error("Failed to save accent preference:", e);
    }
  }

  async setTextColors(primary: string, secondary: string): Promise<void> {
    this.setState({
      textPrimary: primary,
      textSecondary: secondary,
      activePresetId: null,
    });
    this.applyToDom();
    try {
      await Promise.all([
        invoke("set_preference", {
          key: PREF_KEYS.activePresetId,
          value: null, // Explicitly delete activePresetId when customizing
        }),
        invoke("set_preference", {
          key: PREF_KEYS.textPrimary,
          value: primary,
        }),
        invoke("set_preference", {
          key: PREF_KEYS.textSecondary,
          value: secondary,
        }),
      ]);
    } catch (e) {
      console.error("Failed to save text color preferences:", e);
    }
  }

  async setBackground(bgType: BackgroundType, bgValue: string): Promise<void> {
    this.setState({ bgType, bgValue, activePresetId: null });
    this.applyToDom();
    try {
      await Promise.all([
        invoke("set_preference", {
          key: PREF_KEYS.activePresetId,
          value: null, // Explicitly delete activePresetId when customizing
        }),
        invoke("set_preference", { key: PREF_KEYS.bgType, value: bgType }),
        invoke("set_preference", { key: PREF_KEYS.bgValue, value: bgValue }),
      ]);
    } catch (e) {
      console.error("Failed to save background preference:", e);
    }
  }

  async resetToDefaults(): Promise<void> {
    this.setState(defaultState);
    await this.applyToDom();
    try {
      await persistState(defaultState);
    } catch (e) {
      console.error("Failed to reset theme preferences:", e);
    }
  }
}

export const themeStore = new ThemeStore();
