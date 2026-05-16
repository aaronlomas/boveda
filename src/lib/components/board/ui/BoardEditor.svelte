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
    IconEye,
    IconLock,
    IconChevronDown,
    IconEraser,
    IconAlertTriangle,
    IconX
  } from "@tabler/icons-svelte";
  import { execBoardCommand, getCommandState, getAlignment, setFontSize, getFontSizeAtCaret } from "$lib/utils/board/board";

  let { 
    content = $bindable(""), 
    onviewnotes, 
    onsave 
  }: { 
    content: string; 
    onviewnotes: () => void; 
    onsave: () => void 
  } = $props();

  let editorRef: HTMLDivElement | undefined = $state();
  let isBold = $state(false);
  let isItalic = $state(false);
  let isUnderline = $state(false);
  let isStrikethrough = $state(false);
  let isListUl = $state(false);
  let isListOl = $state(false);
  let textAlign = $state<"left" | "center" | "right">("left");
  let currentSize = $state(14);
  let showSizeDropdown = $state(false);
  let pasteWarning = $state("");

  // Initialize content once
  $effect(() => {
    if (editorRef && !editorRef.innerHTML && content) {
      editorRef.innerHTML = content;
    }
  });

  function updateState() {
    isBold = getCommandState("bold");
    isItalic = getCommandState("italic");
    isUnderline = getCommandState("underline");
    isStrikethrough = getCommandState("strikeThrough");
    isListUl = getCommandState("insertUnorderedList");
    isListOl = getCommandState("insertOrderedList");
    textAlign = getAlignment();
    const detected = getFontSizeAtCaret();
    if (detected !== null) currentSize = detected;
  }

  function handleCommand(cmd: string, val?: string) {
    execBoardCommand(cmd, val);
    updateState();
    if (editorRef) content = editorRef.innerHTML;
  }

  function handleInput() {
    updateState();
    if (editorRef) content = editorRef.innerHTML;
  }

  function handleFontSize(size: number) {
    setFontSize(size);
    currentSize = size;
    if (editorRef) content = editorRef.innerHTML;
  }

  function handlePaste(e: ClipboardEvent) {
    e.preventDefault();
    let text = e.clipboardData?.getData("text/plain");
    
    if (text) {
      // 1. Sanitize: Remove zero-width spaces and dangerous non-printable control characters
      const sanitizedText = text.replace(/[\u200B-\u200D\uFEFF\u0000-\u0008\u000B\u000C\u000E-\u001F\u007F]/g, "");
      
      // 2. Detect: Expanded dangerous patterns
      const dangerousPatterns = /(\bsudo\b|\bcurl\b|\bwget\b|\bpowershell\b|\bbash\b|\bsh -c\b|\brm -rf\b|\biex\b|\binvoke-expression\b|\bshutdown\b|\bnet user\b|\bformat\b|\bdel \/f\b)/gi;
      
      if (dangerousPatterns.test(sanitizedText)) {
        pasteWarning = $_("documents.paste_warning") || "¡Atención! El texto pegado contiene comandos de sistema. Nunca copies y pegues esto en una terminal a menos que sepas exactamente lo que hace.";
        
        // 3. Unmasking: Highlight malicious parts in red
        // Escape HTML to prevent XSS from the text itself
        const escaped = sanitizedText.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
        const highlightedHTML = escaped.replace(dangerousPatterns, (match) => {
          return `<span style="color: #ef4444; font-weight: bold; background: rgba(239, 68, 68, 0.15); padding: 1px 4px; border-radius: 4px; border: 1px solid rgba(239, 68, 68, 0.3);" title="Detección de Bóveda: Comando peligroso">${match}</span>`;
        });
        
        document.execCommand("insertHTML", false, highlightedHTML);
      } else {
        document.execCommand("insertText", false, sanitizedText);
      }
      
      if (editorRef) content = editorRef.innerHTML;
    }
  }
</script>

<div
  class="bg-surface/4 backdrop-blur-2xl rounded-xs border border-surface/8 shadow-xl overflow-hidden flex flex-col animate-in fade-in slide-in-from-bottom-4 duration-500"
