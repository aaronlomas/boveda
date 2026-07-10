<script lang="ts">
  import { _ } from "svelte-i18n";
  import {
    IconInfoCircle,
    IconAlertTriangle,
    IconTrash,
    IconEye,
    IconEyeOff,
    IconLock,
  } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Button from "../../../core/primitives/Button.svelte";
  import Modal from "../../../core/primitives/Modal.svelte";
  import ListItem from "../../../core/primitives/ListItem.svelte";
  import Input from "../../../core/primitives/Input.svelte";
  import { toast } from "$lib/stores/toast.svelte";
  import { sessionState } from "$lib/stores/stores.svelte";
  import { check, type Update } from "@tauri-apps/plugin-updater";

  let appInfo = $state({ app_version: "...", core_version: "..." });

  let showDeleteConfirm = $state(false);
  let password = $state("");
  let showPassword = $state(false);
  let loadingDelete = $state(false);
  let errorDelete = $state("");

  let updateState = $state<
    "idle" | "checking" | "available" | "downloading" | "done"
  >("idle");
  let updateData = $state<Update | null>(null);

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
      toast.success($_("settings.account_system.delete_success"));
      showDeleteConfirm = false;
      sessionState.isUnlocked = false; // Redirects to login
    } catch (e: any) {
      errorDelete = e.toString();
    } finally {
      loadingDelete = false;
    }
  }

  async function handleUpdater() {
    if (updateState === "idle") {
      updateState = "checking";
      try {
        const update = await check();
        if (update) {
          updateData = update;
          updateState = "available";
        } else {
          toast.success($_("settings.updater.update_status"));
          updateState = "idle";
        }
      } catch (err) {
        console.error(err);
        toast.error($_("settings.updater.update_error"));
        updateState = "idle";
      }
    } else if (updateState === "available" && updateData) {
      updateState = "downloading";
      try {
        await updateData.downloadAndInstall();
        updateState = "done";
        toast.success($_("settings.updater.update_ready"));
      } catch (err: any) {
        console.error("Update error:", err);
        // Fallback for unsupported platforms (like .deb on Linux)
        if (err.toString().toLowerCase().includes("unsupported")) {
          toast.info(
            "Update available. Please run: 'sudo apt update && sudo apt install boveda'",
          );
        } else {
          toast.error("Update failed: " + err.toString());
        }
        updateState = "idle";
      }
    }
  }
</script>

<div class="grid gap-4">
  <!-- About: Title and description -->

  <div>
    <h2 class="text-xl font-bold text-text-primary flex items-center gap-2">
      <IconInfoCircle class="text-accent" />
      {$_("settings.about.title")}
    </h2>
    <p class="text-text-muted text-xs mt-1">
      {$_("settings.about.desc")}
    </p>
  </div>

  <!-- Items -->

  <div class="flex justify-between items-center border-b border-surface/8">
    <span class="text-sm text-text-secondary"
      >{$_("settings.about.version_label")}</span
    >
    <span class="text-sm font-mono text-accent">v{appInfo.app_version}</span>
  </div>

  <div class="flex justify-between items-center border-b border-surface/8">
    <span class="text-sm text-text-secondary"
      >{$_("settings.about.core_label")}</span
    >
    <span class="text-sm font-mono text-accent">v{appInfo.core_version}</span>
  </div>

  <!-- updater -->

  <h1 class="text-text-primary text-xl font-bold border-b border-surface/8">
    {$_("settings.updater.title")}
  </h1>
  <ListItem layout="double" flush={true}>
    <div>
      <p class="text-sm">{$_("settings.updater.sub_title")}</p>
      <p class="text-xs text-text-muted">
        {#if updateState === "available" && updateData}
          Versión {updateData.version} disponible
        {:else}
          {$_("settings.updater.desc")}
        {/if}
      </p>
    </div>
    <Button
      variant={updateState === "available" ? "primary" : "secondary"}
      onclick={handleUpdater}
      disabled={updateState === "checking" ||
        updateState === "downloading" ||
        updateState === "done"}
    >
      {#if updateState === "idle"}
        {$_("actions.search")}
      {:else if updateState === "checking"}
        <span
          class="w-3 h-3 border-2 border-text-primary/30 border-t-text-primary rounded-full animate-spin mr-1.5"
        ></span>
        Buscando...
      {:else if updateState === "available"}
        {$_("actions.update")}
      {:else if updateState === "downloading"}
        <span
          class="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin mr-1.5"
        ></span>
        Actualizando...
      {:else if updateState === "done"}
        Listo
      {/if}
    </Button>
  </ListItem>

  <!-- Account System -->

  <h1 class="text-text-primary text-xl font-bold border-b border-surface/8">
    {$_("settings.account_system.title")}
  </h1>
  <ListItem layout="double" flush={true}>
    <div>
      <h3 class="text-sm font-medium text-danger">
        {$_("settings.account_system.delete_account")}
      </h3>
      <p class="text-xs text-text-muted">
        {$_("settings.account_system.delete_account_desc")}
      </p>
    </div>
    <Button variant="danger" onclick={() => (showDeleteConfirm = true)}>
      {$_("actions.delete")}
    </Button>
  </ListItem>
</div>

{#if showDeleteConfirm}
  <Modal
    show={true}
    onclose={() => (showDeleteConfirm = false)}
    title={$_("settings.account_system.delete_account")}
  >
    <div class="space-y-6">
      <div
        class="bg-danger/10 border border-danger/20 rounded-xl p-4 flex gap-3"
      >
        <IconAlertTriangle class="text-danger shrink-0" size={20} />
        <p class="text-xs text-danger/90 leading-relaxed">
          {$_("settings.account_system.delete_confirm")}
        </p>
      </div>

      <form
        id="delete-vault-form"
        onsubmit={handleDeleteVault}
        class="space-y-4"
      >
        <div class="space-y-2">
          <Input
            id="del-pw"
            type={showPassword ? "text" : "password"}
            label={$_("unlock_screen.master_password_label")}
            bind:value={password}
            variant="triple"
            required
            autofocus
          >
            {#snippet icon()}
              <IconLock size={18} class="text-text-muted" />
            {/snippet}
            {#snippet action()}
              <button
                type="button"
                class="text-text-muted hover:text-text-primary transition-colors flex items-center justify-center"
                onclick={() => (showPassword = !showPassword)}
              >
                {#if showPassword}
                  <IconEyeOff size={18} />
                {:else}
                  <IconEye size={18} />
                {/if}
              </button>
            {/snippet}
          </Input>
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
