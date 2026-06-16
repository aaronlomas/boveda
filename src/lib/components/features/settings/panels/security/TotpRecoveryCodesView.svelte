<script lang="ts">
  /**
   * @component TotpRecoveryCodesView
   * @description View of step 3 of TOTP configuration.
   * Displays the generated backup recovery codes and manages copying to the clipboard.
   */
  import { _ } from "svelte-i18n";
  import { IconCheck, IconCopy } from "@tabler/icons-svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { UI_CONFIG } from "$lib/config/ui";
  import Button from "../../../../core/primitives/Button.svelte";

  // Props
  let { recoveryCodes, onDone } = $props<{
    recoveryCodes: string[];
    onDone: () => void;
  }>();

  // Local status for copy feedback
  let copied = $state(false);

  // Local logic for copying code to the clipboard
  async function handleCopy() {
    if (!recoveryCodes || recoveryCodes.length === 0) return;
    const text = recoveryCodes.join("\n");
    try {
      await writeText(text);
      copied = true;
      setTimeout(() => (copied = false), UI_CONFIG.COPY_FEEDBACK_DURATION);
    } catch (e) {
      console.error("Failed to copy recovery codes:", e);
    }
  }
</script>
<!-- SUCCESS SCREEN AND RECOVERY CODES -->
<div class="pt-4 border-t border-surface/8 text-center space-y-3 animate-in zoom-in-95 col-span-3">
  
  <div class="w-12 h-12 bg-success/20 text-success rounded-full flex items-center justify-center mx-auto">
    <IconCheck size={28} />
  </div>
  
  <!-- Confirmation Header -->
  <h2 class="text-lg font-bold text-text-primary">
    {$_("settings.security.totp_success_title")}
  </h2>
  <p class="text-xs text-text-muted px-6">
    {$_("settings.security.totp_success_desc")}
  </p>

  <!-- Recovery code block -->
  <div class="mt-6 space-y-4">
    <div class="bg-surface/5 border border-surface/10 rounded-xl p-4">
      
      <!-- Header Row and Copy Action -->
      <div class="flex items-center justify-between mb-3 px-1">
        <span class="text-xs font-bold uppercase tracking-widest text-text-muted">
          {$_("settings.security.totp_recovery_title")}
        </span>
        <button 
          class="text-xs text-accent hover:text-accent-light transition-all flex items-center gap-1.5 font-semibold cursor-pointer"
          onclick={handleCopy}
        >
          {#if copied}
            <IconCheck size={14} />
            {$_("actions.copied")}
          {:else}
            <IconCopy size={14} />
            {$_("actions.copy")}
          {/if}
        </button>
      </div>

      <!-- Code Grid -->
      <div class="grid grid-cols-2 gap-2 text-left">
        {#each recoveryCodes as code}
          <code class="text-xs font-mono bg-surface/10 p-2 rounded-md text-text-primary border border-surface/5">
            {code}
          </code>
        {/each}
      </div>
    </div>

    <!-- Saving Warning -->
    <div class="p-3 bg-warning/10 border border-warning/20 rounded-lg">
      <p class="text-xs text-warning text-center leading-relaxed font-medium">
        {$_("settings.security.totp_recovery_warning_setup")}
      </p>
    </div>
  </div>

  <Button
    class="mt-4 w-full py-3 rounded-xl font-bold shadow-lg shadow-accent/20"
    onclick={onDone}
  >
    {$_("settings.security.totp_done_btn")}
  </Button>
</div>
