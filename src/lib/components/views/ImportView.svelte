<script lang="ts">
  import { uiState, dataState } from "$lib/stores/stores.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { toast } from "$lib/stores/toast.svelte";
  import { IconDatabaseImport, IconFileImport, IconLoader2 } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { open } from "@tauri-apps/plugin-dialog";
  import { addAccount, readExternalFile, getAccounts } from "$lib/utils/tauri";
  import { parseCsv } from "$lib/utils/csvImporter";

  let isImporting = $state(false);

  async function handleExternalImport() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "CSV", extensions: ["csv"] }]
      });
      if (!selected) return;

      isImporting = true;
      const content = await readExternalFile(selected as string);

      // Toda la lógica de conversión y normalización está en csvImporter.ts
      const { credentials, skipped, detectedFormat } = parseCsv(content);

      if (credentials.length === 0) {
        toast.error("No se encontraron contraseñas válidas en el archivo.");
        return;
      }

      for (const cred of credentials) {
        await addAccount(cred.site, cred.username, cred.password, "", cred.notes);
      }

      dataState.accounts = await getAccounts();

      const skippedMsg = skipped > 0 ? ` (${skipped} filas sin contraseña ignoradas)` : "";
      toast.success(
        `Se importaron ${credentials.length} cuentas desde ${detectedFormat}.${skippedMsg}`
      );
      uiState.activeView = "accounts";
    } catch (e: any) {
      toast.error(`Error en importación: ${e.message || e}`);
    } finally {
      isImporting = false;
    }
  }
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10"
>
  <div class="mb-8">
    <h1
      class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent"
    >
      {$_("actions.import")}
    </h1>
    <p class="text-text-muted text-sm mt-1">
      Selecciona el tipo de importación.
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
    <!-- Card Importar DB Bóveda -->
    <button
      class="flex flex-col items-start text-left p-6 gap-4 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl hover:border-accent/30 hover:bg-surface/7 transition-all group"
      onclick={() => modal.openImportPackage()}
    >
      <div
        class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform"
      >
        <IconDatabaseImport size={24} />
      </div>
      <div>
        <h3 class="text-lg font-semibold text-text-primary mb-1">
          Importar DB Bóveda
        </h3>
        <p class="text-sm text-text-muted">
          Restaura una copia de seguridad previamente exportada.
        </p>
      </div>
    </button>

    <!-- Card Importación Externa -->
    <button
      class="flex flex-col items-start text-left p-6 gap-4 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl hover:border-accent/30 hover:bg-surface/7 transition-all group disabled:opacity-50 disabled:cursor-not-allowed"
      onclick={handleExternalImport}
      disabled={isImporting}
    >
      <div
        class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform"
      >
        {#if isImporting}
          <IconLoader2 size={24} class="animate-spin" />
        {:else}
          <IconFileImport size={24} />
        {/if}
      </div>
      <div>
        <h3 class="text-lg font-semibold text-text-primary mb-1">
          Importación Externa
        </h3>
        <p class="text-sm text-text-muted">
          Importa contraseñas desde un archivo CSV (ej. Google Chrome).
        </p>
      </div>
    </button>
  </div>
</div>
