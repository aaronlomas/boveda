<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import type { Snippet } from "svelte";
  import { COMPONENTS, COLORS } from "$lib/config/design-tokens";

  interface Props extends HTMLInputAttributes {
    label?: string;
    error?: string;
    value?: string;
    /** Input structure type */
    variant?: "simple" | "double" | "triple";
    /** Initial icon (Required only in 'triple') */
    icon?: Snippet;
    /**Show and hide icon (Required in 'double' and 'triple') */
    action?: Snippet;
  }

  let {
    label,
    error,
    value = $bindable(""),
    variant = "simple",
    icon,
    action,
    class: className = "",
    id = crypto.randomUUID(),
    ...rest
  }: Props = $props();

  // Layout: Mapping based on the variant chosen
  const gridLayouts = {
    simple: "grid-cols-1",
    double: "grid-cols-[1fr_auto]",
    triple: "grid-cols-[auto_1fr_auto]"
  };
</script>

<div class="flex flex-col gap-1 w-full">
  {#if label}
    <label for={id} class="text-sm font-medium {COLORS.text.secondary}">
      {label}
    </label>
  {/if}

  <!-- Base Container -->
  <div 
    class="
      {COMPONENTS.input} 
      grid items-center gap-2 px-3
      {gridLayouts[variant]}
      {error ? 'border-danger focus-within:ring-danger focus-within:border-danger' : ''} 
      {className}
    "
  >
  
    {#if variant === "triple" && icon}
      <div class="flex items-center justify-center shrink-0">
        {@render icon()}
      </div>
    {/if}

    <!-- 2. Simple input, to collect only informational data -->
    <input
      {id}
      bind:value
      class="w-full bg-transparent py-1 border-none outline-none focus:ring-0 focus:outline-none"
      {...rest}
    />

    <!-- Action icon (Variants 'double' and 'triple') -->
    {#if (variant === "double" || variant === "triple") && action}
      <div class="flex items-center justify-center shrink-0">
        {@render action()}
      </div>
    {/if}
  </div>

  {#if error}
    <span class="text-xs text-danger">{error}</span>
  {/if}
</div>
