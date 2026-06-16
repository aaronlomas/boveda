<script lang="ts">
  /**
   * @component TotpStatusView
   * @description Displays the current status of two-factor authentication (TOTP).
   * Enables or disables the security feature.
   */
  import { _ } from "svelte-i18n";
  import { IconAuth2fa, IconTrash } from "@tabler/icons-svelte";
  import Button from "$lib/components/core/primitives/Button.svelte";
  import Switch from "$lib/components/core/primitives/Switch.svelte";

  //Props
  let { isEnabled, processing, error, onStartSetup, onConfirmDisable } =
    $props<{
      isEnabled: boolean;
      processing: boolean;
      error: string;
      onStartSetup: () => void;
      onConfirmDisable: () => void;
    }>();
</script>

<div
  class="w-10 h-10 rounded-full flex items-center justify-center {isEnabled
    ? 'bg-success/10 text-success'
    : 'bg-surface/10 text-text-muted'}"
>
  <IconAuth2fa size={20} />
</div>
<div>
  <p class="text-sm font-semibold text-text-primary">
    {$_("settings.security.totp_label")}
  </p>
  <p class="text-xs text-text-muted">
    {$_("settings.security.totp_desc")}
  </p>
</div>

<!-- Activation/Deactivation Controls -->
<div class="flex items-center gap-2">
  <Switch
    checked={isEnabled}
    disabled={processing}
    onclick={(e) => {
      e.preventDefault();
      if (isEnabled) {
        onConfirmDisable();
      } else {
        onStartSetup();
      }
    }}
  />

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
