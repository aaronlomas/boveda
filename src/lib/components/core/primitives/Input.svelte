<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { COMPONENTS, COLORS } from "$lib/config/design-tokens";

  interface Props extends HTMLInputAttributes {
    label?: string;
    error?: string;
    value?: string;
  }

  let {
    label,
    error,
    value = $bindable(""),
    class: className = "",
    id = crypto.randomUUID(),
    ...rest
  }: Props = $props();
</script>

<div class="flex flex-col gap-1 w-full">
  {#if label}
    <label for={id} class="text-sm font-medium {COLORS.text.secondary}">
      {label}
    </label>
  {/if}
  <input
    {id}
    bind:value
    class="{COMPONENTS.input} {error ? 'border-danger focus:ring-danger focus:border-danger' : ''} {className}"
    {...rest}
  />
  {#if error}
    <span class="text-xs text-danger">{error}</span>
  {/if}
</div>
