<!-- Fix for Tailwind 4 parser bug -->
<script lang="ts">
  import Sidebar from "$lib/components/ui/Sidebar.svelte";
  import Dashboard from "$lib/components/views/Dashboard.svelte";
  import UnlockScreen from "$lib/components/views/UnlockScreen.svelte";
  import { globalState } from "$lib/stores/stores.svelte";
  import { startAutoLock, stopAutoLock } from "$lib/utils/autoLock";

  // Watch for unlock state changes to start/stop the auto-lock timer
  $effect(() => {
    if (globalState.isUnlocked) {
      startAutoLock({ onLock: () => (globalState.isUnlocked = false) });
    } else {
      stopAutoLock();
    }
  });
</script>

{#if globalState.isUnlocked}
  <div class="app-layout">
    <Sidebar />
    <main class="main-content" class:collapsed={globalState.sidebarCollapsed}>
      <Dashboard />
    </main>
  </div>
{:else}
  <UnlockScreen />
{/if}
