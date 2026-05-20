<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock, IconEye, IconEyeOff, IconDownload} from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  import Modal from "../../core/primitives/Modal.svelte";
  import Button from "../../core/primitives/Button.svelte";
  
  import { invoke } from "@tauri-apps/api/core";
  import { save } from "@tauri-apps/plugin-dialog";
  import { toast } from "$lib/stores/toast.svelte";
  import { get } from "svelte/store";

  let { onconfirm, oncancel } = $props<{
    onconfirm: (password: string) => void;
    oncancel: () => void;
  }>();

  let password = $state("");
  let showPassword = $state(false);
  let error = $state("");
  let loading = $state(false);

  async function handleExportSecure() {
    const t = get(_);
    loading = true;
    try {
      const filePath = await save({
        title: t("export_pack.title"),
        defaultPath: "Boveda_Export.bvda.pack",
        filters: [
          { name: "Bóveda Secure Package", extensions: ["pack", "bvda.pack"] },
        ],
      });

      if (filePath) {
        await invoke("export_secure_package", {
          destPath: filePath,
          password: password,
        });
        toast.success(
          t("export_pack.success", { values: { path: filePath } }),
        );
        onconfirm(password);
      }
    } catch (e: any) {
      console.error("Export failed:", e);
      toast.error(t("export_pack.error") + ": " + e.toString());
    } finally {
      loading = false;
    }
  }

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (password.length < 8) {
      error = $_("export_pack.password_error");
      return;
    }
    handleExportSecure();
  }
</script>

<Modal 
  show={true} 
  onclose={oncancel} 
  title={$_("export_pack.title")}
>
  <div class="space-y-6">
    <div class="bg-accent/5 border border-accent/10 rounded-xl p-4">
      <p class="text-xs text-text-secondary leading-relaxed">
        {$_("export_pack.warning")}
      </p>
    </div>

    <form onsubmit={handleSubmit} class="space-y-4" id="export-form">
      <div class="space-y-2">
        <label for="export-pw" class="text-xs font-bold text-text-muted uppercase tracking-wider">
          {$_("export_pack.password_label")}
        </label>
        <div class="relative">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted">
            <IconLock size={18} />
          </div>
          <input
            id="export-pw"
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
            {error}
          </p>
        {/if}
      </div>
    </form>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={oncancel}>
      {$_("actions.cancel")}
    </Button>
    <Button type="submit" form="export-form" class="gap-2" disabled={loading}>
      {#if loading}
        <span class="w-3 h-3 border-2 border-surface/30 border-t-white rounded-full animate-spin mr-1.5"></span>
        {$_("actions.status.encrypting")}
      {:else}
        <IconDownload size={18} />
        {$_("actions.export")}
      {/if}
    </Button>
  {/snippet}
</Modal>
