<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock, IconTrash, IconEdit, IconLoader2, IconNotes, IconPlus } from "@tabler/icons-svelte";
  import Button from "$lib/components/core/primitives/Button.svelte";
  import { getContext } from "svelte";
  import { DOCUMENTS_CTX, type DocumentsContext } from "../context";

  const docState = getContext<DocumentsContext>(DOCUMENTS_CTX);

  /** Formats an ISO date string as a relative label (today / yesterday / date). */
  function formatRelative(iso: string): string {
    const d = new Date(iso);
    const now = new Date();
    const diffMs = now.getTime() - d.getTime();
    const diffMin = Math.round(diffMs / 60000);
    if (diffMin < 1) return "Justo ahora";
    if (diffMin < 60) return `Hace ${diffMin} min`;
    const diffH = Math.round(diffMin / 60);
    if (diffH < 24) return `Hace ${diffH}h`;
    const diffD = Math.round(diffH / 24);
    if (diffD === 1) return "Ayer";
    return d.toLocaleDateString();
  }
</script>

{#if docState.loading}
  <div class="flex justify-center items-center py-24">
    <IconLoader2 size={36} class="animate-spin text-accent" />
  </div>
{:else if docState.documents.length === 0}
  <div
    class="flex flex-col items-center justify-center py-28 text-center animate-in fade-in duration-500"
  >
    <div
      class="w-20 h-20 rounded-2xl bg-surface/10 text-text-muted flex items-center justify-center mb-5"
    >
      <IconNotes size={40} />
    </div>
    <h3 class="text-xl font-bold text-text-primary mb-2">
      {$_("documents.empty_title") || "Ninguna nota guardada"}
    </h3>
    <p class="text-text-muted text-sm max-w-xs">
      {$_("documents.empty_desc") || "Crea tu primera nota cifrada usando el editor."}
    </p>
    <Button variant="primary" size="md" onclick={docState.startNew} class="mt-6 gap-2">
      <IconPlus size={18} />
      {$_("documents_mode.create_note")}
    </Button>
  </div>
{:else}
  <div
    class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 animate-in fade-in slide-in-from-bottom-4 duration-500"
  >
    {#each docState.documents as doc (doc.id)}
      <div
        class="p-6 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 hover:border-accent/30 transition-all group shadow-xl relative overflow-hidden flex flex-col"
      >
        <div class="flex items-center justify-between mb-5">
          <div
            class="w-12 h-12 rounded-xl bg-accent/10 text-accent flex items-center justify-center shadow-inner"
          >
            <IconLock size={24} />
          </div>
          <span
            class="text-xs text-accent font-mono uppercase tracking-widest bg-accent/5 border border-accent/10 px-2.5 py-1 rounded-full"
          >
            Cifrado
          </span>
        </div>
        <h4
          class="text-text-primary font-bold text-lg mb-2 group-hover:text-accent transition-colors line-clamp-1"
        >
          {doc.title}
        </h4>
        <p class="text-text-muted text-sm line-clamp-2 leading-relaxed flex-1">
          {#if doc.encrypted_description}
            <span class="italic opacity-60">descripción cifrada</span>
          {:else}
            <span class="opacity-40">Sin descripción</span>
          {/if}
        </p>
        <div
          class="mt-6 pt-4 border-t border-surface/8 flex items-center justify-between"
        >
          <span class="text-xs text-text-muted/60 font-medium">
            {formatRelative(doc.updated_at)}
          </span>
          <div class="flex items-center gap-2">
            <button
              onclick={() => docState.removeDocument(doc.id)}
              class="text-text-muted/40 hover:text-danger text-sm transition-colors p-1 rounded-lg hover:bg-danger/10"
              title="Eliminar"
            >
              <IconTrash size={15} />
            </button>
            <button
              onclick={() => docState.openDocument(doc.id)}
              class="text-accent text-sm font-bold hover:text-accent-light transition-colors flex items-center gap-1"
            >
              <IconEdit size={14} />
              Abrir
            </button>
          </div>
        </div>
      </div>
    {/each}
  </div>
{/if}
