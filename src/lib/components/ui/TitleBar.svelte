<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";

  const appWindow = getCurrentWindow();
  let isMaximized = $state(false);
  let isFocused = $state(true);

  async function updateMaximizedState() {
    isMaximized = await appWindow.isMaximized();
  }

  onMount(() => {
    updateMaximizedState();
    const unlistenPromise = appWindow.onResized(() => updateMaximizedState());
    const unlistenFocus = appWindow.onFocusChanged(({ payload: focused }) => {
      isFocused = focused;
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
      unlistenFocus.then((unlisten) => unlisten());
    };
  });

  // We use data-tauri-drag-region for better native feel on Windows/Linux
  // but keep the function for cases where we might need manual control.
  function startDrag() {
    appWindow.startDragging();
  }

  async function handleMinimize() {
    await appWindow.minimize();
  }

  async function handleMaximize() {
    if (await appWindow.isMaximized()) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
  }

  async function handleClose() {
    await appWindow.close();
  }
</script>

<div
  class="h-9 flex justify-between items-center bg-panel/30 backdrop-blur-xl border-b border-surface/8 select-none w-full shrink-0 z-50 cursor-default"
  role="none"
>
  <div
    class="flex items-center pl-3.5 h-full gap-2.5 flex-1"
    data-tauri-drag-region
    ondblclick={handleMaximize}
    role="none"
  >
    <div
      class="w-4 h-4 bg-linear-to-br from-accent rounded-full pointer-events-none"
    ></div>
  </div>

  <div class="flex h-full" onmousedown={(e) => e.stopPropagation()} role="none">
    <button
      class="w-12 h-full flex items-center justify-center text-text-muted hover:bg-surface/10 hover:text-text-primary transition-colors"
      onclick={handleMinimize}
      aria-label={$_("actions.minimize")}
      title={$_("actions.minimize")}
    >
      <svg width="10" height="1" viewBox="0 0 10 1"
        ><rect width="10" height="1" fill="currentColor" /></svg
      >
    </button>

    <button
      class="w-12 h-full flex items-center justify-center text-text-muted hover:bg-surface/10 hover:text-text-primary transition-colors"
      onclick={handleMaximize}
      aria-label={isMaximized
        ? $_("actions.restore")
        : $_("actions.maximize")}
      title={isMaximized ? $_("actions.restore") : $_("actions.maximize")}
    >
      {#if isMaximized}
        <svg width="10" height="10" viewBox="0 0 10 10"
          ><path
            d="M2.5,2.5 L2.5,0.5 L9.5,0.5 L9.5,7.5 L7.5,7.5 M2.5,2.5 L7.5,2.5 L7.5,9.5 L0.5,9.5 L0.5,2.5 Z"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
          /></svg
        >
      {:else}
        <svg width="10" height="10" viewBox="0 0 10 10"
          ><rect
            x="0.5"
            y="0.5"
            width="9"
            height="9"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
          /></svg
        >
      {/if}
    </button>

    <button
      class="w-12 h-full flex items-center justify-center text-text-muted hover:bg-danger hover:text-white transition-colors"
      onclick={handleClose}
      aria-label={$_("actions.close")}
      title={$_("actions.close")}
    >
      <svg width="10" height="10" viewBox="0 0 10 10"
        ><path
          d="M0.5,0.5 L9.5,9.5 M9.5,0.5 L0.5,9.5"
          fill="none"
          stroke="currentColor"
          stroke-width="1"
        /></svg
      >
    </button>
  </div>
</div>
