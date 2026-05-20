<script lang="ts">
  import { _ } from "svelte-i18n";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import {
    IconRocket,
    IconToggleLeft,
    IconToggleRight,
  } from "@tabler/icons-svelte";
</script>

<!-- ─ Performance Section ─ -->
<p class="text-xs text-text-muted mb-4">
  {$_("settings.performance.description")}
</p>

<!-- Massive List Toggle Card -->
<div
  class="grid grid-cols-[auto_1fr_auto] gap-x-4 p-5 rounded-2xl border transition-all cursor-pointer
    {performanceStore.massiveList
    ? 'border-accent bg-accent-dim'
    : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
  onclick={() => performanceStore.toggle()}
  role="button"
  tabindex="0"
  onkeydown={(e) => e.key === 'Enter' && performanceStore.toggle()}
>
  <!-- Icon (Spans Badge and Toggle rows) -->
  <div
    class="row-span-2 w-10 h-10 rounded-xl flex items-center justify-center self-center
      {performanceStore.massiveList
      ? 'bg-accent/20 text-accent'
      : 'bg-surface/8 text-text-muted'}"
  >
    <IconRocket size={22} />
  </div>

  <!-- Header Info (Spans Badge and Toggle rows) -->
  <div class="row-span-2 flex flex-col justify-center">
    <p class="text-sm font-bold text-text-primary">
      {$_("settings.performance.massive_list_title")}
    </p>
    <p class="text-xs text-text-muted mt-0.5">
      {performanceStore.massiveList
        ? $_("settings.performance.massive_list_enabled")
        : $_("settings.performance.massive_list_disabled")}
    </p>
  </div>

  <!-- Badge (Top Right) -->
  <span
    class="justify-self-end text-[10px] font-bold uppercase tracking-widest px-2 py-0.5 rounded-full
      {performanceStore.massiveList
      ? 'bg-accent text-white'
      : 'bg-surface/10 text-text-muted'}"
  >
    {$_("settings.performance.massive_list_badge")}
  </span>

  <!-- Toggle icon (Below Badge) -->
  <div class="justify-self-end mt-1 {performanceStore.massiveList ? 'text-accent' : 'text-text-muted'}">
    {#if performanceStore.massiveList}
      <IconToggleRight size={32} />
    {:else}
      <IconToggleLeft size={32} />
    {/if}
  </div>

  <!-- Description (Full width) -->
  <p class="col-span-3 mt-4 text-xs text-text-secondary leading-relaxed">
    {$_("settings.performance.massive_list_desc")}
  </p>

  <!-- Threshold note (Full width) -->
  <div
    class="col-span-3 mt-4 flex items-center gap-2 text-xs rounded-lg px-3 py-2
      {performanceStore.massiveList
      ? 'bg-accent/10 text-accent-light border border-accent/20'
      : 'bg-surface/5 text-text-muted border border-surface/8'}"
  >
    <IconRocket size={12} />
    {$_("settings.performance.massive_list_threshold")}
  </div>
</div>


