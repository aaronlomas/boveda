<script lang="ts">
  import { _ } from "svelte-i18n";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import {
    IconEye,
    IconEyeOff,
    IconTrash,
    IconCopy,
    IconDotsVertical,
    IconFolderPlus,
    IconFolderX,
  } from "@tabler/icons-svelte";
  import { dataState, type Pin } from "$lib/stores/stores.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { updatePinGroup } from "$lib/utils/tauri";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/stores/toast.svelte";
  import Capsule from "$lib/components/core/primitives/Capsule.svelte";

  // ── Props ──────────────────────────────────────────────────────────────────
  let {
    pinEntry,
    locale,
    ondelete,
    onrefresh,
  }: {
    pinEntry: Pin;
    locale: string;
    ondelete: (id: string) => void;
    onrefresh?: () => void;
  } = $props();

  // ── Local State ─────────────────────────────────────────────────────────────
  let revealed = $state(false);
  let decryptedPin: string | null = $state(null);
  let decryptedNotes: string | null = $state(null);
  let copyTimer: number | null = $state(null);
  let menuOpen = $state(false);

  // ── Helpers ─────────────────────────────────────────────────────────────────

  function formatDate(iso: string): string {
    try {
      return new Date(iso).toLocaleDateString(
        locale === "es" ? "es-ES" : "en-US",
        {
          day: "2-digit",
          month: "short",
          year: "numeric",
        },
      );
    } catch {
      return iso;
    }
  }

  async function copyToClipboard(text: string): Promise<void> {
    try {
      await writeText(text);
    } catch (e) {
      console.warn("Tauri clipboard write failed:", e);
      return;
    }
    startCountdown();
  }

  async function copyPin() {
    try {
      const plain = await invoke<string>("decrypt_secret", {
        ciphertext: pinEntry.encrypted_pin,
      });
      await copyToClipboard(plain);
      toast.success($_("actions.copied"));
    } catch (e) {
      console.error("Failed to decrypt for copy", e);
    }
  }

  async function toggleReveal() {
    if (revealed) {
      revealed = false;
      decryptedPin = null;
      decryptedNotes = null;
    } else {
      try {
        decryptedPin = await invoke<string>("decrypt_secret", {
          ciphertext: pinEntry.encrypted_pin,
        });
        if (pinEntry.encrypted_notes) {
          decryptedNotes = await invoke<string>("decrypt_secret", {
            ciphertext: pinEntry.encrypted_notes,
          });
        }
        revealed = true;
      } catch (e) {
        console.error("Failed to decrypt", e);
      }
    }
  }

  function startCountdown(): void {
    const SECONDS = 30;
    copyTimer = SECONDS;
    const interval = setInterval(() => {
      if (copyTimer === null || copyTimer <= 1) {
        clearInterval(interval);
        copyTimer = null;
        writeText("").catch(() => {});
      } else {
        copyTimer--;
      }
    }, 1000);
  }

  async function openAssignGroup(): Promise<void> {
    menuOpen = false;
    const assigned = await modal.openAssignGroup({
      accountId: pinEntry.id,
      itemType: "pin",
      currentGroup: pinEntry.group_name,
    });
    if (assigned) onrefresh?.();
  }

  async function removeFromGroup(): Promise<void> {
    menuOpen = false;
    try {
      await updatePinGroup(pinEntry.id, null);
      onrefresh?.();
      toast.success($_("groups.removed_success"));
    } catch (e) {
      console.error(e);
      toast.error($_("groups.removed_error"));
    }
  }
</script>

<!-- Click-outside handler to close menu -->
<svelte:window
  onclick={(e) => {
    if (
      menuOpen &&
      !(e.target as HTMLElement)?.closest(`[data-card-id="${pinEntry.id}"]`)
    ) {
      menuOpen = false;
    }
  }}
/>

<Capsule
  class="border-surface/8 hover:border-accent/30"
  data-card-id={pinEntry.id}
