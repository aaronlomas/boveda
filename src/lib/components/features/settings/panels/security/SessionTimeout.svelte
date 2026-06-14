<script lang="ts">
  import { _ } from "svelte-i18n";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Switch from "$lib/components/core/primitives/Switch.svelte";
  import { IconClock } from "@tabler/icons-svelte";
  import { updateAutoLockSeconds } from "$lib/utils/autoLock";

  const PREF_KEY = "session_timeout_seconds";
  const DEFAULT_SECONDS = 60;

  let isEnabled = $state(false);
  let rangeValue: number = $state(DEFAULT_SECONDS);
  const maxRange = 300;
  const minRange = 10;

  // Load persisted preference on mount
  onMount(async () => {
    const raw = await invoke<string | null>("get_preference", {
      key: PREF_KEY,
    });
    if (raw !== null) {
      const saved = parseInt(raw, 10);
      if (saved > 0) {
        rangeValue = saved;
        isEnabled = true;
        updateAutoLockSeconds(saved);
      }
    }
  });

  // Reactively save and apply whenever toggle or slider changes
  $effect(() => {
    const seconds = isEnabled ? rangeValue : 0;
    updateAutoLockSeconds(seconds);
    invoke("set_preference", {
      key: PREF_KEY,
      value: isEnabled ? String(rangeValue) : "0",
    }).catch(console.error);
  });
</script>

<div
  class="w-10 h-10 rounded-full flex items-center justify-center transition-colors {isEnabled
    ? 'bg-accent/10 text-accent'
    : 'bg-surface/10 text-text-muted'}"
>
  <IconClock size={20} />
</div>
<div>
  <h2 class="text-sm font-semibold text-text-primary">
    {$_("settings.automatic_session_close.title")}
  </h2>
  <p class="text-xs text-text-muted">
    {$_("settings.automatic_session_close.desc")}
  </p>
</div>

<Switch bind:checked={isEnabled} />

{#if isEnabled}
  <div
    class="pt-4 border-t border-surface/8 animate-in fade-in slide-in-from-top-2 col-span-3"
  >
    <div class="flex justify-between text-sm text-text-muted mb-3">
      <span>{$_("settings.automatic_session_close.timer_prefix")}</span>
      <span class="font-medium text-text-primary"
        >{rangeValue}{$_("settings.automatic_session_close.timer_suffix")}</span
      >
    </div>

    <input
      type="range"
      step="10"
      bind:value={rangeValue}
      max={maxRange}
      min={minRange}
      class="w-full accent-accent cursor-pointer"
    />
    <div class="flex justify-between text-xs text-text-muted mt-1">
      <span>{minRange}s</span>
      <span>{maxRange}s</span>
    </div>
  </div>
{/if}
