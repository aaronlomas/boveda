<script lang="ts">
  import { _ } from "svelte-i18n";
  import {
    IconBold,
    IconItalic,
    IconAlignLeft,
    IconAlignCenter,
    IconAlignRight,
    IconUnderline,
    IconStrikethrough,
    IconList,
    IconListNumbers,
    IconChevronDown,
    IconEraser
  } from "@tabler/icons-svelte";
  import type { BoardStore } from "../store.svelte";

  let { store }: { store: BoardStore } = $props();
  let showSizeDropdown = $state(false);
</script>

<div class="h-12 flex items-center gap-2 p-3 border-b border-surface/8 bg-surface/2">
  <div class="flex items-center gap-1 rounded-lg p-1">
    <!-- Custom Font Size Dropdown -->
    <div class="relative">
      <button
        onclick={() => (showSizeDropdown = !showSizeDropdown)}
        class="h-8 px-3 flex items-center gap-2 bg-surface/5 border border-surface/10 rounded-xs text-xs text-text-primary hover:bg-surface/10 hover:border-surface/20 transition-all min-w-18 justify-between group"
      >
        <span class="font-medium">{store.currentSize}</span>
        <IconChevronDown
          size={12}
          class="text-text-muted group-hover:text-text-primary transition-colors {showSizeDropdown
            ? 'rotate-180'
            : ''}"
        />
      </button>

      {#if showSizeDropdown}
        <div
          class="absolute top-full left-0 mt-2 py-2 bg-transparent backdrop-blur-3xl rounded-xs border border-surface/10 shadow-2xl z-50 min-w-16 max-h-64 overflow-y-auto custom-scrollbar animate-in fade-in zoom-in-95 duration-200"
        >
          {#each [8, 9, 10, 11, 12, 14, 16, 18, 20, 24, 28, 32, 36, 48, 72] as size}
            <button
              onclick={() => {
                store.handleFontSize(size);
                showSizeDropdown = false;
              }}
              class="w-full text-left px-4 py-2 text-xs transition-colors {store.currentSize ===
              size
                ? 'text-accent bg-accent/5 font-bold'
                : 'text-text-muted hover:text-text-primary hover:bg-surface/10'}"
            >
              {size}
            </button>
          {/each}
        </div>

        <div
          role="presentation"
          class="fixed inset-0 z-40"
          onclick={() => (showSizeDropdown = false)}
          onkeydown={(e) => { if (e.key === 'Escape') showSizeDropdown = false; }}
        ></div>
      {/if}
    </div>

    <button
      class="p-2 rounded-md hover:text-accent-light transition-colors {store.isBold ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("bold")}
      title={$_("board.bold")}
    >
      <IconBold size={18} stroke={2.5} />
    </button>

    <button
      class="p-2 rounded-md hover:text-text-primary transition-colors {store.isItalic ? 'text-text-primary bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("italic")}
      title={$_("board.italic")}
    >
      <IconItalic size={18} stroke={2.5} />
    </button>
    
    <button
      class="p-2 rounded-md hover:text-text-primary transition-colors {store.isUnderline ? 'text-text-primary bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("underline")}
      title={$_("board.underline")}
    >
      <IconUnderline size={18} stroke={2.5} />
    </button>
    
    <button
      class="p-2 rounded-md hover:text-text-primary transition-colors {store.isStrikethrough ? 'text-text-primary bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("strikeThrough")}
      title={$_("board.strikethrough")}
    >
      <IconStrikethrough size={18} stroke={2.5} />
    </button>
  </div>

  <div class="w-px h-6 bg-surface/20 mx-1"></div>

  <div class="flex items-center gap-1 rounded-lg p-1">
    <button
      class="p-2 rounded-md transition-colors {store.isListUl ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("insertUnorderedList")}
      title={$_("board.list_ul")}
    >
      <IconList size={18} stroke={2.5} />
    </button>
    <button
      class="p-2 rounded-md transition-colors {store.isListOl ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("insertOrderedList")}
      title={$_("board.list_ol")}
    >
      <IconListNumbers size={18} stroke={2.5} />
    </button>
  </div>

  <div class="w-px h-6 bg-surface/20 mx-1"></div>

  <div class="flex items-center gap-1 rounded-lg p-1">
    <button
      class="p-2 rounded-md transition-colors {store.textAlign === 'left' ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("justifyLeft")}
      title={$_("board.align_left")}
    >
      <IconAlignLeft size={18} stroke={2.5} />
    </button>
    <button
      class="p-2 rounded-md transition-colors {store.textAlign === 'center' ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("justifyCenter")}
      title={$_("board.align_center")}
    >
      <IconAlignCenter size={18} stroke={2.5} />
    </button>
    <button
      class="p-2 rounded-md transition-colors {store.textAlign === 'right' ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
      onclick={() => store.handleCommand("justifyRight")}
      title={$_("board.align_right")}
    >
      <IconAlignRight size={18} stroke={2.5} />
    </button>
  </div>

  <div class="w-px h-6 bg-surface/20 mx-1"></div>

  <button
    class="p-2 rounded-md text-text-secondary hover:text-danger transition-colors"
    onclick={() => store.handleCommand("removeFormat")}
    title={$_("board.clear_format")}
  >
    <IconEraser size={18} stroke={2.5} />
  </button>
</div>
