<script lang="ts">
  /**
   * @component AccountsGroupFilter
   * @description Component to filter credentials by group and manage tags/groups.
   */
  import { onMount } from "svelte";
  import { _ } from "svelte-i18n";
  import {
    IconPlus,
    IconCheck,
    IconX,
    IconPencil,
    IconTrash,
  } from "@tabler/icons-svelte";
  import { uiState } from "$lib/stores/stores.svelte";
  import { useGroups } from "$lib/composables/useGroups.svelte";
  import { focus, selectOnFocus } from "$lib/utils/actions";

  // ── Composable and Initialization
  const groupService = useGroups();

  let editingGroup = $state<string | null>(null); // editing group
  let editingValue = $state(""); // current rename input value
  let addingGroup = $state(false); // new group input visibility
  let newGroupName = $state(""); // new group input value

  // ── Handlers
  // ── Manejo de grupos ───────────────────────────────────────────────────────────────
  async function handleAddGroup(): Promise<void> {
    const trimmed = newGroupName.trim();
    if (!trimmed) return;
    await groupService.add(trimmed);
    newGroupName = "";
    addingGroup = false;
  }

  function startRename(group: string): void {
    editingGroup = group;
    editingValue = group;
  }

  async function confirmRename(): Promise<void> {
    if (!editingGroup) return;
    await groupService.rename(editingGroup, editingValue);
    cancelRename();
  }

  function cancelRename(): void {
    editingGroup = null;
    editingValue = "";
  }

  onMount(() => {
    groupService.refresh();
  });
</script>

<!-- Filter by group  -->

<div class="flex flex-wrap items-center gap-2 mb-6">
  <!-- All -->
  <button
    class="px-3 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border
      {uiState.activeGroup === null
      ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
      : 'bg-surface/5 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
    onclick={() => (uiState.activeGroup = null)}
  >
    {$_("groups.all")}
  </button>

  {#each groupService.groups as group (group)}
    <div class="relative group/chip flex items-center">
      {#if editingGroup === group}
        <!-- Rename input -->
        <div
          class="flex items-center gap-1 px-2 py-2 rounded-full border border-accent/50 bg-surface/10"
        >
          <input
            class="text-xs bg-transparent text-text-primary focus:outline-none w-24"
            bind:value={editingValue}
            onkeydown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                confirmRename();
              }
              if (e.key === "Escape") cancelRename();
            }}
            use:selectOnFocus
          />
          <button
            class="text-accent cursor-pointer"
            onclick={confirmRename}
            aria-label={$_("groups.confirm_rename")}
          >
            <IconCheck size={11} />
          </button>
          <button
            class="text-text-muted cursor-pointer"
            onclick={cancelRename}
            aria-label={$_("actions.cancel")}
          >
            <IconX size={11} />
          </button>
        </div>
      {:else}
        <!-- User-created group -->
        <button
          class="px-3 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border
            {uiState.activeGroup === group
            ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
            : 'bg-surface/5 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
          onclick={() =>
            (uiState.activeGroup =
              uiState.activeGroup === group ? null : group)}
        >
          {group}
        </button>
        <!-- Edit / delete group buttons -->
        <div
          class="absolute -right-1 -top-1 hidden group-hover/chip:flex items-center bg-panel/90 backdrop-blur-sm border border-surface/15 rounded-full shadow-sm z-10"
        >
          <button
            class="p-1 text-text-muted hover:text-accent cursor-pointer transition-colors"
            onclick={() => startRename(group)}
            aria-label={$_("actions.rename")}
            title={$_("actions.rename")}
          >
            <IconPencil size={10} />
          </button>
          <button
            class="p-1 text-text-muted hover:text-danger cursor-pointer transition-colors"
            onclick={() => groupService.delete(group)}
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
    <div
      class="flex items-center gap-1 px-2 py-2 rounded-full border border-accent/40 bg-surface/8"
    >
      <input
        class="text-xs bg-transparent text-text-primary focus:outline-none w-24 placeholder:text-text-muted"
        bind:value={newGroupName}
        placeholder={$_("groups.new_group_placeholder")}
        use:focus
        onkeydown={(e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            handleAddGroup();
          }
          if (e.key === "Escape") {
            addingGroup = false;
            newGroupName = "";
          }
        }}
      />
      <button
        class="text-accent cursor-pointer"
        onclick={handleAddGroup}
        aria-label={$_("groups.confirm_new_group")}
      >
        <IconCheck size={11} />
      </button>
      <button
        class="text-text-muted cursor-pointer"
        onclick={() => {
          addingGroup = false;
          newGroupName = "";
        }}
        aria-label={$_("actions.cancel")}
      >
        <IconX size={11} />
      </button>
    </div>
  {:else}
    <button
      class="px-3 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border border-dashed border-surface/15 text-text-muted hover:border-accent/50 hover:text-accent inline-flex items-center gap-1"
      onclick={() => (addingGroup = true)}
      aria-label={$_("groups.new_group")}
    >
      <IconPlus size={11} />
      {$_("groups.new_group")}
    </button>
  {/if}
</div>
