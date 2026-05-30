<script lang="ts">
  import { _ } from "svelte-i18n";
  import { getContext } from "svelte";
  import { IconPlus, IconNotes } from "@tabler/icons-svelte";
  import { DOCUMENTS_CTX, type DocumentsContext } from "./context";
  import Card from "$lib/components/core/primitives/Card.svelte";

  const docState = getContext<DocumentsContext>(DOCUMENTS_CTX);
</script>

<div class="grid grid-cols-3 gap-4">
  <!-- Nueva nota -->
  <Card
    title={$_("documents_mode.create_note")}
    description={$_("documents_mode.create_note_desc")}
    icon={IconPlus}
    onclick={docState.startNew}
  />

  <!-- Mis notas -->
  <Card
    title={$_("documents_mode.my_notes") || "Mis notas"}
    description={$_("documents_mode.my_notes_desc") || "Accede a tus notas cifradas guardadas."}
    icon={IconNotes}
    onclick={docState.startList}
  />
</div>

