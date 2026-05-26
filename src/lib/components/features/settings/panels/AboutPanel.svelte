<script lang="ts">
  import { t } from "$lib/i18n";
  import { IconInfoCircle } from "@tabler/icons-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let appInfo = $state({ app_version: "...", core_version: "..." });

  onMount(async () => {
    try {
      appInfo = await invoke("get_app_info");
    } catch (e) {
      console.error("Failed to fetch app info:", e);
    }
  });
</script>

<div class="space-y-6">
  <div>
    <h2 class="text-lg font-bold text-text-primary flex items-center gap-2">
      <IconInfoCircle class="text-accent" />
      {t("settings.about.title", "Acerca de Bóveda")}
    </h2>
    <p class="text-text-muted text-sm mt-1">
      {t("settings.about.desc", "Información sobre la aplicación y licencias.")}
    </p>
  </div>

  <div class="space-y-4">
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary">Versión</span>
        <span class="text-sm font-mono text-accent">{appInfo.app_version}</span>
      </div>
    </div>
    
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary">Motor Core</span>
        <span class="text-sm font-mono text-text-primary">boveda-core v{appInfo.core_version}</span>
      </div>
    </div>

    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <p class="text-xs text-text-muted leading-relaxed">
        Bóveda es un gestor de credenciales con filosofía "Seguridad por Aislamiento". Nunca usamos la nube, ni ningun servicio externo para guardar tus datos.
      </p>
    </div>
  </div>
</div>
