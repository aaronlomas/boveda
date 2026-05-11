<script lang="ts">
  import {
    IconAlertTriangle,
    IconX,
    IconTrash,
    IconLoader2,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  let { onconfirm, oncancel, processing = false } = $props();
</script>

<div class="fixed inset-0 z-60 flex items-center justify-center p-4">
  <!-- Backdrop -->
  <button
    type="button"
    class="absolute inset-0 bg-panel/70 backdrop-blur-sm border-none w-full h-full cursor-default"
    onclick={oncancel}
    title={$_("settings.close")}
    aria-label={$_("settings.close")}
    disabled={processing}
  ></button>

  <!-- Modal -->
  <div
    class="relative w-full max-w-md bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl p-6 shadow-2xl space-y-5"
    role="dialog"
    aria-modal="true"
    aria-labelledby="disable-confirm-title"
    tabindex="-1"
  >
    <!-- Header -->
    <div class="flex items-start justify-between">
      <div class="flex items-center gap-3">
        <div
          class="w-10 h-10 rounded-xl bg-warning/15 border border-warning/20 flex items-center justify-center text-warning shrink-0"
        >
          <IconAlertTriangle size={20} />
        </div>
        <div>
          <h2 id="disable-confirm-title" class="text-base font-bold text-text-primary">
            {$_("settings.security.totp_disable_title")}
          </h2>
        </div>
      </div>
      <button
        class="p-1.5 rounded-lg text-text-muted hover:text-text-primary hover:bg-surface/8 transition-all"
        onclick={oncancel}
        aria-label={$_("settings.close")}
        disabled={processing}
      >
        <IconX size={16} />
      </button>
    </div>

    <!-- Warning Subtitle -->
    <div class="p-4 bg-warning/10 border border-warning/20 rounded-xl flex items-start gap-3 text-warning animate-in fade-in slide-in-from-top-1">
      <IconAlertTriangle size={18} class="shrink-0 mt-0.5" />
      <p class="text-xs font-medium leading-relaxed">
        {$_("settings.security.totp_disable_subtitle")}
      </p>
    </div>

    <div class="flex gap-3 pt-1">
      <button
        class="flex-1 px-4 py-2 bg-surface/5 border border-surface/10 rounded-lg text-text-secondary text-sm font-semibold hover:bg-surface/10 transition-all"
        onclick={oncancel}
        disabled={processing}
      >
        {$_("global.cancel")}
      </button>
      <button
        class="flex-1 px-4 py-2 bg-danger text-white rounded-lg text-sm font-bold hover:brightness-110 transition-all disabled:opacity-50 flex items-center justify-center gap-2 shadow-sm shadow-danger/20"
        onclick={onconfirm}
        disabled={processing}
      >
        {#if processing}
          <IconLoader2 size={16} class="animate-spin" />
        {:else}
          <IconTrash size={16} />
          {$_("settings.security.totp_disable_confirm_btn")}
        {/if}
      </button>
    </div>
  </div>
</div>
