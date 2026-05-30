<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    description: string;
    icon?: any;
    iconSnippet?: Snippet;
    onclick?: (e: MouseEvent) => void;
    disabled?: boolean;
    class?: string;
  }

  let {
    title,
    description,
    icon: Icon,
    iconSnippet,
    onclick,
    disabled = false,
    class: className = ""
  }: Props = $props();
</script>

<button
  class="flex flex-col items-start text-left p-6 gap-4 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl hover:border-accent/30 hover:bg-surface/7 transition-all group disabled:opacity-50 disabled:cursor-not-allowed {className}"
  {onclick}
  {disabled}
>
  {#if Icon || iconSnippet}
    <div
      class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform shrink-0"
    >
      {#if iconSnippet}
        {@render iconSnippet()}
      {:else}
        <Icon size={24} />
      {/if}
    </div>
  {/if}
  <div>
    <h3 class="text-lg font-semibold text-text-primary mb-1">
      {title}
    </h3>
    <p class="text-sm text-text-muted">
      {description}
    </p>
  </div>
</button>
