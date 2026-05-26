<script lang="ts">
  import { uiState, sessionState } from "$lib/stores/stores.svelte";
  import { lockVault } from "$lib/utils/tauri";
  import { modal } from "$lib/stores/modal.svelte";
  import { _ } from "svelte-i18n";
  import {
    IconShieldHalfFilled,
    IconLayoutGrid,
    IconFiles,
    IconDatabaseImport,
    IconArchive,
    IconInfoCircle,
    IconSettings,
    IconLogout,
    IconChevronLeft,
  } from "@tabler/icons-svelte";

  let navItems = $derived([
    {
      icon: IconLayoutGrid,
      label: $_("sidebar.general"),
      id: "general",
      action: () => (uiState.activeView = "general"),
    },
    {
      icon: IconFiles,
      label: $_("sidebar.documents"),
      id: "documents",
      action: () => (uiState.activeView = "documents"),
    },
    {
      icon: IconArchive,
      label: $_("sidebar.export_db"),
      id: "export",
      action: () => modal.openExportPackage(),
    },
    {
      icon: IconDatabaseImport,
      label: $_("sidebar.import_db"),
      id: "import",
      action: () => modal.openImportPackage(),
    },
    {
      icon: IconInfoCircle,
      label: $_("sidebar.about"),
      id: "about",
      action: () => (uiState.activeView = "about"),
    },
    {
      icon: IconSettings,
      label: $_("sidebar.settings"),
      id: "settings",
      action: () => (uiState.activeView = "settings"),
    },
  ]);

  import { UI_CONFIG } from "$lib/config/ui";
  
  function toggle() {
    uiState.sidebarCollapsed = !uiState.sidebarCollapsed;
  }

  async function logout() {
    try {
      await lockVault();
      sessionState.isUnlocked = false;
    } catch (e) {
      console.error("Logout error:", e);
    }
  }
</script>

<aside
  class="h-screen bg-panel/30 border-r border-surface/8 transition-all overflow-hidden backdrop-blur-2xl flex flex-col py-4 px-2"
  style="width: {uiState.sidebarCollapsed
    ? UI_CONFIG.SIDEBAR_COLLAPSED_WIDTH
    : UI_CONFIG.SIDEBAR_FULL_WIDTH}; min-width: {uiState.sidebarCollapsed
    ? UI_CONFIG.SIDEBAR_COLLAPSED_WIDTH
    : UI_CONFIG.SIDEBAR_FULL_WIDTH}; transition-duration: {UI_CONFIG.ANIMATION_DURATION_MS}ms;"
>
  <!-- Brand -->
  <div
    class="flex items-center gap-2 px-1 pb-5 border-b border-surface/8 mb-3 overflow-hidden whitespace-nowrap"
  >
    <div
      class="text-2xl shrink-0 w-10 h-10 flex items-center justify-center bg-transparent rounded-sm"
    >
      <img src="../src/assets/logo-minimal.svg" alt="logo">
    </div>
    {#if !uiState.sidebarCollapsed}
      <div class="flex flex-col">
        <span
          class="text-base font-bold text-text-primary block pointer-events-none"
          >Bóveda</span
        >
        <span
          class="text-xs text-text-muted uppercase tracking-wider pointer-events-none"
          >{$_("sidebar.my_credentials")}</span
        >
      </div>
    {/if}
  </div>

  <!-- Nav items -->
  <nav class="flex-1 flex flex-col gap-1">
    {#each navItems as item}
      {@const Icon = item.icon}
      <button
        class="nav-item-btn {uiState.activeView === item.id
          ? 'active'
          : ''}"
        onclick={item.action ?? undefined}
        data-tooltip={uiState.sidebarCollapsed ? item.label : undefined}
      >
        <div class="shrink-0 w-5 flex justify-center">
          <Icon size={20} />
        </div>
        {#if !uiState.sidebarCollapsed}
          <span class="flex-1">{item.label}</span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- Logout button -->
  <button
    class="mt-auto flex justify-center items-center gap-3 py-2.5 px-3 border-none rounded-sm bg-transparent text-text-muted cursor-pointer font-medium text-xs transition-full whitespace-nowrap overflow-hidden w-full text-left hover:bg-danger/10 hover:text-text-primary"
    onclick={logout}
    data-tooltip={uiState.sidebarCollapsed
      ? $_("sidebar.logout")
      : undefined}
  >
    <div class="shrink-0 w-5 flex justify-center">
      <IconLogout size={20} />
    </div>
    {#if !uiState.sidebarCollapsed}
      <span class="flex-1">{$_("sidebar.logout")}</span>
    {/if}
  </button>

  <!-- Collapse toggle -->
  <button
    class="flex items-center gap-2.5 mb-6 p-4 border-none border-t border-surface/8 rounded-sm bg-transparent text-text-muted cursor-pointer font-medium text-xs transition-all whitespace-nowrap overflow-hidden w-full mt-2 pt-4 hover:text-text-secondary"
    onclick={toggle}
    aria-label="Toggle sidebar"
  >
    <div
      class="transition-transform duration-300 shrink-0 w-5 flex justify-center"
      class:rotate-180={uiState.sidebarCollapsed}
    >
      <IconChevronLeft size={20} />
    </div>
    {#if !uiState.sidebarCollapsed}
      <span class="text-xs">{$_("sidebar.collapse")}</span>
    {/if}
  </button>
</aside>
