<script lang="ts">
  /**
   * @component ListItem
   * @description Flexible row component for configuration lists.
   */
  export let layout: "simple" | "double" | "triple" = "simple";
  
  export let flush: boolean = false;

  const layouts = {
    simple: "space-y-4",
    double: "grid grid-cols-[1fr_auto] gap-4 items-center",
    triple: "grid grid-cols-[auto_1fr_auto] gap-4 items-center"
  };

  // Remove background and border by adding the 'flush' property
  $: variantStyles = flush 
    ? "" 
    : "bg-surface/3 border border-surface/8 rounded-2xl p-4";
</script>

<div class="{variantStyles} {layouts[layout]}">
  <slot />
</div>

