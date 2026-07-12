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
    IconLock,
    IconLockOpen,
  } from "@tabler/icons-svelte";
  import { uiState, dataState } from "$lib/stores/stores.svelte";
  import { useGroups } from "$lib/composables/useGroups.svelte";
  import GroupColorPicker from "$lib/components/features/accounts/groupColors/GroupColorPicker.svelte";
  import { focus, selectOnFocus } from "$lib/utils/actions";

  // ── Composable and Initialization
  const groupService = useGroups();

  let editingGroup = $state<string | null>(null); // editing group
  let editingValue = $state(""); // current rename input value
  let editingColor = $state<string | null>(null); // current rename color
  let addingGroup = $state(false); // new group input visibility
  let newGroupName = $state(""); // new group input value
  let newGroupColor = $state<string | null>(null); // new group color

  // ── Handlers
  // ── Manejo de grupos ───────────────────────────────────────────────────────────────
  async function handleAddGroup(): Promise<void> {
    const trimmed = newGroupName.trim();
    if (!trimmed) return;
    await groupService.add(trimmed, newGroupColor);
    newGroupName = "";
    newGroupColor = null;
    addingGroup = false;
  }

  function startRename(group: string): void {
    editingGroup = group;
    editingValue = group;
    editingColor = dataState.groupColors[group] || null;
  }

  async function confirmRename(): Promise<void> {
    if (!editingGroup) return;
    await groupService.rename(editingGroup, editingValue, editingColor);
    cancelRename();
  }

  function cancelRename(): void {
    editingGroup = null;
    editingValue = "";
    editingColor = null;
  }

  onMount(() => {
    groupService.refresh();
  });
</script>

<!-- Filter by group  -->

<div class="flex flex-wrap items-center gap-2 mb-6">
  <!-- All -->
  <button
    class="px-3 py-2 rounded-sm text-xs font-semibold transition-all cursor-pointer border leading-none
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
          class="flex items-center gap-1 px-3 py-2 rounded-full border border-accent/50 bg-surface/10"
        >
          <input
            class="text-xs bg-transparent text-text-primary focus:outline-none w-24 leading-none"
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
            <IconCheck size={14} />
          </button>
          <button
            class="text-text-muted cursor-pointer"
            onclick={cancelRename}
            aria-label={$_("actions.cancel")}
          >
            <IconX size={14} />
          </button>
          <GroupColorPicker bind:selectedColor={editingColor} />
        </div>
      {:else}
        <!-- User-created group -->
        <!-- --------- -->
        <!-- | Group | -->
        <!-- --------- -->
        <button
          class="px-3 py-2 rounded-sm text-xs font-semibold transition-all cursor-pointer border
            {uiState.activeGroup === group
            ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
            : 'bg-surface/5 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
          onclick={() =>
            (uiState.activeGroup =
              uiState.activeGroup === group ? null : group)}
        >
          <span class="flex items-center leading-none gap-1">
            {#if dataState.groupColors[group]}
              <span
                class="w-2 h-2 rounded-full"
                style="background-color: {dataState.groupColors[group]};"
              ></span>
            {/if}
            <!-- text for groups already created -->
            {group}
          </span>
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
      class="flex items-center gap-1 px-3 py-2 rounded-full border border-accent/40 bg-surface/8"
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
        <IconCheck size={14} />
      </button>
      <button
        class="text-text-muted cursor-pointer"
        onclick={() => {
          addingGroup = false;
          newGroupName = "";
          newGroupColor = null;
        }}
        aria-label={$_("actions.cancel")}
      >
        <IconX size={14} />
      </button>
      <GroupColorPicker bind:selectedColor={newGroupColor} />
    </div>
  {:else}
    <button
      class="px-3 py-2 rounded-full text-xs font-semibold transition-all cursor-pointer border border-dashed border-surface/15 text-text-muted hover:border-accent/50 hover:text-accent inline-flex items-center gap-1 leading-none"
      onclick={() => (addingGroup = true)}
      aria-label={$_("groups.new_group")}
    >
      <IconPlus size={11} />
      {$_("groups.new_group")}
    </button>
  {/if}

  <button
    class="border border-surface/15 rounded-full p-2 cursor-pointer transition-colors hover:border-accent/50 {uiState.capsuleLocked
      ? 'text-accent border-accent/50'
      : 'text-text-muted'}"
    onclick={() => (uiState.capsuleLocked = !uiState.capsuleLocked)}
    aria-label={uiState.capsuleLocked ? $_("actions.lock_capsule_label") : $_("actions.unlock_capsule_label")}
    title={uiState.capsuleLocked ? $_("actions.lock_capsule_label") : $_("actions.unlock_capsule_label")}
  >
    {#if uiState.capsuleLocked}
      <IconLock size={14} />
    {:else}
      <IconLockOpen size={14} />
    {/if}
  </button>
</div>
