<script lang="ts">
// Copyright 2026 Aaron Lomas - Bóveda

  import Sidebar from "$lib/components/Sidebar.svelte";
  import Dashboard from "$lib/components/Dashboard.svelte";
  import UnlockScreen from "$lib/components/UnlockScreen.svelte";
  import { isUnlocked, sidebarCollapsed } from "$lib/stores";
  import { startAutoLock, stopAutoLock } from "$lib/autoLock";

  // Watch for unlock state changes to start/stop the auto-lock timer
  $: {
    if ($isUnlocked) {
      startAutoLock({ onLock: () => isUnlocked.set(false) });
    } else {
      stopAutoLock();
    }
  }
</script>

{#if $isUnlocked}
  <div class="app-layout">
    <Sidebar />
    <main class="main-content" class:collapsed={$sidebarCollapsed}>
      <Dashboard />
    </main>
  </div>
{:else}
  <UnlockScreen />
{/if}

<style>
  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 32px 36px;
    min-width: 0;
    transition: padding-left var(--transition);
  }
</style>
