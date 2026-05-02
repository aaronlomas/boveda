<script lang="ts">
  import { onMount } from "svelte";
  import { accounts, showAddModal } from "$lib/stores";
  import { getAccounts, deleteAccount } from "$lib/tauri";
  import AddCredentialModal from "./AddCredentialModal.svelte";
  import ConfirmModal from "./ConfirmModal.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import {
    IconEye,
    IconEyeOff,
    IconTrash,
    IconCopy,
    IconPlus,
    IconSearch,
  } from "@tabler/icons-svelte";
  import { activeView } from "$lib/stores";
  import type { Account } from "$lib/stores";
  import { _, locale } from "svelte-i18n";

  let search = "";
  let copyTimers: Record<string, number> = {}; // accountId → seconds remaining
  let copyIntervals: Record<string, ReturnType<typeof setInterval>> = {};
  let revealed: Record<string, boolean> = {};

  // Confirm modal state
  let showConfirmDelete = false;
  let accountToDelete: string | null = null;

  $: filtered = $accounts.filter(
    (a) =>
      a.site.toLowerCase().includes(search.toLowerCase()) ||
      a.username.toLowerCase().includes(search.toLowerCase()),
  );

  onMount(async () => {
    await refresh();
  });

  async function refresh() {
    try {
      const data = await getAccounts();
      accounts.set(data);
    } catch (e) {
      console.error("Failed to load accounts:", e);
    }
  }

  async function copyToClipboard(id: string, text: string, _label: string) {
    try {
      await writeText(text);
      startCountdown(id);
      return;
    } catch (e) {
      console.warn("Tauri clipboard falló:", e);
    }

    try {
      if (navigator.clipboard && window.isSecureContext) {
        await navigator.clipboard.writeText(text);
        startCountdown(id);
        return;
      }
    } catch (err) {
      console.warn("Navigator clipboard falló:", err);
    }

    try {
      const textArea = document.createElement("textarea");
      textArea.value = text;
      textArea.style.position = "fixed";
      textArea.style.opacity = "0";
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      document.execCommand("copy");
      document.body.removeChild(textArea);
      startCountdown(id);
    } catch (err2) {
      console.error("ExecCommand fail:", err2);
    }
  }

  function startCountdown(id: string) {
    if (copyIntervals[id]) clearInterval(copyIntervals[id]);
    copyTimers[id] = 30;
    copyTimers = { ...copyTimers };

    copyIntervals[id] = setInterval(async () => {
      copyTimers[id]--;
      copyTimers = { ...copyTimers };

      if (copyTimers[id] <= 0) {
        clearInterval(copyIntervals[id]);
        delete copyIntervals[id];
        delete copyTimers[id];
        copyTimers = { ...copyTimers };
        try {
          await writeText("");
        } catch {
          try {
            await navigator.clipboard.writeText("");
          } catch {}
        }
      }
    }, 1000);
  }

  function handleDelete(id: string) {
    accountToDelete = id;
    showConfirmDelete = true;
  }

  async function confirmDelete() {
    if (accountToDelete) {
      await deleteAccount(accountToDelete);
      await refresh();
    }
    closeConfirm();
  }

  function closeConfirm() {
    showConfirmDelete = false;
    accountToDelete = null;
  }

  function getSiteInitial(site: string): string {
    const cleaned = site.replace(/^https?:\/\//, "").replace(/^www\./, "");
    return cleaned.charAt(0).toUpperCase();
  }

  function formatDate(iso: string): string {
    try {
      return new Date(iso).toLocaleDateString(
        $locale === "es" ? "es-ES" : "en-US",
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
</script>

<div
  class="max-w-[1100px] mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300"
>
  <!-- Header -->
  <div class="flex items-center justify-between mb-7 gap-4">
    <div class="flex gap-x-4">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-lg text-text-muted hover:text-text-primary transition-colors row-span-2 my-auto"
        on:click={() => activeView.set("general")}
      >
        ← Volver
      </button>
      <div>
        <h1
          class="text-[26px] font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent pointer-events-none"
        >
          {$_("dashboard.title")}
        </h1>
        <p class="text-text-muted text-[13px] mt-0.5 pointer-events-none">
          {$_("dashboard.credentials_count", {
            values: { count: $accounts.length },
          })}
        </p>
      </div>
    </div>
    <button
      class="inline-flex items-center justify-center gap-2 h-[35px] px-4 rounded-lg text-[13px] font-medium cursor-pointer transition-all border border-transparent bg-accent text-white hover:bg-accent-hover shadow-lg shadow-accent/20"
      on:click={() => showAddModal.set(true)}
    >
      <IconPlus size={16} />
      {$_("sidebar.new_credential")}
    </button>
  </div>

  <!-- Search -->
  <div class="relative mb-6">
    <div
      class="absolute left-3.5 top-1/2 -translate-y-1/2 text-text-muted text-lg pointer-events-none"
    >
      <IconSearch size={18} />
    </div>
    <input
      class="w-full px-4 py-3 pl-10 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-[14px] placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all"
      bind:value={search}
      placeholder={$_("dashboard.search_placeholder")}
    />
  </div>

  <!-- Grid -->
  {#if filtered.length === 0}
    <div
      class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary"
    >
      <div class="text-[56px] mb-2">🔒</div>
      <h3 class="text-lg text-text-primary font-semibold">
        {$_("dashboard.no_credentials")}
      </h3>
      <p class="text-text-muted">{$_("dashboard.no_credentials_desc")}</p>
      <button
        class="inline-flex items-center justify-center gap-2 h-[35px] px-4 mt-2 rounded-lg text-[13px] font-medium cursor-pointer transition-all border border-transparent bg-accent text-white hover:bg-accent-hover shadow-lg shadow-accent/20"
        on:click={() => showAddModal.set(true)}
      >
        <IconPlus size={16} />
        {$_("sidebar.new_credential")}
      </button>
    </div>
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(320px,1fr))] gap-4">
      {#each filtered as account (account.id)}
        <div
          class="p-5 flex flex-col gap-3.5 transition-all bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl hover:border-accent/30 hover:translate-y-[-2px]"
        >
          <!-- favicon / avatar -->
          <div class="flex items-center gap-3">
            <div
              class="w-11 h-11 rounded-[10px] bg-surface/5 border border-surface/10 grid place-items-center shrink-0 text-lg font-bold text-text-primary relative overflow-hidden"
            >
              {#if account.favicon_url}
                <img
                  src={account.favicon_url}
                  alt={account.site}
                  on:error={(e) => {
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
              <span class="text-text-muted text-[12px]"
                >@{account.username}</span
              >
            </div>
            <button
              class="p-2 text-danger hover:bg-danger/10 rounded-md transition-colors"
              on:click={() => handleDelete(account.id)}
              data-tooltip={$_("dashboard.delete_tooltip")}
              aria-label={$_("dashboard.delete_tooltip")}
            >
              <IconTrash size={16} />
            </button>
          </div>

          <!-- Password Field -->
          <div class="flex flex-col gap-1.5 mt-1">
            <span
              class="text-[10px] text-text-muted uppercase tracking-[0.08em] font-bold ml-1"
              >Contraseña</span
            >
            <div
              class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-lg p-2.5 px-3 transition-colors hover:bg-surface/[0.07]"
            >
              <code
                class="flex-1 font-mono text-[13px] text-text-secondary whitespace-nowrap overflow-hidden text-ellipsis tracking-wider"
                class:text-white={revealed[account.id]}
              >
                {revealed[account.id] ? account.password : "••••••••••••••••"}
              </code>
              <div class="flex items-center gap-0.5">
                <button
                  class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all"
                  on:click={() =>
                    (revealed[account.id] = !revealed[account.id])}
                  aria-label="Mostrar/ocultar"
                  data-tooltip={revealed[account.id]
                    ? $_("dashboard.hide_tooltip")
                    : $_("dashboard.show_tooltip")}
                >
                  {#if revealed[account.id]}
                    <IconEyeOff size={16} />
                  {:else}
                    <IconEye size={16} />
                  {/if}
                </button>

                <button
                  class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all relative {copyTimers[
                    account.id
                  ] != null
                    ? 'bg-accent-dim border border-accent/30 text-accent-light'
                    : ''}"
                  on:click={() =>
                    copyToClipboard(
                      account.id,
                      account.password,
                      $_("dashboard.password_label"),
                    )}
                  aria-label={$_("dashboard.copy_password_tooltip")}
                  data-tooltip={copyTimers[account.id] != null
                    ? $_("dashboard.clearing_in", {
                        values: { seconds: copyTimers[account.id] },
                      })
                    : $_("dashboard.copy_password_tooltip")}
                >
                  {#if copyTimers[account.id] != null}
                    <span class="text-[11px] font-bold min-w-[16px] text-center"
                      >{copyTimers[account.id]}</span
                    >
                  {:else}
                    <IconCopy size={16} />
                  {/if}
                </button>
              </div>
            </div>
          </div>

          <!-- Username Field -->
          <div class="flex flex-col gap-1.5">
            <span
              class="text-[10px] text-text-muted uppercase tracking-[0.08em] font-bold ml-1"
              >Usuario</span
            >
            <div
              class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-lg p-2.5 px-3 transition-colors hover:bg-surface/[0.07]"
            >
              <code
                class="flex-1 font-mono text-[13px] text-text-secondary whitespace-nowrap overflow-hidden text-ellipsis tracking-wider"
                >{account.username}</code
              >
              <button
                class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all"
                on:click={() =>
                  copyToClipboard(
                    account.id + "_user",
                    account.username,
                    $_("dashboard.username_label"),
                  )}
                aria-label={$_("dashboard.copy_username_tooltip")}
                data-tooltip={$_("dashboard.copy_username_tooltip")}
              >
                <IconCopy size={16} />
              </button>
            </div>
          </div>

          {#if account.notes}
            <div
              class="text-[12px] text-text-muted p-2 px-2.5 bg-panel/15 rounded-sm border-l-2 border-accent-dim whitespace-pre-wrap max-h-[60px] overflow-hidden"
            >
              {account.notes}
            </div>
          {/if}

          <div class="text-[11px] text-text-muted text-right">
            {$_("dashboard.added_at", {
              values: { date: formatDate(account.created_at) },
            })}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

{#if $showAddModal}
  <AddCredentialModal on:added={refresh} />
{/if}

{#if showConfirmDelete}
  <ConfirmModal
    title={$_("dashboard.delete_confirm_title")}
    message={$_("dashboard.delete_confirm_message")}
    confirmText={$_("dashboard.delete_confirm_button")}
    type="danger"
    on:confirm={confirmDelete}
    on:cancel={closeConfirm}
  />
{/if}
