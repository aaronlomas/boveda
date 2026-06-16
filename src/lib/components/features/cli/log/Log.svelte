<script lang="ts">
  import { logStore } from "$lib/stores/log.svelte";
  import { onMount, tick } from "svelte";
  import Terminal from "../terminal/Terminal.svelte";
  import { typewriter } from "$lib/utils/actions";

  let logContainer: HTMLDivElement | undefined = $state();
  let isExpanded = $state(true);
  let viewMode = $state<"log" | "terminal">("log");
  let terminalRef: ReturnType<typeof Terminal> | undefined = $state();

  $effect(() => {
    if (viewMode === "log" && logStore.entries.length > 0 && logContainer && isExpanded) {
      tick().then(() => {
        if (logContainer) {
          logContainer.scrollTop = logContainer.scrollHeight;
        }
      });
    }
  });

  function clearCurrentView() {
    if (viewMode === "log") {
      logStore.clear();
    } else if (viewMode === "terminal" && terminalRef) {
      terminalRef.clear();
    }
  }

  // RESIZEABLE LOG
  let isResizing = $state(false);
  let startY = 0;
  let startHeight = 0;
  let currentHeight = $state(200);

  function startResizing(e: MouseEvent) {
    isResizing = true;
    startY = e.clientY;
    startHeight = currentHeight;
    document.body.style.cursor = 'ns-resize';
    e.preventDefault();
  }

  function stopResizing() {
    isResizing = false;
    document.body.style.cursor = '';
  }

  function resize(e: MouseEvent) {
    if (isResizing) {
      const delta = startY - e.clientY;
      currentHeight = Math.max(100, Math.min(startHeight + delta, window.innerHeight * 0.8));
    }
  }
  
</script>

<svelte:window onmousemove={resize} onmouseup={stopResizing} />

<div 
  class="terminal-log-container relative flex flex-col w-full bg-panel/30 border-t border-surface/8"
  style={isExpanded ? `height: ${currentHeight}px;` : ''}
>
  <!-- Drag Handle -->
  {#if isExpanded}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
      class="absolute top-0 left-0 w-full h-1 cursor-ns-resize hover:bg-accent/40 transition-colors z-20"
      onmousedown={startResizing}
    ></div>
  {/if}

  <div class="terminal-header flex items-center justify-between px-4 py-2 border-b border-surface/8">
    <div class="flex items-center gap-4">
      <button 
        class="text-xs font-medium transition-colors focus:outline-none {viewMode === 'log' ? 'text-accent' : 'text-text-muted hover:text-text-primary'}"
        onclick={() => viewMode = "log"}
      >
        Log
      </button>
      <button 
        class="text-xs font-medium transition-colors focus:outline-none {viewMode === 'terminal' ? 'text-accent' : 'text-text-muted hover:text-text-primary'}"
        onclick={() => viewMode = "terminal"}
      >
        Terminal
      </button>
    </div>

    <div class="flex items-center gap-2">
      <button 
        class="text-text-muted hover:text-danger transition-colors focus:outline-none"
        onclick={clearCurrentView}
        title="Clear"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" y1="11" x2="10" y2="17"/><line x1="14" y1="11" x2="14" y2="17"/></svg>
      </button>
      <button 
        class="text-text-muted hover:text-text-primary transition-colors focus:outline-none"
        onclick={() => isExpanded = !isExpanded}
        title={isExpanded ? "Collapse" : "Expand"}
      >
        {#if isExpanded}
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
        {:else}
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>
        {/if}
      </button>
    </div>
  </div>
  
  {#if isExpanded}
    {#if viewMode === "log"}
      <div 
        bind:this={logContainer}
        class="terminal-content flex-1 px-4 py-2 overflow-y-auto font-mono text-sm leading-relaxed custom-scrollbar"
      >
        {#each logStore.entries as entry (entry.id)}
          <div class="log-line flex p-1 hover:bg-white/5 transition-colors rounded-sm text-text-primary/80">
            <span class="shrink-0 mr-2 whitespace-pre text-text-muted" use:typewriter={{speed: 15}}>[{entry.timestamp}]</span>
            <span class="shrink-0 w-24 whitespace-pre font-medium text-text-primary/90" use:typewriter={{speed: 15}}>[{entry.category}]</span>
            <span class="break-all ml-1 text-text-primary/90" use:typewriter={{speed: 15}}>{entry.text}</span>
          </div>
        {/each}
        {#if logStore.entries.length === 0}
          <div class="text-text-muted italic">Waiting for system events...</div>
        {/if}
      </div>
    {:else}
      <Terminal bind:this={terminalRef} />
    {/if}
  {/if}
</div>

<style>
  .terminal-log-container {
    font-family: var(--font-mono, monospace);
  }
  
  .log-line:hover {
    color: var(--color-text-primary);
  }
</style>