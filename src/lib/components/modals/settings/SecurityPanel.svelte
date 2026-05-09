<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {
    IconShieldCheck,
    IconAlertTriangle,
    IconQrcode,
    IconCheck,
    IconTrash,
    IconLoader2,
    IconX,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";

  let isEnabled = $state(false);
  let loading = $state(true);
  let setupData = $state<{ otpauth_url: string; qr_png_b64: string } | null>(
    null,
  );
  let verificationCode = $state("");
  let step = $state(1); // 1: Initial/Enabled view, 2: QR Scan, 3: Success
  let error = $state("");
  let processing = $state(false);
  let showDisableConfirm = $state(false);

  onMount(async () => {
    await checkStatus();
  });

  async function checkStatus() {
    loading = true;
    try {
      isEnabled = await invoke<boolean>("totp_is_enabled");
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function startSetup() {
    processing = true;
    error = "";
    try {
      setupData = await invoke("totp_setup");
      step = 2;
    } catch (e: any) {
      console.error("TOTP Setup failed:", e);
      error = e.toString();
    } finally {
      processing = false;
    }
  }

  async function verifySetup() {
    if (verificationCode.length !== 6) return;
    processing = true;
    error = "";
    try {
      const valid = await invoke<boolean>("totp_verify_setup", {
        code: verificationCode,
      });
      if (valid) {
        isEnabled = true;
        step = 3;
      } else {
        error = $_("settings.security.totp_error_invalid");
      }
    } catch (e: any) {
      console.error("TOTP Verification failed:", e);
      error = e.toString();
    } finally {
      processing = false;
    }
  }

  async function confirmDisable() {
    showDisableConfirm = true;
  }

  async function disableTotp() {
    showDisableConfirm = false;
    processing = true;
    try {
      await invoke("totp_disable");
      isEnabled = false;
      step = 1;
      setupData = null;
      verificationCode = "";
    } catch (e: any) {
      error = e.toString();
    } finally {
      processing = false;
    }
  }

  // Escape key handler is now managed by <svelte:window> in the template
</script>
<svelte:window
  onkeydown={(e) => {
    if (showDisableConfirm && e.key === "Escape") showDisableConfirm = false;
  }}
/>

<div class="space-y-6">
  <header class="flex items-center gap-3">
    <div class="p-2 bg-accent/10 rounded-lg text-accent">
      <IconShieldCheck size={24} />
    </div>
    <div>
      <h1 class="text-xl font-bold text-text-primary">
        {$_("settings.security.title")}
      </h1>
      <p class="text-xs text-text-muted">{$_("settings.security.desc")}</p>
    </div>
  </header>

  {#if loading}
    <div class="flex justify-center py-12">
      <IconLoader2 size={32} class="animate-spin text-accent" />
    </div>
  {:else}
    <div class="bg-surface/3 border border-surface/8 rounded-2xl p-5 space-y-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-full flex items-center justify-center {isEnabled
              ? 'bg-success/10 text-success'
              : 'bg-surface/10 text-text-muted'}"
          >
            <IconShieldCheck size={20} />
          </div>
          <div>
            <p class="text-sm font-semibold text-text-primary">
              {$_("settings.security.totp_label")}
            </p>
            <p class="text-xs text-text-muted">
              {isEnabled
                ? $_("settings.security.totp_enabled")
                : $_("settings.security.totp_disabled")}
            </p>
          </div>
        </div>

        {#if !isEnabled && step === 1}
          <button
            class="px-4 py-2 bg-accent text-white text-xs font-bold rounded-lg hover:bg-accent-hover transition-all disabled:opacity-50"
            onclick={startSetup}
            disabled={processing}
          >
            {$_("settings.security.totp_enable_btn")}
          </button>
        {:else if isEnabled}
          <button
            class="p-2 text-danger hover:bg-danger/10 rounded-lg transition-all"
            onclick={confirmDisable}
            title={$_("settings.security.totp_disable_btn")}
            disabled={processing}
          >
            <IconTrash size={20} />
          </button>
        {/if}
      </div>

      {#if error}
        <div
          class="p-3 bg-danger/10 border border-danger/20 rounded-lg flex items-center gap-2 text-danger animate-in fade-in slide-in-from-top-1"
        >
          <IconAlertTriangle size={16} />
          <p class="text-xs font-medium">{error}</p>
        </div>
      {/if}

      {#if !isEnabled && step === 2 && setupData}
        <div
          class="pt-4 border-t border-surface/8 space-y-5 animate-in fade-in slide-in-from-top-2"
        >
          <div
            class="bg-accent/10 border border-accent/20 p-4 rounded-xl flex gap-3"
          >
            <IconShieldCheck size={20} class="text-accent shrink-0" />
            <p class="text-[11px] text-text-primary leading-relaxed">
              {$_("settings.security.totp_warning")}
            </p>
          </div>

          <div class="flex flex-col items-center gap-4">
            <div class="bg-white p-3 rounded-xl shadow-xl">
              <img
                src="data:image/png;base64,{setupData.qr_png_b64}"
                alt="TOTP QR Code"
                class="w-40 h-40"
              />
            </div>
            <p class="text-xs text-text-muted text-center max-w-xs">
              {$_("settings.security.totp_scan_desc")}
            </p>
          </div>

          <div class="space-y-2">
            <label
              for="totp-verify"
              class="text-xs font-semibold text-text-secondary"
            >
              {$_("settings.security.totp_verify_label")}
            </label>
            <div class="flex flex-col gap-3">
              <input
                id="totp-verify"
                type="text"
                maxlength="6"
                placeholder="000000"
                bind:value={verificationCode}
                class="w-full bg-surface/5 border border-surface/10 rounded-lg px-4 py-2 text-center text-lg font-mono tracking-[0.5em] focus:outline-none focus:border-accent text-text-primary"
              />
              <div class="flex gap-2">
                <button
                  type="button"
                  class="flex-1 px-4 py-2 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-xs font-semibold hover:bg-surface/10 transition-all"
                  onclick={() => (step = 1)}
                  disabled={processing}
                >
                  {$_("global.cancel")}
                </button>
                <button
                  class="flex-1 px-6 py-2 bg-accent text-white rounded-lg font-bold hover:bg-accent-hover transition-all disabled:opacity-50 flex items-center justify-center gap-2"
                  disabled={verificationCode.length !== 6 || processing}
                  onclick={verifySetup}
                >
                  {#if processing}
                    <IconLoader2 size={18} class="animate-spin" />
                  {:else}
                    <IconCheck size={18} />
                    {$_("settings.security.totp_verify_btn")}
                  {/if}
                </button>
              </div>
            </div>
          </div>
        </div>
      {:else if step === 3}
        <div
          class="pt-4 border-t border-surface/8 text-center space-y-3 animate-in zoom-in-95"
        >
          <div
            class="w-12 h-12 bg-success/20 text-success rounded-full flex items-center justify-center mx-auto"
          >
            <IconCheck size={28} />
          </div>
          <h2 class="text-lg font-bold text-text-primary">
            {$_("settings.security.totp_success_title")}
          </h2>
          <p class="text-xs text-text-muted px-6">
            {$_("settings.security.totp_success_desc")}
          </p>
          <button
            class="mt-4 px-6 py-2 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-sm hover:bg-surface/10 transition-all"
            onclick={() => (step = 1)}
          >
            {$_("settings.security.totp_done_btn")}
          </button>
        </div>
      {/if}
    </div>
  {/if}

  <div class="p-4 bg-accent/5 border border-accent/10 rounded-xl">
    <h3 class="text-sm font-bold text-accent-light mb-1">
      {$_("settings.security.pro_tip_title")}
    </h3>
    <p class="text-xs text-text-muted leading-relaxed">
      {$_("settings.security.pro_tip_desc")}
    </p>
  </div>
</div>

{#if showDisableConfirm}
  <div class="fixed inset-0 z-60 flex items-center justify-center p-4">
    <button
      type="button"
      class="absolute inset-0 bg-panel/70 backdrop-blur-sm border-none w-full h-full cursor-default"
      onclick={() => (showDisableConfirm = false)}
      title={$_("settings.close")}
      aria-label={$_("settings.close")}
    ></button>
    <div
      class="relative w-full max-w-md bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl p-6 shadow-2xl space-y-5"
      role="dialog"
      aria-modal="true"
      aria-labelledby="disable-confirm-title"
      tabindex="-1"
    >
      <!-- Header -->
      <div class="flex items-start justify-between">
        <div class="flex items-center gap-3">
          <div
            class="w-10 h-10 rounded-xl bg-warning/15 border border-warning/20 flex items-center justify-center text-warning shrink-0"
          >
            <IconAlertTriangle size={20} />
          </div>
          <div>
            <h2 id="disable-confirm-title" class="text-base font-bold text-text-primary">
              {$_("settings.security.totp_disable_title")}
            </h2>
          </div>
        </div>
        <button
          class="p-1.5 rounded-lg text-text-muted hover:text-text-primary hover:bg-surface/8 transition-all"
          onclick={() => (showDisableConfirm = false)}
          aria-label={$_("settings.close")}
        >
          <IconX size={16} />
        </button>
      </div>

      <div class="p-4 bg-danger/10 border border-danger/20 rounded-xl flex items-start gap-3 text-danger animate-in fade-in slide-in-from-top-1">
        <IconAlertTriangle size={18} class="shrink-0 mt-0.5" />
        <p class="text-xs font-medium leading-relaxed">
          {$_("settings.security.totp_disable_subtitle")}
        </p>
      </div>

      <!-- Warning steps -->
      <div class="bg-warning/5 border border-warning/15 rounded-xl p-4 space-y-3">
        <p class="text-xs font-semibold text-warning uppercase tracking-wide">
          {$_("settings.security.totp_disable_steps_title")}
        </p>
        <ol
          class="space-y-2 text-xs text-text-secondary list-decimal list-inside leading-relaxed"
        >
          <li>{$_("settings.security.totp_disable_step1")}</li>
          <li>{$_("settings.security.totp_disable_step2")}</li>
        </ol>
      </div>

      <p class="text-xs text-text-muted leading-relaxed">
        {$_("settings.security.totp_disable_seed_warning")}
      </p>

      <!-- Actions -->
      <div class="flex gap-3 pt-1">
        <button
          class="flex-1 px-4 py-2 bg-surface/5 border border-surface/10 rounded-lg text-text-secondary text-sm font-semibold hover:bg-surface/10 transition-all"
          onclick={() => (showDisableConfirm = false)}
          disabled={processing}
        >
          {$_("global.cancel")}
        </button>
        <button
          class="flex-1 px-4 py-2 bg-danger text-white rounded-lg text-sm font-bold hover:brightness-110 transition-all disabled:opacity-50 flex items-center justify-center gap-2 shadow-sm shadow-danger/20"
          onclick={disableTotp}
          disabled={processing}
        >
          {#if processing}
            <IconLoader2 size={16} class="animate-spin" />
          {:else}
            <IconTrash size={16} />
            {$_("settings.security.totp_disable_confirm_btn")}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
