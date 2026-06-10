<script lang="ts">
  /**
   * @component Button
   * @description Reusable button component with multiple variations and sizes.
* Extends the standard attributes of an HTML button.
   * 
   * @param {Object} props
   * @param {"primary" | "secondary" | "danger" | "ghost"} [props.variant="primary"] - Visual style.
   * @param {"sm" | "md" | "lg" | "icon"} [props.size="md"] - button size.
   * @param {import("svelte").Snippet} [props.children] - button content.
   */
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
    onclick,
    ...rest
  }: Props = $props();

  import { COLORS, SIZING, ANIMATION } from "$lib/config/design-tokens";

  const baseStyles = `inline-flex items-center justify-center font-medium rounded-xl cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent disabled:opacity-50 disabled:pointer-events-none ${ANIMATION.transitionAll}`;

  const variants = {
    primary: `${COLORS.accent.DEFAULT} text-white ${COLORS.accent.hover} ${ANIMATION.hoverLift} ${SIZING.shadow.md} shadow-accent/20`,
    secondary: `${COLORS.surface[5]} ${COLORS.text.secondary} border ${COLORS.surface.borderLight} hover:${COLORS.surface[10]} hover:${COLORS.text.primary}`,
    danger: `bg-danger text-white hover:bg-danger/90`,
    ghost: `bg-transparent ${COLORS.text.secondary} hover:${COLORS.surface[5]} hover:${COLORS.text.primary}`,
  };

  const sizes = {
    sm: SIZING.button.sm,
    md: SIZING.button.md,
    lg: SIZING.button.lg,
    icon: SIZING.button.icon,
  };
</script>

<button
  class="{baseStyles} {variants[variant]} {sizes[size]} {className}"
  {onclick}
  {...rest}
>
  {#if children}
    {@render children()}
  {/if}
</button>
