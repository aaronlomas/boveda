<script lang="ts">
  /**
   * @component TotpStatusView
   * @description Displays the current status of two-factor authentication (TOTP).
   * Enables or disables the security feature.
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

<!-- VIEW RENDERING -->
<div class="flex items-center justify-between">
  <!-- status information -->
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

  <!-- Activation/Deactivation Controls -->
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
