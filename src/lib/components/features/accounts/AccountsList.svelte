<script lang="ts">
  /**
   * @component AccountsList
   * @description Component for rendering the credentials list, supporting virtualized scrolling.
   * Componente para renderizar la lista de credenciales, soportando scroll virtualizado.
   */
  import { locale } from "svelte-i18n";
  import { IconRocket } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import VirtualList from "svelte-virtual-list";
  import AccountCard from "./AccountCard.svelte";
  import type { Account } from "$lib/stores/data.svelte";

  let {
    filtered,
    useVirtualScroll,
    ondelete,
    onrefresh,
  }: {
    filtered: Account[];
    useVirtualScroll: boolean;
    ondelete: (id: string) => void;
    onrefresh: () => void;
  } = $props();
</script>

{#if useVirtualScroll}
  <!-- Massive List active banner -->
  <!-- Banner activo de lista masiva -->
  <div
    class="flex items-center gap-2 mb-4 px-3 py-2 rounded-lg bg-accent/10 border border-accent/20 text-accent-light text-xs font-medium"
  >
    <IconRocket size={14} />
    {$_("settings.performance.massive_list_active_badge", {
      values: { count: filtered.length },
    })}
  </div>

  <!-- Virtual scrolling list (single column) -->
  <!-- Lista de desplazamiento virtual (columna única) -->
  <div class="virtual-list-wrapper">
    <VirtualList items={filtered} let:item itemHeight={220}>
      <div class="pb-4">
        <AccountCard
          account={item}
          locale={$locale ?? "es"}
          ondelete={ondelete}
          onrefresh={onrefresh}
        />
      </div>
    </VirtualList>
  </div>
{:else}
  <!-- Normal grid -->
  <div class="grid grid-cols-[repeat(auto-fill,minmax(320px,1fr))] gap-4">
    {#each filtered as account (account.id)}
      <AccountCard
        {account}
        locale={$locale ?? "es"}
        ondelete={ondelete}
        onrefresh={onrefresh}
      />
    {/each}
  </div>
{/if}
