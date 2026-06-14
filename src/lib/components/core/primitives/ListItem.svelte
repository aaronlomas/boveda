<script lang="ts">
  /**
   * @component ListItem
   * @description Flexible row component for configuration lists.
   */
  export let layout: "simple" | "double" | "triple" = "simple";

  const layouts = {
    simple: "space-y-4",
    double: "grid grid-cols-[1fr_auto] gap-4 items-center",
    triple: "grid grid-cols-[auto_1fr_auto] gap-4 items-center"
  };
</script>

<div class="bg-surface/3 border border-surface/8 rounded-2xl p-4 {layouts[layout]}">
  <slot />
</div>

