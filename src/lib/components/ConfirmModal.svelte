<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { IconAlertTriangle } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  export let title = $_("global.confirm");
  export let message = "";
  export let confirmText = $_("global.confirm");
  export let cancelText = $_("global.cancel");
  export let type: "danger" | "warning" | "info" = "danger";

  const dispatch = createEventDispatcher();

  function confirm() {
    dispatch("confirm");
  }

  function close(e?: MouseEvent | KeyboardEvent) {
    // Simulamos el comportamiento de |self manualmente para Svelte 5
    if (e && e.target !== e.currentTarget) return;
    dispatch("cancel");
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") close();
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
  on:click={close}
  on:keydown={handleKeydown}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
>
  <div class="w-full max-w-[440px] bg-white/4 backdrop-blur-2xl border border-white/10 rounded-2xl p-8 shadow-2xl relative">
    <div class="flex items-center gap-3 mb-4">
      <div class="w-9 h-9 rounded-lg grid place-items-center {type === 'danger' ? 'bg-danger/15 text-[#f87171] border border-danger/20' : ''}">
        <IconAlertTriangle size={20} />
      </div>
      <h2 class="text-lg font-bold text-text-primary">{title}</h2>
    </div>

    <div class="py-3 pb-6">
      <p class="text-text-secondary text-[14px] leading-relaxed">{message}</p>
    </div>

    <div class="flex gap-3 justify-end border-t border-white/8 pt-4">
      <button type="button" class="inline-flex items-center justify-center h-9.5 px-5 rounded-sm text-[14px] font-bold cursor-pointer transition-all border border-white/10 bg-white/5 text-text-secondary hover:bg-white/10 hover:text-text-primary min-w-[100px]" on:click={() => dispatch("cancel")}>
        {cancelText}
      </button>
      <button 
        type="button" 
        class="inline-flex items-center justify-center h-9.5 px-5 rounded-sm text-[14px] font-bold cursor-pointer transition-all border-none text-white min-w-[100px] shadow-lg hover:-translate-y-px active:scale-95 {type === 'danger' ? 'bg-danger shadow-danger/30 hover:bg-[#f87171]' : 'bg-accent shadow-accent/20 hover:brightness-110'}"
        on:click={confirm}
      >
        {confirmText}
      </button>
    </div>
  </div>
</div>

<style>
  /* Tailwind handles most styles now */
</style>
