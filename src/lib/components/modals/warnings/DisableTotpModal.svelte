<script lang="ts">
  import {
    IconAlertTriangle,
    IconX,
    IconTrash,
    IconLoader2,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";

  let { onconfirm, oncancel, processing = false } = $props();
</script>

<Modal 
  show={true} 
  onclose={oncancel} 
  title={$_("settings.security.totp_disable_title")}
>
  <!-- Warning Content -->
  <div class="space-y-4">
    <div class="p-4 bg-warning/10 border border-warning/20 rounded-xl flex items-start gap-3 text-warning animate-in fade-in slide-in-from-top-1">
      <IconAlertTriangle size={18} class="shrink-0 mt-0.5" />
      <p class="text-xs font-medium leading-relaxed">
        {$_("settings.security.totp_disable_subtitle")}
      </p>
    </div>
    <p class="text-sm text-text-muted px-1">
      {$_("settings.security.totp_disable_desc") || "¿Estás seguro de que deseas desactivar la autenticación de dos factores? Esto reducirá la seguridad de tu bóveda."}
    </p>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={oncancel} disabled={processing}>
      {$_("global.cancel")}
    </Button>
    <Button 
      variant="danger" 
      onclick={onconfirm} 
      disabled={processing}
      class="gap-2"
    >
      {#if processing}
        <IconLoader2 size={16} class="animate-spin" />
      {:else}
        <IconTrash size={16} />
        {$_("settings.security.totp_disable_confirm_btn")}
      {/if}
    </Button>
  {/snippet}
</Modal>