>
  {#snippet header(expanded)}
    <!-- Header -->
    <div class="flex items-center gap-4">
    <div
      class="w-11 h-11 rounded-sm grid place-items-center shrink-0 text-lg font-bold relative overflow-hidden {!(pinEntry.group_name && dataState.groupColors[pinEntry.group_name]) ? 'bg-accent/5 border border-accent/10 text-accent-light' : 'text-white border border-transparent'}"
      style={pinEntry.group_name && dataState.groupColors[pinEntry.group_name] ? `background-color: ${dataState.groupColors[pinEntry.group_name]}; border-color: ${dataState.groupColors[pinEntry.group_name]}; box-shadow: 0 0 10px ${dataState.groupColors[pinEntry.group_name]}40;` : ""}
    >
      <span class="text-lg font-bold">#</span>
    </div>

    <div class="flex-1 min-w-0">
      <span
        class="block font-semibold text-text-primary whitespace-nowrap overflow-hidden text-ellipsis"
        >{pinEntry.name}</span
      >
      <span class="text-text-muted text-xs"
        >{pinEntry.group_name || $_("groups.none")} • {formatDate(pinEntry.created_at)}</span
      >
    </div>

    <!-- Actions row -->
    <div class="flex items-center shrink-0">
      <div class="relative">
        <button
          class="text-text-muted rounded-full transition-colors p-1 {expanded ? 'hover:bg-surface/10 hover:text-text-primary cursor-pointer' : 'opacity-50 cursor-not-allowed'}"
          onclick={(e) => { if (expanded) menuOpen = !menuOpen; }}
          aria-label="Menu"
          disabled={!expanded}
        >
          <IconDotsVertical size={16} />
        </button>

        {#if menuOpen}
          <div
            class="absolute right-0 top-full mt-1 z-20 min-w-44 border border-surface/20 rounded-sm overflow-hidden animate-in fade-in zoom-in-95 duration-150 bg-panel/50 backdrop-blur-2xl"
          >
            <button
              class="w-full flex items-center gap-2 p-2 px-3 text-sm text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-colors cursor-pointer text-left"
              onclick={openAssignGroup}
            >
              <IconFolderPlus size={15} class="text-accent shrink-0" />
              {pinEntry.group_name
                ? $_("groups.change_group")
                : $_("groups.add_to_group")}
            </button>

            {#if pinEntry.group_name}
              <button
                class="w-full flex items-center gap-2 p-2 px-3 text-sm text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-colors cursor-pointer text-left"
                onclick={removeFromGroup}
              >
                <IconFolderX size={15} class="text-text-muted shrink-0" />
                {$_("groups.remove_from_group")}
              </button>
            {/if}

            <div class="h-px bg-surface/8 mx-2 my-1"></div>

            <button
              class="w-full flex items-center gap-2 p-2 px-3 text-sm text-danger hover:bg-danger/8 transition-colors cursor-pointer text-left"
              onclick={() => {
                menuOpen = false;
                ondelete(pinEntry.id);
              }}
            >
              <IconTrash size={15} class="shrink-0" />
              {$_("actions.delete")}
            </button>
          </div>
        {/if}
      </div>
    </div>
    </div>
  {/snippet}

  <!-- PIN Field -->
  <div class="grid gap-2 mt-4">
    <span class="text-xs text-text-muted uppercase tracking-wider font-bold">
      {$_("pin_security.pin_label")}
    </span>
    <div
      class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-xl p-2 px-3 transition-colors hover:bg-surface/[0.07]"
    >
      <code
        class="flex-1 font-mono text-lg text-text-secondary whitespace-nowrap overflow-hidden text-ellipsis tracking-[0.2em]"
        class:text-white={revealed}
      >
        {revealed && decryptedPin ? decryptedPin : "••••"}
      </code>

      <div class="flex items-center">
        <button
          class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer"
          onclick={toggleReveal}
          aria-label={revealed ? "Hide" : "Show"}
        >
          {#if revealed}
            <IconEyeOff size={18} />
          {:else}
            <IconEye size={18} />
          {/if}
        </button>

        <button
          class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer {copyTimer !==
          null
            ? 'text-accent-light'
            : ''}"
          onclick={copyPin}
          aria-label="Copy PIN"
        >
          <div class="w-4 h-4 flex items-center justify-center">
            {#if copyTimer !== null}
              <span class="text-xs font-bold leading-none">{copyTimer}</span>
            {:else}
              <IconCopy size={18} />
            {/if}
          </div>
        </button>
      </div>
    </div>
  </div>

  <!-- Notes (optional) -->
  {#if pinEntry.encrypted_notes}
    {#if decryptedNotes}
      <div
        class="text-xs text-text-muted p-2 px-2 bg-panel/15 rounded-sm border-l-2 border-accent-dim whitespace-pre-wrap max-h-15 overflow-auto mt-1"
      >
        {decryptedNotes}
      </div>
    {:else}
      <div class="text-xs text-text-muted/40 italic p-1 px-2 mt-1">
        [{$_("pin_security.notes_label")}]
      </div>
    {/if}
  {/if}
</Capsule>
