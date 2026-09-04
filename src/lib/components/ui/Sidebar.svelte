<script lang="ts">
  import { uiState, sessionState } from "$lib/stores/stores.svelte";
  import { lockVault } from "$lib/utils/tauri";
  import { modal } from "$lib/stores/modal.svelte";
  import { _ } from "svelte-i18n";
  import Logo from "$lib/components/ui/Logo.svelte";
  import {
    IconLayoutGrid,
    IconFiles,
    IconDatabaseImport,
    IconArchive,
    IconInfoCircle,
    IconSettings,
    IconLogout,
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
      action: () => (uiState.activeView = "import"),
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

  async function logout() {
    try {
      await lockVault();
      sessionState.isUnlocked = false;
    } catch (e) {
      console.error("Logout error:", e);
    }
  }

  let isResizing = $state(false);
  let isCollapsed = $derived(uiState.sidebarWidth <= 72);

  function startResize(e: PointerEvent) {
    e.preventDefault();
    isResizing = true;
    document.body.classList.add('is-resizing');
    const target = e.target as HTMLElement;
    target.setPointerCapture(e.pointerId);
  }

  function doResize(e: PointerEvent) {
    if (!isResizing) return;
    let newWidth = e.clientX;
    
    // Snap to icon-only view if dragged too far left
    if (newWidth < 140) {
      newWidth = 72;
    } else if (newWidth < 200) {
      newWidth = 200;
    } else if (newWidth > 600) {
      newWidth = 600;
    }
    
    uiState.sidebarWidth = newWidth;
  }

  function stopResize(e: PointerEvent) {
    if (isResizing) {
      isResizing = false;
      document.body.classList.remove('is-resizing');
      const target = e.target as HTMLElement;
      target.releasePointerCapture(e.pointerId);
    }
  }
</script>

<aside
  class="h-screen bg-panel/30 border-r border-surface/8 transition-colors overflow-hidden backdrop-blur-2xl flex flex-col px-2 gap-4 relative select-none"
  style="
    width: {uiState.sidebarWidth}px; 
    min-width: {uiState.sidebarWidth}px; 
    will-change: width, min-width; 
    transform: translateZ(0);    
  "
>

  <!-- Brand -->
  <div class="flex items-center gap-2 py-4 border-b border-surface/8 overflow-hidden whitespace-nowrap">
    <div class="text-2xl shrink-0 w-10 h-10 flex items-center justify-center bg-transparent rounded-sm">
      <Logo size={32} class="text-text-primary" />
    </div>
    <div class="flex flex-col align-center">
      {#if !isCollapsed}
        <span class="text-base font-bold text-text-primary block pointer-events-none">Bóveda</span>
        <span class="text-xs text-text-muted uppercase tracking-wider pointer-events-none">{$_("sidebar.my_credentials")}</span>
      {/if}
    </div>
  </div>

  <!-- Nav items -->
  <nav class="flex flex-col gap-1">
    {#each navItems as item}
      {@const Icon = item.icon}
      <button
        class="nav-item-btn {uiState.activeView === item.id ? 'active' : ''}"
        onclick={item.action ?? undefined}
      >
        <div class="shrink-0 w-5 flex justify-center">
          <Icon size={20} />
        </div>
        {#if !isCollapsed}
          <span class="flex-1 text-left">{item.label}</span>
        {/if}
      </button>
    {/each}
    <!-- Logout button -->
    <button
      class="nav-item-btn hover:bg-danger/10 hover:text-text-primary mt-auto mb-4"
      onclick={logout}
    >
      <div class="shrink-0 w-5 flex justify-center">
        <IconLogout size={20} />
      </div>
      {#if !isCollapsed}
        <span class="flex-1 text-left">{$_("sidebar.logout")}</span>
      {/if}
    </button>
  </nav>

  <!-- Resize Handle -->
  <div
    class="absolute top-0 right-0 w-2 h-full cursor-col-resize hover:bg-primary/20 transition-colors z-10"
    onpointerdown={startResize}
    onpointermove={doResize}
    onpointerup={stopResize}
    onpointercancel={stopResize}
    aria-label="Resize sidebar"
    role="separator"
  ></div>
</aside>

<style>
  :global(body.is-resizing *) {
    cursor: col-resize !important;
    user-select: none !important;
  }
</style>
