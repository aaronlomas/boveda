<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconFileDescription } from "@tabler/icons-svelte";
</script>

<div class="max-w-[1100px] mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300">
  <div class="mb-8">
    <h1 class="text-[26px] font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent">
      {$_("views.documents_title")}
    </h1>
    <p class="text-text-muted text-[14px] mt-1">
      {$_("views.documents_desc")}
    </p>
  </div>

  <div class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl">
    <div class="w-16 h-16 rounded-2xl bg-accent/10 text-accent-light border border-accent/20 flex items-center justify-center mb-2">
      <IconFileDescription size={32} />
    </div>
    <h3 class="text-lg text-text-primary font-semibold">En construcción</h3>
    <p class="text-text-muted max-w-sm">Pronto podrás guardar documentos, imágenes y notas cifradas de forma segura dentro de tu bóveda local.</p>
  </div>
</div>
