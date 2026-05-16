<script lang="ts">
  import { onMount } from "svelte";
  import { globalState } from "$lib/stores/stores.svelte";
  import { getAccounts, deleteAccount, getGroups, saveGroups, renameGroup, deleteGroup } from "$lib/utils/tauri";
  import { _, locale } from "svelte-i18n";
  import { toast } from "$lib/stores/toast.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { performanceStore } from "$lib/stores/performance.svelte";
  import CredentialCard from "../ui/CredentialCard.svelte";
  import Button from "../ui/primitives/Button.svelte";
  import VirtualList from "svelte-virtual-list";
  import { IconPlus, IconSearch, IconRocket, IconPencil, IconTrash, IconCheck, IconX, IconShieldLock } from "@tabler/icons-svelte";
  import { focus, selectOnFocus } from "$lib/utils/actions";

  // ── State ───────────────────────────────────────────────────────────────────
  let search = $state("");
  let loadError = $state<string | null>(null);

  // Group management state
  let editingGroup = $state<string | null>(null);   // group name being renamed
  let editingValue = $state("");                     // current input value for rename
  let addingGroup = $state(false);                   // is the new-group input visible?
  let newGroupName = $state("");                     // value for the new-group input

  const VIRTUAL_SCROLL_THRESHOLD = 100;

  let filtered = $derived(
    globalState.accounts.filter((a) => {
      const matchSearch =
        a.site.toLowerCase().includes(search.toLowerCase()) ||
        a.username.toLowerCase().includes(search.toLowerCase());
      const matchGroup =
        globalState.activeGroup === null
          ? true
          : a.group_name === globalState.activeGroup;
      return matchSearch && matchGroup;
    }),
  );

  /** True only when user opted-in AND there are enough items to warrant it */
  let useVirtualScroll = $derived(
    performanceStore.massiveList && filtered.length > VIRTUAL_SCROLL_THRESHOLD,
  );

  onMount(async () => {
    await refresh();
    // Load persisted groups
    globalState.groups = await getGroups();
  });

  // ── Actions ─────────────────────────────────────────────────────────────────

  async function refresh(): Promise<void> {
    try {
      loadError = null;
      globalState.accounts = await getAccounts();
    } catch (e) {
      console.error("Failed to load accounts:", e);
      loadError = $_("accounts.load_error");
    }
  }

  function handleDelete(id: string): void {
    modal.openConfirm({
      title: $_("accounts.delete_confirm_title"),
      message: $_("accounts.delete_confirm_message"),
      confirmText: $_("accounts.delete_confirm_button"),
      type: "danger",
      onconfirm: async () => {
        try {
          await deleteAccount(id);
          await refresh();
          toast.success($_("accounts.delete_success"));
        } catch (e) {
          console.error("Delete failed:", e);
          toast.error($_("accounts.delete_error"));
        }
      },
    });
  }

  // ── Group panel actions ─────────────────────────────────────────────────────

  async function handleAddGroup(): Promise<void> {
    const trimmed = newGroupName.trim();
    if (!trimmed || globalState.groups.includes(trimmed)) return;
    const updated = [...globalState.groups, trimmed];
    await saveGroups(updated);
    globalState.groups = updated;
    newGroupName = "";
    addingGroup = false;
  }

  function startRename(group: string): void {
    editingGroup = group;
    editingValue = group;
  }

  async function confirmRename(): Promise<void> {
    const trimmed = editingValue.trim();
    if (!trimmed || !editingGroup) { cancelRename(); return; }
    if (trimmed === editingGroup) { cancelRename(); return; }
    try {
      await renameGroup(editingGroup, trimmed);
      // Update global state list
      globalState.groups = globalState.groups.map((g) =>
        g === editingGroup ? trimmed : g,
      );
      // Update accounts in memory
      globalState.accounts = globalState.accounts.map((a) =>
        a.group_name === editingGroup ? { ...a, group_name: trimmed } : a,
      );
      if (globalState.activeGroup === editingGroup) globalState.activeGroup = trimmed;
      toast.success($_("groups.renamed_success"));
    } catch (e) {
      toast.error($_("groups.renamed_error"));
    }
    cancelRename();
  }

  function cancelRename(): void {
    editingGroup = null;
    editingValue = "";
  }

  async function handleDeleteGroup(name: string): Promise<void> {
    try {
      await deleteGroup(name);
      globalState.groups = globalState.groups.filter((g) => g !== name);
      if (globalState.activeGroup === name) globalState.activeGroup = null;
      toast.success($_("groups.deleted_success"));
    } catch (e: any) {
      toast.error(e?.toString() ?? $_("groups.deleted_error"));
    }
  }
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10"
>
  <!-- Header -->
  <header class="flex items-center justify-between mb-7 gap-4">
    <div class="flex gap-x-4">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-lg text-text-muted hover:text-text-primary transition-colors my-auto cursor-pointer"
        onclick={() => (globalState.activeView = "general")}
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
            values: { count: globalState.accounts.length },
          })}
        </p>
      </div>
    </div>

    <Button
      variant="primary"
      onclick={() => modal.openAddCredential({ onadded: refresh })}
      class="shadow-md shadow-accent/20"
    >
      <IconPlus size={16} class="mr-2" />
      {$_("sidebar.new_credential")}
    </Button>
  </header>

  <!-- Search -->
  <div
    class="flex items-center mb-4 border border-surface/10 rounded-lg text-text-primary px-4 py-3"
  >
    <div class="text-text-muted text-lg">
      <IconSearch size={18} />
    </div>
    <input
      class="w-full pl-4 bg-none text-sm placeholder:text-text-muted focus:outline-none transition-all"
      bind:value={search}
      placeholder={$_("accounts.search_placeholder")}
    />
  </div>

  <!-- ── Group panel ─────────────────────────────────────────────────────── -->
  <div class="flex flex-wrap items-center gap-2 mb-6">
    <!-- All chip -->
    <button
      class="px-3 py-1.5 rounded-full text-xs font-semibold transition-all cursor-pointer border
        {globalState.activeGroup === null
          ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
          : 'bg-surface/5 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
      onclick={() => (globalState.activeGroup = null)}
    >
      {$_("groups.all")}
    </button>

    {#each globalState.groups as group (group)}
      <div class="relative group/chip flex items-center">
        {#if editingGroup === group}
          <!-- Rename input -->
          <div class="flex items-center gap-1 px-2 py-1 rounded-full border border-accent/50 bg-surface/10">
            <input
              class="text-xs bg-transparent text-text-primary focus:outline-none w-24"
              bind:value={editingValue}
              onkeydown={(e) => {
                if (e.key === "Enter") { e.preventDefault(); confirmRename(); }
                if (e.key === "Escape") cancelRename();
              }}
              use:selectOnFocus
            />
            <button class="text-accent cursor-pointer" onclick={confirmRename} aria-label={$_("groups.confirm_rename")}>
              <IconCheck size={11} />
            </button>
            <button class="text-text-muted cursor-pointer" onclick={cancelRename} aria-label={$_("global.cancel")}>
              <IconX size={11} />
            </button>
          </div>
        {:else}
          <!-- Normal chip -->
          <button
            class="px-3 py-1.5 rounded-full text-xs font-semibold transition-all cursor-pointer border
              {globalState.activeGroup === group
                ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
                : 'bg-surface/5 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
            onclick={() =>
              (globalState.activeGroup =
                globalState.activeGroup === group ? null : group)}
          >
            {group}
          </button>
          <!-- Hover actions -->
          <div class="absolute -right-1 -top-1 hidden group-hover/chip:flex items-center gap-0.5 bg-panel/90 backdrop-blur-sm border border-surface/15 rounded-full px-1 py-0.5 shadow-sm z-10">
            <button
              class="p-0.5 text-text-muted hover:text-accent cursor-pointer transition-colors"
              onclick={() => startRename(group)}
              aria-label={$_("groups.rename")}
              title={$_("groups.rename")}
            >
              <IconPencil size={10} />
            </button>
            <button
              class="p-0.5 text-text-muted hover:text-danger cursor-pointer transition-colors"
              onclick={() => handleDeleteGroup(group)}
              aria-label={$_("groups.delete")}
              title={$_("groups.delete")}
            >
              <IconTrash size={10} />
            </button>
          </div>
        {/if}
      </div>
    {/each}

    <!-- Add group button / input -->
    {#if addingGroup}
      <div class="flex items-center gap-1 px-2 py-1 rounded-full border border-accent/40 bg-surface/8">
        <input
          class="text-xs bg-transparent text-text-primary focus:outline-none w-24 placeholder:text-text-muted"
          bind:value={newGroupName}
          placeholder={$_("groups.new_group_placeholder")}
          use:focus
          onkeydown={(e) => {
            if (e.key === "Enter") { e.preventDefault(); handleAddGroup(); }
            if (e.key === "Escape") { addingGroup = false; newGroupName = ""; }
          }}
        />
        <button class="text-accent cursor-pointer" onclick={handleAddGroup} aria-label={$_("groups.confirm_new_group")}>
          <IconCheck size={11} />
        </button>
        <button class="text-text-muted cursor-pointer" onclick={() => { addingGroup = false; newGroupName = ""; }} aria-label={$_("global.cancel")}>
          <IconX size={11} />
        </button>
      </div>
    {:else}
      <button
        class="px-3 py-1.5 rounded-full text-xs font-semibold transition-all cursor-pointer border border-dashed border-surface/15 text-text-muted hover:border-accent/50 hover:text-accent inline-flex items-center gap-1"
        onclick={() => (addingGroup = true)}
        aria-label={$_("groups.new_group")}
      >
        <IconPlus size={11} />
        {$_("groups.new_group")}
      </button>
    {/if}
  </div>
  <!-- ── / Group panel ───────────────────────────────────────────────────── -->

  <!-- Error State -->
  {#if loadError}
    <div class="text-center py-16 flex flex-col items-center gap-3">
      <p
        class="text-danger text-sm bg-danger/10 border border-danger/20 rounded-xl px-6 py-4"
      >
        {loadError}
      </p>
      <button
        class="text-sm text-accent underline cursor-pointer"
        onclick={refresh}
      >
        Reintentar
      </button>
    </div>

    <!-- Empty State -->
  {:else if filtered.length === 0}
    <div
      class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary"
    >
      <div class="text-accent/20 mb-2">
        <IconShieldLock size={80} stroke={1.5} />
      </div>
      <h3 class="text-lg text-text-primary font-semibold">
        {globalState.activeGroup
          ? $_("groups.no_accounts_in_group", { values: { group: globalState.activeGroup } })
          : $_("accounts.no_credentials")}
      </h3>
      <p class="text-text-muted">
        {globalState.activeGroup ? "" : $_("accounts.no_credentials_desc")}
      </p>
      {#if !globalState.activeGroup}
        <Button
          variant="primary"
          onclick={() => modal.openAddCredential({ onadded: refresh })}
          class="mt-2 shadow-lg shadow-accent/20"
        >
          <IconPlus size={16} class="mr-2" />
          {$_("sidebar.new_credential")}
        </Button>
      {/if}
    </div>

    <!-- Grid / Virtual List -->
  {:else}
    {#if useVirtualScroll}
      <!-- ── Massive List active banner ── -->
      <div
        class="flex items-center gap-2 mb-4 px-3 py-2 rounded-lg bg-accent/10 border border-accent/20 text-accent-light text-xs font-medium"
      >
        <IconRocket size={14} />
        {$_("settings.performance.massive_list_active_badge", {
          values: { count: filtered.length },
        })}
      </div>

      <!-- ── Virtual scrolling list (single column) ── -->
      <div class="virtual-list-wrapper">
        <VirtualList items={filtered} let:item itemHeight={220}>
          <div class="pb-4">
            <CredentialCard
              account={item}
              locale={$locale ?? "es"}
              ondelete={handleDelete}
              onrefresh={refresh}
            />
          </div>
        </VirtualList>
      </div>
    {:else}
      <!-- ── Normal grid ── -->
      <div class="grid grid-cols-[repeat(auto-fill,minmax(320px,1fr))] gap-4">
        {#each filtered as account (account.id)}
          <CredentialCard
            {account}
            locale={$locale ?? "es"}
            ondelete={handleDelete}
            onrefresh={refresh}
          />
        {/each}
      </div>
    {/if}
  {/if}
</div>
