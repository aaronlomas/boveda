<script lang="ts">
  import { _ } from "svelte-i18n";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import {
    IconEye,
    IconEyeOff,
    IconTrash,
    IconCopy,
    IconDotsVertical,
  } from "@tabler/icons-svelte";
  import type { Pin } from "$lib/stores/stores.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/stores/toast.svelte";

  // ── Props ──────────────────────────────────────────────────────────────────
  let {
    pinEntry,
    locale,
    ondelete,
  }: {
    pinEntry: Pin;
    locale: string;
    ondelete: (id: string) => void;
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
    } catch {
      try {
        await navigator.clipboard.writeText(text);
      } catch (e) {
        console.warn("Clipboard write failed:", e);
        return;
      }
    }
    startCountdown();
  }

  async function copyPin() {
    try {
      const plain = await invoke<string>("decrypt_secret", {
        ciphertext: pinEntry.encrypted_pin,
      });
      await copyToClipboard(plain);
      toast.success($_("dashboard.copied_success"));
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
        writeText("").catch(() =>
          navigator.clipboard.writeText("").catch(() => {}),
        );
      } else {
        copyTimer--;
      }
    }, 1000);
  }
</script>

<!-- Click-outside handler to close menu -->
<svelte:window
  onclick={(e) => {
    if (menuOpen && !(e.target as HTMLElement)?.closest(`[data-pin-id="${pinEntry.id}"]`)) {
      menuOpen = false;
    }
  }}
/>

<div
  class="p-5 flex flex-col gap-3.5 transition-all bg-panel/30 backdrop-blur-2xl rounded-2xl border border-surface/8 hover:border-accent/30 hover:translate-y-[-2px] relative"
  data-pin-id={pinEntry.id}
>
  <!-- Header -->
  <div class="flex items-center gap-3">
    <div
      class="w-11 h-11 rounded-sm bg-accent/5 border border-accent/10 grid place-items-center shrink-0 text-accent-light"
    >
      <span class="text-lg font-bold">#</span>
    </div>

    <div class="flex-1 min-w-0">
      <span
        class="block font-semibold text-text-primary whitespace-nowrap overflow-hidden text-ellipsis"
        >{pinEntry.name}</span
      >
      <span class="text-text-muted text-xs">{formatDate(pinEntry.created_at)}</span>
    </div>

    <!-- Actions row -->
    <div class="flex items-center gap-0.5 shrink-0">
      <div class="relative">
        <button
          class="text-text-muted rounded-md transition-colors cursor-pointer p-1 hover:bg-surface/10 hover:text-text-primary"
          onclick={() => (menuOpen = !menuOpen)}
          aria-label="Menu"
        >
          <IconDotsVertical size={16} />
        </button>

        {#if menuOpen}
          <div
            class="absolute right-0 top-full mt-1 z-20 min-w-32 border border-surface/20 rounded-sm overflow-hidden animate-in fade-in zoom-in-95 duration-150 bg-panel/50 backdrop-blur-2xl"
          >
            <button
              class="w-full flex items-center gap-2.5 px-3 py-2.5 text-sm text-danger hover:bg-danger/8 transition-colors cursor-pointer text-left"
              onclick={() => { menuOpen = false; ondelete(pinEntry.id); }}
            >
              <IconTrash size={15} class="shrink-0" />
              {$_("dashboard.delete_tooltip")}
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- PIN Field -->
  <div class="flex flex-col gap-1.5 mt-1">
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
        {revealed && decryptedPin ? decryptedPin : "••••+"}
      </code>

      <div class="flex items-center gap-0.5">
        <button
          class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer"
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
          class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer {copyTimer !== null ? 'bg-accent-dim border border-accent/30 text-accent-light' : ''}"
          onclick={copyPin}
          aria-label="Copy PIN"
        >
          {#if copyTimer !== null}
            <span class="text-xs font-bold min-w-4 text-center">{copyTimer}</span>
          {:else}
            <IconCopy size={18} />
          {/if}
        </button>
      </div>
    </div>
  </div>

  <!-- Notes (optional) -->
  {#if pinEntry.encrypted_notes}
    {#if decryptedNotes}
      <div
        class="text-xs text-text-muted p-2 px-2.5 bg-panel/15 rounded-sm border-l-2 border-accent-dim whitespace-pre-wrap max-h-15 overflow-auto mt-1"
      >
        {decryptedNotes}
      </div>
    {:else}
      <div class="text-xs text-text-muted/40 italic p-1 px-2.5 mt-1">
        [{$_("pin_security.notes_label")}]
      </div>
    {/if}
  {/if}
</div>