>
  <!-- Toolbar -->
  <div class="h-12 flex items-center gap-2 p-3 border-b border-surface/8 bg-surface/2">
    <div class="flex items-center gap-1 rounded-lg p-1">
      <!-- Custom Font Size Dropdown -->
      <div class="relative">
        <button
          onclick={() => (showSizeDropdown = !showSizeDropdown)}
          class="h-8 px-3 flex items-center gap-2 bg-surface/5 border border-surface/10 rounded-xs text-xs text-text-primary hover:bg-surface/10 hover:border-surface/20 transition-all min-w-18 justify-between group"
        >
          <span class="font-medium">{currentSize}</span>
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
                  currentSize = size;
                  handleFontSize(size);
                  showSizeDropdown = false;
                }}
                class="w-full text-left px-4 py-2 text-xs transition-colors {currentSize ===
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
        class="p-2 rounded-md hover:text-accent-light transition-colors {isBold ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("bold")}
        title={$_("documents.bold")}
      >
        <IconBold size={18} stroke={2.5} />
      </button>

      <button
        class="p-2 rounded-md hover:text-text-primary transition-colors {isItalic ? 'text-text-primary bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("italic")}
        title={$_("documents.italic")}
      >
        <IconItalic size={18} stroke={2.5} />
      </button>
      
      <button
        class="p-2 rounded-md hover:text-text-primary transition-colors {isUnderline ? 'text-text-primary bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("underline")}
        title={$_("documents.underline")}
      >
        <IconUnderline size={18} stroke={2.5} />
      </button>
      
      <button
        class="p-2 rounded-md hover:text-text-primary transition-colors {isStrikethrough ? 'text-text-primary bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("strikeThrough")}
        title={$_("documents.strikethrough")}
      >
        <IconStrikethrough size={18} stroke={2.5} />
      </button>
    </div>

    <div class="w-px h-6 bg-surface/20 mx-1"></div>

    <div class="flex items-center gap-1 rounded-lg p-1">
      <button
        class="p-2 rounded-md transition-colors {isListUl ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("insertUnorderedList")}
        title={$_("documents.list_ul")}
      >
        <IconList size={18} stroke={2.5} />
      </button>
      <button
        class="p-2 rounded-md transition-colors {isListOl ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("insertOrderedList")}
        title={$_("documents.list_ol")}
      >
        <IconListNumbers size={18} stroke={2.5} />
      </button>
    </div>

    <div class="w-px h-6 bg-surface/20 mx-1"></div>

    <div class="flex items-center gap-1 rounded-lg p-1">
      <button
        class="p-2 rounded-md transition-colors {textAlign === 'left' ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("justifyLeft")}
        title={$_("documents.align_left")}
      >
        <IconAlignLeft size={18} stroke={2.5} />
      </button>
      <button
        class="p-2 rounded-md transition-colors {textAlign === 'center' ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("justifyCenter")}
        title={$_("documents.align_center")}
      >
        <IconAlignCenter size={18} stroke={2.5} />
      </button>
      <button
        class="p-2 rounded-md transition-colors {textAlign === 'right' ? 'text-accent-light bg-surface/10' : 'text-text-secondary'}"
        onclick={() => handleCommand("justifyRight")}
        title={$_("documents.align_right")}
      >
        <IconAlignRight size={18} stroke={2.5} />
      </button>
    </div>

    <div class="w-px h-6 bg-surface/20 mx-1"></div>

    <button
      class="p-2 rounded-md text-text-secondary hover:text-danger transition-colors"
      onclick={() => handleCommand("removeFormat")}
      title={$_("documents.clear_format")}
    >
      <IconEraser size={18} stroke={2.5} />
    </button>
  </div>

  <!-- Security Alert Banner -->
  {#if pasteWarning}
    <div class="px-4 py-2.5 bg-warning/10 border-b border-warning/20 flex items-center justify-between text-warning text-xs animate-in fade-in slide-in-from-top-2 duration-300">
      <div class="flex items-center gap-2">
        <IconAlertTriangle size={16} class="min-w-4" />
        <span class="font-medium">{pasteWarning}</span>
      </div>
      <button class="hover:text-warning-light p-1 rounded-md transition-colors" onclick={() => (pasteWarning = "")}>
        <IconX size={14} />
      </button>
    </div>
  {/if}

  <!-- Editor Area -->
  <div class="relative flex-1 min-h-70 p-0">
    <div
      bind:this={editorRef}
      contenteditable="true"
      role="textbox"
      tabindex="0"
      aria-multiline="true"
      aria-label={$_("documents.placeholder")}
      class="editor-pizarra w-full h-full min-h-100 p-8 bg-transparent border-none outline-none text-text-primary transition-all focus:ring-0 leading-relaxed overflow-y-auto"
      placeholder={$_("documents.placeholder")}
      onkeyup={updateState}
      onmouseup={updateState}
      oninput={handleInput}
      onblur={handleInput}
      onpaste={handlePaste}
    ></div>
  </div>

  <!-- Actions Footer -->
  <div
    class="p-4 border-t border-surface/8 bg-surface/5 flex justify-end gap-3 items-center"
  >
    <button
      onclick={onviewnotes}
      class="px-4 py-2 rounded-xl border border-surface/20 text-text-secondary font-medium hover:bg-surface/10 hover:text-text-primary transition-all flex items-center gap-2"
    >
      <IconEye size={20} />
      {$_("documents.view_notes")}
    </button>
    <button
      onclick={onsave}
      class="px-4 py-2 rounded-xl bg-accent text-white font-bold hover:bg-accent-light hover:-translate-y-0.5 hover:shadow-lg hover:shadow-accent/20 active:translate-y-0 transition-all flex items-center gap-2"
    >
      <IconLock size={20} />
      {$_("documents.generate_encrypt")}
    </button>
  </div>
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
