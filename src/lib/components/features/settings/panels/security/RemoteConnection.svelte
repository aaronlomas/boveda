<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Switch from "$lib/components/core/primitives/Switch.svelte";
  import { IconSpyOff } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  const PREF_KEY = "security.block_remote";
  let isEnabled = $state(false);

  onMount(async () => {
    const raw = await invoke<string | null>("get_preference", {
      key: PREF_KEY,
    });
    if (raw === "true") {
      isEnabled = true;
    }
  });

  $effect(() => {
    invoke("set_preference", {
      key: PREF_KEY,
      value: isEnabled ? "true" : "false",
    }).catch(console.error);
  });
</script>

<div
  class="w-10 h-10 rounded-full flex items-center justify-center transition-colors {isEnabled
    ? 'bg-accent/10 text-accent'
    : 'bg-surface/10 text-text-muted'}"
>
  <IconSpyOff size={20} />
</div>
<div>
  <h2 class="text-sm font-semibold text-text-primary">
    {$_("settings.remote_connection.title")}
  </h2>
  <p class="text-xs text-text-muted">
    {$_("settings.remote_connection.desc")}
  </p>
</div>
<Switch bind:checked={isEnabled} />
