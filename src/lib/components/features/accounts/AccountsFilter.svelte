<script lang="ts">
  /**
   * @component AccountsFilter
   * @description Search input to filter credentials.
   */
  import { _ } from "svelte-i18n";
  import { IconSearch } from "@tabler/icons-svelte";

  let { search = $bindable() }: { search: string } = $props();
</script>

<div
  class="flex items-center mb-4 border border-surface/10 rounded-lg text-text-primary px-4 py-3 focus-within:border-accent/50 transition-colors"
>
  <div class="text-text-muted text-lg">
    <IconSearch size={18} />
  </div>
  <input
    class="w-full pl-4 bg-transparent text-sm placeholder:text-text-muted focus:outline-none transition-all"
    bind:value={search}
    placeholder={$_("accounts.search_placeholder")}
  />
</div>
