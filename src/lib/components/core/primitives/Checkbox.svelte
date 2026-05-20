<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

  interface Props extends Omit<HTMLInputAttributes, 'type'> {
    label?: string;
    checked?: boolean;
    error?: string;
  }

  let {
    label,
    checked = $bindable(false),
    error,
    class: className = "",
    id = crypto.randomUUID(),
    ...rest
  }: Props = $props();
  import { COLORS, ANIMATION } from "$lib/config/design-tokens";
</script>

<div class="flex items-center space-x-2">
  <input
    type="checkbox"
    {id}
    bind:checked
    class="peer h-4 w-4 shrink-0 rounded-sm border {COLORS.surface.borderMedium} {COLORS.surface[5]} {COLORS.accent.text} ring-offset-bg-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-accent data-[state=checked]:text-white {ANIMATION.transitionAll} {className}"
    {...rest}
  />
  {#if label}
    <label
      for={id}
      class="text-sm font-medium leading-none text-text-primary peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
    >
      {label}
    </label>
  {/if}
</div>
{#if error}
  <span class="text-xs text-danger mt-1 block">{error}</span>
{/if}
