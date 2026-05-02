<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconShieldLock } from "@tabler/icons-svelte";
  import { activeView } from "$lib/stores";
</script>

<div class="max-w-[1100px] mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300">
  <div class="mb-8 flex items-center gap-4">
    <button class="p-2 bg-surface/5 hover:bg-surface/10 rounded-lg text-text-muted hover:text-text-primary transition-colors" on:click={() => activeView.set('general')}>
      ← Volver
    </button>
    <div>
      <h1 class="text-[26px] font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent">
        {$_("general_view.tokens_title")}
      </h1>
      <p class="text-text-muted text-[14px] mt-1">
        {$_("general_view.tokens_desc")}
      </p>
    </div>
  </div>

  <div class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl">
    <div class="w-16 h-16 rounded-2xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center mb-2">
      <IconShieldLock size={32} />
    </div>
    <h3 class="text-lg text-text-primary font-semibold">En construcción</h3>
    <p class="text-text-muted max-w-sm">Próximamente podrás generar tokens de verificación en dos pasos (2FA) para tus cuentas.</p>
  </div>
</div>
