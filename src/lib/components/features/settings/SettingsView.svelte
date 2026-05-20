<script lang="ts">
  /**
   * @component SettingsView
   * @description Vista principal de configuración de la aplicación. 
   * Organiza las opciones en paneles (Tema, Idioma, Seguridad, Rendimiento, etc.)
   * y maneja la navegación lateral.
   */
  import { themeStore } from "$lib/stores/theme.svelte";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import { uiState } from "$lib/stores/stores.svelte";
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
  import SecurityPanel from "./panels/security/SecurityPanel.svelte";
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
    { id: "about", label: t("settings.tabs.about", "Acerca de"), icon: IconInfoCircle },
  ]);

  const panelMap: any = {
    theme: ThemePanel,
    language: LanguagePanel,
    security: SecurityPanel,
    performance: PerformancePanel,
    about: AboutPanel
  };

  const ActivePanel = $derived(panelMap[activeSection]);
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10 h-full grid grid-rows-[auto_auto_1fr] gap-4"
>
  <!-- HEADER -->
  <header class="flex items-center justify-between gap-4 shrink-0">
    <div class="flex gap-x-4 items-center">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-full text-text-muted hover:text-text-primary transition-colors cursor-pointer"
        onclick={() => (uiState.activeView = "general")}
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
        <p class="text-text-muted text-sm mt-1 pointer-events-none">
          {t("settings.subtitle", "Personaliza tu experiencia en Bóveda")}
        </p>
      </div>
    </div>

    <button
      class="inline-flex items-center justify-center gap-2 py-2 px-4 rounded-lg text-sm font-medium cursor-pointer transition-all border border-surface/10 bg-surface/5 text-text-secondary hover:bg-surface/10 hover:text-text-primary"
      onclick={handleRestore}
    >
      <IconRefresh size={16} />
      {$_("actions.restore")}
    </button>
  </header>

  <!-- OPTIONS -->
  <div class="flex overflow-hidden border border-surface/8">
    <!-- Sidebar Navigation -->
    <SettingsNav {sections} bind:activeSection />
  </div>

  <!-- CONTENT AREA -->
  <main
    class="bg-surface/4 w-full h-full p-4 overflow-y-auto backdrop-blur-sm shadow-xl shadow-black/5"
  >
    <div class="max-w-2xl mx-auto h-full">
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

<style>
  main {
    scrollbar-gutter: stable;
  }
</style>
