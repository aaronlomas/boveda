<!-- Fix for Tailwind 4 parser bug -->
<script lang="ts">
  import Sidebar from "$lib/components/ui/Sidebar.svelte";
  import MainView from "$lib/components/views/MainView.svelte";
  import UnlockScreen from "$lib/components/views/UnlockScreen.svelte";
  import { sessionState, uiState } from "$lib/stores/stores.svelte";
  import { themeStore } from "$lib/stores/theme.svelte";
  import { startAutoLock, stopAutoLock } from "$lib/utils/autoLock";
  import { invoke } from "@tauri-apps/api/core";

  async function loadTimeoutSeconds(): Promise<number> {
    const raw = await invoke<string | null>("get_preference", { key: "session_timeout_seconds" });
    const parsed = raw ? parseInt(raw, 10) : 0;
    return isNaN(parsed) ? 0 : parsed;
  }

  function doLock() {
    sessionState.isUnlocked = false;
    stopAutoLock();
  }

  let remoteGuardInterval: ReturnType<typeof setInterval> | null = null;

  function startRemoteGuard() {
    remoteGuardInterval = setInterval(async () => {
      try {
        const locked = await invoke<boolean>("is_vault_locked");
        if (locked && sessionState.isUnlocked) {
          doLock();
        }
      } catch {}
    }, 5000);
  }

  function stopRemoteGuard() {
    if (remoteGuardInterval !== null) {
      clearInterval(remoteGuardInterval);
      remoteGuardInterval = null;
    }
  }

  $effect(() => {
    if (sessionState.isUnlocked) {
      loadTimeoutSeconds().then((seconds) => {
        startAutoLock({ onLock: doLock, seconds });
      });
      startRemoteGuard();
    } else {
      stopAutoLock();
      stopRemoteGuard();
    }
  });
</script>


{#if sessionState.isUnlocked}
  <div class="app-layout">
    <Sidebar />
    <main class="main-content flex-1 min-w-0 overflow-hidden" class:collapsed={uiState.sidebarCollapsed}>
      <MainView />
    </main>
  </div>
{:else}
  <UnlockScreen />
{/if}
