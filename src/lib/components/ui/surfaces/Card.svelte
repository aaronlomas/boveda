<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
    class?: string;
  }

  let { children, class: className = "" }: Props = $props();
</script>

<div class="rounded-xl border border-surface/10 bg-surface/4 text-text-primary shadow-sm backdrop-blur-md {className}">
  {#if children}
    {@render children()}
  {/if}
</div>
