<script lang="ts">
  /**
   * @component CredentialHeader
   * @description Credential card header.
   * Renders the site favicon/initial, the assigned group, and manages the contextual actions menu.
   */
  import { _ } from "svelte-i18n";
  import {
    IconDotsVertical,
    IconFolderPlus,
    IconFolderX,
    IconTrash,
  } from "@tabler/icons-svelte";
  import { dataState, type Account } from "$lib/stores/stores.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { updateAccountGroup } from "$lib/utils/tauri";
  import { toast } from "$lib/stores/toast.svelte";

  // Props
  let {
    account,
    ondelete,
    onrefresh,
    expanded = false,
  } = $props<{
    account: Account;
    ondelete: () => void;
    onrefresh?: () => void;
    expanded?: boolean;
  }>();

  // Local state for controlling the opening of the context menu
  let menuOpen = $state(false);

  // RENDER HELPERS
  /**
   * Gets the initial letter of the website cleaning common protocols and subdomains.
   */
  function getSiteInitial(site: string): string {
    return site
      .replace(/^https?:\/\//, "")
      .replace(/^www\./, "")
      .charAt(0)
      .toUpperCase();
  }

  // ACTION FLOWS AND CONTEXT MENU
  /**
   * Opens the asynchronous modal to assign or change the account group.
   */
  async function openAssignGroup(): Promise<void> {
    menuOpen = false;
    const assigned = await modal.openAssignGroup({
      accountId: account.id,
      currentGroup: account.group_name,
    });
    if (assigned) onrefresh?.();
  }

  /**
   * Removes the account from its current group safely in the local database.
   */
  async function removeFromGroup(): Promise<void> {
    menuOpen = false;
    try {
      await updateAccountGroup(account.id, null);
      onrefresh?.();
      toast.success($_("groups.removed_success"));
    } catch (e) {
      console.error(e);
      toast.error($_("groups.removed_error"));
    }
  }
</script>

<!-- GLOBAL WINDOW EVENTS -->
<svelte:window
  onclick={(e) => {
    if (
      menuOpen &&
      !(e.target as HTMLElement)?.closest(`[data-card-id="${account.id}"]`)
    ) {
      menuOpen = false;
    }
  }}
/>

<div class="flex items-center gap-2">
  <!-- Favicon -->
  <div
    class="w-11 h-11 rounded-sm grid place-items-center shrink-0 text-lg font-bold relative {!(
      account.group_name && dataState.groupColors[account.group_name]
    )
      ? 'bg-surface/5 border border-surface/10 text-text-primary'
      : 'text-white border border-transparent'}"
    style={account.group_name && dataState.groupColors[account.group_name]
      ? `background-color: ${dataState.groupColors[account.group_name]}; border-color: ${dataState.groupColors[account.group_name]}; box-shadow: 0 0 10px ${dataState.groupColors[account.group_name]}40;`
      : ""}
  >
    {#if account.favicon_url}
      <img
        src={account.favicon_url}
        alt={account.site}
        onerror={(e) => {
          (e.target as HTMLImageElement).style.display = "none";
        }}
        class="absolute inset-0 w-full h-full object-contain p-2 rounded-inherit"
      />
    {/if}
    <span class="initial">{getSiteInitial(account.site)}</span>
  </div>

  <!-- Site and Group Information -->
  <div class="flex-1 min-w-0">
    <span
      class="block font-semibold text-text-primary whitespace-nowrap overflow-hidden text-ellipsis"
      >{account.site}</span
    >
    <span class="text-text-muted text-xs"
      >{account.group_name || $_("groups.none")}</span
    >
  </div>

  <!-- More Actions Button -->
  <div class="flex items-center shrink-0">
    <!-- More Actions Button -->
    <div class="relative">
      <button
        class="text-text-muted rounded-md transition-colors p-1 {expanded
          ? 'hover:bg-surface/10 hover:text-text-primary cursor-pointer'
          : 'opacity-50 cursor-default'}"
        onclick={(e) => {
          if (expanded) menuOpen = !menuOpen;
        }}
        aria-label={$_("groups.menu_label")}
        aria-expanded={menuOpen}
        aria-haspopup="menu"
        disabled={!expanded}
      >
        <IconDotsVertical size={16} />
      </button>

      <!-- Dropdown Menu -->
      {#if menuOpen}
        <div
          class="absolute right-0 top-full mt-1 z-20 min-w-44 border border-surface/20 rounded-sm overflow-hidden animate-in fade-in zoom-in-95 duration-150 bg-panel/50 backdrop-blur-2xl"
          role="menu"
        >
          <!-- Assign/Change Group -->
          <button
            class="w-full flex items-center gap-2 p-2 text-sm text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-colors cursor-pointer text-left"
            onclick={openAssignGroup}
            role="menuitem"
          >
            <IconFolderPlus size={15} class="text-accent shrink-0" />
            {account.group_name
              ? $_("groups.change_group")
              : $_("groups.add_to_group")}
          </button>

          <!-- Remove from Group -->
          {#if account.group_name}
            <button
              class="w-full flex items-center gap-2 p-2 text-sm text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-colors cursor-pointer text-left"
              onclick={removeFromGroup}
              role="menuitem"
            >
              <IconFolderX size={15} class="text-text-muted shrink-0" />
              {$_("groups.remove_from_group")}
            </button>
          {/if}

          <!-- Separator -->
          <div class="h-px bg-surface/8 mx-2"></div>

          <!-- Delete Credential -->
          <button
            class="w-full flex items-center gap-2 p-2 text-sm text-danger hover:bg-danger/8 transition-colors cursor-pointer text-left"
            onclick={() => {
              menuOpen = false;
              ondelete();
            }}
            role="menuitem"
          >
            <IconTrash size={15} class="shrink-0" />
            {$_("actions.delete")}
          </button>
        </div>
      {/if}
    </div>
  </div>
</div>
