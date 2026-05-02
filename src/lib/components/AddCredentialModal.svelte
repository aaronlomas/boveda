<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { showAddModal } from "$lib/stores";
  import { addAccount, generatePassword } from "$lib/tauri";
  import { IconEye, IconEyeOff } from '@tabler/icons-svelte';
  import { _ } from "svelte-i18n";

  const dispatch = createEventDispatcher();

  let site = "";
  let username = "";
  let password = "";
  let notes = "";
  let loading = false;
  let error = "";
  let showPw = false;
  let showGenerator = false;

  // Generator state
  let genLength = 20;
  let genSymbols = true;
  let genPreview = "";
  let genLoading = false;
  let genCopied = false;

  $: strength = measureStrength(password);

  function measureStrength(pw: string): {
    score: number;
    label: string;
    color: string;
  } {
    if (!pw) return { score: 0, label: "", color: "#333" };
    let score = 0;
    if (pw.length >= 8) score++;
    if (pw.length >= 14) score++;
    if (/[A-Z]/.test(pw)) score++;
    if (/[0-9]/.test(pw)) score++;
    if (/[^A-Za-z0-9]/.test(pw)) score++;
    const labels = [
      "",
      $_("add_credential.strength.very_weak"),
      $_("add_credential.strength.weak"),
      $_("add_credential.strength.regular"),
      $_("add_credential.strength.strong"),
      $_("add_credential.strength.very_strong"),
    ];
    const colors = [
      "#333",
      "#ef4444",
      "#f97316",
      "#f59e0b",
      "#10b981",
      "#6ee7b7",
    ];
    return { score, label: labels[score], color: colors[score] };
  }

  async function refreshPreview() {
    genLoading = true;
    genPreview = await generatePassword(genLength, genSymbols);
    genLoading = false;
    genCopied = false;
  }

  async function copyGenerated() {
    if (!genPreview) return;
    
    // 1. Tauri plugin
    try {
      const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
      await writeText(genPreview);
      genCopied = true;
      setTimeout(() => (genCopied = false), 2000);
      return;
    } catch (e) {
      console.warn("Tauri clipboard error:", e);
    }
    
    // 2. Navigator clipboard
    try {
      if (navigator.clipboard && window.isSecureContext) {
        await navigator.clipboard.writeText(genPreview);
        genCopied = true;
        setTimeout(() => (genCopied = false), 2000);
        return;
      }
    } catch (err) {
      console.warn("Navigator clipboard error:", err);
    }
    
    // 3. Fallback execCommand
    try {
      const textArea = document.createElement("textarea");
      textArea.value = genPreview;
      textArea.style.position = "fixed";
      textArea.style.opacity = "0";
      document.body.appendChild(textArea);
      textArea.focus();
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
      genCopied = true;
      setTimeout(() => (genCopied = false), 2000);
    } catch (err2) {
      console.error("ExecCommand fallback error:", err2);
    }
  }

  function useGenerated() {
    password = genPreview;
    showGenerator = false;
  }

  async function submit() {
    error = "";
    if (!site.trim()) {
      error = $_("add_credential.error_site");
      return;
    }
    if (!username.trim()) {
      error = $_("add_credential.error_user");
      return;
    }
    if (!password.trim()) {
      error = $_("add_credential.error_password");
      return;
    }

    loading = true;
    try {
      await addAccount(site.trim(), username.trim(), password, notes.trim());
      dispatch("added");
      showAddModal.set(false);
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }

  function close() {
    showAddModal.set(false);
  }
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-panel/60 backdrop-blur-sm"
  on:click={(e) => { if (e.target === e.currentTarget) close(); }}
  on:keydown={(e) => { if (e.key === "Escape") close(); }}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
>
  <div class="w-full max-w-md max-h-[90vh] bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl shadow-2xl relative flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between p-6 pb-4 border-b border-surface/5 shrink-0">
      <h2 class="text-lg font-bold text-text-primary">{$_("add_credential.title")}</h2>
    </div>

    <!-- Scrollable Body -->
    <div class="flex-1 overflow-y-auto p-6 pt-4 custom-scrollbar">
      <form id="add-credential-form" class="flex flex-col gap-4" on:submit|preventDefault={submit}>
      <!-- Site -->
      <div class="flex flex-col gap-1.5">
        <label for="add-site" class="text-xs font-medium text-text-secondary">{$_("add_credential.site_label")}</label>
        <input
          id="add-site"
          class="w-full px-4 py-2.5 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all"
          bind:value={site}
          placeholder={$_("add_credential.site_placeholder")}
        />
      </div>

      <!-- Username -->
      <div class="flex flex-col gap-1.5">
        <label for="add-user" class="text-xs font-medium text-text-secondary">{$_("add_credential.user_label")}</label>
        <input
          id="add-user"
          class="w-full px-4 py-2.5 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all"
          bind:value={username}
          placeholder={$_("add_credential.user_placeholder")}
          autocomplete="off"
        />
      </div>

      <!-- Password -->
      <div class="flex flex-col gap-1.5">
        <div class="flex items-center justify-between">
          <label for="add-pw" class="text-xs font-medium text-text-secondary">{$_("add_credential.password_label")}</label>
          <button
            type="button"
            class="bg-none border-none text-accent-light text-xs cursor-pointer transition-opacity hover:opacity-80"
            on:click={() => {
              showGenerator = !showGenerator;
              if (showGenerator && !genPreview) refreshPreview();
            }}
          >
            {showGenerator ? $_("add_credential.hide") : $_("add_credential.generate")}
          </button>
        </div>
        <div class="relative">
          <input
            id="add-pw"
            class="w-full px-4 py-2.5 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all pr-10"
            type={showPw ? "text" : "password"}
            bind:value={password}
            placeholder={$_("add_credential.placeholder") || "•••••••••••••••"}
            autocomplete="new-password"
          />
          <button
            type="button"
            class="absolute right-2.5 top-1/2 -translate-y-1/2 bg-none border-none cursor-pointer p-1 text-text-muted hover:text-text-primary opacity-60 hover:opacity-100 transition-all flex items-center"
            on:click={() => (showPw = !showPw)}
          >
            {#if showPw}
              <IconEyeOff size={18} />
            {:else}
              <IconEye size={18} />
            {/if}
          </button>
        </div>

        <!-- Strength bar -->
        {#if password}
          <div class="flex items-center gap-2.5 mt-1">
            <div class="flex-1 h-1 rounded-sm bg-surface/10 overflow-hidden">
              <div
                class="h-full rounded-sm transition-[width,background] duration-300"
                style="width: {strength.score * 20}%; background: {strength.color}"
              ></div>
            </div>
            <span class="text-xs font-bold min-w-16" style="color: {strength.color}"
              >{strength.label}</span
            >
          </div>
        {/if}

        <!-- Generator panel -->
        {#if showGenerator}
          <div class="p-4 flex flex-col gap-3 rounded-xl border border-accent/20 bg-accent/10 backdrop-blur-md">
            <div class="flex items-center gap-2 bg-panel/30 rounded-sm p-2.5 px-3">
              <code class="flex-1 font-mono text-sm text-accent-light break-all select-all"
                >{genLoading ? $_("add_credential.generating") : genPreview}</code>
              <button
                type="button"
                class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/7 rounded-md transition-all flex items-center justify-center"
                on:click={refreshPreview}
                data-tooltip={$_("add_credential.regenerate_tooltip")}
                aria-label={$_("add_credential.regenerate_tooltip")}>↻</button>
            </div>

            <div class="flex flex-col gap-2.5">
              <div class="flex flex-col gap-1.5">
                <label for="gen-length" class="text-text-secondary"
                  >{$_("add_credential.length_label")} <strong class="text-text-primary">{genLength}</strong></label
                >
                <input
                  id="gen-length"
                  type="range"
                  min="8"
                  max="64"
                  bind:value={genLength}
                  on:input={refreshPreview}
                  class="w-full accent-accent cursor-pointer"
                />
              </div>
              <label class="flex items-center gap-2 cursor-pointer text-text-secondary text-xs">
                <input
                  type="checkbox"
                  class="accent-accent cursor-pointer"
                  bind:checked={genSymbols}
                  on:change={refreshPreview}
                />
                <span>{$_("add_credential.symbols_label")}</span>
              </label>
            </div>

            <div class="flex gap-2 justify-end mt-1 flex-wrap">
              <button
                type="button"
                class="inline-flex items-center justify-center h-9 px-4 rounded-sm text-sm font-bold cursor-pointer transition-all border border-surface/10 bg-surface/5 text-text-secondary hover:bg-surface/10 hover:text-text-primary"
                on:click={copyGenerated}
              >
                {genCopied ? $_("add_credential.copied_button") : $_("add_credential.copy_button")}
              </button>
              <button
                type="button"
                class="inline-flex items-center justify-center h-9 px-4 rounded-sm text-sm font-bold cursor-pointer transition-all border-none bg-accent text-white shadow-lg shadow-accent/20 hover:brightness-110 hover:-translate-y-px"
                on:click={useGenerated}
              >
                {$_("add_credential.use_password_button")}
              </button>
            </div>
          </div>
        {/if}
      </div>

      <!-- Notes -->
      <div class="flex flex-col gap-1.5">
        <label for="add-notes" class="text-xs font-medium text-text-secondary">{$_("add_credential.notes_label")}</label>
        <textarea
          id="add-notes"
          class="w-full px-4 py-2.5 bg-surface/5 border border-surface/10 rounded-lg text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:border-accent/50 focus:bg-surface/8 transition-all resize-vertical min-h-18"
          bind:value={notes}
          placeholder={$_("add_credential.notes_placeholder")}
          rows="3"
        ></textarea>
      </div>

      {#if error}
        <p class="text-danger text-xs py-2 px-3 bg-danger/10 border border-danger/20 rounded-md">{error}</p>
      {/if}

      </form>
    </div>

    <!-- Fixed Footer -->
    <div class="p-6 pt-4 border-t border-surface/5 flex gap-2.5 justify-end shrink-0">
      <button type="button" class="inline-flex items-center justify-center h-10 px-5 rounded-sm text-sm font-bold cursor-pointer transition-all border border-surface/10 bg-surface/5 text-text-secondary hover:bg-surface/10 hover:text-text-primary min-w-24" on:click={close}
        >{$_("add_credential.cancel_button")}</button
      >
      <button form="add-credential-form" type="submit" class="inline-flex items-center justify-center h-10 px-5 rounded-sm text-sm font-bold cursor-pointer transition-all border-none bg-accent text-white shadow-lg shadow-accent/20 hover:brightness-110 hover:-translate-y-px active:scale-95 disabled:opacity-60 disabled:cursor-not-allowed min-w-24 gap-1.5" disabled={loading}>
        {#if loading}
          <span class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin"></span> {$_("add_credential.saving")}
        {:else}
          {$_("add_credential.save_button")}
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  /* Tailwind handles most styles now */
</style>
