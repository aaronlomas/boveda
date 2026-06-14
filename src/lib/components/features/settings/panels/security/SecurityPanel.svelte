<script lang="ts">
  /**
   * @component SecurityPanel
   * @description Main security panel for account configuration.
   * Orchestrates the two-factor authentication (TOTP), scanning, and recovery status components.
   */
  import { invoke } from "@tauri-apps/api/core";
  import { IconLoader2, IconAlertTriangle } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";
  import DisableTotpModal from "../../../../modals/warnings/DisableTotpModal.svelte";

  // Decoupled Components
  import ListItem from "$lib/components/core/primitives/ListItem.svelte";
  import TotpStatusView from "./TotpStatusView.svelte";
  import TotpSetupView from "./TotpSetupView.svelte";
  import TotpRecoveryCodesView from "./TotpRecoveryCodesView.svelte";
  import ExportPasswordModal from "../../../../modals/forms/ExportPasswordModal.svelte";
  import SessionTimeout from "./SessionTimeout.svelte";
  import RemoteConnection from "./RemoteConnection.svelte";

  // COMPONENT STATUS
  let isEnabled = $state(false);
  let loading = $state(true);
  let setupData = $state<{
    otpauth_url: string;
    qr_png_b64: string;
    recovery_codes: string[];
  } | null>(null);
  let verificationCode = $state("");
  let step = $state(1); // 1: Initial View/Status, 2: QR Scan, 3: Recovery Codes
  let error = $state("");
  let processing = $state(false);
  let showDisableConfirm = $state(false);
  let showExportForDisable = $state(false);

  // LIFECYCLE AND STATE LOAD
  onMount(async () => {
    await checkStatus();
  });

  /**
   * Check in the Rust backend if TOTP is active on the current Bóveda.
   */
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

  // 2FA
  /**
   * Request the backend to initiate TOTP configuration. Generate secret key and recovery codes.
   */
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

  // VERIFICATION LOGIC (STEP 2 to STEP 3)
  /**
   * Send the 6-digit verification code entered by the user.
   */
  async function verifySetup() {
    const cleanCode = verificationCode.replace(/\s/g, "");
    if (cleanCode.length !== 6) return;
    processing = true;
    error = "";
    try {
      const valid = await invoke<boolean>("totp_verify_setup", {
        code: cleanCode,
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

  // TOTP DEACTIVATION LOGIC
  /**
   * The deactivation flow begins by displaying the confirmation modal.
   */
  function confirmDisable() {
    showDisableConfirm = true;
  }

  function handleDisableTotpConfirm() {
    showDisableConfirm = false;
    showExportForDisable = true;
  }

  /**
   * Send the full deactivation request to the Rust backend.
   */
  async function disableTotp() {
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
      error = ""; // Clear accumulated errors
      processing = false;
    }
  }
</script>

<!-- GLOBAL WINDOW EVENTS -->
<svelte:window
  onkeydown={(e) => {
    if (showDisableConfirm && e.key === "Escape" && !processing)
      showDisableConfirm = false;
    if (showExportForDisable && e.key === "Escape")
      showExportForDisable = false;
  }}
/>
<div class="grid gap-4">
  <div>
    <h1 class="text-xl font-bold text-text-primary">
      {$_("settings.security.title")}
    </h1>
    <p class="text-xs text-text-muted">{$_("settings.security.desc")}</p>
  </div>

  {#if loading}
    <div class="flex justify-center py-12">
      <IconLoader2 size={32} class="animate-spin text-accent" />
    </div>
  {:else}
    <ListItem layout="triple">
      <TotpStatusView
        {isEnabled}
        {processing}
        {error}
        onStartSetup={startSetup}
        onConfirmDisable={confirmDisable}
      />

      <!-- Backend Error Alert Box -->
      {#if error}
        <div
          class="p-3 bg-danger/10 border border-danger/20 rounded-lg flex items-center gap-2 text-danger animate-in fade-in slide-in-from-top-1"
        >
          <IconAlertTriangle size={16} />
          <p class="text-xs font-medium">{error}</p>
        </div>
      {/if}

      <!-- Decoupled Component 2: Configuration QR Scan (Step 2) -->
      {#if !isEnabled && step === 2 && setupData}
        <TotpSetupView
          {setupData}
          {processing}
          bind:verificationCode
          onCancel={() => (step = 1)}
          onVerify={verifySetup}
        />

        <!-- Decoupled Component 3: Confirmation and Backup Codes (Step 3) -->
      {:else if step === 3}
        <TotpRecoveryCodesView
          recoveryCodes={setupData?.recovery_codes || []}
          onDone={() => (step = 1)}
        />
      {/if}
    </ListItem>
  {/if}

  <!-- Deactivation Warning Modality -->
  {#if showDisableConfirm}
    <DisableTotpModal
      onconfirm={handleDisableTotpConfirm}
      oncancel={() => (showDisableConfirm = false)}
      {processing}
    />
  {/if}

  <!--Export Modal Required to Disable 2FA-->
  {#if showExportForDisable}
    <ExportPasswordModal
      customTitle={$_("settings.security.totp_export_title")}
      customWarning={$_("settings.security.totp_export_warning")}
      onconfirm={async () => {
        showExportForDisable = false;
        await disableTotp();
      }}
      oncancel={() => (showExportForDisable = false)}
    />
  {/if}

  <ListItem layout="triple">
    <SessionTimeout />
  </ListItem>

  <ListItem layout="triple">
    <RemoteConnection />
  </ListItem>
</div>
