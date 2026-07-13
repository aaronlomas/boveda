<script lang="ts">
  import { onDestroy } from "svelte";
  import { generatePassword } from "$lib/utils/tauri";
  import { _ } from "svelte-i18n";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import Button from "../../core/primitives/Button.svelte";

  let { onuse }: { onuse: (pw: string) => void } = $props();

  let genLength = $state(20);
  let genSymbols = $state(true);
  let genPreview = $state("");
  let genLoading = $state(false);
  let genCopied = $state(false);
  let clipboardCountdown = $state<number | null>(null);
  let clipboardInterval: ReturnType<typeof setInterval> | null = null;

  export async function refreshPreview() {
    genLoading = true;
    try {
      genPreview = await generatePassword(genLength, genSymbols);
    } catch (e) {
      console.error("Failed to generate password:", e);
      genPreview = "Error generating password";
    } finally {
      genLoading = false;
      genCopied = false;
    }
  }

  function startClipboardCleanupCountdown() {
    if (clipboardInterval) {
      clearInterval(clipboardInterval);
    }

    clipboardCountdown = 30;
    genCopied = true;

    clipboardInterval = setInterval(() => {
      if (clipboardCountdown === null || clipboardCountdown <= 1) {
        clearInterval(clipboardInterval!);
        clipboardInterval = null;
        clipboardCountdown = null;
        writeText("").catch(() => {});
        genCopied = false;
      } else {
        clipboardCountdown -= 1;
      }
    }, 1000);
  }

  async function copyGenerated() {
    if (!genPreview) return;
    try {
      await writeText(genPreview);
      startClipboardCleanupCountdown();
    } catch (e) {
      console.warn("Tauri clipboard error:", e);
    }
  }

  onDestroy(() => {
    if (clipboardInterval) {
      clearInterval(clipboardInterval);
    }
  });

  // Generate initial password when mounted
  $effect(() => {
    if (!genPreview) refreshPreview();
  });
</script>

<div
  class="p-4 flex flex-col gap-3 rounded-xl border border-accent/20 bg-accent/10 backdrop-blur-md"
>
  <div class="flex items-center gap-2 bg-panel/30 rounded-sm p-2.5 px-3">
    <code
      class="flex-1 font-mono text-sm text-accent-light break-all select-all"
    >
      {genLoading ? $_("add_credential.generating") : genPreview}
    </code>
    <button
      type="button"
      class="p-1.5 text-text-muted hover:text-text-primary hover:bg-surface/7 rounded-md transition-all flex items-center justify-center"
      onclick={refreshPreview}
      data-tooltip={$_("add_credential.regenerate_tooltip")}
      aria-label={$_("add_credential.regenerate_tooltip")}
    >
      ↻
    </button>
  </div>

  <div class="flex flex-col gap-2.5">
    <div class="flex flex-col gap-1.5">
      <label for="gen-length" class="text-text-secondary text-xs">
        {$_("add_credential.length_label")}
        <strong class="text-text-primary">{genLength}</strong>
      </label>
      <input
        id="gen-length"
        type="range"
        min="8"
        max="64"
        bind:value={genLength}
        oninput={refreshPreview}
        class="w-full accent-accent cursor-pointer"
      />
    </div>
    <label
      class="flex items-center gap-2 cursor-pointer text-text-secondary text-xs"
    >
      <input
        type="checkbox"
        class="accent-accent cursor-pointer"
        bind:checked={genSymbols}
        onchange={refreshPreview}
      />
      <span>{$_("add_credential.symbols_label")}</span>
    </label>
  </div>

  <div class="flex gap-2 justify-end mt-1 flex-wrap">
    <Button type="button" variant="secondary" size="sm" onclick={copyGenerated}>
      {clipboardCountdown !== null
        ? `${clipboardCountdown}s`
        : genCopied
          ? $_("actions.copied")
          : $_("actions.copy")}
    </Button>
    <Button
      type="button"
      variant="primary"
      size="sm"
      onclick={() => onuse(genPreview)}
    >
      {$_("add_credential.use_password_button")}
    </Button>
  </div>
</div>
