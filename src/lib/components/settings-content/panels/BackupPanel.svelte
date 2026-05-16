<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconDatabaseExport } from "@tabler/icons-svelte";
</script>

<div class="space-y-6">
  <div>
    <h2 class="text-lg font-bold text-text-primary flex items-center gap-2">
      <IconDatabaseExport class="text-accent" />
      {$_("settings.backup.title") || "Copia de Seguridad"}
    </h2>
    <p class="text-text-muted text-sm mt-1">
      {$_("settings.backup.desc") || "Gestiona tus respaldos y la exportación de datos."}
    </p>
  </div>

  <div class="p-6 bg-surface/5 border border-surface/10 rounded-2xl flex flex-col items-center justify-center py-12 text-center">
    <div class="w-16 h-16 rounded-full bg-accent/10 text-accent flex items-center justify-center mb-4">
      <IconDatabaseExport size={32} />
    </div>
    <h3 class="font-bold text-text-primary mb-2">Próximamente</h3>
    <p class="text-xs text-text-muted max-w-xs">
      Estamos trabajando en un sistema de respaldo automático y exportación en la nube cifrada.
    </p>
  </div>
</div>
