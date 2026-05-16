<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconInfoCircle } from "@tabler/icons-svelte";
</script>

<div class="space-y-6">
  <div>
    <h2 class="text-lg font-bold text-text-primary flex items-center gap-2">
      <IconInfoCircle class="text-accent" />
      {$_("settings.about.title") || "Acerca de Bóveda"}
    </h2>
    <p class="text-text-muted text-sm mt-1">
      {$_("settings.about.desc") || "Información sobre la aplicación y licencias."}
    </p>
  </div>

  <div class="space-y-4">
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary">Versión</span>
        <span class="text-sm font-mono text-accent">1.0.0-beta</span>
      </div>
    </div>
    
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary">Motor Core</span>
        <span class="text-sm font-mono text-text-primary">boveda-core v0.8.2</span>
      </div>
    </div>

    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <p class="text-xs text-text-muted leading-relaxed">
        Bóveda es un gestor de contraseñas de código abierto enfocado en la seguridad y la privacidad total del usuario. Todos los datos se cifran localmente antes de ser guardados.
      </p>
    </div>
  </div>
</div>
