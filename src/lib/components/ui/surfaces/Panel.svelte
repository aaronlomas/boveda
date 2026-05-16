<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
    class?: string;
  }

  let { children, class: className = "" }: Props = $props();
</script>

<div class="rounded-2xl bg-surface/5 border border-surface/8 shadow-xl overflow-hidden {className}">
  {#if children}
    {@render children()}
  {/if}
</div>
