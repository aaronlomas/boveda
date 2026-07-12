<script lang="ts">
  /**
   * Primitive component of type Capsule that encapsulates the credentials.
   */
  import type { Snippet } from "svelte";
  import { slide } from "svelte/transition";
  import { uiState } from "$lib/stores/ui.svelte";

  interface Props {
    header: Snippet;
    children?: Snippet;
    class?: string;
    style?: string;
    "data-card-id"?: string;
  }

  let {
    header,
    children,
    class: className = "",
    style = "",
    "data-card-id": dataCardId = "",
  }: Props = $props();

  let expanded = $state(false);

  /**
   * Toggles the visibility of the encapsulated content
   */
  function toggle() {
    if (uiState.capsuleLocked) return;
    expanded = !expanded;
  }
</script>

<div
  class="flex flex-col h-fit transition-all bg-panel/30 backdrop-blur-2xl rounded-2xl border {className}"
  {style}
  data-card-id={dataCardId}
>
  <div class="p-4">
    {@render header()}
  </div>

  {#if expanded && children}
    <div class="p-4 pt-0 flex flex-col gap-4" transition:slide>
      {@render children()}
    </div>
  {/if}

  {#if children}
    <button
      onclick={toggle}
      aria-label={expanded ? "Collapse" : "Expand"}
      class="w-full flex items-center justify-center h-6 bg-surface/5 hover:bg-surface/10 transition-colors rounded-b-2xl border-t border-surface/8 cursor-pointer text-text-muted hover:text-text-primary"
    >
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="currentColor"
        aria-hidden="true"
        class="transition-transform duration-300 {expanded ? 'rotate-180' : ''}"
      >
        <path d="M7 10L12 15L17 10H7Z" />
      </svg>
    </button>
  {/if}
</div>
