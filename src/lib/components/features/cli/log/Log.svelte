<script lang="ts">
  import { logStore } from "$lib/stores/log.svelte";
  import { onMount, tick } from "svelte";

  let logContainer: HTMLDivElement | undefined = $state();
  let isExpanded = $state(true);

  $effect(() => {
    if (logStore.entries.length > 0 && logContainer && isExpanded) {
      tick().then(() => {
        if (logContainer) {
          logContainer.scrollTop = logContainer.scrollHeight;
        }
      });
    }
  });
</script>

<div class="terminal-log-container flex flex-col w-full bg-panel/30 border-t border-surface/20">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div 
    class="terminal-header flex items-center justify-between px-4 py-2 border-b border-white/5 cursor-pointer hover:bg-white/5 transition-colors"
    onclick={() => isExpanded = !isExpanded}
  >
    <span class="text-xs font-medium text-text-primary/90">Log</span>
    <button class="text-text-muted hover:text-text-primary transition-colors focus:outline-none">
      {#if isExpanded}
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>
      {:else}
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>
      {/if}
    </button>
  </div>
  
  {#if isExpanded}
    <div 
      bind:this={logContainer}
      class="terminal-content px-4 py-3 overflow-y-auto font-mono text-sm leading-relaxed custom-scrollbar max-h-45 min-h-25"
    >
    {#each logStore.entries as entry (entry.id)}
      <div class="log-line flex py-1 hover:bg-white/5 transition-colors rounded-sm text-text-primary/80">
        <span class="shrink-0 mr-2 whitespace-pre">
          <span class="text-text-muted">[</span>{entry.timestamp}<span class="text-text-muted">]</span>
        </span>
        <span class="shrink-0 w-20 whitespace-pre">
          <span class="text-text-muted">[</span><span class="font-medium text-text-primary/90">{entry.category}</span><span class="text-text-muted">]</span>
        </span>
        <span class="break-all ml-1 text-text-primary/90">{entry.text}</span>
      </div>
    {/each}
    {#if logStore.entries.length === 0}
      <div class="text-text-muted italic">Waiting for system events...</div>
    {/if}
  </div>
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