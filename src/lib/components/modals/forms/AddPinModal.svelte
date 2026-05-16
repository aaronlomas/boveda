<script lang="ts">
  import { addPin } from "$lib/utils/tauri";
  import { IconEye, IconEyeOff } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { focus } from "$lib/utils/actions";
  import { useForm } from "$lib/validation/useForm";
  import { pinSchema, type PinForm } from "$lib/validation/schemas";

  let { onadded, onclose }: { onadded?: () => void; onclose?: () => void } =
    $props();

  let showPin = $state(false);

  const form = useForm<PinForm>(
    pinSchema,
    { name: "", pin: "", notes: "" },
    async (values) => {
      await addPin(values.name.trim(), values.pin.trim(), values.notes?.trim() || "");
      onadded?.();
      onclose?.();
    }
  );

  function close() {
    onclose?.();
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-panel/60 backdrop-blur-sm"
  onclick={(e) => {
    if (e.target === e.currentTarget) close();
  }}
  onkeydown={(e) => {
    if (e.key === "Escape") close();
  }}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
>
  <div
    class="w-full max-w-md max-h-[90vh] bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl shadow-2xl relative flex flex-col overflow-hidden"
  >
    <!-- Header -->
    <div
      class="flex items-center justify-between p-6 pb-4 border-b border-surface/5 shrink-0"
    >
      <h2 class="text-lg font-bold text-text-primary">
        {$_("pin_security.new_pin")}
      </h2>
    </div>

    <!-- Scrollable Body -->
    <div class="flex-1 overflow-y-auto p-6 pt-4 custom-scrollbar">
      <form
        id="add-pin-form"
        class="flex flex-col gap-4"
        onsubmit={(e) => {
          e.preventDefault();
          form.handleSubmit();
        }}
      >
        <!-- Name -->
        <div class="flex flex-col gap-1.5">
          <label for="pin-name" class="text-xs font-medium text-text-secondary"
            >{$_("pin_security.name_label")}</label
          >
          <input
            id="pin-name"
            class="w-full px-4 py-2.5 bg-surface/5 border {form.errors.name ? 'border-danger' : 'border-surface/10'} rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all"
            bind:value={form.values.name}
            placeholder={$_("pin_security.name_placeholder")}
            autocomplete="off"
            use:focus
          />
          {#if form.errors.name}
            <span class="text-[11px] text-danger animate-in fade-in slide-in-from-top-1">
              {$_(`pin_security.${form.errors.name}`)}
            </span>
          {/if}
        </div>

        <!-- PIN -->
        <div class="flex flex-col gap-1.5">
          <label for="pin-code" class="text-xs font-medium text-text-secondary"
            >{$_("pin_security.pin_label")}</label
          >
          <div class="relative">
            <input
              id="pin-code"
              class="w-full px-4 py-2.5 bg-surface/5 border {form.errors.pin ? 'border-danger' : 'border-surface/10'} rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all pr-10"
              type={showPin ? "text" : "password"}
              bind:value={form.values.pin}
              placeholder={$_("pin_security.pin_placeholder")}
              autocomplete="new-password"
            />
            <button
              type="button"
              class="absolute right-2.5 top-1/2 -translate-y-1/2 bg-none border-none cursor-pointer p-1 text-text-muted hover:text-text-primary opacity-60 hover:opacity-100 transition-all flex items-center"
              onclick={() => (showPin = !showPin)}
            >
              {#if showPin}
                <IconEyeOff size={18} />
              {:else}
                <IconEye size={18} />
              {/if}
            </button>
          </div>
          {#if form.errors.pin}
            <span class="text-[11px] text-danger animate-in fade-in slide-in-from-top-1">
              {$_(`pin_security.${form.errors.pin}`)}
            </span>
          {/if}
        </div>

        <!-- Notes -->
        <div class="flex flex-col gap-1.5">
          <label for="pin-notes" class="text-xs font-medium text-text-secondary"
            >{$_("pin_security.notes_label")}</label
          >
          <textarea
            id="pin-notes"
            class="w-full px-4 py-2.5 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all resize-vertical min-h-18"
            bind:value={form.values.notes}
            placeholder={$_("pin_security.notes_placeholder")}
            rows="3"
          ></textarea>
        </div>

        {#if form.globalError}
          <p
            class="text-danger text-xs py-2 px-3 bg-danger/10 border border-danger/20 rounded-md"
          >
            {form.globalError}
          </p>
        {/if}
      </form>
    </div>

    <!-- Fixed Footer -->
    <div
      class="p-6 pt-4 border-t border-surface/5 flex gap-2.5 justify-end shrink-0"
    >
      <button
        type="button"
        class="inline-flex items-center justify-center h-10 px-5 rounded-sm text-sm font-bold cursor-pointer transition-all border border-surface/10 bg-surface/5 text-text-secondary hover:bg-surface/10 hover:text-text-primary min-w-24"
        onclick={close}>{$_("pin_security.cancel_button")}</button
      >
      <button
        form="add-pin-form"
        type="submit"
        class="inline-flex items-center justify-center h-10 px-5 rounded-sm text-sm font-bold cursor-pointer transition-all border-none bg-accent text-white shadow-lg shadow-accent/20 hover:brightness-110 hover:-translate-y-px active:scale-95 disabled:opacity-60 disabled:cursor-not-allowed min-w-24 gap-1.5"
        disabled={form.loading}
      >
        {#if form.loading}
          <span
            class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin"
          ></span>
          {$_("pin_security.save_button")}...
        {:else}
          {$_("pin_security.save_button")}
        {/if}
      </button>
    </div>
  </div>
</div>
