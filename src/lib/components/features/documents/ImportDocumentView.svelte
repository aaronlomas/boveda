<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconFileImport } from "@tabler/icons-svelte";
  import Button from "../../core/primitives/Button.svelte";
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
  <Button variant="primary" size="lg" class="gap-3 font-bold">
    <IconFileImport size={24} />
    {$_("actions.select") || "Select file"}
  </Button>
</div>
