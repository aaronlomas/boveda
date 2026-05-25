<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconFileImport } from "@tabler/icons-svelte";
  import Button from "../../core/primitives/Button.svelte";
  import { getContext } from "svelte";
  import { DOCUMENTS_CTX, type DocumentsContext } from "./context";
  import { toast } from "$lib/stores/toast.svelte";

  const docState = getContext<DocumentsContext>(DOCUMENTS_CTX);

  let fileInput: HTMLInputElement | undefined = $state();

  function triggerSelect() {
    fileInput?.click();
  }

  async function handleFileSelected(e: Event) {
    const target = e.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;

    const file = target.files[0];
    
    try {
      const text = await file.text();
      docState.startImported(text);
    } catch (err) {
      console.error(err);
      toast.error($_("documents.read_error") || "Error reading file");
    }
  }
</script>

<div
  class="flex flex-col items-center justify-center py-24 px-5 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl animate-in fade-in slide-in-from-bottom-4 duration-500"
>
  <div
    class="w-20 h-20 rounded-3xl bg-accent/10 text-accent border border-accent/20 flex items-center justify-center mb-6 shadow-lg shadow-accent/5"
  >
    <IconFileImport size={40} />
  </div>
  <h3 class="text-2xl text-text-primary font-bold mb-3">
    {$_("documents.import_document") || "Import document"}
  </h3>
  <p class="text-text-muted max-w-md text-center mb-10 leading-relaxed">
    {$_("documents.import_document_desc")}
  </p>
  <input 
    type="file" 
    bind:this={fileInput} 
    onchange={handleFileSelected} 
    class="hidden" 
    accept=".txt,.md,.json,.csv,.log,.html,.xml,.yml,.yaml,.toml" 
  />

  <Button variant="primary" size="lg" class="gap-3 font-bold" onclick={triggerSelect}>
    <IconFileImport size={24} />
    {$_("actions.select") || "Select file"}
  </Button>
</div>
