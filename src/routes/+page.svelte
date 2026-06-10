<!-- Fix for Tailwind 4 parser bug -->
<script lang="ts">
  import Sidebar from "$lib/components/ui/Sidebar.svelte";
  import MainView from "$lib/components/views/MainView.svelte";
  import UnlockScreen from "$lib/components/views/UnlockScreen.svelte";
  import Log from "$lib/components/features/cli/log/Log.svelte";
  import { sessionState, uiState } from "$lib/stores/stores.svelte";
  import { themeStore } from "$lib/stores/theme.svelte";
  import { startAutoLock, stopAutoLock } from "$lib/utils/autoLock";

  // Watch for unlock state changes to start/stop the auto-lock timer
  $effect(() => {
    if (sessionState.isUnlocked) {
      startAutoLock({ onLock: () => (sessionState.isUnlocked = false) });
    } else {
      stopAutoLock();
    }
  });
</script>

{#if sessionState.isUnlocked}
  <div class="app-layout">
    <Sidebar />
    <div class="flex flex-col flex-1 min-w-0 overflow-hidden">
      <main class="main-content flex-1" class:collapsed={uiState.sidebarCollapsed}>
        <MainView />
      </main>
      <Log />
    </div>
  </div>
{:else}
  <UnlockScreen />
{/if}
