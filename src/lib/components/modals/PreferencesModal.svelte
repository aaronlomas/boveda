<script lang="ts">
  import { themeStore } from "$lib/stores/theme.svelte";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import {
    IconX,
    IconPalette,
    IconLanguage,
    IconRocket,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { changeLanguage } from "$lib/i18n";
  import ThemePanel from "./settings/ThemePanel.svelte";
  import LanguagePanel from "./settings/LanguagePanel.svelte";
  import PerformancePanel from "./settings/PerformancePanel.svelte";

  let { onclose }: { onclose?: () => void } = $props();

  let activeSection: "theme" | "language" | "performance" = $state("theme");

  function close(e: MouseEvent) {
    if (e.target === e.currentTarget) onclose?.();
  }

  async function handleRestore() {
    // 1. Reset theme
    await themeStore.resetToDefaults();

    // 2. Reset language to Spanish (default)
    await changeLanguage("es");

    // 3. Reset Lista Masiva
    performanceStore.setMassiveList(false);
  }
</script>

<!-- Backdrop -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="fixed inset-0 bg-panel/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  onclick={(e) => close(e as MouseEvent)}
>
  <div
    class="w-full max-w-xl max-h-[90vh] bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl shadow-2xl flex flex-col overflow-hidden"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between px-6 py-4 border-b border-surface/8"
    >
      <h2 class="text-sm font-semibold text-text-primary">
        {$_("settings.title")}
      </h2>
      <button
        class="p-1.5 rounded-lg text-text-muted hover:text-text-primary hover:bg-surface/8 transition-all"
        onclick={() => onclose?.()}
        aria-label={$_("settings.close")}
      >
        <IconX size={18} />
      </button>
    </div>

    <!-- Tabs -->
    <div class="flex border-b border-surface/8">
      <button
        class="flex items-center gap-2 px-6 py-3 text-xs font-medium transition-all border-b-2 {activeSection ===
        'theme'
          ? 'border-accent text-accent-light'
          : 'border-transparent text-text-muted hover:text-text-secondary'}"
        onclick={() => (activeSection = "theme")}
      >
        <IconPalette size={15} />
        {$_("settings.tabs.theme")}
      </button>
      <button
        class="flex items-center gap-2 px-6 py-3 text-xs font-medium transition-all border-b-2 {activeSection ===
        'language'
          ? 'border-accent text-accent-light'
          : 'border-transparent text-text-muted hover:text-text-secondary'}"
        onclick={() => (activeSection = "language")}
      >
        <IconLanguage size={15} />
        {$_("settings.tabs.language")}
      </button>
      <button
        class="flex items-center gap-2 px-6 py-3 text-xs font-medium transition-all border-b-2 {activeSection ===
        'performance'
          ? 'border-accent text-accent-light'
          : 'border-transparent text-text-muted hover:text-text-secondary'}"
        onclick={() => (activeSection = "performance")}
      >
        <IconRocket size={15} />
        {$_("settings.tabs.performance")}
      </button>
    </div>

    <!-- Body -->
    <div
      class="p-6 flex flex-col gap-5 flex-1 overflow-y-auto custom-scrollbar bg-panel/20"
    >
      {#if activeSection === "theme"}
        <ThemePanel />
      {:else if activeSection === "language"}
        <LanguagePanel />
      {:else if activeSection === "performance"}
        <PerformancePanel />
      {/if}
    </div>

    <!-- Footer -->
    <div
      class="px-6 py-4 border-t border-surface/8 flex justify-end gap-3 bg-panel/40"
    >
      <button
        class="px-5 py-2 text-sm font-medium bg-surface/5 border border-surface/10 rounded-lg text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-all"
        onclick={handleRestore}
      >
        {$_("settings.restore")}
      </button>
      <button
        class="px-5 py-2 text-sm font-medium bg-accent text-white rounded-lg hover:bg-accent-hover transition-all shadow-lg shadow-accent/20"
        onclick={() => onclose?.()}
      >
        {$_("settings.close")}
      </button>
    </div>
  </div>
</div>
