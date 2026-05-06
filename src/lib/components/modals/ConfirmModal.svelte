<script lang="ts">
  import { IconAlertTriangle } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  // ── Props (Svelte 5 runes) ──────────────────────────────────────────────────
  let {
    title = $_("global.confirm"),
    message = "",
    confirmText = $_("global.confirm"),
    cancelText = $_("global.cancel"),
    type = "danger" as "danger" | "warning" | "info",
    onconfirm,
    oncancel,
  }: {
    title?: string;
    message?: string;
    confirmText?: string;
    cancelText?: string;
    type?: "danger" | "warning" | "info";
    onconfirm?: () => void;
    oncancel?: () => void;
  } = $props();

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) oncancel?.();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") oncancel?.();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-panel/60 backdrop-blur-sm"
  onclick={handleBackdropClick}
  onkeydown={handleKeydown}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
>
  <div
    class="w-full max-w-md bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl p-8 shadow-2xl relative"
  >
    <div class="flex items-center gap-3 mb-4">
      <div
        class="w-9 h-9 rounded-lg grid place-items-center {type === 'danger'
          ? 'bg-danger/15 text-danger border border-danger/20'
          : ''}"
      >
        <IconAlertTriangle size={20} />
      </div>
      <h2 class="text-lg font-bold text-text-primary">{title}</h2>
    </div>

    <div class="py-3 pb-6">
      <p class="text-text-secondary text-sm leading-relaxed">{message}</p>
    </div>

    <div class="flex gap-3 justify-end border-t border-surface/8 pt-4">
      <button
        type="button"
        class="inline-flex items-center justify-center h-10 px-5 rounded-sm text-sm font-bold cursor-pointer transition-all border border-surface/10 bg-surface/5 text-text-secondary hover:bg-surface/10 hover:text-text-primary min-w-24"
        onclick={() => oncancel?.()}
      >
        {cancelText}
      </button>
      <button
        type="button"
        class="inline-flex items-center justify-center h-10 px-5 rounded-sm text-sm font-bold cursor-pointer transition-all border-none text-white min-w-24 shadow-sm hover:-translate-y-px active:scale-95
               {type === 'danger'
          ? 'bg-danger shadow-danger/30 hover:brightness-110'
          : 'bg-accent shadow-accent/20 hover:brightness-110'}"
        onclick={() => onconfirm?.()}
      >
        {confirmText}
      </button>
    </div>
  </div>
</div>
