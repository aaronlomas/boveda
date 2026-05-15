<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock, IconX } from "@tabler/icons-svelte";

  let { 
    onclose, 
    onsave 
  }: { 
    onclose: () => void; 
    onsave: (title: string, description: string) => void 
  } = $props();

  let title = $state("");
  let description = $state("");
  let error = $state("");
  let titleInput: HTMLInputElement | undefined = $state();

  $effect(() => {
    titleInput?.focus();
  });

  function handleSubmit() {
    if (!title.trim()) {
      error = $_("documents.note_title_label") + " is required";
      return;
    }
    onsave(title, description);
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-panel/60 backdrop-blur-sm animate-in fade-in duration-300"
  onclick={(e) => {
    if (e.target === e.currentTarget) onclose();
  }}
  onkeydown={(e) => {
    if (e.key === "Escape") onclose();
  }}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
>
  <div
    class="w-full max-w-100 bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-3xl shadow-2xl relative flex flex-col overflow-hidden animate-in zoom-in-95 duration-300"
  >
    <!-- Header -->
    <div class="flex items-center justify-between p-4 border-b border-surface/5">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-accent/10 text-accent flex items-center justify-center">
          <IconLock size={24} />
        </div>
        <h2 class="text-xl font-bold text-text-primary">
          {$_("documents.save_note_title")}
        </h2>
      </div>
      <button 
        onclick={onclose}
        class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-xl transition-all"
      >
        <IconX size={20} />
      </button>
    </div>

    <!-- Body -->
    <div class="p-8 space-y-6">
      <div class="space-y-2">
        <label for="note-title" class="text-sm font-bold text-text-secondary">
          {$_("documents.note_title_label")}
        </label>
        <input
          id="note-title"
          bind:this={titleInput}
          bind:value={title}
          placeholder={$_("documents.note_title_placeholder")}
          class="w-full bg-surface/10 border border-surface/20 rounded-sm px-4 py-2 text-text-primary outline-none focus:border-accent/50 focus:ring-1 focus:ring-accent/20 transition-all shadow-inner"
        />
      </div>

      <div class="space-y-2">
        <label for="note-desc" class="text-sm font-bold text-text-secondary">
          {$_("documents.note_desc_label")}
        </label>
        <textarea
          id="note-desc"
          bind:value={description}
          placeholder={$_("documents.note_desc_placeholder")}
          class="w-full bg-surface/10 border border-surface/20 rounded-sm px-4 py-2 text-text-primary outline-none focus:border-accent/50 focus:ring-1 focus:ring-accent/20 transition-all min-h-8 resize-none shadow-inner"
        ></textarea>
      </div>

      {#if error}
        <p class="text-danger text-sm bg-danger/10 p-3 rounded-xl border border-danger/20">
          {error}
        </p>
      {/if}
    </div>

    <!-- Footer -->
    <div class="p-2 border-t border-surface/5 flex gap-4 justify-end">
      <button
        onclick={onclose}
        class="px-4 py-2 rounded-xl border border-surface/20 text-text-secondary font-bold hover:bg-surface/10 hover:text-text-primary transition-all min-w-[120px]"
      >
        {$_("documents.cancel")}
      </button>
      <button
        onclick={handleSubmit}
        class="px-4 py-2 rounded-xl bg-accent text-bg-primary font-bold hover:bg-accent-light hover:-translate-y-0.5 hover:shadow-lg hover:shadow-accent/20 active:translate-y-0 transition-all flex items-center justify-center gap-2"
      >
        {$_("documents.encrypt_save")}
      </button>
    </div>
  </div>
</div>
