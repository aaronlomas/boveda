<script lang="ts">
  import { globalState } from "$lib/stores/stores.svelte";
  import { unlockVault, isVaultInitialized } from "$lib/utils/tauri";
  import { onMount } from "svelte";
  import {
    IconEye,
    IconEyeOff,
    IconLock,
    IconShieldHalfFilled,
    IconSignRight,
    IconArrowLeft,
    IconShieldCheck,
    IconLifebuoy,
    IconCircleCheck,
  } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { _ } from "svelte-i18n";
  import { focus } from "$lib/utils/actions";

  const version = import.meta.env.APP_VERSION;
  const status = import.meta.env.APP_STATUS;

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

  let recoveryCode = $state("");
  let isRecovery = $state(false);
  let recoverySuccess = $state(false);
  let isShaking = $state(false);

  // Auto-submit TOTP when 6 digits are reached
  $effect(() => {
    const clean = totpCode.replace(/\s/g, "");
    if (clean.length === 6 && !loading && !error) {
      submit();
    }
  });

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
          // We don't clear password yet because we might need it for re-auth if TOTP fails?
          // Actually, once status is totp_required, the vault is partially unlocked in the backend.
        } else {
          password = ""; // Security: Clear password from memory
          globalState.isUnlocked = true;
        }
      } catch (e: any) {
        handleError(e);
      } finally {
        loading = false;
      }
    } else {
      const cleanTotp = totpCode.replace(/\s/g, "");
      if (cleanTotp.length !== 6) {
        error = $_("settings.security.totp_error_invalid");
        return;
      }
      loading = true;
      try {
        const valid = await invoke<boolean>("totp_check", { code: cleanTotp });
        if (valid) {
          password = ""; // Security: Clear secrets
          totpCode = "";
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

  async function submitRecovery() {
    if (recoverySuccess) {
      completeRecovery();
      return;
    }
    const cleanRecovery = recoveryCode.replace(/\s/g, "");
    if (!cleanRecovery) return;
    loading = true;
    error = "";
    try {
      const valid = await invoke<boolean>("totp_recovery_check", {
        code: cleanRecovery,
      });
      if (valid) {
        recoverySuccess = true;
      }
    } catch (e: any) {
      error = $_("settings.security.totp_recovery_error");
    } finally {
      loading = false;
    }
  }

  function completeRecovery() {
    isRecovery = false;
    pendingTotp = false;
    recoverySuccess = false;
    recoveryCode = "";
    password = "";
    isNew = false;
    error = "";
  }

  function handleError(e: any) {
    error = $_("unlock_screen.error_incorrect");
    isShaking = true;
    setTimeout(() => (isShaking = false), 500);

    // Add cooldown (1-3 seconds) to prevent rapid brute force
    cooldown = Math.floor(Math.random() * 3) + 1;
    const timer = setInterval(() => {
      cooldown--;
      if (cooldown <= 0) clearInterval(timer);
    }, 1000);
  }

  function resetUnlock() {
    pendingTotp = false;
    isRecovery = false;
    totpCode = "";
    recoveryCode = "";
    error = "";
    invoke("lock_vault");
  }
</script>

<div class="h-full grid grid-rows-[auto_1fr_auto]">
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

    <div class="flex items-center gap-3"></div>
  </header>

  <div
    class="max-w-90 m-auto p-8 flex flex-col items-center gap-2 bg-panel/30 backdrop-blur-2xl rounded-2xl border border-surface/8 {isShaking
      ? 'animate-shake'
      : ''}"
  >
    <div class="flex mb-4">
      {#if recoverySuccess}
        <IconCircleCheck size={72} class="text-accent" />
      {:else if isRecovery}
        <IconLifebuoy size={72} class="text-warning" />
      {:else if pendingTotp}
        <IconShieldCheck size={72} class="text-accent" />
      {:else}
        <IconShieldHalfFilled size={72} />
      {/if}
    </div>
    <p
      class="text-text-primary text-sm text-center mb-6 max-w-70 pointer-events-none"
    >
      {#if recoverySuccess}
        {$_("settings.security.totp_recovery_success")}
      {:else if isRecovery}
        {$_("settings.security.totp_recovery_desc")}
      {:else if pendingTotp}
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
        if (recoverySuccess) completeRecovery();
        else if (isRecovery) submitRecovery();
        else submit();
      }}
      class="w-full flex flex-col gap-4"
    >
      {#if !pendingTotp}
        <div class="flex flex-col gap-1.5">
          <label for="master-pw" class="text-xs text-text-primary"
            >{$_("unlock_screen.master_password_label")}</label
          >

          <div
            class="flex border border-surface/10 rounded-lg px-4 py-2 gap-x-2 bg-transparent {cooldown >
            0
              ? 'opacity-50 grayscale'
              : ''}"
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
      {:else if isRecovery}
        <div
          class="flex flex-col gap-1.5 animate-in fade-in slide-in-from-right-4"
        >
          {#if !recoverySuccess}
            <label for="recovery-unlock" class="text-xs text-text-primary"
              >{$_("settings.security.totp_recovery_title")}</label
            >
            <div
              class="flex border border-surface/10 rounded-lg px-4 py-2 bg-transparent"
            >
              <input
                id="recovery-unlock"
                use:focus
                class="w-full border-0 text-text-primary text-center text-lg font-mono tracking-tight focus:outline-none focus:bg-transparent"
                type="text"
                maxlength="24"
                bind:value={recoveryCode}
                placeholder={$_("settings.security.totp_recovery_placeholder")}
                disabled={recoverySuccess}
              />
            </div>
            <button
              type="button"
              class="text-xs text-text-muted hover:text-accent-light transition-all flex items-center gap-1 mt-2 self-start cursor-pointer"
              onclick={() => {
                isRecovery = false;
                error = "";
              }}
            >
              <IconArrowLeft size={14} />
              {$_("settings.security.totp_back_to_password")}
            </button>
          {/if}
        </div>
      {:else}
        <div
          class="flex flex-col gap-1.5 animate-in fade-in slide-in-from-right-4"
        >
          <label for="totp-unlock" class="text-xs text-text-primary"
            >{$_("settings.security.totp_verify_label")}</label
          >
          <div
            class="flex border border-surface/10 rounded-lg px-4 py-2 bg-transparent"
          >
            <input
              id="totp-unlock"
              use:focus
              class="w-full border-0 text-text-primary text-center text-lg font-mono tracking-[0.5em] focus:outline-none focus:bg-transparent"
              type="text"
              maxlength="6"
              inputmode="numeric"
              autocomplete="one-time-code"
              bind:value={totpCode}
              placeholder="000000"
            />
          </div>

          <div class="flex items-center mt-2">
            <button
              type="button"
              class="text-xs text-text-muted hover:text-accent-light transition-all flex items-center gap-1 cursor-pointer"
              onclick={resetUnlock}
            >
              <IconArrowLeft size={14} />
              {$_("settings.security.totp_back_to_password")}
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
        class="w-full justify-center py-2 px-4 text-sm font-semibold bg-accent hover:bg-accent-light text-white rounded-lg shadow-sm shadow-accent/20 transition-all active:scale-[0.98] disabled:cursor-not-allowed flex items-center gap-2 cursor-pointer disabled:bg-surface/10 disabled:text-text-muted disabled:shadow-none"
        disabled={loading || cooldown > 0}
      >
        {#if loading}
          <span
            class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin"
          ></span>
          {$_("unlock_screen.button_processing")}
        {:else if cooldown > 0}
          <span class="animate-pulse"
            >{$_("unlock_screen.wait_seconds", {
              values: { seconds: cooldown },
            })}</span
          >
        {:else if recoverySuccess}
          <IconCircleCheck size={18} /> {$_("settings.done_btn") || "OK"}
        {:else if isRecovery}
          <IconLifebuoy size={18} /> {$_("settings.security.totp_recovery_btn")}
        {:else if isNew}
          {$_("unlock_screen.button_create")}
        {:else}
          <IconLock size={18} /> {$_("unlock_screen.button_unlock")}
        {/if}
      </button>

      {#if pendingTotp && !isRecovery && !recoverySuccess}
        <button
          type="button"
          class="text-[10px] uppercase tracking-wider text-text-muted hover:text-warning transition-all cursor-pointer font-bold mt-2 self-center"
          onclick={() => {
            isRecovery = true;
            error = "";
          }}
        >
          {$_("settings.security.totp_recovery_link")}
        </button>
      {/if}
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
      <span class="font-semibold text-accent-light">v{version}</span> |
      {status}
    </p>
  </footer>
</div>
