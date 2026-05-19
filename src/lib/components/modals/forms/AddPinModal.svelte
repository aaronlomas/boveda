<script lang="ts">
  import { addPin } from "$lib/utils/tauri";
  import { IconEye, IconEyeOff } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { focus } from "$lib/utils/actions";
  import { useForm } from "$lib/validation/useForm.svelte";
  import { pinSchema, type PinForm } from "$lib/validation/schemas";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";

  let { onadded, onclose }: { onadded?: () => void; onclose?: () => void } =
    $props();

  let showPin = $state(false);

  const form = useForm<PinForm>(
    pinSchema,
    { name: "", pin: "", notes: "" },
    async (values) => {
      await addPin(
        values.name.trim(),
        values.pin.trim(),
        values.notes?.trim() || "",
      );
      onadded?.();
      onclose?.();
    },
  );
</script>

<Modal 
  show={true} 
  onclose={onclose} 
  title={$_("pin_security.new_pin")}
>
  <form
    id="add-pin-form"
    class="flex flex-col gap-4"
    onsubmit={(e) => {
      e.preventDefault();
      form.handleSubmit();
    }}
  >
    <!-- Name -->
    <div class="flex flex-col gap-2">
      <label for="pin-name" class="text-xs font-medium text-text-secondary"
        >{$_("pin_security.name_label")}</label
      >
      <input
        id="pin-name"
        class="w-full px-4 py-2 bg-surface/5 border {form.errors.name
          ? 'border-danger'
          : 'border-surface/10'} rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all"
        bind:value={form.values.name}
        placeholder={$_("pin_security.name_placeholder")}
        autocomplete="off"
        use:focus
      />
      {#if form.errors.name}
        <span
          class="text-xs text-danger animate-in fade-in slide-in-from-top-1"
        >
          {$_(`pin_security.${form.errors.name}`)}
        </span>
      {/if}
    </div>

    <!-- PIN -->
    <div class="flex flex-col gap-2">
      <label for="pin-code" class="text-xs font-medium text-text-secondary"
        >{$_("pin_security.pin_label")}</label
      >
      <div class="relative">
        <input
          id="pin-code"
          class="w-full px-4 py-2 bg-surface/5 border {form.errors.pin
            ? 'border-danger'
            : 'border-surface/10'} rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all pr-10"
          type={showPin ? "text" : "password"}
          bind:value={form.values.pin}
          placeholder={$_("pin_security.pin_placeholder")}
          autocomplete="new-password"
        />
        <button
          type="button"
          class="absolute right-2 top-1/2 -translate-y-1/2 bg-none border-none cursor-pointer p-1 text-text-muted hover:text-text-primary opacity-60 hover:opacity-100 transition-all flex items-center"
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
        <span
          class="text-xs text-danger animate-in fade-in slide-in-from-top-1"
        >
          {$_(`pin_security.${form.errors.pin}`)}
        </span>
      {/if}
    </div>

    <!-- Notes -->
    <div class="flex flex-col gap-2">
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

  {#snippet footer()}
    <Button variant="ghost" onclick={onclose}>
      {$_("actions.cancel")}
    </Button>
    <Button 
      type="submit" 
      form="add-pin-form" 
      disabled={form.loading}
      class="min-w-24"
    >
      {#if form.loading}
        <span
          class="w-3 h-3 border-2 border-surface/30 border-t-white rounded-full animate-spin mr-1.5"
        ></span>
        {$_("actions.status.saving")}
      {:else}
        {$_("actions.save")}
      {/if}
    </Button>
  {/snippet}
</Modal>
