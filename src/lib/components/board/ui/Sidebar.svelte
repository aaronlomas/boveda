<script lang="ts">
  import { _ } from "svelte-i18n";
  import { t } from "$lib/i18n";
  import { IconEye, IconInfoCircle } from "@tabler/icons-svelte";
  import type { BoardStore } from "../store.svelte";

  let { store, onviewnotes }: { store: BoardStore, onviewnotes?: () => void } = $props();
</script>

<aside class="w-64 border-l border-surface/8 bg-surface/2 flex-col hidden lg:flex">
  <div class="p-4 border-b border-surface/8">
    <h3 class="text-sm font-bold text-text-primary flex items-center gap-2">
      <IconInfoCircle size={18} class="text-accent" />
      {t("documents.board_info", "Información del Board")}
    </h3>
  </div>
  
  <div class="flex-1 p-4 flex flex-col gap-4">
    <p class="text-xs text-text-muted leading-relaxed">
      {t("documents.board_description", "Utiliza este editor para redactar tus documentos de forma segura. Todo el contenido se cifra localmente.")}
    </p>

    {#if onviewnotes}
      <button
        onclick={onviewnotes}
        class="w-full px-4 py-2 rounded-xl border border-surface/20 text-text-secondary text-sm font-medium hover:bg-surface/10 hover:text-text-primary transition-all flex items-center justify-center gap-2"
      >
        <IconEye size={18} />
        {$_("documents.view_notes")}
      </button>
    {/if}
  </div>
  
  <div class="p-4 border-t border-surface/8 text-[10px] text-text-muted text-center italic">
    Bóveda Secure Editor v1.0
  </div>
</aside>
