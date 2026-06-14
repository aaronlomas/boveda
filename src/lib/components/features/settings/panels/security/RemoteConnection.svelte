<script lang="ts">
  import Switch from "$lib/components/core/primitives/Switch.svelte";
  import { IconSpyOff } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  let isEnabled = false;
</script>

<div
  class="w-10 h-10 rounded-full flex items-center justify-center {isEnabled
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
<Switch />
