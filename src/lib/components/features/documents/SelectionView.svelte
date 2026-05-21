<script lang="ts">
  import { _ } from "svelte-i18n";
  import { getContext } from "svelte";
  import { IconPlus, IconNotes, IconFileImport } from "@tabler/icons-svelte";
  import { DOCUMENTS_CTX, type DocumentsContext } from "./context";

  const docState = getContext<DocumentsContext>(DOCUMENTS_CTX);
</script>

<div
  class="grid grid-cols-1 md:grid-cols-3 gap-6 animate-in fade-in zoom-in-95 duration-500"
>
  <!-- Nueva nota -->
  <button
    onclick={docState.startNew}
    class="group relative flex flex-col items-center text-center p-8 bg-surface/4 backdrop-blur-2xl rounded-3xl border border-surface/8 hover:border-accent/30 hover:bg-surface/10 transition-all duration-300 shadow-xl"
  >
    <div
      class="w-20 h-20 rounded-2xl bg-accent/10 text-accent flex items-center justify-center mb-6 group-hover:scale-110 group-hover:rotate-3 transition-transform duration-500 shadow-lg shadow-accent/5"
    >
      <IconPlus size={40} stroke={2} />
    </div>
    <h3
      class="text-xl font-bold text-text-primary mb-2 group-hover:text-accent transition-colors"
    >
      {$_("documents_mode.create_note")}
    </h3>
    <p class="text-text-muted text-sm max-w-xs">
      {$_("documents_mode.create_note_desc")}
    </p>
    <div
      class="absolute inset-0 border-2 border-accent/0 group-hover:border-accent/20 rounded-3xl transition-all duration-500 pointer-events-none"
    ></div>
  </button>

  <!-- Mis notas -->
  <button
    onclick={docState.startList}
    class="group relative flex flex-col items-center text-center p-8 bg-surface/4 backdrop-blur-2xl rounded-3xl border border-surface/8 hover:border-accent/30 hover:bg-surface/10 transition-all duration-300 shadow-xl"
  >
    <div
      class="w-20 h-20 rounded-2xl bg-accent/10 text-accent flex items-center justify-center mb-6 group-hover:scale-110 group-hover:-rotate-2 transition-transform duration-500 shadow-lg shadow-accent/5"
    >
      <IconNotes size={40} stroke={2} />
    </div>
    <h3
      class="text-xl font-bold text-text-primary mb-2 group-hover:text-accent transition-colors"
    >
      {$_("documents_mode.my_notes") || "Mis notas"}
    </h3>
    <p class="text-text-muted text-sm max-w-xs">
      {$_("documents_mode.my_notes_desc") ||
        "Accede a tus notas cifradas guardadas."}
    </p>
    <div
      class="absolute inset-0 border-2 border-accent/0 group-hover:border-accent/20 rounded-3xl transition-all duration-500 pointer-events-none"
    ></div>
  </button>

  <!-- Importar documento -->
  <button
    onclick={docState.startExisting}
    class="group relative flex flex-col items-center text-center p-8 bg-surface/4 backdrop-blur-2xl rounded-3xl border border-surface/8 hover:border-accent/30 hover:bg-surface/10 transition-all duration-300 shadow-xl"
  >
    <div
      class="w-20 h-20 rounded-2xl bg-accent/10 text-accent flex items-center justify-center mb-6 group-hover:scale-110 group-hover:-rotate-3 transition-transform duration-500 shadow-lg shadow-accent/5"
    >
      <IconFileImport size={40} stroke={2} />
    </div>
    <h3
      class="text-xl font-bold text-text-primary mb-2 group-hover:text-accent transition-colors"
    >
      {$_("documents.save_existing_card")}
    </h3>
    <p class="text-text-muted text-sm max-w-xs">
      {$_("documents.save_existing_desc")}
    </p>
    <div
      class="absolute inset-0 border-2 border-accent/0 group-hover:border-accent/20 rounded-3xl transition-all duration-500 pointer-events-none"
    ></div>
  </button>
</div>
