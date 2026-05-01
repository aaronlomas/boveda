<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconLock } from "@tabler/icons-svelte";
  import { activeView } from "$lib/stores";
</script>

<div class="max-w-[1100px] mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300">
  <div class="mb-8 flex items-center gap-4">
    <button class="p-2 bg-white/5 hover:bg-white/10 rounded-lg text-text-muted hover:text-white transition-colors" on:click={() => activeView.set('general')}>
      ← Volver
    </button>
    <div>
      <h1 class="text-[26px] font-bold bg-linear-to-br from-white to-accent-light bg-clip-text text-transparent">
        {$_("general_view.pin_title")}
      </h1>
      <p class="text-text-muted text-[14px] mt-1">
        {$_("general_view.pin_desc")}
      </p>
    </div>
  </div>

  <div class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl">
    <div class="w-16 h-16 rounded-2xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center mb-2">
      <IconLock size={32} />
    </div>
    <h3 class="text-lg text-text-primary font-semibold">En construcción</h3>
    <p class="text-text-muted max-w-sm">Próximamente podrás agregar protección con código PIN para mayor seguridad.</p>
  </div>
</div>
