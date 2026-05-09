<script lang="ts">
  import { themeStore } from "$lib/stores/theme.svelte";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import { globalState } from "$lib/stores/stores.svelte";
  import {
    IconPalette,
    IconLanguage,
    IconRocket,
    IconShieldCheck,
    IconArrowLeft,
    IconRefresh,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { changeLanguage } from "$lib/i18n";
  import ThemePanel from "../settings-content/panels/ThemePanel.svelte";
  import LanguagePanel from "../settings-content/panels/LanguagePanel.svelte";
  import PerformancePanel from "../settings-content/panels/PerformancePanel.svelte";
  import SecurityPanel from "../settings-content/panels/SecurityPanel.svelte";

  let activeSection: "theme" | "language" | "performance" | "security" =
    $state("theme");

  async function handleRestore() {
    // 1. Reset theme
    await themeStore.resetToDefaults();

    // 2. Reset language to Spanish (default)
    await changeLanguage("es");

    // 3. Reset Performance
    performanceStore.setMassiveList(false);
  }

  const sections = $derived([
    { id: "theme", label: $_("settings.tabs.theme"), icon: IconPalette },
    { id: "language", label: $_("settings.tabs.language"), icon: IconLanguage },
    {
      id: "security",
      label: $_("settings.tabs.security"),
      icon: IconShieldCheck,
    },
    {
      id: "performance",
      label: $_("settings.tabs.performance"),
      icon: IconRocket,
    },
  ]);
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10 h-full grid grid-rows-[auto_auto_1fr]"
>
  <!-- HEADER -->

  <header class="flex items-center justify-between mb-8 gap-4 shrink-0">
    <div class="flex gap-x-4 items-center">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-full text-text-muted hover:text-text-primary transition-colors cursor-pointer"
        onclick={() => (globalState.activeView = "general")}
        aria-label="Back"
      >
        <IconArrowLeft size={20} />
      </button>
      <div>
        <h1
          class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent pointer-events-none"
        >
          {$_("settings.title")}
        </h1>
        <p class="text-text-muted text-sm mt-0.5 pointer-events-none">
          {$_("settings.subtitle") || "Personaliza tu experiencia en Bóveda"}
        </p>
      </div>
    </div>

    <button
      class="inline-flex items-center justify-center gap-2 py-2 px-4 rounded-lg text-sm font-medium cursor-pointer transition-all border border-surface/10 bg-surface/5 text-text-secondary hover:bg-surface/10 hover:text-text-primary"
      onclick={handleRestore}
    >
      <IconRefresh size={16} />
      {$_("settings.restore")}
    </button>
  </header>

  <!-- OPTIONS -->

  <div class="flex overflow-hidden border border-surface/8">
    <!-- Sidebar Navigation -->
    <aside class="w-64 shrink-0 flex">
      {#each sections as section}
        {@const Icon = section.icon}
        <button
          class="flex items-center gap-3 px-4 py-2 text-sm font-medium transition-all text-left border cursor-pointer
            {activeSection === section.id
            ? 'bg-accent/10 border-accent/20 text-accent-light shadow-sm'
            : 'bg-transparent border-transparent text-text-muted hover:bg-surface/5 hover:text-text-secondary'}"
          onclick={() => (activeSection = section.id as any)}
        >
          <Icon
            size={18}
            class={activeSection === section.id ? "text-accent" : ""}
          />
          <span class="flex-1">{section.label}</span>
          {#if activeSection === section.id}
            <div class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse"></div>
          {/if}
        </button>
      {/each}
    </aside>
  </div>

  <!-- CONTENT AREA -->

  <main
    class="bg-surface/4 w-full h-full p-4 overflow-y-auto backdrop-blur-sm shadow-xl shadow-black/5"
  >
    <div class="max-w-2xl mx-auto h-full">
      {#if activeSection === "theme"}
        <div class="animate-in fade-in slide-in-from-right-4 duration-300">
          <ThemePanel />
        </div>
      {:else if activeSection === "language"}
        <div class="animate-in fade-in slide-in-from-right-4 duration-300">
          <LanguagePanel />
        </div>
      {:else if activeSection === "security"}
        <div class="animate-in fade-in slide-in-from-right-4 duration-300">
          <SecurityPanel />
        </div>
      {:else if activeSection === "performance"}
        <div class="animate-in fade-in slide-in-from-right-4 duration-300">
          <PerformancePanel />
        </div>
      {/if}
    </div>
  </main>
</div>

<style>
  /* Ensure the content area fills the available space correctly */
  main {
    scrollbar-gutter: stable;
  }
</style>
