<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";

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

<div class="flex flex-col gap-1.5 w-full">
  {#if label}
    <label for={id} class="text-sm font-medium text-text-secondary">
      {label}
    </label>
  {/if}
  <input
    {id}
    bind:value
    class="flex h-10 w-full rounded-md border border-surface/10 bg-surface/4 px-3 py-2 text-sm text-text-primary ring-offset-bg-primary file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:border-accent disabled:cursor-not-allowed disabled:opacity-50 {error ? 'border-danger focus-visible:ring-danger focus-visible:border-danger' : ''} {className}"
    {...rest}
  />
  {#if error}
    <span class="text-xs text-danger">{error}</span>
  {/if}
</div>
