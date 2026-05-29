<script lang="ts">
  import { _ } from "svelte-i18n";
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
      {$_("settings.about.title")}
    </h2>
    <p class="text-text-muted text-sm mt-1">
      {$_("settings.about.desc")}
    </p>
  </div>

  <div class="space-y-4">
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary">{$_("settings.about.version_label")}</span>
        <span class="text-sm font-mono text-accent">{appInfo.app_version}</span>
      </div>
    </div>
    
    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <div class="flex justify-between items-center">
        <span class="text-sm text-text-secondary">{$_("settings.about.core_label")}</span>
        <span class="text-sm font-mono text-text-primary">boveda-core v{appInfo.core_version}</span>
      </div>
    </div>

    <div class="p-4 bg-surface/5 border border-surface/10 rounded-xl">
      <p class="text-xs text-text-muted leading-relaxed">
        {$_("settings.about.philosophy")}
      </p>
    </div>
  </div>
</div>
