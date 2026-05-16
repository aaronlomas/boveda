import { init, register, getLocaleFromNavigator, locale } from "svelte-i18n";
import { invoke } from "@tauri-apps/api/core";

// Register locales
register("en", () => import("./locales/en.json"));
register("es", () => import("./locales/es.json"));

let isInitialized = false;

export async function initI18n() {
  if (isInitialized) return;

  try {
    // Attempt to load the user's preferred language from Tauri preferences
    const savedLanguage = await invoke<string | null>("get_preference", {
      key: "language",
    });

    // Determine the initial locale
    const initialLocale = savedLanguage || getLocaleFromNavigator() || "es";

    // Fallback to "en" or "es" based on initialLocale
    const lang = initialLocale.startsWith("en") ? "en" : "es";

    init({
      fallbackLocale: "es",
      initialLocale: lang,
    });

    isInitialized = true;
  } catch (error) {
    console.error("Failed to initialize i18n:", error);
    // Ultimate fallback
    init({
      fallbackLocale: "es",
      initialLocale: "es",
    });
  }
}

// Function to change language and save preference
export async function changeLanguage(lang: "en" | "es") {
  locale.set(lang);
  try {
    await invoke("set_preference", { key: "language", value: lang });
  } catch (error) {
    console.error("Failed to save language preference:", error);
  }
}

/**
 * Robust translation helper with fallback
 * Usage: t("key", "Default Text")
 */
import { _ } from "svelte-i18n";
import { get } from "svelte/store";

export function t(key: string, defaultText?: string, vars?: any): string {
  const translate = get(_);
  const result = translate(key, vars);
  
  // svelte-i18n returns the key if it's missing
  if (result === key && defaultText) {
    return defaultText;
  }
  
  return result;
}
