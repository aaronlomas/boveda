<script lang="ts">
  import { globalState } from "$lib/stores/stores.svelte";
  import { unlockVault, isVaultInitialized } from "$lib/utils/tauri";
  import { onMount } from "svelte";
  import {
    IconEye,
    IconEyeOff,
    IconLock,
    IconShieldHalfFilled,
    IconDatabaseImport,
    IconSignRight,
    IconArrowLeft,
    IconShieldCheck,
  } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { _ } from "svelte-i18n";
  import { focus } from "$lib/utils/actions";


  const version = import.meta.env.APP_VERSION;

  let password = $state("");
  let totpCode = $state("");
  let pendingTotp = $state(false);
  let error = $state("");
  let loading = $state(false);
  let isNew = $state(false);
  let confirmPassword = $state("");
  let showPassword = $state(false);
  let showConfirmPassword = $state(false);
  let cooldown = $state(0);

  onMount(async () => {
    try {
      isNew = !(await isVaultInitialized());
    } catch {}
  });

  async function submit() {
    if (cooldown > 0) return;
    error = "";
    
    if (!pendingTotp) {
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
        const status = await unlockVault(password);
        if (status === "totp_required") {
          pendingTotp = true;
        } else {
          globalState.isUnlocked = true;
        }
      } catch (e: any) {
        handleError(e);
      } finally {
        loading = false;
      }
    } else {
      if (totpCode.length !== 6) {
        error = $_("settings.security.totp_error_invalid");
        return;
      }
      loading = true;
      try {
        const valid = await invoke<boolean>("totp_check", { code: totpCode });
        if (valid) {
          globalState.isUnlocked = true;
        } else {
          error = $_("settings.security.totp_error_invalid");
        }
      } catch (e: any) {
        error = $_("settings.security.totp_error_invalid");
      } finally {
        loading = false;
      }
    }
  }

  function handleError(e: any) {
    error = $_("unlock_screen.error_incorrect");
    // Add cooldown (1-3 seconds) to prevent rapid brute force
    cooldown = Math.floor(Math.random() * 3) + 1;
    const timer = setInterval(() => {
      cooldown--;
      if (cooldown <= 0) clearInterval(timer);
    }, 1000);
  }

  function resetUnlock() {
    pendingTotp = false;
    totpCode = "";
    error = "";
    invoke("lock_vault");
  }

  async function handleImport() {
    try {
      const filePath = await open({
        title: $_("global.select_db_title"),
        filters: [
          { name: $_("global.db_filter_name"), extensions: ["bvda", "db"] },
        ],
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

<div class="h-full grid grid-rows-[auto_1fr_auto] dark">
  <header
    class=" bg-panel/30 px-6 py-4 flex items-center justify-between m-2 rounded-sm"
  >
    <a
      class="flex items-center gap-x-2 text-lg font-bold text-text-primary tracking-tight cursor-pointer transition-all"
      target="_blank"
      href="https://github.com/aaronlomas/boveda/releases"
      >{$_("unlock_screen.news")}
      <IconSignRight size={16} />
    </a>

    <div class="flex items-center gap-3">
      <button
        class="flex items-center gap-2 bg-surface/5 border border-surface/5 text-text-primary text-xs cursor-pointer py-2 px-4 rounded-lg transition-all hover:bg-surface/10 backdrop-blur-2xl"
        type="button"
        onclick={handleImport}
      >
        <IconDatabaseImport size={16} />
        <span class="font-semibold">{$_("unlock_screen.header_import")}</span>
      </button>
    </div>
  </header>

  <div
    class="max-w-90 m-auto p-8 flex flex-col items-center gap-2 bg-panel/30 backdrop-blur-2xl rounded-2xl border border-surface/8"
  >
    <div class="flex mb-4">
      {#if pendingTotp}
        <IconShieldCheck size={72} class="text-accent" />
      {:else}
        <IconShieldHalfFilled size={72} />
      {/if}
    </div>
    <p
      class="text-text-primary text-sm text-center mb-6 max-w-70 pointer-events-none"
    >
      {#if pendingTotp}
        {$_("settings.security.totp_unlock_desc")}
      {:else if isNew}
        {$_("unlock_screen.new_vault_desc")}
      {:else}
        {$_("unlock_screen.unlock_vault_desc")}
      {/if}
    </p>

    <form
      onsubmit={(e) => {
        e.preventDefault();
        submit();
      }}
      class="w-full flex flex-col gap-4"
    >
      {#if !pendingTotp}
        <div class="flex flex-col gap-1.5">
          <label for="master-pw" class="text-xs text-text-primary"
            >{$_("unlock_screen.master_password_label")}</label
          >

          <div
            class="flex border border-surface/10 rounded-lg px-4 py-2 gap-x-2 bg-transparent {cooldown > 0 ? 'opacity-50 grayscale' : ''}"
          >
            <input
              id="master-pw"
              use:focus
              class="w-full border-0 text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:bg-transparent tracking-widest disabled:cursor-not-allowed"
              type={showPassword ? "text" : "password"}
              bind:value={password}
              placeholder={$_("unlock_screen.placeholder")}
              autocomplete="current-password"
              disabled={cooldown > 0}
            />
            <button
              type="button"
              class="bg-none border-none cursor-pointer text-text-muted hover:text-text-primary transition-all flex items-center disabled:opacity-50"
              onclick={() => (showPassword = !showPassword)}
              disabled={cooldown > 0}
              aria-label={showPassword
                ? $_("dashboard.hide_tooltip")
                : $_("dashboard.show_tooltip")}
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
                onclick={() => (showConfirmPassword = !showConfirmPassword)}
                aria-label={showConfirmPassword
                  ? $_("dashboard.hide_tooltip")
                  : $_("dashboard.show_tooltip")}
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
      {:else}
        <div class="flex flex-col gap-1.5 animate-in fade-in slide-in-from-right-4">
          <label for="totp-unlock" class="text-xs text-text-primary"
            >{$_("settings.security.totp_verify_label")}</label
          >
          <div class="flex border border-surface/10 rounded-lg px-4 py-2 bg-transparent">
            <input
              id="totp-unlock"
              use:focus
              class="w-full border-0 text-text-primary text-center text-lg font-mono tracking-[0.5em] focus:outline-none focus:bg-transparent"
              type="text"
              maxlength="6"
              bind:value={totpCode}
              placeholder="000000"
            />
          </div>
          <button 
            type="button" 
            class="text-xs text-text-muted hover:text-accent-light transition-all flex items-center gap-1 mt-2 self-start cursor-pointer"
            onclick={resetUnlock}
          >
            <IconArrowLeft size={14} />
            {$_("settings.security.totp_back_to_password")}
          </button>
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
        class="w-full justify-center py-2 px-4 text-sm font-semibold bg-accent hover:bg-accent-light text-white rounded-lg shadow-sm shadow-accent/20 transition-all active:scale-[0.98] disabled:cursor-not-allowed flex items-center gap-2 cursor-pointer disabled:bg-surface/10 disabled:text-text-muted disabled:shadow-none"
        disabled={loading || cooldown > 0}
      >
        {#if loading}
          <span
            class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin"
          ></span>
          {$_("unlock_screen.button_processing")}
        {:else if cooldown > 0}
          <span class="animate-pulse">{$_("unlock_screen.wait_seconds", { values: { seconds: cooldown } })}</span>
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

  <footer class="mb-4 text-center text-text-muted text-xs tracking-wide">
    <p>
      {$_("unlock_screen.footer_rights")} -
      <span class="font-semibold text-accent-light">v{version}</span>
    </p>
  </footer>
</div>

