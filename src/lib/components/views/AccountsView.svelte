<script lang="ts">
  /**
   * @component AccountsView
   * @description Vista principal para la gestión de credenciales (Accounts).
   * Orquesta los componentes especializados AccountsFilter, AccountsGroupFilter
   * y AccountsList, y usa los composables reactivos useAccounts y uiState.
   */
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import { IconPlus, IconShieldLock } from "@tabler/icons-svelte";
  import { uiState } from "$lib/stores/stores.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import { useAccounts } from "$lib/composables/useAccounts.svelte";
  import Button from "../core/primitives/Button.svelte";
  import AccountsFilter from "../features/accounts/AccountsFilter.svelte";
  import AccountsGroupFilter from "../features/accounts/AccountsGroupFilter.svelte";
  import AccountsList from "../features/accounts/AccountsList.svelte";

  // ── Composable e Inicialización ────────────────────────────────────────────
  const accountService = useAccounts();
  const VIRTUAL_SCROLL_THRESHOLD = 100;

  // ── Local UI State ─────────────────────────────────────────────────────────
  let search = $state("");

  // ── Derived State ──────────────────────────────────────────────────────────
  let filtered = $derived(
    accountService.accounts.filter((a) => {
      const matchSearch =
        a.site.toLowerCase().includes(search.toLowerCase()) ||
        a.username.toLowerCase().includes(search.toLowerCase());
      const matchGroup =
        uiState.activeGroup === null
          ? true
          : a.group_name === uiState.activeGroup;
      return matchSearch && matchGroup;
    })
  );

  let useVirtualScroll = $derived(
    performanceStore.massiveList && filtered.length > VIRTUAL_SCROLL_THRESHOLD
  );

  // ── Handlers ───────────────────────────────────────────────────────────────
  async function handleNewCredential() {
    const added = await modal.openAddCredential();
    if (added) {
      await accountService.refresh();
    }
  }

  onMount(() => {
    accountService.refresh();
  });
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10"
>
  <!-- Header -->
  <header class="flex items-center justify-between mb-7 gap-4">
    <div class="flex gap-x-4">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-lg text-text-muted hover:text-text-primary transition-colors my-auto cursor-pointer"
        onclick={() => (uiState.activeView = "general")}
        aria-label="Back"
      >
        ←
      </button>
      <div>
        <h1
          class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent pointer-events-none"
        >
          {$_("accounts.title")}
        </h1>
        <p class="text-text-muted text-sm mt-0.5 pointer-events-none">
          {$_("accounts.credentials_count", {
            values: { count: accountService.accounts.length },
          })}
        </p>
      </div>
    </div>

    <Button
      variant="primary"
      onclick={handleNewCredential}
      class="shadow-md shadow-accent/20"
    >
      <IconPlus size={16} class="mr-2" />
      {$_("accounts.new_credential")}
    </Button>
  </header>

  <!-- Filter Input -->
  <AccountsFilter bind:search />

  <!-- Group Management & Filtering Chips -->
  <AccountsGroupFilter />

  <!-- Main Content View -->
  {#if accountService.error}
    <div class="text-center py-16 flex flex-col items-center gap-3">
      <p
        class="text-danger text-sm bg-danger/10 border border-danger/20 rounded-xl px-6 py-4"
      >
        {accountService.error}
      </p>
      <button
        class="text-sm text-accent underline cursor-pointer"
        onclick={() => accountService.refresh()}
      >
        {$_("actions.retry")}
      </button>
    </div>
  {:else if filtered.length === 0}
    <div
      class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary"
    >
      <div class="text-accent/20 mb-2">
        <IconShieldLock size={80} stroke={1.5} />
      </div>
      <h3 class="text-lg text-text-primary font-semibold">
        {uiState.activeGroup
          ? $_("groups.no_accounts_in_group", { values: { group: uiState.activeGroup } })
          : $_("accounts.no_credentials")}
      </h3>
      <p class="text-text-muted">
        {uiState.activeGroup ? "" : $_("accounts.no_credentials_desc")}
      </p>
      {#if !uiState.activeGroup}
        <Button
          variant="primary"
          onclick={handleNewCredential}
          class="mt-2 shadow-lg shadow-accent/20"
        >
          <IconPlus size={16} class="mr-2" />
          {$_("accounts.new_credential")}
        </Button>
      {/if}
    </div>
  {:else}
    <AccountsList
      {filtered}
      {useVirtualScroll}
      ondelete={(id) => accountService.delete(id)}
      onrefresh={() => accountService.refresh()}
    />
  {/if}
</div>
