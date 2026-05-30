<script lang="ts">
  import { _ } from "svelte-i18n";
  import {
    IconInfoCircle,
    IconAlertTriangle,
    IconTrash,
    IconLock,
    IconEye,
    IconEyeOff,
  } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Button from "../../../core/primitives/Button.svelte";
  import Modal from "../../../core/primitives/Modal.svelte";
  import { toast } from "$lib/stores/toast.svelte";
  import { sessionState } from "$lib/stores/stores.svelte";
  import { focus } from "$lib/utils/actions";

  let appInfo = $state({ app_version: "...", core_version: "..." });

  let showDeleteConfirm = $state(false);
  let password = $state("");
  let showPassword = $state(false);
  let loadingDelete = $state(false);
  let errorDelete = $state("");

  onMount(async () => {
    try {
      appInfo = await invoke("get_app_info");
    } catch (e) {
      console.error("Failed to fetch app info:", e);
    }
  });

  async function handleDeleteVault(e: SubmitEvent) {
    e.preventDefault();
    if (!password) {
      errorDelete = $_("validations.is_required");
      return;
    }
    loadingDelete = true;
    errorDelete = "";
    try {
      await invoke("delete_vault", { password });
      toast.success($_("settings.about.delete_success"));
      showDeleteConfirm = false;
      sessionState.isUnlocked = false; // Redirects to login
    } catch (e: any) {
      errorDelete = e.toString();
    } finally {
      loadingDelete = false;
    }
  }
</script>

<div class="space-y-6">
  <div>
    <h2 class="text-lg font-bold text-text-primary flex items-center gap-2">
      <IconInfoCircle class="text-accent" />
      {$_("settings.about.title")}
    </h2>
    <p class="text-text-muted text-sm mt-1">
      {$_("settings.about.desc")}
    </p>
  </div>

  <div class="space-y-4">
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary"
          >{$_("settings.about.version_label")}</span
        >
        <span class="text-sm font-mono text-accent">{appInfo.app_version}</span>
      </div>
    </div>

    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary"
          >{$_("settings.about.core_label")}</span
        >
        <span class="text-sm font-mono text-text-primary"
          >boveda-core v{appInfo.core_version}</span
        >
      </div>
    </div>

    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <p class="text-xs text-text-muted leading-relaxed">
        {$_("settings.about.philosophy")}
      </p>
    </div>

    <div class="pt-4 border-t border-surface/10">
      <div class="flex justify-between items-center">
        <div>
          <h3 class="text-sm font-medium text-danger">
            {$_("settings.about.delete_account")}
          </h3>
          <p class="text-xs text-text-muted mt-1">
            {$_("settings.about.delete_account_desc")}
          </p>
        </div>
        <Button variant="danger" onclick={() => (showDeleteConfirm = true)}>
          {$_("actions.delete")}
        </Button>
      </div>
    </div>
  </div>
</div>

{#if showDeleteConfirm}
  <Modal
    show={true}
    onclose={() => (showDeleteConfirm = false)}
    title={$_("settings.about.delete_account")}
  >
    <div class="space-y-6">
      <div
        class="bg-danger/10 border border-danger/20 rounded-xl p-4 flex gap-3"
      >
        <IconAlertTriangle class="text-danger shrink-0" size={20} />
        <p class="text-xs text-danger/90 leading-relaxed">
          {$_("settings.about.delete_confirm")}
        </p>
      </div>

      <form
        id="delete-vault-form"
        onsubmit={handleDeleteVault}
        class="space-y-4"
      >
        <div class="space-y-2">
          <label
            for="del-pw"
            class="text-xs font-bold text-text-muted uppercase tracking-wider"
          >
            {$_("unlock_screen.master_password_label")}
          </label>
          <div class="relative">
            <div
              class="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted"
            >
              <IconLock size={18} />
            </div>
            <input
              id="del-pw"
              type={showPassword ? "text" : "password"}
              bind:value={password}
              class="w-full bg-surface/5 border border-surface/10 rounded-lg pl-10 pr-12 py-3 text-text-primary placeholder:text-text-muted/30 focus:outline-none focus:border-danger transition-colors text-sm"
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
          {#if errorDelete}
            <p
              class="text-xs text-danger font-medium animate-in fade-in slide-in-from-top-1"
            >
              {errorDelete}
            </p>
          {/if}
        </div>
      </form>
    </div>

    {#snippet footer()}
      <Button variant="ghost" onclick={() => (showDeleteConfirm = false)}>
        {$_("actions.cancel")}
      </Button>
      <Button
        type="submit"
        form="delete-vault-form"
        variant="danger"
        disabled={loadingDelete}
      >
        {#if loadingDelete}
          <span
            class="w-3 h-3 border-2 border-danger/30 border-t-white rounded-full animate-spin mr-1.5"
          ></span>
          {$_("actions.status.deleting")}
        {:else}
          <IconTrash size={18} class="mr-1.5" />
          {$_("actions.delete")}
        {/if}
      </Button>
    {/snippet}
  </Modal>
{/if}
