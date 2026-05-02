<script lang="ts">
  import { isUnlocked } from "$lib/stores";
  import { unlockVault, isVaultInitialized } from "$lib/tauri";
  import { onMount } from "svelte";
  import {
    IconEye,
    IconEyeOff,
    IconSettings,
    IconLock,
    IconShieldHalfFilled,
    IconDatabaseImport
  } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { _ } from "svelte-i18n";
  import PreferencesModal from "./PreferencesModal.svelte";

  let showPreferences = false;

  const version = import.meta.env.APP_VERSION;

  let password = "";
  let error = "";
  let loading = false;
  let isNew = false;
  let confirmPassword = "";
  let showPassword = false;
  let showConfirmPassword = false;

  onMount(async () => {
    try {
      isNew = !(await isVaultInitialized());
    } catch {}
  });

  async function submit() {
    error = "";
    if (!password) {
      error = $_("unlock_screen.error_empty");
      return;
    }
    if (isNew && password !== confirmPassword) {
      error = $_("unlock_screen.error_mismatch");
      return;
    }

    loading = true;
    try {
      await unlockVault(password);
      isUnlocked.set(true);
    } catch (e: any) {
      error = $_("unlock_screen.error_incorrect");
    } finally {
      loading = false;
    }
  }

  async function handleImport() {
    try {
      const filePath = await open({
        title: $_("global.select_db_title"),
        filters: [{ name: $_("global.db_filter_name"), extensions: ["db"] }],
      });
      if (filePath) {
        await invoke("import_db", { srcPath: filePath });
      }
    } catch (e) {
      console.error("Import failed:", e);
      alert($_("global.error_import"));
    }
  }
</script>

<div class="min-h-screen grid grid-rows-[auto_1fr_auto]">
  <header
    class=" bg-panel/30 rounded-2xl px-6 py-4 flex items-center justify-between m-2"
  >
    <div
      class="text-xl font-bold text-text-primary tracking-tight pointer-events-none"
    >
      Bóveda
    </div>

    <div class="flex items-center gap-3">
      <button
        class="flex items-center gap-2 bg-surface/5 border border-surface/5 text-text-primary text-xs cursor-pointer py-2 px-4 rounded-lg transition-all hover:bg-surface/10 backdrop-blur-2xl"
        type="button"
        on:click={handleImport}
      >
        <IconDatabaseImport size={16} />
        <span>{$_("unlock_screen.header_import")}</span>
      </button>
      <button
        class="flex items-center gap-2 bg-surface/5 border border-surface/5 text-text-primary text-xs cursor-pointer py-2 px-4 rounded-lg transition-all hover:bg-surface/10 backdrop-blur-2xl"
        type="button"
        on:click={() => (showPreferences = true)}
      >
        <IconSettings size={16} />
        <span>{$_("unlock_screen.header_settings")}</span>
      </button>
    </div>
  </header>

  <div
    class="max-w-90 m-auto p-8 flex flex-col items-center gap-2 bg-panel/30 backdrop-blur-2xl rounded-2xl border border-surface/8"
  >
    <div class="flex mb-4">
      <!-- <img
        src="/boveda.svg"
        alt="Logo"
        width="72"
        height="72"
      /> -->
      <IconShieldHalfFilled size={72} />
    </div>
    <p
      class="text-text-primary text-sm text-center mb-6 max-w-70 pointer-events-none"
    >
      {isNew
        ? $_("unlock_screen.new_vault_desc")
        : $_("unlock_screen.unlock_vault_desc")}
    </p>

    <form on:submit|preventDefault={submit} class="w-full flex flex-col gap-4">
      <div class="flex flex-col gap-1.5">
        <label for="master-pw" class="text-xs text-text-primary"
          >{$_("unlock_screen.master_password_label")}</label
        >

        <div
          class="flex border border-surface/10 rounded-lg px-4 py-2 gap-x-2 bg-transparent"
        >
          <input
            id="master-pw"
            class="w-full border-0 text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:bg-transparent tracking-widest"
            type={showPassword ? "text" : "password"}
            bind:value={password}
            placeholder={$_("unlock_screen.placeholder")}
            autocomplete="current-password"
          />
          <button
            type="button"
            class="bg-none border-none cursor-pointer text-text-muted hover:text-text-primary transition-all flex items-center"
            on:click={() => (showPassword = !showPassword)}
          >
            {#if showPassword}
              <IconEyeOff size={18} />
            {:else}
              <IconEye size={18} />
            {/if}
          </button>
        </div>
      </div>

      {#if isNew}
        <div class="flex flex-col gap-1.5">
          <label for="confirm-pw" class="text-xs text-text-primary"
            >{$_("unlock_screen.confirm_password_label")}</label
          >

          <div
            class="flex border border-surface/10 rounded-lg px-4 py-2 gap-x-2 bg-transparent"
          >
            <input
              id="confirm-pw"
              class="w-full border-0 text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:bg-transparent tracking-widest"
              type={showConfirmPassword ? "text" : "password"}
              bind:value={confirmPassword}
              placeholder={$_("unlock_screen.placeholder")}
            />
            <button
              type="button"
              class="bg-none border-none cursor-pointer text-text-muted hover:text-text-primary transition-all flex items-center"
              on:click={() => (showConfirmPassword = !showConfirmPassword)}
            >
              {#if showConfirmPassword}
                <IconEyeOff size={18} />
              {:else}
                <IconEye size={18} />
              {/if}
            </button>
          </div>
        </div>
      {/if}

      {#if error}
        <p
          class="text-danger text-xs py-2 px-3 bg-danger/10 border border-danger/20 rounded-md"
        >
          {error}
        </p>
      {/if}

      <button
        type="submit"
        class="w-full justify-center py-2 px-4 text-sm font-semibold bg-accent hover:bg-accent-light text-white rounded-lg shadow-lg shadow-accent/20 transition-all active:scale-[0.98] disabled:cursor-not-allowed flex items-center gap-2 cursor-pointer"
        disabled={loading}
      >
        {#if loading}
          <span
            class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin"
          ></span>
          {$_("unlock_screen.button_processing")}
        {:else if isNew}
          {$_("unlock_screen.button_create")}
        {:else}
          <IconLock size={18} /> {$_("unlock_screen.button_unlock")}
        {/if}
      </button>
    </form>

    {#if isNew}
      <p
        class="text-xs text-warning text-center mt-2 p-2 px-3 bg-warning/10 rounded-md border border-warning/20"
      >
        {$_("unlock_screen.warning_new")}
      </p>
    {/if}
  </div>

  <footer class="mb-4 text-center text-text-muted text-[11px] tracking-wide">
    <p>
      {$_("unlock_screen.footer_rights")} -
      <span class="font-semibold text-accent-light">v{version}</span>
    </p>
  </footer>
</div>

{#if showPreferences}
  <PreferencesModal on:close={() => (showPreferences = false)} />
{/if}

<style>
  /* Tailwind handles most styles now */
</style>
