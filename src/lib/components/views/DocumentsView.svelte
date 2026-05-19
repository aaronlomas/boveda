<script lang="ts">
  import { _ } from "svelte-i18n";
  import {
    IconLock,
    IconPlus,
    IconFileImport,
    IconArrowLeft,
  } from "@tabler/icons-svelte";
  import SaveNoteModal from "../modals/forms/SaveNoteModal.svelte";
  import BoardEditor from "../board/BoardEditor.svelte";

  let content = $state("");
  let currentView: "selection" | "editor" | "import" | "list" =
    $state("selection");
  let showSaveModal = $state(false);

  function startNew() {
    content = "";
    currentView = "editor";
  }

  function startExisting() {
    currentView = "import";
  }

  function startList() {
    currentView = "list";
  }

  function startSave() {
    showSaveModal = true;
  }

  function handleSave(title: string, description: string) {
    console.log("Saving note:", { title, description, content });
    showSaveModal = false;
    // Aquí irá la lógica de cifrado real
  }

  function goBack() {
    currentView = "selection";
  }
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10"
>
  <div class="mb-8 flex items-center justify-between">
    <div>
      <h1
        class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent"
      >
        {$_("documents_mode.title")}
      </h1>
      <p class="text-text-muted text-sm mt-1">
        {$_("documents_mode.desc")}
      </p>
    </div>
    {#if currentView !== "selection"}
      <button
        onclick={goBack}
        class="flex items-center gap-2 text-text-secondary hover:text-text-primary transition-colors bg-surface/10 px-4 py-2 rounded-xl border border-surface/20"
      >
        <IconArrowLeft size={18} />
        {$_("actions.back")}
      </button>
    {/if}
  </div>

  {#if currentView === "selection"}
    <div
      class="grid grid-cols-1 md:grid-cols-2 gap-6 animate-in fade-in zoom-in-95 duration-500"
    >
      <button
        onclick={startNew}
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

      <button
        onclick={startExisting}
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
  {:else if currentView === "import"}
    <div
      class="flex flex-col items-center justify-center py-24 px-5 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl animate-in fade-in slide-in-from-bottom-4 duration-500"
    >
      <div
        class="w-20 h-20 rounded-3xl bg-accent/10 text-accent border border-accent/20 flex items-center justify-center mb-6 shadow-lg shadow-accent/5"
      >
        <IconFileImport size={40} />
      </div>
      <h3 class="text-2xl text-text-primary font-bold mb-3">
        Importar documento
      </h3>
      <p class="text-text-muted max-w-md text-center mb-10 leading-relaxed">
        Selecciona un archivo de tu equipo para cifrarlo y guardarlo de forma
        segura en tu bóveda. Soportamos documentos de texto, PDF e imágenes.
      </p>
      <button
        class="px-10 py-4 rounded-2xl bg-accent text-white font-bold hover:bg-accent-light hover:-translate-y-1 hover:shadow-2xl hover:shadow-accent/30 active:translate-y-0 transition-all flex items-center gap-3 text-lg"
      >
        <IconFileImport size={24} />
        Seleccionar archivo
      </button>
    </div>
  {:else if currentView === "list"}
    <div
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 animate-in fade-in slide-in-from-bottom-4 duration-500"
    >
      {#each Array(6) as _, i}
        <div
          class="p-6 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 hover:border-accent/30 transition-all group cursor-pointer shadow-xl relative overflow-hidden"
        >
          <div class="flex items-center justify-between mb-5">
            <div
              class="w-12 h-12 rounded-xl bg-accent/10 text-accent flex items-center justify-center shadow-inner"
            >
              <IconLock size={24} />
            </div>
            <span
              class="text-[10px] text-accent font-mono uppercase tracking-widest bg-accent/5 border border-accent/10 px-2.5 py-1 rounded-full"
            >
              Cifrado
            </span>
          </div>
          <h4
            class="text-text-primary font-bold text-lg mb-2 group-hover:text-accent transition-colors"
          >
            Nota Secreta {i + 1}
          </h4>
          <p class="text-text-muted text-sm line-clamp-2 leading-relaxed">
            Este es un resumen del contenido protegido que se encuentra
            almacenado de forma segura en tu base de datos local...
          </p>
          <div
            class="mt-6 pt-4 border-t border-surface/8 flex items-center justify-between"
          >
            <span class="text-xs text-text-muted/60 font-medium"
              >Modificado hace {i + 1}h</span
            >
            <button
              class="text-accent text-sm font-bold hover:text-accent-light transition-colors flex items-center gap-1"
            >
              Abrir
              <IconArrowLeft
                size={14}
                class="rotate-180 group-hover:translate-x-0.5 transition-transform"
              />
            </button>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <BoardEditor bind:content onviewnotes={startList} onsave={startSave} />
  {/if}
</div>

{#if showSaveModal}
  <SaveNoteModal onclose={() => (showSaveModal = false)} onsave={handleSave} />
{/if}

<style>
</style>
