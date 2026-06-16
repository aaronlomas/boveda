<script lang="ts">
  import { _ } from "svelte-i18n";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import Switch from "$lib/components/core/primitives/Switch.svelte";
  import ListItem from "$lib/components/core/primitives/ListItem.svelte";
  import { IconRocket } from "@tabler/icons-svelte";
</script>

<!-- ─ Performance Section ─ -->
<h1 class="font-bold text-xl text-text-primary">
  {$_("settings.performance.title")}
</h1>

<ListItem layout="triple">
  <div
    class="p-2 rounded-full
      {performanceStore.massiveList
      ? 'bg-accent/20 text-accent'
      : 'bg-surface/8 text-text-muted'}"
  >
    <IconRocket size={20} />
  </div>

  <div>
    <h2 class="text-sm font-bold text-text-primary">
      {$_("settings.performance.massive_list_title")}
    </h2>
    <p class="text-xs text-text-muted">
      {$_("settings.performance.massive_list_desc")}
    </p>
  </div>

  <Switch bind:checked={performanceStore.massiveList} />
</ListItem>
