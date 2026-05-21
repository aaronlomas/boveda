<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock, IconAlertTriangle, IconX, IconEye } from "@tabler/icons-svelte";
  import { BoardStore } from "./store.svelte";
  import Toolbar from "./ui/Toolbar.svelte";
  import Canvas from "./ui/Canvas.svelte";
  import Sidebar from "./ui/Sidebar.svelte";
  import Button from "../../../core/primitives/Button.svelte";

  let { 
    content = $bindable(""), 
    onviewnotes, 
    onsave 
  }: { 
    content: string; 
    onviewnotes: () => void; 
    onsave: () => void 
  } = $props();

  import { untrack } from "svelte";

  const store = new BoardStore(content);

  // Sync back to parent content
  $effect(() => {
    const nextContent = store.content;
    untrack(() => {
      if (content !== nextContent) {
        content = nextContent;
      }
    });
  });
</script>

<div
  class="bg-surface/4 backdrop-blur-2xl rounded-xs border border-surface/8 shadow-xl overflow-hidden flex flex-col animate-in fade-in slide-in-from-bottom-4 duration-500 h-full"
>
  <Toolbar {store} />

  <!-- Security Alert Banner -->
  {#if store.pasteWarning}
    <div class="px-4 py-2 bg-warning/10 border-b border-warning/20 flex items-center justify-between text-warning text-xs animate-in fade-in slide-in-from-top-2 duration-300">
      <div class="flex items-center gap-2">
        <IconAlertTriangle size={16} class="min-w-4" />
        <span class="font-medium">{store.pasteWarning}</span>
      </div>
      <button class="hover:text-warning-light p-1 rounded-md transition-colors" onclick={() => store.clearPasteWarning()}>
        <IconX size={14} />
      </button>
    </div>
  {/if}

  <div class="flex flex-1 overflow-hidden">
    <Canvas {store} />
    <Sidebar {store} {onviewnotes} />
  </div>

  <!-- Actions Footer -->
  <div
    class="p-4 border-t border-surface/8 bg-surface/5 flex justify-end gap-3 items-center"
  >
    <Button
      onclick={onsave}
      class="px-4 gap-2 font-bold"
    >
      <IconLock size={20} />
      {$_("board.generate_encrypt")}
    </Button>
  </div>
</div>
