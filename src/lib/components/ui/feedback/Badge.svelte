<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    variant?: "default" | "secondary" | "destructive" | "outline" | "accent";
    children?: Snippet;
    class?: string;
  }

  let {
    variant = "default",
    children,
    class: className = "",
  }: Props = $props();

  const variants = {
    default: "border-transparent bg-text-primary text-bg-primary hover:bg-text-primary/80",
    secondary: "border-transparent bg-surface/10 text-text-secondary hover:bg-surface/20",
    destructive: "border-transparent bg-danger text-white hover:bg-danger/80",
    outline: "text-text-primary border-surface/20",
    accent: "border-transparent bg-accent text-bg-primary hover:bg-accent-light",
  };
</script>

<div class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 {variants[variant]} {className}">
  {#if children}
    {@render children()}
  {/if}
</div>
