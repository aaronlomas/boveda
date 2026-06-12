<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { COLORS, ANIMATION } from "$lib/config/design-tokens";

  interface Props extends Omit<HTMLInputAttributes, 'type'> {
    checked?: boolean;
    label?: string;
  }

  let {
    checked = $bindable(false),
    label,
    class: className = "",
    id = crypto.randomUUID(),
    ...rest
  }: Props = $props();
</script>

<div class="flex items-center space-x-3">
  <button
    type="button"
    role="switch"
    aria-checked={checked}
    {id}
    class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-bg-primary disabled:cursor-not-allowed disabled:opacity-50 {checked ? 'bg-accent' : 'bg-surface/20'} {className}"
    onclick={() => checked = !checked}
    {...rest as any}
  >
    <span
      class="pointer-events-none inline-block h-4 w-4 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {checked ? 'translate-x-4' : 'translate-x-0'}"
    ></span>
  </button>
  {#if label}
    <label
      for={id}
      class="text-sm font-medium leading-none text-text-primary cursor-pointer select-none"
    >
      {label}
    </label>
  {/if}
</div>
