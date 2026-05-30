<script lang="ts">
  import { uiState, dataState } from "$lib/stores/stores.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { toast } from "$lib/stores/toast.svelte";
  import {
    IconDatabaseImport,
    IconFileImport,
    IconLoader2,
    IconCopy,
    IconReplace,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { open } from "@tauri-apps/plugin-dialog";
  import { addAccount, readExternalFile, getAccounts, deleteAccount } from "$lib/utils/tauri";
  import { parseCsv } from "$lib/utils/csvImporter";
  import Card from "$lib/components/core/primitives/Card.svelte";
  import Modal from "$lib/components/core/primitives/Modal.svelte";
  import Button from "$lib/components/core/primitives/Button.svelte";

  let isImporting = $state(false);
  let showStrategyModal = $state(false);
  let strategy = $state<"merge" | "replace">("merge");

  function openExternalImport() {
    showStrategyModal = true;
  }

  async function handleExternalImport() {
    showStrategyModal = false;
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });
      if (!selected) return;

      isImporting = true;
      const content = await readExternalFile(selected as string);

      const { credentials, skipped, detectedFormat } = parseCsv(content);

      if (credentials.length === 0) {
        toast.error($_("import_view.error_no_passwords"));
        return;
      }

      // If replace: delete all existing accounts first
      if (strategy === "replace") {
        const existing = await getAccounts();
        for (const acc of existing) {
          await deleteAccount(acc.id);
        }
      }

      for (const cred of credentials) {
        await addAccount(cred.site, cred.username, cred.password, "", cred.notes);
      }

      dataState.accounts = await getAccounts();

      const msg =
        skipped > 0
          ? $_("import_view.success_skipped", {
              values: { count: credentials.length, format: detectedFormat, skipped },
            })
          : $_("import_view.success", {
              values: { count: credentials.length, format: detectedFormat },
            });
      toast.success(msg);
      uiState.activeView = "accounts";
    } catch (e: any) {
      toast.error($_("import_view.error_generic", { values: { message: e.message || e } }));
    } finally {
      isImporting = false;
    }
  }
</script>

<div class="grid gap-6">
  <div>
    <h1
      class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent"
    >
      {$_("actions.import")}
    </h1>
    <p class="text-text-muted text-sm">
      {$_("import_view.subtitle")}
    </p>
  </div>

  <div class="grid grid-cols-3 gap-4">
    <!-- Card Importar DB Bóveda -->
    <Card
      title={$_("import_view.boveda_db_title")}
      description={$_("import_view.boveda_db_desc")}
      icon={IconDatabaseImport}
      onclick={() => modal.openImportPackage()}
    />

    <!-- Card Importación Externa -->
    <Card
      title={$_("import_view.external_title")}
      description={$_("import_view.external_desc")}
      onclick={openExternalImport}
      disabled={isImporting}
    >
      {#snippet iconSnippet()}
        {#if isImporting}
          <IconLoader2 size={24} class="animate-spin" />
        {:else}
          <IconFileImport size={24} />
        {/if}
      {/snippet}
    </Card>
  </div>
</div>

<!-- Strategy selector modal for external import -->
{#if showStrategyModal}
  <Modal
    show={true}
    onclose={() => (showStrategyModal = false)}
    title={$_("import_view.external_title")}
  >
    <div class="space-y-4">
      <div class="grid grid-cols-2 gap-2">
        <button
          type="button"
          class="flex flex-col items-center gap-2 p-3 rounded-xl border transition-all text-center {strategy === 'merge'
            ? 'bg-accent/10 border-accent text-accent'
            : 'bg-surface/5 border-surface/10 text-text-muted hover:bg-surface/10'}"
          onclick={() => (strategy = "merge")}
        >
          <IconCopy size={24} />
          <div class="flex flex-col">
            <span class="text-sm font-bold">{$_("import_pack.keep_both")}</span>
            <span class="text-xs opacity-70">{$_("import_pack.keep_both_desc")}</span>
          </div>
        </button>

        <button
          type="button"
          class="flex flex-col items-center gap-2 p-3 rounded-xl border transition-all text-center {strategy === 'replace'
            ? 'bg-danger/10 border-danger text-danger'
            : 'bg-surface/5 border-surface/10 text-text-muted hover:bg-surface/10'}"
          onclick={() => (strategy = "replace")}
        >
          <IconReplace size={24} />
          <div class="flex flex-col">
            <span class="text-sm font-bold">{$_("actions.replace")}</span>
            <span class="text-xs opacity-70">{$_("import_pack.replace_desc")}</span>
          </div>
        </button>
      </div>
    </div>

    {#snippet footer()}
      <Button variant="ghost" onclick={() => (showStrategyModal = false)}>
        {$_("actions.cancel")}
      </Button>
      <Button onclick={handleExternalImport}>
        <IconFileImport size={18} class="mr-1.5" />
        {$_("actions.import")}
      </Button>
    {/snippet}
  </Modal>
{/if}
