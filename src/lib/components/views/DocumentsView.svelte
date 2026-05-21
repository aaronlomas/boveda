<script lang="ts">
  import { _ } from "svelte-i18n";
  import { setContext } from "svelte";
  import { IconArrowLeft } from "@tabler/icons-svelte";
  import { DOCUMENTS_CTX } from "../features/documents/context";
  import SaveNoteModal from "../modals/forms/SaveNoteModal.svelte";
  import BoardEditor from "../features/documents/board/BoardEditor.svelte";
  import Button from "../core/primitives/Button.svelte";
  import SelectionView from "../features/documents/SelectionView.svelte";
  import ImportDocumentView from "../features/documents/ImportDocumentView.svelte";
  import NotesListView from "../features/documents/list-note/NotesListView.svelte";
  import { useDocuments } from "$lib/composables/useDocuments.svelte";

  const docState = useDocuments();
  setContext(DOCUMENTS_CTX, docState);

  // $derived ensures reactive re-evaluation when currentView changes
  const subViewMap = {
    selection: SelectionView,
    import: ImportDocumentView,
    list: NotesListView,
  } as const;

  const ActiveSubView = $derived(
    docState.currentView !== "editor"
      ? subViewMap[docState.currentView as keyof typeof subViewMap] ?? null
      : null
  );
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
    {#if docState.currentView !== "selection"}
      <Button variant="secondary" onclick={docState.goBack} class="gap-2">
        <IconArrowLeft size={18} />
        {$_("actions.back")}
      </Button>
    {/if}
  </div>

  {#if docState.currentView === "editor"}
    <!-- Editor view — needs bind:content and direct event handlers -->
    <BoardEditor
      bind:content={docState.content}
      onviewnotes={docState.startList}
      onsave={docState.startSave}
    />
  {:else if ActiveSubView}
    <ActiveSubView />
  {/if}
</div>

{#if docState.showSaveModal}
  <SaveNoteModal
    onclose={() => (docState.showSaveModal = false)}
    onsave={docState.handleSave}
  />
{/if}
