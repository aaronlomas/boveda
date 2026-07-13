<script lang="ts">
  import { IconPalette, IconBan, IconPlus } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  let { selectedColor = $bindable(), onselect } = $props<{
    selectedColor: string | null;
    onselect?: (color: string | null) => void;
  }>();

  let isOpen = $state(false);

  // Modern vibrant colors
  const colors = [
    "#ef4444", // red
    "#f97316", // orange
    "#eab308", // yellow
    "#22c55e", // green
    "#06b6d4", // cyan
    "#3b82f6", // blue
    "#8b5cf6", // violet
    "#ec4899", // pink
  ];

  let customColor = $state<string | null>(null);

  $effect(() => {
    if (
      selectedColor &&
      !colors.includes(selectedColor) &&
      customColor !== selectedColor
    ) {
      customColor = selectedColor;
    }
  });

  function selectColor(color: string | null) {
    selectedColor = color;
    isOpen = false;
    if (onselect) onselect(color);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && isOpen) {
      isOpen = false;
    }
  }

  function clickOutside(node: HTMLElement) {
    const handleClick = (e: MouseEvent) => {
      if (!node.contains(e.target as Node)) isOpen = false;
    };
    document.addEventListener("click", handleClick, true);
    return {
      destroy() {
        document.removeEventListener("click", handleClick, true);
      },
    };
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="relative flex items-center" use:clickOutside>
  <button
    class="text-text-muted cursor-pointer hover:text-accent transition-colors flex items-center justify-center rounded-md"
    onclick={() => (isOpen = !isOpen)}
    aria-label={$_("groups.pick_color")}
    title={$_("groups.pick_color")}
  >
    {#if selectedColor}
      <div
        class="w-3 h-3 rounded-full"
        style="background-color: {selectedColor};"
      ></div>
    {:else}
      <IconPalette size={14} />
    {/if}
  </button>

  {#if isOpen}
    <!-- Color picker popover -->
    <div
      class="absolute bottom-full mb-2 left-1/2 -translate-x-1/2 p-2 bg-panel/95 backdrop-blur-md border border-surface/20 rounded-lg shadow-xl shadow-black/20 z-20 flex flex-wrap gap-2 w-max max-w-[140px] justify-center"
    >
      {#each colors as color}
        <button
          class="w-5 h-5 rounded-full cursor-pointer hover:scale-125 transition-transform border border-black/20"
          style="background-color: {color}; {selectedColor === color
            ? 'outline: 2px solid white; outline-offset: 1px;'
            : ''}"
          onclick={() => selectColor(color)}
          aria-label="Color {color}"
        ></button>
      {/each}

      {#if customColor}
        <button
          class="w-5 h-5 rounded-full cursor-pointer hover:scale-125 transition-transform border border-black/20"
          style="background-color: {customColor}; {selectedColor === customColor
            ? 'outline: 2px solid white; outline-offset: 1px;'
            : ''}"
          onclick={() => selectColor(customColor)}
          aria-label="Custom color"
          title="Custom color"
        ></button>
      {/if}

      <!-- Custom color input -->
      <label
        class="w-5 h-5 rounded-full cursor-pointer hover:scale-110 transition-colors border border-surface/40 bg-surface/10 flex items-center justify-center text-text-muted hover:text-white relative overflow-hidden"
        title={$_("groups.create_color")}
      >
        <IconPlus size={12} />
        <input
          type="color"
          class="absolute opacity-0 w-10 h-10 -top-2 -left-2 cursor-pointer"
          oninput={(e) => {
            customColor = e.currentTarget.value;
            selectedColor = customColor;
            if (onselect) onselect(customColor);
          }}
        />
      </label>

      <!-- None option -->
      <button
        class="w-5 h-5 rounded-full cursor-pointer hover:scale-110 transition-colors border border-surface/40 bg-surface/10 flex items-center justify-center text-text-muted hover:text-white"
        onclick={() => selectColor(null)}
        title={$_("groups.no_color")}
      >
        <IconBan size={12} />
      </button>
    </div>
  {/if}
</div>
