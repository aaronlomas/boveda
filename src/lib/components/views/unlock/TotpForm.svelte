<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconArrowLeft } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";

  let { 
    totpCode = $bindable(), 
    onreset 
  } = $props<{
    totpCode: string;
    onreset: () => void;
  }>();
</script>

<div class="flex flex-col gap-1.5 animate-in fade-in slide-in-from-right-4">
  <label for="totp-unlock" class="text-xs text-text-primary">
    {$_("settings.security.totp_verify_label")}
  </label>
  <div class="flex border border-surface/10 rounded-lg px-4 py-2 bg-transparent">
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
      onclick={onreset}
    >
      <IconArrowLeft size={14} />
      {$_("settings.security.totp_back_to_password")}
    </button>
  </div>
</div>
