<script lang="ts">
  import { _ } from "svelte-i18n";

  interface Section {
    id: string;
    label: string;
    icon: any;
  }

  let { 
    sections, 
    activeSection = $bindable() 
  }: { 
    sections: Section[]; 
    activeSection: string;
  } = $props();
</script>

<aside class="w-64 shrink-0 flex flex-col border-r border-surface/8">
  {#each sections as section}
    {@const Icon = section.icon}
    <button
      class="flex items-center gap-3 px-6 py-4 text-sm font-medium transition-all text-left border-b border-surface/5 cursor-pointer
        {activeSection === section.id
        ? 'bg-accent/10 text-accent-light shadow-sm'
        : 'bg-transparent text-text-muted hover:bg-surface/5 hover:text-text-secondary'}"
      onclick={() => (activeSection = section.id)}
    >
      <Icon
        size={18}
        class={activeSection === section.id ? "text-accent" : ""}
      />
      <span class="flex-1">{section.label}</span>
      {#if activeSection === section.id}
        <div class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse"></div>
      {/if}
    </button>
  {/each}
</aside>
