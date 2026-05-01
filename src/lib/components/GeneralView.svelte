<script lang="ts">
  import { activeView } from "$lib/stores";
  import { IconKey, IconLock, IconShieldLock } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
</script>

<div class="max-w-[1100px] mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300">
  <div class="mb-8">
    <h1 class="text-[26px] font-bold bg-linear-to-br from-white to-accent-light bg-clip-text text-transparent">
      {$_("sidebar.general")}
    </h1>
    <p class="text-text-muted text-[14px] mt-1">
      Selecciona una categoría para administrar tu seguridad.
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
    <!-- Card Cuentas -->
    <button class="flex flex-col items-start text-left p-6 gap-4 bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl hover:border-accent/30 hover:bg-white/7 transition-all group cursor-pointer" on:click={() => activeView.set('accounts')}>
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform">
        <IconKey size={24} />
      </div>
      <div>
        <h3 class="text-lg font-semibold text-text-primary mb-1">{$_("general_view.accounts_title")}</h3>
        <p class="text-sm text-text-muted">{$_("general_view.accounts_desc")}</p>
      </div>
    </button>

    <!-- Card Seguridad PIN -->
    <button class="flex flex-col items-start text-left p-6 gap-4 bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl hover:border-accent/30 hover:bg-white/7 transition-all group cursor-pointer relative overflow-hidden" on:click={() => activeView.set('pin')}>
      <div class="absolute top-4 right-4 bg-white/10 text-white text-[10px] uppercase font-bold px-2 py-1 rounded-sm">
        {$_("general_view.coming_soon")}
      </div>
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform">
        <IconLock size={24} />
      </div>
      <div class="opacity-80">
        <h3 class="text-lg font-semibold text-text-primary mb-1">{$_("general_view.pin_title")}</h3>
        <p class="text-sm text-text-muted">{$_("general_view.pin_desc")}</p>
      </div>
    </button>

    <!-- Card Tokens / Claves dinámicas -->
    <button class="flex flex-col items-start text-left p-6 gap-4 bg-white/4 backdrop-blur-2xl rounded-2xl border border-white/8 shadow-xl hover:border-accent/30 hover:bg-white/7 transition-all group cursor-pointer relative overflow-hidden" on:click={() => activeView.set('tokens')}>
      <div class="absolute top-4 right-4 bg-white/10 text-white text-[10px] uppercase font-bold px-2 py-1 rounded-sm">
        {$_("general_view.coming_soon")}
      </div>
      <div class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform">
        <IconShieldLock size={24} />
      </div>
      <div class="opacity-80">
        <h3 class="text-lg font-semibold text-text-primary mb-1">{$_("general_view.tokens_title")}</h3>
        <p class="text-sm text-text-muted">{$_("general_view.tokens_desc")}</p>
      </div>
    </button>
  </div>
</div>
