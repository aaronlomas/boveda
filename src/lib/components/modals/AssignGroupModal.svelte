<script lang="ts">
  import { _ } from "svelte-i18n";
  import { globalState } from "$lib/stores/stores.svelte";
  import { updateAccountGroup, saveGroups } from "$lib/utils/tauri";
  import { toast } from "$lib/stores/toast.svelte";
  import { IconX, IconPlus, IconCheck } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";

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
    if (globalState.groups.includes(trimmed)) {
      selected = trimmed;
      showNewInput = false;
      newGroupName = "";
      return;
    }
    const updated = [...globalState.groups, trimmed];
    await saveGroups(updated);
    globalState.groups = updated;
    selected = trimmed;
    showNewInput = false;
    newGroupName = "";
  }

  function handleKeydown(e: KeyboardEvent): void {
    if (e.key === "Enter") handleAddGroup();
    if (e.key === "Escape") onclose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200"
>
  <!-- Panel -->
  <div
    class="relative w-full max-w-sm bg-panel/90 backdrop-blur-2xl border border-surface/15 rounded-2xl shadow-2xl shadow-black/40 overflow-hidden animate-in zoom-in-95 duration-200"
    role="dialog"
    aria-modal="true"
    aria-label={$_("groups.modal_title")}
    tabindex="-1"
    use:focus
  >
    <!-- Header -->
    <div class="flex items-center justify-between px-5 pt-5 pb-3">
      <h2 class="font-semibold text-text-primary text-base">
        {$_("groups.modal_title")}
      </h2>
      <button
        class="p-1.5 rounded-lg text-text-muted hover:text-text-primary hover:bg-surface/10 transition-colors cursor-pointer"
        onclick={onclose}
        aria-label={$_("global.cancel")}
      >
        <IconX size={16} />
      </button>
    </div>

    <!-- Group grid -->
    <div class="px-5 pb-2">
      <p class="text-xs text-text-muted mb-3">{$_("groups.pick_a_group")}</p>

      <div class="flex flex-wrap gap-2 mb-3">
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

        {#each globalState.groups as group (group)}
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

    <!-- Footer -->
    <div
      class="flex items-center justify-end gap-2 px-5 py-4 border-t border-surface/8"
    >
      <button
        class="px-4 py-2 rounded-lg text-sm text-text-muted hover:text-text-primary hover:bg-surface/10 transition-colors cursor-pointer"
        onclick={onclose}
      >
        {$_("global.cancel")}
      </button>
      <button
        class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed shadow-sm shadow-accent/20"
        onclick={handleSave}
        disabled={saving}
      >
        {saving ? $_("groups.saving") : $_("groups.save")}
      </button>
    </div>
  </div>
</div>
