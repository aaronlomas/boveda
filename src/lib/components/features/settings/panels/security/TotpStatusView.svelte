<script lang="ts">
  /**
   * @component TotpStatusView
   * @description Muestra el estado actual de la autenticación de dos factores (TOTP).
   * Permite activar o desactivar la funcionalidad de seguridad.
   */
  import { _ } from "svelte-i18n";
  import { IconShieldCheck, IconTrash } from "@tabler/icons-svelte";
  import Button from "../../../../core/primitives/Button.svelte";

  //Props
  let { isEnabled, processing, error, onStartSetup, onConfirmDisable } = $props<{
    isEnabled: boolean;
    processing: boolean;
    error: string;
    onStartSetup: () => void;
    onConfirmDisable: () => void;
  }>();
</script>

<!-- ========================================================================= -->
<!-- RENDERING DE VISTA DE ESTADO -->
<!-- ========================================================================= -->
<div class="flex items-center justify-between">
  <!-- Información del Estado -->
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

  <!-- Controles de Activación/Desactivación -->
  {#if (!isEnabled) || (error && error.includes("configurado"))}
    <div class="flex items-center gap-2">
      {#if !isEnabled}
        <Button
          size="sm"
          onclick={onStartSetup}
          disabled={processing}
        >
          {$_("settings.security.totp_enable_btn")}
        </Button>
      {/if}
      
      {#if error && error.includes("configurado")}
        <Button
          variant="ghost"
          size="icon"
          class="text-danger hover:text-danger hover:bg-danger/10"
          onclick={onConfirmDisable}
          title={$_("settings.security.totp_disable_btn")}
          disabled={processing}
        >
          <IconTrash size={20} />
        </Button>
      {/if}
    </div>
  {:else if isEnabled}
    <Button
      variant="ghost"
      size="icon"
      class="text-danger hover:text-danger hover:bg-danger/10"
      onclick={onConfirmDisable}
      title={$_("settings.security.totp_disable_btn")}
      disabled={processing}
    >
      <IconTrash size={20} />
    </Button>
  {/if}
</div>
