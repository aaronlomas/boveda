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
  import type { Account } from "$lib/stores/stores.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { modal } from "$lib/stores/modal.svelte";
  import { updateAccountGroup } from "$lib/utils/tauri";
  import { toast } from "$lib/stores/toast.svelte";

  // ── Props ──────────────────────────────────────────────────────────────────
  let {
    account,
    locale,
    ondelete,
    onrefresh,
  }: {
    account: Account;
    locale: string;
    ondelete: (id: string) => void;
    onrefresh?: () => void;
  } = $props();

  // ── Local State ─────────────────────────────────────────────────────────────
  let revealed = $state(false);
  let decryptedPassword: string | null = $state(null);
  let decryptedRecoveryCode: string | null = $state(null);
  let decryptedNotes: string | null = $state(null);
  let copyTimer: number | null = $state(null);
  let recoveryCopyTimer: number | null = $state(null);
  let userCopyTimer: number | null = $state(null);
  let menuOpen = $state(false);

  // ── Helpers ─────────────────────────────────────────────────────────────────

  function getSiteInitial(site: string): string {
    return site
      .replace(/^https?:\/\//, "")
      .replace(/^www\./, "")
      .charAt(0)
      .toUpperCase();
  }

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

  async function copyToClipboard(
    text: string,
    timerId: "pass" | "user" | "recovery",
  ): Promise<void> {
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
    startCountdown(timerId);
  }

  async function copyPassword() {
    try {
      const plain = await invoke<string>("decrypt_secret", {
        ciphertext: account.password_cipher,
      });
      await copyToClipboard(plain, "pass");
    } catch (e) {
      console.error("Failed to decrypt for copy", e);
    }
  }

  async function copyRecoveryCode() {
    if (!account.recovery_code_cipher) return;
    try {
      const plain = await invoke<string>("decrypt_secret", {
        ciphertext: account.recovery_code_cipher,
      });
      await copyToClipboard(plain, "recovery");
    } catch (e) {
      console.error("Failed to decrypt for copy", e);
    }
  }

  async function toggleReveal() {
    if (revealed) {
      revealed = false;
      decryptedPassword = null;
      decryptedRecoveryCode = null;
      decryptedNotes = null;
    } else {
      try {
        decryptedPassword = await invoke<string>("decrypt_secret", {
          ciphertext: account.password_cipher,
        });
        if (account.recovery_code_cipher) {
          decryptedRecoveryCode = await invoke<string>("decrypt_secret", {
            ciphertext: account.recovery_code_cipher,
          });
        }
        if (account.notes_cipher) {
          decryptedNotes = await invoke<string>("decrypt_secret", {
            ciphertext: account.notes_cipher,
          });
        }
        revealed = true;
      } catch (e) {
        console.error("Failed to decrypt", e);
      }
    }
  }

  function startCountdown(timerId: "pass" | "user" | "recovery"): void {
    const SECONDS = 30;

    if (timerId === "pass") {
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
    } else if (timerId === "recovery") {
      recoveryCopyTimer = SECONDS;
      const interval = setInterval(() => {
        if (recoveryCopyTimer === null || recoveryCopyTimer <= 1) {
          clearInterval(interval);
          recoveryCopyTimer = null;
          writeText("").catch(() =>
            navigator.clipboard.writeText("").catch(() => {}),
          );
        } else {
          recoveryCopyTimer--;
        }
      }, 1000);
    } else {
      userCopyTimer = SECONDS;
      const interval = setInterval(() => {
        if (userCopyTimer === null || userCopyTimer <= 1) {
          clearInterval(interval);
          userCopyTimer = null;
        } else {
          userCopyTimer--;
        }
      }, 1000);
    }
  }

  // ── Group actions ────────────────────────────────────────────────────────────

  async function openAssignGroup(): Promise<void> {
    menuOpen = false;
    const assigned = await modal.openAssignGroup({
      accountId: account.id,
      currentGroup: account.group_name,
    });
    if (assigned) onrefresh?.();
  }

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

<!-- Click-outside handler to close menu -->
<svelte:window
  onclick={(e) => {
    if (menuOpen && !(e.target as HTMLElement)?.closest(`[data-card-id="${account.id}"]`)) {
      menuOpen = false;
    }
  }}
/>

<div
  class="p-5 flex flex-col gap-3.5 transition-all bg-panel/30 backdrop-blur-2xl rounded-2xl border border-surface/8 hover:border-accent/30 hover:translate-y-[-2px] relative"
  data-card-id={account.id}
>
  <!-- Header: favicon / site / actions -->
  <div class="flex items-center gap-2">
    <div
      class="w-11 h-11 rounded-sm bg-surface/5 border border-surface/10 grid place-items-center shrink-0 text-lg font-bold text-text-primary relative overflow-hidden"
    >
      {#if account.favicon_url}
        <img
          src={account.favicon_url}
          alt={account.site}
          onerror={(e) => {
            (e.target as HTMLImageElement).style.display = "none";
          }}
          class="absolute inset-0 w-full h-full object-contain p-1.5 rounded-inherit"
        />
      {/if}
      <span class="initial">{getSiteInitial(account.site)}</span>
    </div>

    <div class="flex-1 min-w-0">
      <span
        class="block font-semibold text-text-primary whitespace-nowrap overflow-hidden text-ellipsis"
        >{account.site}</span
      >
      <span class="text-text-muted text-xs">{account.group_name || $_("groups.none")}</span>
    </div>

    <!-- Actions row -->
    <div class="flex items-center gap-0.5 shrink-0">
      <!-- ⋮ Context menu button -->
      <div class="relative">
        <button
          class="text-text-muted rounded-md transition-colors cursor-pointer p-1 hover:bg-surface/10 hover:text-text-primary"
          onclick={() => (menuOpen = !menuOpen)}
          aria-label={$_("groups.menu_label")}
          aria-expanded={menuOpen}
          aria-haspopup="menu"
        >
          <IconDotsVertical size={16} />
        </button>

        {#if menuOpen}
          <!-- Dropdown -->
          <div
            class="absolute right-0 top-full mt-1 z-20 min-w-44 border border-surface/20 rounded-sm overflow-hidden animate-in fade-in zoom-in-95 duration-150 bg-panel/50 backdrop-blur-2xl"
            role="menu"
          >
            <button
              class="w-full flex items-center gap-2.5 px-3 py-2.5 text-sm text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-colors cursor-pointer text-left"
              onclick={openAssignGroup}
              role="menuitem"
            >
              <IconFolderPlus size={15} class="text-accent shrink-0" />
              {account.group_name
                ? $_("groups.change_group")
                : $_("groups.add_to_group")}
            </button>

            {#if account.group_name}
              <button
                class="w-full flex items-center gap-2.5 px-3 py-2.5 text-sm text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-colors cursor-pointer text-left"
                onclick={removeFromGroup}
                role="menuitem"
              >
                <IconFolderX size={15} class="text-text-muted shrink-0" />
                {$_("groups.remove_from_group")}
              </button>
            {/if}

            <div class="h-px bg-surface/8 mx-2"></div>

            <button
              class="w-full flex items-center gap-2.5 px-3 py-2.5 text-sm text-danger hover:bg-danger/8 transition-colors cursor-pointer text-left"
              onclick={() => { menuOpen = false; ondelete(account.id); }}
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


  <!-- Password Field -->
  <div class="flex flex-col gap-1.5 mt-1">
    <span class="text-xs text-text-muted uppercase tracking-wider font-bold">
      {$_("accounts.password_label")}
    </span>
    <div
      class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-xl p-2 px-3 transition-colors hover:bg-surface/[0.07]"
    >
      <code
        class="flex-1 font-mono text-sm text-text-secondary whitespace-nowrap overflow-hidden text-ellipsis tracking-wider"
        class:text-white={revealed}
      >
        {revealed && decryptedPassword ? decryptedPassword : "••••••••••••••••"}
      </code>

      <div class="flex items-center gap-0.5">
        <!-- Toggle reveal -->
        <button
          class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer"
          onclick={toggleReveal}
          aria-label={revealed
            ? $_("actions.hide")
            : $_("actions.show")}
          data-tooltip={revealed
            ? $_("actions.hide")
            : $_("actions.show")}
        >
          {#if revealed}
            <IconEyeOff size={16} />
          {:else}
            <IconEye size={16} />
          {/if}
        </button>

        <!-- Copy password -->
        <button
          class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer
                 {copyTimer !== null
            ? 'text-accent-light'
            : ''}"
          onclick={copyPassword}
          aria-label={$_("actions.copy")}
          data-tooltip={copyTimer !== null
            ? $_("actions.status.clearing", { values: { seconds: copyTimer } })
            : $_("actions.copy")}
        >
          <div class="w-4 h-4 flex items-center justify-center">
            {#if copyTimer !== null}
              <span class="text-[10px] font-bold leading-none">{copyTimer}</span>
            {:else}
              <IconCopy size={16} />
            {/if}
          </div>
        </button>
      </div>
    </div>
  </div>

  <!-- Recovery Code Field (optional) -->
  {#if account.recovery_code_cipher}
    <div class="flex flex-col gap-1.5 mt-1">
      <span class="text-xs text-text-muted uppercase tracking-wider font-bold">
        {$_("accounts.recovery_code_label")}
      </span>
      <div
        class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-xl p-2 px-3 transition-colors hover:bg-surface/[0.07]"
      >
        <code
          class="flex-1 font-mono text-sm text-text-secondary whitespace-nowrap overflow-hidden text-ellipsis tracking-wider"
          class:text-white={revealed}
        >
          {revealed && decryptedRecoveryCode ? decryptedRecoveryCode : "••••••••••••"}
        </code>

        <div class="flex items-center gap-0.5">
          <!-- Copy recovery code -->
          <button
            class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer
                   {recoveryCopyTimer !== null
              ? 'text-accent-light'
              : ''}"
            onclick={copyRecoveryCode}
            aria-label={$_("actions.copy")}
            data-tooltip={recoveryCopyTimer !== null
              ? $_("actions.status.clearing", { values: { seconds: recoveryCopyTimer } })
              : $_("actions.copy")}
          >
            <div class="w-4 h-4 flex items-center justify-center">
              {#if recoveryCopyTimer !== null}
                <span class="text-[10px] font-bold leading-none">{recoveryCopyTimer}</span>
              {:else}
                <IconCopy size={16} />
              {/if}
            </div>
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Username Field -->
  <div class="flex flex-col gap-1.5">
    <span class="text-xs text-text-muted uppercase tracking-wider font-bold">
      {$_("accounts.username_label")}
    </span>
    <div
      class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-xl py-2 px-3 transition-colors hover:bg-surface/[0.07]"
    >
      <code
        class="flex-1 font-mono text-sm text-text-secondary whitespace-nowrap overflow-hidden text-ellipsis tracking-wider"
      >
        {account.username}
      </code>
      <button
        class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer"
        onclick={() => copyToClipboard(account.username, "user")}
        aria-label={$_("actions.copy")}
        data-tooltip={$_("actions.copy")}
      >
        <IconCopy size={16} />
      </button>
    </div>
  </div>

  <!-- Notes (optional) -->
  {#if account.notes_cipher}
    {#if decryptedNotes}
      <div
        class="text-xs text-text-muted p-2 px-2.5 bg-panel/15 rounded-sm border-l-2 border-accent-dim whitespace-pre-wrap max-h-15 overflow-auto"
      >
        {decryptedNotes}
      </div>
    {:else}
      <div class="text-xs text-text-muted/40 italic p-1 px-2.5">
        [Notas bloqueadas]
      </div>
    {/if}
  {/if}

  <!-- Date -->
  <div class="text-xs text-text-muted text-right">
    {$_("accounts.added_at", {
      values: { date: formatDate(account.created_at) },
    })}
  </div>
</div>
