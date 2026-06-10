<script lang="ts">
  /**
   * @component Badge
   * @description Small label component to display statuses or categories.
   * 
   * @param {Object} props
   * @param {"default" | "secondary" | "destructive" | "outline" | "accent"} [props.variant="default"] - Visual stayle.
   * @param {import("svelte").Snippet} [props.children] - badge content.
   * @param {string} [props.class=""] - Additional CSS classes.
   */
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

  import { COLORS, ANIMATION } from "$lib/config/design-tokens";

  const variants = {
    default: `border-transparent bg-text-primary text-panel hover:bg-text-primary/80`,
    secondary: `border-transparent ${COLORS.surface[10]} ${COLORS.text.secondary} hover:${COLORS.surface[20]}`,
    destructive: `border-transparent bg-danger text-white hover:bg-danger/80`,
    outline: `${COLORS.text.primary} border ${COLORS.surface.borderMedium}`,
    accent: `border-transparent ${COLORS.accent.DEFAULT} text-white ${COLORS.accent.hover}`,
  };
</script>

<div class="inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 {ANIMATION.transitionColors} {variants[variant]} {className}">
  {#if children}
    {@render children()}
  {/if}
</div>
