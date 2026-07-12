<script lang="ts">
  /**
   * @component CredentialField
   * @description Reusable field for displaying credentials (Username, Password, Accounts, etc.).
   * Supports secure mode (encrypted/hidden), copying to clipboard, and cleanup timers.
   */
  import { _ } from "svelte-i18n";
  import { IconEye, IconEyeOff, IconCopy } from "@tabler/icons-svelte";

  // Props
  let {
    label,
    value,
    isSecret = false,
    revealed = false,
    countdown = null,
    showRevealButton = true,
    placeholder = "••••••••••••••••",
    oncopy,
    ontogglereveal,
  } = $props<{
    label: string;
    value: string;
    isSecret?: boolean;
    revealed?: boolean;
    countdown?: number | null;
    showRevealButton?: boolean;
    placeholder?: string;
    oncopy: () => void;
    ontogglereveal?: () => void;
  }>();
</script>

<div class="grid gap-2">
  <!-- Field Label -->
  <span class="text-xs text-text-muted uppercase tracking-wider font-bold">
    {label}
  </span>

  <!-- Container for user and password + control buttons -->
  <div
    class="flex min-w-0 items-center gap-2 bg-surface/5 border border-surface/8 p-1 px-3 transition-colors hover:bg-surface/[0.07]"
  >
    <!-- Text or Mask -->
    <code
      class="flex-1 font-mono text-sm whitespace-nowrap overflow-hidden text-ellipsis tracking-wider will-change-scroll transform-gpu backface-hidden"
      class:text-text-primary={isSecret && revealed}
      class:text-text-secondary={!(isSecret && revealed)}
    >
      {#if isSecret}
        {revealed && value ? value : placeholder}
      {:else}
        {value}
      {/if}
    </code>

    <!-- Interaction Buttons -->
    <div class="flex items-center gap-0.5 shrink-0">
      <!-- Reveal Button (Optional, for secret fields) -->
      {#if isSecret && showRevealButton}
        <button
          class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer"
          onclick={ontogglereveal}
          aria-label={revealed ? $_("actions.hide") : $_("actions.show")}
          data-tooltip={revealed ? $_("actions.hide") : $_("actions.show")}
        >
          {#if revealed}
            <IconEyeOff size={16} />
          {:else}
            <IconEye size={16} />
          {/if}
        </button>
      {/if}

      <!-- Copy to Clipboard Button -->
      <button
        class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer
               {countdown !== null ? 'text-accent-light' : ''}"
        onclick={oncopy}
        aria-label={$_("actions.copy")}
        data-tooltip={countdown !== null
          ? $_("actions.status.clearing", { values: { seconds: countdown } })
          : $_("actions.copy")}
      >
        <div class="w-4 h-4 flex items-center justify-center">
          {#if countdown !== null}
            <!-- Visual Numeric Countdown -->
            <span class="text-[10px] font-bold leading-none">{countdown}</span>
          {:else}
            <IconCopy size={16} />
          {/if}
        </div>
      </button>
    </div>
  </div>
</div>
