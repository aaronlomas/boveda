<script lang="ts">
  import type { HTMLSelectAttributes } from "svelte/elements";
  import type { Snippet } from "svelte";

  interface Props extends HTMLSelectAttributes {
    label?: string;
    error?: string;
    value?: string;
    children?: Snippet;
  }

  let {
    label,
    error,
    value = $bindable(),
    children,
    class: className = "",
    id = crypto.randomUUID(),
    ...rest
  }: Props = $props();
</script>

<div class="flex flex-col gap-1.5 w-full">
  {#if label}
    <label for={id} class="text-sm font-medium text-text-secondary">
      {label}
    </label>
  {/if}
  <select
    {id}
    bind:value
    class="flex h-10 w-full items-center justify-between rounded-md border border-surface/10 bg-surface/4 px-3 py-2 text-sm text-text-primary ring-offset-bg-primary placeholder:text-text-muted focus:outline-none focus:ring-2 focus:ring-accent focus:border-accent disabled:cursor-not-allowed disabled:opacity-50 {error ? 'border-danger focus:ring-danger focus:border-danger' : ''} {className}"
    {...rest}
  >
    {#if children}
      {@render children()}
    {/if}
  </select>
  {#if error}
    <span class="text-xs text-danger">{error}</span>
  {/if}
</div>
