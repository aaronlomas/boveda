<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface Props extends HTMLButtonAttributes {
    variant?: "primary" | "secondary" | "danger" | "ghost";
    size?: "sm" | "md" | "lg" | "icon";
    children?: Snippet;
  }

  let {
    variant = "primary",
    size = "md",
    children,
    class: className = "",
    ...rest
  }: Props = $props();

  const baseStyles = "inline-flex items-center justify-center font-medium rounded-lg transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent disabled:opacity-50 disabled:pointer-events-none";

  const variants = {
    primary: "bg-accent text-bg-primary hover:bg-accent-light",
    secondary: "bg-surface/5 text-text-secondary border border-surface/10 hover:bg-surface/10 hover:text-text-primary",
    danger: "bg-danger text-white hover:bg-danger/90",
    ghost: "bg-transparent text-text-secondary hover:bg-surface/5 hover:text-text-primary",
  };

  const sizes = {
    sm: "h-8 px-3 text-xs",
    md: "px-4 py-2 text-sm",
    lg: "h-12 px-8 text-base",
    icon: "h-10 w-10",
  };
</script>

<button
  class="{baseStyles} {variants[variant]} {sizes[size]} {className}"
  {...rest}
>
  {#if children}
    {@render children()}
  {/if}
</button>
