<script lang="ts">
  /**
   * @component SettingsNav
   * @description Side navigation for settings view.
   *
   * @param {Object} props
   * @param {Section[]} props.sections - List of available sections.
   * @param {string} props.activeSection - (bindable) The ID of the currently selected section.
   */
  import { _ } from "svelte-i18n";

  interface Section {
    id: string;
    label: string;
    icon: any;
  }

  let {
    sections,
    activeSection = $bindable(),
  }: {
    sections: Section[];
    activeSection: string;
  } = $props();
</script>

<aside class="flex">
  {#each sections as section}
    {@const Icon = section.icon}
    <button
      class="flex items-center gap-2 px-4 py-2 text-sm font-medium transition-all text-left border
        {activeSection === section.id
        ? 'bg-accent/10 border-accent/20 text-accent-light shadow-sm'
        : 'bg-transparent border-transparent text-text-muted hover:bg-surface/5 hover:text-text-secondary'}"
      onclick={() => (activeSection = section.id)}
    >
      <Icon
        size={18}
        class={activeSection === section.id ? "text-accent" : ""}
      />
      <span>{section.label}</span>
      {#if activeSection === section.id}
        <div class="w-2 h-2 rounded-full bg-accent animate-pulse ml-1"></div>
      {/if}
    </button>
  {/each}
</aside>
