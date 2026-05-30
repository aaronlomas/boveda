<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconArrowLeft } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";

  let { 
    recoveryCode = $bindable(), 
    recoverySuccess, 
    oncancel 
  } = $props<{
    recoveryCode: string;
    recoverySuccess: boolean;
    oncancel: () => void;
  }>();
</script>

<div class="flex flex-col gap-1.5 animate-in fade-in slide-in-from-right-4">
  {#if !recoverySuccess}
    <label for="recovery-unlock" class="text-xs text-text-primary">
      {$_("settings.security.totp_recovery_title")}
    </label>
    <div class="flex border border-surface/10 rounded-lg px-4 py-2 bg-transparent">
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
      onclick={oncancel}
    >
      <IconArrowLeft size={14} />
      {$_("settings.security.totp_back_to_password")}
    </button>
  {/if}
</div>
