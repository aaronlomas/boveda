<script lang="ts">
  /**
   * @component TotpSetupView
   * @description View of step 2 of TOTP configuration.
   * Displays the QR code to scan and the verification input with action buttons.
   */
  import { _ } from "svelte-i18n";
  import { IconShieldCheck, IconLoader2, IconCheck } from "@tabler/icons-svelte";
  import Button from "../../../../core/primitives/Button.svelte";
  import Input from "../../../../core/primitives/Input.svelte";

  // Props
  let { setupData, processing, verificationCode = $bindable(), onCancel, onVerify } = $props<{
    setupData: { qr_png_b64: string; otpauth_url: string };
    processing: boolean;
    verificationCode: string;
    onCancel: () => void;
    onVerify: () => void;
  }>();
</script>
<!-- MAIN CONFIGURATION CONTAINER -->
<div class="pt-4 border-t border-surface/8 space-y-5 animate-in fade-in slide-in-from-top-2">
  
  <!-- Advertencia de Seguridad -->
  <div class="bg-accent/10 border border-accent/20 p-4 rounded-xl flex gap-3">
    <IconShieldCheck size={20} class="text-accent shrink-0" />
    <p class="text-xs text-text-primary leading-relaxed">
      {$_("settings.security.totp_warning")}
    </p>
  </div>

  <!-- QR Code -->
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

  <!-- Verification Code and Actions Field -->
  <div class="space-y-4">
    <div class="flex flex-col gap-3">
      <!-- Input Primitive -->
      <Input
        id="totp-verify"
        type="text"
        label={$_("settings.security.totp_verify_label")}
        maxlength={16}
        placeholder="000000"
        bind:value={verificationCode}
        class="text-center text-lg font-mono tracking-[0.5em] focus-visible:ring-accent"
      />
      
      <!-- Buttons -->
      <div class="flex gap-2 pt-2">
        <Button
          variant="secondary"
          class="flex-1"
          onclick={onCancel}
          disabled={processing}
        >
          {$_("actions.cancel")}
        </Button>
        
        <Button
          class="flex-1 gap-2 font-bold"
          disabled={verificationCode.replace(/\s/g, "").length !== 6 || processing}
          onclick={onVerify}
        >
          {#if processing}
            <IconLoader2 size={18} class="animate-spin" />
          {:else}
            <IconCheck size={18} />
            {$_("settings.security.totp_verify_btn")}
          {/if}
        </Button>
      </div>
    </div>
  </div>
</div>
