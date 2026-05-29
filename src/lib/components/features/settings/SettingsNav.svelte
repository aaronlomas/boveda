<script lang="ts">
  /**
   * @component SettingsNav
   * @description Navegación lateral para la vista de configuración.
   *
   * @param {Object} props
   * @param {Section[]} props.sections - Lista de secciones disponibles.
   * @param {string} props.activeSection - (bindable) El ID de la sección actualmente seleccionada.
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

<aside class="shrink-0 flex">
  {#each sections as section}
    {@const Icon = section.icon}
    <button
      class="flex items-center gap-3 px-4 py-2 text-sm font-medium transition-all text-left border
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
        <div
          class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse ml-1"
        ></div>
      {/if}
    </button>
  {/each}
</aside>
