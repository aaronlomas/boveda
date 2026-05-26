<script lang="ts">
  import { uiState } from "$lib/stores/stores.svelte";
  import { IconKey, IconLock } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10"
>
  <div class="mb-8">
    <h1
      class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent"
    >
      {$_("general_view.general")}
    </h1>
    <p class="text-text-muted text-sm mt-1">
      {$_("general_view.desc")}
    </p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
    <!-- Card Cuentas -->
    <button
      class="flex flex-col items-start text-left p-6 gap-4 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl hover:border-accent/30 hover:bg-surface/7 transition-all group"
      onclick={() => (uiState.activeView = "accounts")}
    >
      <div
        class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform"
      >
        <IconKey size={24} />
      </div>
      <div>
        <h3 class="text-lg font-semibold text-text-primary mb-1">
          {$_("accounts_mode.title")}
        </h3>
        <p class="text-sm text-text-muted">
          {$_("accounts_mode.desc")}
        </p>
      </div>
    </button>

    <!-- Card Save PIN -->
    <button
      class="flex flex-col items-start text-left p-6 gap-4 bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl hover:border-accent/30 hover:bg-surface/7 transition-all group"
      onclick={() => (uiState.activeView = "pin")}
    >
      <div
        class="w-12 h-12 rounded-xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center group-hover:scale-110 transition-transform"
      >
        <IconLock size={24} />
      </div>
      <div>
        <h3 class="text-lg font-semibold text-text-primary mb-1">
          {$_("pin_security_mode.title")}
        </h3>
        <p class="text-sm text-text-muted">{$_("pin_security_mode.desc")}</p>
      </div>
    </button>
  </div>
</div>
