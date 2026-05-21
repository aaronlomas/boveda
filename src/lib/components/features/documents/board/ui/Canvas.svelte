<script lang="ts">
  import { _ } from "svelte-i18n";
  import { handlePasteSecurity } from "../utils/formatting";
  import type { BoardStore } from "../store.svelte";

  let { store }: { store: BoardStore } = $props();

  // Initialize content once
  $effect(() => {
    if (store.editorRef && !store.editorRef.innerHTML && store.content) {
      store.editorRef.innerHTML = store.content;
    }
  });

  import { t } from "$lib/i18n";

  function handlePaste(e: ClipboardEvent) {
    handlePasteSecurity(e, (warningKey) => {
      store.setPasteWarning(t(`documents.${warningKey}`, "¡Atención! El texto pegado contiene comandos de sistema."));
    });
    store.syncContent();
  }

  function handleInput() {
    store.updateState();
    store.syncContent();
  }
</script>

<div class="relative flex-1 min-h-70 p-0">
  <div
    bind:this={store.editorRef}
    contenteditable="true"
    role="textbox"
    tabindex="0"
    aria-multiline="true"
    aria-label={$_("board.placeholder")}
    class="editor-pizarra w-full h-full min-h-100 p-8 bg-transparent border-none outline-none text-text-primary transition-all focus:ring-0 leading-relaxed overflow-y-auto"
    placeholder={$_("board.placeholder")}
    onkeyup={() => store.updateState()}
    onmouseup={() => store.updateState()}
    oninput={handleInput}
    onblur={handleInput}
    onpaste={handlePaste}
  ></div>
</div>

<style>
  .editor-pizarra {
    font-size: 14pt;
  }
  .editor-pizarra:empty:before {
    content: attr(placeholder);
    color: #666; /* Fallback */
    color: var(--color-text-muted);
    opacity: 0.5;
    pointer-events: none;
  }
  
  /* Ensure proper list styling in editor */
  .editor-pizarra :global(ul) {
    list-style-type: disc;
    padding-left: 2em;
    margin: 0.5em 0;
  }
  
  .editor-pizarra :global(ol) {
    list-style-type: decimal;
    padding-left: 2em;
    margin: 0.5em 0;
  }
</style>
