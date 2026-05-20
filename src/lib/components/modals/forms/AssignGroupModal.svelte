<script lang="ts">
  import { _ } from "svelte-i18n";
  import { dataState } from "$lib/stores/stores.svelte";
  import { updateAccountGroup, saveGroups } from "$lib/utils/tauri";
  import { toast } from "$lib/stores/toast.svelte";
  import { IconPlus, IconCheck } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  import Modal from "../../core/primitives/Modal.svelte";
  import Button from "../../core/primitives/Button.svelte";

  // ── Props ──────────────────────────────────────────────────────────────────
  let {
    accountId,
    currentGroup,
    onassigned,
    onclose,
  }: {
    accountId: string;
    currentGroup?: string | null;
    onassigned?: () => void;
    onclose: () => void;
  } = $props();

  // ── State ──────────────────────────────────────────────────────────────────
  let selected = $state<string | null>(null);
  let newGroupName = $state("");
  let showNewInput = $state(false);
  let saving = $state(false);

  // Keep state in sync with prop and fix "state_referenced_locally"
  $effect(() => {
    selected = currentGroup ?? null;
  });

  // ── Actions ────────────────────────────────────────────────────────────────

  async function handleSave(): Promise<void> {
    saving = true;
    try {
      await updateAccountGroup(accountId, selected);
      onassigned?.();
      toast.success($_("groups.assigned_success"));
      onclose();
    } catch (e) {
      console.error(e);
      toast.error($_("groups.assigned_error"));
    } finally {
      saving = false;
    }
  }

  async function handleAddGroup(): Promise<void> {
    const trimmed = newGroupName.trim();
    if (!trimmed) return;
    if (dataState.groups.includes(trimmed)) {
      selected = trimmed;
      showNewInput = false;
      newGroupName = "";
      return;
    }
    const updated = [...dataState.groups, trimmed];
    await saveGroups(updated);
    dataState.groups = updated;
    selected = trimmed;
    showNewInput = false;
    newGroupName = "";
  }
</script>

<Modal 
  show={true} 
  onclose={onclose} 
  title={$_("groups.modal_title")}
>
  <div class="space-y-4">
    <p class="text-xs text-text-muted">{$_("groups.pick_a_group")}</p>

    <div class="flex flex-wrap gap-2">
      <!-- "None" chip -->
      <button
        class="px-3 py-1.5 rounded-full text-xs font-medium transition-all cursor-pointer border
          {selected === null
          ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
          : 'bg-surface/8 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
        onclick={() => (selected = null)}
      >
        {$_("groups.none")}
      </button>

      {#each dataState.groups as group (group)}
        <button
          class="px-3 py-1.5 rounded-full text-xs font-medium transition-all cursor-pointer border
            {selected === group
            ? 'bg-accent text-white border-accent shadow-sm shadow-accent/30'
            : 'bg-surface/8 text-text-muted border-surface/10 hover:border-accent/40 hover:text-text-primary'}"
          onclick={() => (selected = group)}
        >
          {#if selected === group}
            <span class="inline-flex items-center gap-1">
              <IconCheck size={11} />
              {group}
            </span>
          {:else}
            {group}
          {/if}
        </button>
      {/each}

      <!-- Add new group button / input -->
      {#if showNewInput}
        <div class="flex items-center gap-1">
          <input
            class="px-2 py-1 rounded-full text-xs bg-surface/10 border border-accent/40 text-text-primary focus:outline-none focus:border-accent w-28"
            bind:value={newGroupName}
            placeholder={$_("groups.new_group_placeholder")}
            use:focus
            onkeydown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                handleAddGroup();
              }
              if (e.key === "Escape") {
                showNewInput = false;
                newGroupName = "";
              }
            }}
          />
          <button
            class="p-1 rounded-full bg-accent text-white cursor-pointer hover:bg-accent-hover transition-colors"
            onclick={handleAddGroup}
            aria-label={$_("groups.confirm_new_group")}
          >
            <IconCheck size={11} />
          </button>
        </div>
      {:else}
        <button
          class="px-3 py-1.5 rounded-full text-xs font-medium transition-all cursor-pointer border border-dashed border-surface/20 text-text-muted hover:border-accent/50 hover:text-accent inline-flex items-center gap-1"
          onclick={() => (showNewInput = true)}
        >
          <IconPlus size={11} />
          {$_("groups.new_group")}
        </button>
      {/if}
    </div>
  </div>

  {#snippet footer()}
    <Button variant="ghost" onclick={onclose}>
      {$_("actions.cancel")}
    </Button>
    <Button 
      onclick={handleSave} 
      disabled={saving}
      class="min-w-24"
    >
      {saving ? $_("actions.saving") : $_("actions.save")}
    </Button>
  {/snippet}
</Modal>
