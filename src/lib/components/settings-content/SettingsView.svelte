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
    IconDatabaseExport,
    IconInfoCircle
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { changeLanguage, t } from "$lib/i18n";
  
  import SettingsNav from "./SettingsNav.svelte";
  
  // Panels
  import ThemePanel from "./panels/ThemePanel.svelte";
  import LanguagePanel from "./panels/LanguagePanel.svelte";
  import PerformancePanel from "./panels/PerformancePanel.svelte";
  import SecurityPanel from "./panels/SecurityPanel.svelte";
  import BackupPanel from "./panels/BackupPanel.svelte";
  import AboutPanel from "./panels/AboutPanel.svelte";

  let activeSection = $state("theme");

  async function handleRestore() {
    await themeStore.resetToDefaults();
    await changeLanguage("es");
    performanceStore.setMassiveList(false);
  }

  const sections = $derived([
    { id: "theme", label: $_("settings.tabs.theme"), icon: IconPalette },
    { id: "language", label: $_("settings.tabs.language"), icon: IconLanguage },
    { id: "security", label: $_("settings.tabs.security"), icon: IconShieldCheck },
    { id: "performance", label: $_("settings.tabs.performance"), icon: IconRocket },
    { id: "backup", label: $_("settings.tabs.backup") || "Backup", icon: IconDatabaseExport },
    { id: "about", label: $_("settings.tabs.about") || "About", icon: IconInfoCircle },
  ]);

  const panelMap: any = {
    theme: ThemePanel,
    language: LanguagePanel,
    security: SecurityPanel,
    performance: PerformancePanel,
    backup: BackupPanel,
    about: AboutPanel
  };

  const ActivePanel = $derived(panelMap[activeSection]);
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10 h-full flex flex-col"
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
          {t("settings.subtitle", "Personaliza tu experiencia en Bóveda")}
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

  <!-- MAIN LAYOUT -->
  <div class="flex-1 flex overflow-hidden bg-surface/4 backdrop-blur-2xl rounded-3xl border border-surface/8 shadow-2xl">
    <SettingsNav {sections} bind:activeSection />

    <!-- CONTENT AREA -->
    <main class="flex-1 p-8 overflow-y-auto custom-scrollbar">
      <div class="max-w-2xl">
        {#key activeSection}
          <div class="animate-in fade-in slide-in-from-right-4 duration-300">
            {#if ActivePanel}
              <ActivePanel />
            {/if}
          </div>
        {/key}
      </div>
    </main>
  </div>
</div>

<style>
  main {
    scrollbar-gutter: stable;
  }
</style>
