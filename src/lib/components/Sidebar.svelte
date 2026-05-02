<script lang="ts">
  import { sidebarCollapsed, showAddModal, isUnlocked, activeView } from "$lib/stores";
  import { lockVault } from "$lib/tauri";
  import ConfirmModal from "./ConfirmModal.svelte";
  import PreferencesModal from "./PreferencesModal.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
  import { _ } from "svelte-i18n";
  import {IconShieldHalfFilled} from '@tabler/icons-svelte';

  let showConfirmImport = false;
  let showPreferences = false;

  $: navItems = [
    { icon: "⊞", label: $_("sidebar.general"), id: 'general', action: () => activeView.set('general') },
    { icon: "▤", label: $_("sidebar.documents"), id: 'documents', action: () => activeView.set('documents') },
    { icon: "⤓", label: $_("dashboard.export_db"), id: 'export', action: () => exportDb() },
    { icon: "⎘", label: $_("sidebar.import_db"), id: 'import', action: () => { showConfirmImport = true; } },
    { icon: "🛡", label: $_("sidebar.about"), id: 'about', action: () => activeView.set('about') },
    { icon: "⚙", label: $_("sidebar.settings"), id: 'settings', action: () => { showPreferences = true; } },
  ];

  async function handleImport() {
    showConfirmImport = false;
    try {
      const filePath = await open({
        title: $_("global.select_db_title"),
        filters: [{ name: $_("global.db_filter_name"), extensions: ["db"] }]
      });
      if (filePath) {
        await invoke("import_db", { srcPath: filePath });
      }
    } catch (e) {
      console.error("Import failed:", e);
      alert($_("global.error_import"));
    }
  }

  async function exportDb() {
    try {
      const filePath = await save({
        title: $_("dashboard.export_db_title"),
        defaultPath: "Boveda_Backup.db",
        filters: [{ name: $_("global.db_filter_name"), extensions: ["db"] }]
      });
      
      if (filePath) {
        await invoke("export_db", { destPath: filePath });
        alert($_("dashboard.export_success", { values: { path: filePath } }));
      }
    } catch (e) {
      console.error("Export failed:", e);
      alert($_("dashboard.export_error"));
    }
  }

  function toggle() {
    sidebarCollapsed.update((v) => !v);
  }

  async function logout() {
    try {
      await lockVault();
      isUnlocked.set(false);
    } catch (e) {
      console.error("Logout error:", e);
    }
  }
</script>

<aside 
  class="h-screen bg-surface/4 border-r border-surface/8 flex flex-col p-6 px-4 gap-2 transition-[width,min-width] duration-300 overflow-hidden backdrop-blur-2xl relative z-10"
  style="width: {$sidebarCollapsed ? 'var(--sidebar-collapsed)' : 'var(--sidebar-w)'}; min-width: {$sidebarCollapsed ? 'var(--sidebar-collapsed)' : 'var(--sidebar-w)'};"
>
  <!-- Brand -->
  <div class="flex items-center gap-3 px-1 pb-5 border-b border-surface/8 mb-3 overflow-hidden whitespace-nowrap">
    <div class="text-2xl shrink-0 w-10 h-10 flex items-center justify-center bg-transparent rounded-sm border-none">
      <IconShieldHalfFilled size={40} />
    </div>
    {#if !$sidebarCollapsed}
      <div class="flex flex-col">
        <span class="text-base font-bold text-text-primary block pointer-events-none">Bóveda</span>
        <span class="text-[10px] text-text-muted uppercase tracking-[0.08em] pointer-events-none">{$_("sidebar.my_credentials")}</span>
      </div>
    {/if}
  </div>

  <!-- Nav items -->
  <nav class="flex-1 flex flex-col gap-1">
    {#each navItems as item}
      <button
        class="nav-item-btn {$activeView === item.id ? 'active' : ''}"
        on:click={item.action ?? undefined}
        data-tooltip={$sidebarCollapsed ? item.label : undefined}
      >
        <span class="text-base shrink-0 w-5 text-center">{item.icon}</span>
        {#if !$sidebarCollapsed}
          <span class="flex-1">{item.label}</span>
        {/if}
      </button>
    {/each}
  </nav>

  <!-- Logout button -->
  <button
    class="mt-auto flex items-center gap-3 p-2.5 px-3 border-none rounded-sm bg-transparent text-text-muted cursor-pointer font-medium text-sm transition-all whitespace-nowrap overflow-hidden w-full text-left hover:bg-danger/10 hover:text-text-primary"
    on:click={logout}
    data-tooltip={$sidebarCollapsed ? $_("sidebar.logout") : undefined}
  >
    <span class="text-base shrink-0 w-5 text-center">⎋</span>
    {#if !$sidebarCollapsed}
      <span class="flex-1">{$_("sidebar.logout")}</span>
    {/if}
  </button>

  <!-- Collapse toggle -->
  <button 
    class="flex items-center gap-2.5 p-2.5 px-3 border-none border-t border-surface/8 rounded-sm bg-transparent text-text-muted cursor-pointer font-medium text-xs transition-all whitespace-nowrap overflow-hidden w-full mt-2 pt-4 hover:text-text-secondary" 
    on:click={toggle} 
    aria-label="Toggle sidebar"
  >
    <span class="text-lg inline-block transition-transform duration-300 shrink-0 w-5 text-center" class:rotate-180={$sidebarCollapsed}>‹</span>
    {#if !$sidebarCollapsed}
      <span class="text-xs">{$_("sidebar.collapse")}</span>
    {/if}
  </button>
</aside>

{#if showConfirmImport}
  <ConfirmModal
    title={$_("sidebar.import_confirm_title")}
    message={$_("sidebar.import_confirm_message")}
    confirmText={$_("sidebar.import_confirm_button")}
    type="danger"
    on:confirm={handleImport}
    on:cancel={() => (showConfirmImport = false)}
  />
{/if}

{#if showPreferences}
  <PreferencesModal on:close={() => (showPreferences = false)} />
{/if}

<style>
  .nav-item-btn {
    display: flex;
    align-items: center;
    gap: 0.875rem;
    padding: 0.75rem 1rem;
    border: 1px solid transparent;
    border-radius: 0.5rem;
    background-color: transparent;
    color: var(--color-text-muted);
    cursor: pointer;
    font-weight: 500;
    font-size: 0.8125rem;
    transition: all 0.2s;
    white-space: nowrap;
    overflow: hidden;
    width: 100%;
    text-align: left;
  }

  .nav-item-btn:hover {
    background-color: var(--color-surface-dim, rgba(255, 255, 255, 0.07));
    color: var(--color-accent);
  }

  .nav-item-btn.active {
    background-color: var(--color-accent-dim);
    color: var(--color-accent);
    border-color: color-mix(in srgb, var(--color-accent) 20%, transparent);
  }
</style>
