<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock } from "@tabler/icons-svelte";
  import { useForm } from "$lib/validation/useForm.svelte";
  import { noteSchema, type NoteForm } from "$lib/validation/schemas";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";

  let { 
    onclose, 
    onsave 
  }: { 
    onclose: () => void; 
    onsave: (title: string, description: string) => void 
  } = $props();

  const form = useForm<NoteForm>(
    noteSchema,
    { title: "", description: "" },
    async (values) => {
      onsave(values.title, values.description || "");
    }
  );

  let titleInput: HTMLInputElement | undefined = $state();

  $effect(() => {
    titleInput?.focus();
  });
</script>

<Modal 
  show={true} 
  onclose={onclose} 
  title={$_("documents.save_note_title")}
>
  <!-- Body -->
  <div class="space-y-6">
    <div class="flex items-center gap-4 mb-2">
      <div class="w-10 h-10 rounded-xl bg-accent/10 text-accent flex items-center justify-center shrink-0">
        <IconLock size={22} />
      </div>
      <p class="text-xs text-text-muted leading-relaxed">
        {$_("documents.save_note_desc") || "Asigna un nombre para identificar este documento cifrado en tu bóveda."}
      </p>
    </div>

    <form 
      id="save-note-form" 
      class="space-y-4"
      onsubmit={(e) => { e.preventDefault(); form.handleSubmit(); }}
    >
      <div class="space-y-1.5">
        <label for="note-title" class="text-xs font-semibold text-text-secondary">
          {$_("documents.note_title_label")}
        </label>
        <input
          id="note-title"
          type="text"
          bind:this={titleInput}
          bind:value={form.values.title}
          placeholder={$_("documents.note_title_placeholder") || "Mi nota secreta..."}
          class="w-full bg-surface/5 border {form.errors.title ? 'border-danger' : 'border-surface/10'} rounded-lg px-4 py-2 text-text-primary outline-none focus:border-accent/50 transition-all shadow-inner text-sm"
        />
        {#if form.errors.title}
          <span class="text-xs text-danger animate-in fade-in slide-in-from-top-1">
            {$_("documents.note_title_label")} {$_("common.is_required") || "es obligatorio"}
          </span>
        {/if}
      </div>

      <div class="space-y-1.5">
        <label for="note-desc" class="text-xs font-semibold text-text-secondary">
          {$_("documents.note_desc_label")}
        </label>
        <textarea
          id="note-desc"
          bind:value={form.values.description}
          placeholder={$_("documents.note_desc_placeholder") || "Información adicional..."}
          rows="3"
          class="w-full bg-surface/5 border border-surface/10 rounded-lg px-4 py-2 text-text-primary outline-none focus:border-accent/50 transition-all shadow-inner text-sm resize-none"
        ></textarea>
      </div>

      {#if form.globalError}
        <p class="text-danger text-xs bg-danger/10 p-3 rounded-lg border border-danger/20">
          {form.globalError}
        </p>
      {/if}
    </form>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={onclose}>
      {$_("global.cancel")}
    </Button>
    <Button 
      type="submit" 
      form="save-note-form" 
      disabled={form.loading}
      class="min-w-32"
    >
      {#if form.loading}
        <span class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin mr-2"></span>
        {$_("common.saving") || "Guardando..."}
      {:else}
        {$_("documents.save_button_action") || "Guardar Documento"}
      {/if}
    </Button>
  {/snippet}
</Modal>
