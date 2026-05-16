<script lang="ts">
  import { _ } from "svelte-i18n";
  import { 
    IconLock, 
    IconEye, 
    IconEyeOff, 
    IconDatabaseImport,
    IconCopy,
    IconReplace
  } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";
  
  let { onconfirm, oncancel, title, desc = "", buttonText } = $props<{
    onconfirm: (password: string, strategy: 'merge' | 'replace') => void;
    oncancel: () => void;
    title: string;
    desc?: string;
    buttonText: string;
  }>();

  let password = $state("");
  let strategy = $state<'merge' | 'replace'>('merge');
  let showPassword = $state(false);
  let error = $state("");

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (password.length < 8) {
      error = "La contraseña debe tener al menos 8 caracteres.";
      return;
    }
    onconfirm(password, strategy);
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
        <IconDatabaseImport size={24} />
      </div>
      <div>
        {#if desc}
          <p class="text-xs text-text-muted">{$_(desc)}</p>
        {/if}
      </div>
    </div>

    <form onsubmit={handleSubmit} class="space-y-6" id="import-form">
      <!-- Strategy Selection -->
      <div class="space-y-3">
        <span class="text-xs font-bold text-text-muted uppercase tracking-wider block mb-1">
          {$_("import_pack.strategy_label")}
        </span>
        
        <div class="grid grid-cols-2 gap-3">
          <button
            type="button"
            class="flex flex-col items-center gap-2 p-3 rounded-xl border transition-all text-center {strategy === 'merge' ? 'bg-accent/10 border-accent text-accent' : 'bg-surface/5 border-surface/10 text-text-muted hover:bg-surface/10'}"
            onclick={() => strategy = 'merge'}
          >
            <IconCopy size={24} />
            <div class="flex flex-col">
              <span class="text-sm font-bold">{$_("import_pack.keep_both")}</span>
              <span class="text-[10px] opacity-70">{$_("import_pack.keep_both_desc")}</span>
            </div>
          </button>

          <button
            type="button"
            class="flex flex-col items-center gap-2 p-3 rounded-xl border transition-all text-center {strategy === 'replace' ? 'bg-danger/10 border-danger text-danger' : 'bg-surface/5 border-surface/10 text-text-muted hover:bg-surface/10'}"
            onclick={() => strategy = 'replace'}
          >
            <IconReplace size={24} />
            <div class="flex flex-col">
              <span class="text-sm font-bold">{$_("import_pack.replace")}</span>
              <span class="text-[10px] opacity-70">{$_("import_pack.replace_desc")}</span>
            </div>
          </button>
        </div>
      </div>

      <!-- Password Field -->
      <div class="space-y-2">
        <label for="import-pw" class="text-xs font-bold text-text-muted uppercase tracking-wider">
          {$_("import_pack.password_label")}
        </label>
        <div class="relative">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted">
            <IconLock size={18} />
          </div>
          <input
            id="import-pw"
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
            {$_("import_pack.password_error")}
          </p>
        {/if}
      </div>
    </form>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={oncancel}>
      {$_("global.cancel")}
    </Button>
    <Button type="submit" form="import-form" class="gap-2">
      <IconDatabaseImport size={18} />
      {$_(buttonText)}
    </Button>
  {/snippet}
</Modal>
