<script lang="ts">
  import { _ } from "svelte-i18n";
  import { 
    IconLock, 
    IconEye, 
    IconEyeOff, 
    IconDatabaseImport,
    IconCopy,
    IconReplace
  } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";
  
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/stores/toast.svelte";
  import { get } from "svelte/store";
  
  let { onconfirm, oncancel } = $props<{
    onconfirm: (password: string, strategy: 'merge' | 'replace') => void;
    oncancel: () => void;
  }>();

  let password = $state("");
  let strategy = $state<'merge' | 'replace'>('merge');
  let showPassword = $state(false);
  let error = $state("");
  let loading = $state(false);

  async function handleImport() {
    const t = get(_);
    loading = true;
    try {
      const filePath = await open({
        title: $_("global.select_db_title") || "Select Database",
        filters: [
          { name: "Bóveda Vaults", extensions: ["bvda", "db", "pack", "bvda.pack"] },
        ],
      });
      
      if (!filePath) return;

      if (filePath.endsWith(".pack") || filePath.endsWith(".bvda.pack")) {
        // Secure Package Import
        try {
          await invoke("import_secure_package", { 
            srcPath: filePath, 
            password: password, 
            strategy: strategy 
          });
          toast.success(strategy === 'replace' ? t("import_pack.success_replace") : t("import_pack.success_merge"));
          onconfirm(password, strategy);
        } catch (e: any) {
          console.error("Secure import failed:", e);
          error = e.toString();
          toast.error(t("global.error_import") + ": " + e.toString());
        }
      } else {
        // Legacy DB Import (SQLite file replacement)
        try {
          await invoke("import_db", { srcPath: filePath });
          toast.success($_("sidebar.import_confirm_button") || "Base de datos importada");
          onconfirm(password, strategy);
        } catch (e: any) {
          console.error("Import failed:", e);
          error = e.toString();
          toast.error(t("global.error_import") + ": " + e.toString());
        }
      }
    } catch (e: any) {
      console.error("Import selection failed:", e);
    } finally {
      loading = false;
    }
  }

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (password.length < 8) {
      error = "La contraseña debe tener al menos 8 caracteres.";
      return;
    }
    handleImport();
  }
</script>

<Modal 
  show={true} 
  onclose={oncancel} 
  title={$_("import_pack.title")}
>
  <div class="space-y-6">

    <form onsubmit={handleSubmit} class="space-y-6" id="import-form">
      <!-- Strategy Selection -->
      <div class="space-y-3">
        
        <div class="grid grid-cols-2 gap-2">
          <button
            type="button"
            class="flex flex-col items-center gap-2 p-3 rounded-xl border transition-all text-center {strategy === 'merge' ? 'bg-accent/10 border-accent text-accent' : 'bg-surface/5 border-surface/10 text-text-muted hover:bg-surface/10'}"
            onclick={() => strategy = 'merge'}
          >
            <IconCopy size={24} />
            <div class="flex flex-col">
              <span class="text-sm font-bold">{$_("import_pack.keep_both")}</span>
              <span class="text-xs opacity-70">{$_("import_pack.keep_both_desc")}</span>
            </div>
          </button>

          <button
            type="button"
            class="flex flex-col items-center gap-2 p-3 rounded-xl border transition-all text-center {strategy === 'replace' ? 'bg-danger/10 border-danger text-danger' : 'bg-surface/5 border-surface/10 text-text-muted hover:bg-surface/10'}"
            onclick={() => strategy = 'replace'}
          >
            <IconReplace size={24} />
            <div class="flex flex-col">
              <span class="text-sm font-bold">{$_("actions.replace")}</span>
              <span class="text-xs opacity-70">{$_("import_pack.replace_desc")}</span>
            </div>
          </button>
        </div>
      </div>

      <!-- Password Field -->
      <div class="space-y-2">
        <label for="import-pw" class="text-xs font-bold text-text-muted uppercase tracking-wider">
          {$_("import_pack.password_label")}
        </label>
        <div class="relative">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted">
            <IconLock size={18} />
          </div>
          <input
            id="import-pw"
            type={showPassword ? "text" : "password"}
            bind:value={password}
            placeholder="••••••••••••"
            class="w-full bg-surface/5 border border-surface/10 rounded-lg pl-10 pr-12 py-3 text-text-primary placeholder:text-text-muted/30 focus:outline-none focus:border-accent transition-colors text-sm"
            required
            use:focus
          />
          <button
            type="button"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-text-muted hover:text-text-primary transition-colors"
            onclick={() => (showPassword = !showPassword)}
          >
            {#if showPassword}
              <IconEyeOff size={18} />
            {:else}
              <IconEye size={18} />
            {/if}
          </button>
        </div>
        {#if error}
          <p class="text-xs text-danger font-medium animate-in fade-in slide-in-from-top-1">
            {$_("import_pack.password_error")}
          </p>
        {/if}
      </div>
    </form>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={oncancel}>
      {$_("actions.cancel")}
    </Button>
    <Button type="submit" form="import-form" class="gap-2" disabled={loading}>
      {#if loading}
        <span class="w-3 h-3 border-2 border-surface/30 border-t-white rounded-full animate-spin mr-1.5"></span>
        {$_("actions.status.decryption")}
      {:else}
        <IconDatabaseImport size={18} />
        {$_("actions.import")}
      {/if}
    </Button>
  {/snippet}
</Modal>
