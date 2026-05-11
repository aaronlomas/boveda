<script lang="ts">
  import { _ } from "svelte-i18n";
  import { 
    IconShieldLock, 
    IconLock, 
    IconEye, 
    IconEyeOff, 
    IconDatabaseImport,
    IconCopy,
    IconReplace
  } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  
  let { onconfirm, oncancel, title, desc, buttonText } = $props<{
    onconfirm: (password: string, strategy: 'merge' | 'replace') => void;
    oncancel: () => void;
    title: string;
    desc: string;
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

<div class="fixed inset-0 z-100 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-in fade-in duration-200">
  <div 
    class="w-full max-w-md bg-panel border border-surface/10 rounded-2xl shadow-2xl overflow-hidden animate-in zoom-in-95 duration-200"
    role="dialog"
    aria-modal="true"
  >
    <div class="p-6">
      <div class="flex items-center gap-3 mb-6">
        <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent flex items-center justify-center">
          <IconShieldLock size={24} />
        </div>
        <div>
          <h2 class="text-xl font-bold text-text-primary">{$_(title)}</h2>
          <p class="text-sm text-text-muted">{$_(desc)}</p>
        </div>
      </div>

      <form onsubmit={handleSubmit} class="space-y-6">
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
              class="w-full bg-surface/5 border border-surface/10 rounded-lg pl-10 pr-12 py-3 text-text-primary placeholder:text-text-muted/30 focus:outline-none focus:border-accent transition-colors"
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

        <div class="flex gap-3 pt-2">
          <button
            type="button"
            class="flex-1 py-3 px-4 bg-surface/5 hover:bg-surface/10 text-text-primary font-bold rounded-xl transition-all"
            onclick={oncancel}
          >
            {$_("global.cancel")}
          </button>
          <button
            type="submit"
            class="flex-2 py-3 px-6 bg-accent hover:bg-accent-hover text-white font-bold rounded-xl transition-all shadow-lg shadow-accent/20 flex items-center justify-center gap-2"
          >
            <IconDatabaseImport size={20} />
            {$_(buttonText)}
          </button>
        </div>
      </form>
    </div>
  </div>
</div>
