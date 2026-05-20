<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    children?: Snippet;
    class?: string;
  }

  let { children, class: className = "" }: Props = $props();
  import { COMPONENTS } from "$lib/config/design-tokens";
</script>

<div class="{COMPONENTS.panel} overflow-hidden {className}">
  {#if children}
    {@render children()}
  {/if}
</div>
