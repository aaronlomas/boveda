<script lang="ts">
  /**
   * @component SecurityPanel
   * @description Panel principal de seguridad para la configuración de la cuenta.
   * Orquesta los componentes de estado de autenticación en dos factores (TOTP), escaneo y recuperación.
   */
  import { invoke } from "@tauri-apps/api/core";
  import { IconLoader2, IconAlertTriangle } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";
  import DisableTotpModal from "../../../../modals/warnings/DisableTotpModal.svelte";

  // Componentes Desacoplados
  import TotpStatusView from "./TotpStatusView.svelte";
  import TotpSetupView from "./TotpSetupView.svelte";
  import TotpRecoveryCodesView from "./TotpRecoveryCodesView.svelte";
  import ExportPasswordModal from "../../../../modals/forms/ExportPasswordModal.svelte";

  // =========================================================================
  // ESTADOS DEL COMPONENTE
  // =========================================================================
  let isEnabled = $state(false);
  let loading = $state(true);
  let setupData = $state<{
    otpauth_url: string;
    qr_png_b64: string;
    recovery_codes: string[];
  } | null>(null);
  let verificationCode = $state("");
  let step = $state(1); // 1: Vista inicial/Estado, 2: Escaneo QR, 3: Códigos de recuperación
  let error = $state("");
  let processing = $state(false);
  let showDisableConfirm = $state(false);
  let showExportForDisable = $state(false);

  // =========================================================================
  // CICLO DE VIDA Y CARGA DE ESTADO
  // =========================================================================
  onMount(async () => {
    await checkStatus();
  });

  /**
   * Verifica en el backend de Rust si TOTP está activo en la bóveda actual.
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

  // =========================================================================
  // LÓGICA DE ACTIVACIÓN DE TOTP (PASO 1 A PASO 2)
  // =========================================================================
  /**
   * Solicita al backend iniciar la configuración de TOTP. Genera clave secreta y códigos de recuperación.
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

  // =========================================================================
  // LÓGICA DE VERIFICACIÓN (PASO 2 A PASO 3)
  // =========================================================================
  /**
   * Envía el código de verificación de 6 dígitos ingresado por el usuario.
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

  // =========================================================================
  // LÓGICA DE DESACTIVACIÓN DE TOTP
  // =========================================================================
  /**
   * Inicia el flujo de desactivación mostrando el modal de confirmación.
   */
  function confirmDisable() {
    showDisableConfirm = true;
  }

  function handleDisableTotpConfirm() {
    showDisableConfirm = false;
    showExportForDisable = true;
  }

  /**
   * Envía la solicitud de desactivación total al backend de Rust.
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
      error = ""; // Limpia los errores acumulados
      processing = false;
    }
  }
</script>

<!-- ========================================================================= -->
<!-- EVENTOS DE VENTANA GLOBAL -->
<!-- ========================================================================= -->
<svelte:window
  onkeydown={(e) => {
    if (showDisableConfirm && e.key === "Escape" && !processing)
      showDisableConfirm = false;
    if (showExportForDisable && e.key === "Escape")
      showExportForDisable = false;
  }}
/>

<!-- ========================================================================= -->
<!-- MAQUETACIÓN E INTERFAZ DEL PANEL -->
<!-- ========================================================================= -->
<div class="space-y-6">
  <!-- Cabecera del Panel -->
  <header class="flex items-center gap-2">
    <div>
      <h1 class="text-xl font-bold text-text-primary">
        {$_("settings.security.title")}
      </h1>
      <p class="text-xs text-text-muted">{$_("settings.security.desc")}</p>
    </div>
  </header>

  <!-- Contenedor Principal o Cargador -->
  {#if loading}
    <div class="flex justify-center py-12">
      <IconLoader2 size={32} class="animate-spin text-accent" />
    </div>
  {:else}
    <div class="bg-surface/3 border border-surface/8 rounded-2xl p-5 space-y-4">
      <!-- Componente Desacoplado 1: Estado General -->
      <TotpStatusView
        {isEnabled}
        {processing}
        {error}
        onStartSetup={startSetup}
        onConfirmDisable={confirmDisable}
      />

      <!-- Caja de Alertas de Error del Backend -->
      {#if error}
        <div
          class="p-3 bg-danger/10 border border-danger/20 rounded-lg flex items-center gap-2 text-danger animate-in fade-in slide-in-from-top-1"
        >
          <IconAlertTriangle size={16} />
          <p class="text-xs font-medium">{error}</p>
        </div>
      {/if}

      <!-- Componente Desacoplado 2: Escaneo QR de Configuración (Paso 2) -->
      {#if !isEnabled && step === 2 && setupData}
        <TotpSetupView
          {setupData}
          {processing}
          bind:verificationCode
          onCancel={() => (step = 1)}
          onVerify={verifySetup}
        />

        <!-- Componente Desacoplado 3: Confirmación y Códigos de Respaldo (Paso 3) -->
      {:else if step === 3}
        <TotpRecoveryCodesView
          recoveryCodes={setupData?.recovery_codes || []}
          onDone={() => (step = 1)}
        />
      {/if}
    </div>
  {/if}

  <!-- Sugerencias de Seguridad -->

  <p class="text-xs text-text-muted leading-relaxed">
    {$_("settings.security.pro_tip_desc")}
  </p>
</div>

<!-- Modal de Advertencia de Desactivación -->
{#if showDisableConfirm}
  <DisableTotpModal
    onconfirm={handleDisableTotpConfirm}
    oncancel={() => (showDisableConfirm = false)}
    {processing}
  />
{/if}

<!-- Modal de Exportación Requerida para Desactivar 2FA -->
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
