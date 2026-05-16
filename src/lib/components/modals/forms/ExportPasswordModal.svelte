<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock, IconEye, IconEyeOff, IconDownload, IconDatabaseExport } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";
  
  let { onconfirm, oncancel, title, desc = "", buttonText } = $props<{
    onconfirm: (password: string) => void;
    oncancel: () => void;
    title: string;
    desc?: string;
    buttonText: string;
  }>();

  let password = $state("");
  let showPassword = $state(false);
  let error = $state("");

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (password.length < 8) {
      error = $_("export_pack.password_error");
      return;
    }
    onconfirm(password);
  }
</script>

<Modal 
  show={true} 
  onclose={oncancel} 
  title={$_(title)}
>
  <div class="space-y-6">
    <div class="flex items-center gap-3 mb-2">
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent flex items-center justify-center shrink-0">
        <IconDatabaseExport size={24} />
      </div>
      <div>
        {#if desc}
          <p class="text-xs text-text-muted">{$_(desc)}</p>
        {/if}
      </div>
    </div>

    <div class="bg-accent/5 border border-accent/10 rounded-xl p-4">
      <p class="text-xs text-text-secondary leading-relaxed">
        {$_("export_pack.warning")}
      </p>
    </div>

    <form onsubmit={handleSubmit} class="space-y-4" id="export-form">
      <div class="space-y-2">
        <label for="export-pw" class="text-xs font-bold text-text-muted uppercase tracking-wider">
          {$_("export_pack.password_label")}
        </label>
        <div class="relative">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted">
            <IconLock size={18} />
          </div>
          <input
            id="export-pw"
            type={showPassword ? "text" : "password"}
            bind:value={password}
            placeholder="••••••••••••"
            class="w-full bg-surface/5 border border-surface/10 rounded-lg pl-10 pr-12 py-3 text-text-primary placeholder:text-text-muted/30 focus:outline-none focus:border-accent transition-colors text-sm"
            required
            use:focus
          />
          <button
            type="button"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-text-muted hover:text-text-primary transition-colors"
            onclick={() => (showPassword = !showPassword)}
          >
            {#if showPassword}
              <IconEyeOff size={18} />
            {:else}
              <IconEye size={18} />
            {/if}
          </button>
        </div>
        {#if error}
          <p class="text-[10px] text-danger font-medium animate-in fade-in slide-in-from-top-1">
            {error}
          </p>
        {/if}
      </div>
    </form>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={oncancel}>
      {$_("global.cancel")}
    </Button>
    <Button type="submit" form="export-form" class="gap-2">
      <IconDownload size={18} />
      {$_(buttonText)}
    </Button>
  {/snippet}
</Modal>
