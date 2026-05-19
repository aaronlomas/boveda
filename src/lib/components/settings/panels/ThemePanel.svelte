<script lang="ts">
  import { themeStore, PRESETS, type ColorScheme } from "$lib/stores/theme.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import {
    IconPalette,
    IconPhoto,
    IconCheck,
    IconTypography,
    IconSun,
    IconMoon,
    IconDeviceDesktop,
  } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";

  let loadingImage = $state(false);

  async function handleAccentChange(e: Event) {
    const newColor = (e.target as HTMLInputElement).value;
    await themeStore.setAccentColor(newColor);
  }

  async function handleColorSchemeChange(scheme: ColorScheme) {
    await themeStore.setColorScheme(scheme);
  }

  async function handleTextPrimaryChange(e: Event) {
    const primary = (e.target as HTMLInputElement).value;
    await themeStore.setTextColors(primary, themeStore.textSecondary);
  }

  async function handleTextSecondaryChange(e: Event) {
    const secondary = (e.target as HTMLInputElement).value;
    await themeStore.setTextColors(themeStore.textPrimary, secondary);
  }

  async function handleSelectImage() {
    loadingImage = true;
    try {
      const filePath = await open({
        title: $_("actions.select"),
        filters: [
          {
            name: $_("actions.images_filter_name"),
            extensions: ["jpg", "jpeg", "png", "webp", "gif"],
          },
        ],
      });
      if (filePath) {
        const filename = await invoke<string>("import_background_image", {
          srcPath: filePath,
        });
        await themeStore.setBackground("image", filename);
      }
    } catch (e) {
      console.error("Error importing background:", e);
    } finally {
      loadingImage = false;
    }
  }

  async function applyGradient() {
    await themeStore.setBackground("gradient", "");
  }

  async function applySolid() {
    // If we are already solid, apply the current solid color, otherwise apply a default
    const colorToApply = themeStore.bgType === "solid" ? themeStore.bgValue : "#0d0d1a";
    await themeStore.setBackground("solid", colorToApply);
  }

  async function onSolidColorChange(e: Event) {
    const newColor = (e.target as HTMLInputElement).value;
    await themeStore.setBackground("solid", newColor);
  }
</script>

<!-- ─ Presets Section ─ -->
<div class="space-y-4">
  <h1 class="text-text-primary text-xl font-bold flex items-center gap-2">
    <IconPalette size={18} class="text-accent" />
    {$_("settings.theme.presets_title")}
  </h1>
  <div class="flex overflow-x-auto gap-3 pb-2 custom-scrollbar">
    {#each PRESETS as preset}
      <button
        class="min-w-32 p-3 rounded-xl border transition-all flex flex-col gap-3 text-left
          {themeStore.activePresetId === preset.id
          ? 'border-accent bg-accent-dim'
          : 'border-surface/8 bg-surface/3 hover:bg-surface/5'}"
        onclick={() => themeStore.applyPreset(preset)}
      >
        <div class="flex gap-2">
          <div
            class="w-6 h-6 rounded-full border border-surface/10"
            style="background-color: {preset.dark.accentColor}"
          ></div>
          <div
            class="w-6 h-6 rounded-full border border-surface/10"
            style="background-color: {preset.dark.bgType === 'solid'
              ? preset.dark.bgValue
              : '#07070e'}"
          ></div>
        </div>
        <span class="text-xs font-medium text-text-primary leading-tight"
          >{$_(preset.nameKey)}</span
        >
      </button>
    {/each}
  </div>
</div>

<hr class="border-surface/8 my-1" />

<!-- ─ Appearance Section ─ -->
<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h1 class="text-text-primary text-xl font-bold flex items-center gap-2">
      <IconDeviceDesktop size={20} class="text-accent" />
      {$_("settings.theme.appearance")}
    </h1>
  </div>

  <div class="grid grid-cols-3 gap-3">
    <button
      class="p-3 rounded-xl border transition-all flex flex-col items-center gap-2 {themeStore.colorScheme ===
      'light'
        ? 'border-accent bg-accent-dim text-accent'
        : 'border-surface/8 bg-surface/3 text-text-muted hover:border-surface/20 hover:text-text-primary'}"
      onclick={() => handleColorSchemeChange("light")}
    >
      <IconSun size={24} />
      <span class="text-xs font-medium"
        >{$_("settings.theme.mode_light")}</span
      >
    </button>
    <button
      class="p-3 rounded-xl border transition-all flex flex-col items-center gap-2 {themeStore.colorScheme ===
      'dark'
        ? 'border-accent bg-accent-dim text-accent'
        : 'border-surface/8 bg-surface/3 text-text-muted hover:border-surface/20 hover:text-text-primary'}"
      onclick={() => handleColorSchemeChange("dark")}
    >
      <IconMoon size={24} />
      <span class="text-xs font-medium"
        >{$_("settings.theme.mode_dark")}</span
      >
    </button>
    <button
      class="p-3 rounded-xl border transition-all flex flex-col items-center gap-2 {themeStore.colorScheme ===
      'system'
        ? 'border-accent bg-accent-dim text-accent'
        : 'border-surface/8 bg-surface/3 text-text-muted hover:border-surface/20 hover:text-text-primary'}"
      onclick={() => handleColorSchemeChange("system")}
    >
      <IconDeviceDesktop size={24} />
      <span class="text-xs font-medium"
        >{$_("settings.theme.mode_system")}</span
      >
    </button>
  </div>
</div>

<hr class="border-surface/8 my-1" />

<!-- ─ Accent Color Section ─ -->
<div class="space-y-4">
  <h1 class="text-text-primary text-xl font-bold flex items-center gap-2">
    <IconPalette size={20} class="text-accent" />
    {$_("settings.theme.title_options")}
  </h1>

  <div
    class="flex items-center gap-4 p-4 rounded-xl border border-accent/20 bg-accent-dim transition-all"
  >
    <div
      class="w-12 h-12 rounded-full shadow-lg ring-4 ring-surface/5 shrink-0"
      style="background-color: {themeStore.accentColor};"
    ></div>
    <div class="flex-1">
      <p class="text-sm font-bold text-text-primary">
        {$_("settings.theme.color_label")}
      </p>
      <p class="text-xs text-text-muted">
        {$_("settings.theme.color_desc")}
      </p>
    </div>
    <div
      class="shrink-0 flex items-center gap-2 bg-surface/5 px-2.5 py-1.5 rounded-lg border border-surface/10 hover:bg-surface/10 transition-all"
    >
      <span class="text-xs font-mono text-text-secondary uppercase"
        >{themeStore.accentColor}</span
      >
      <input
        type="color"
        value={themeStore.accentColor}
        oninput={handleAccentChange}
        onclick={(e) => e.stopPropagation()}
        class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent p-0"
        aria-label={$_("settings.theme.color_label")}
      />
    </div>
  </div>
</div>

<!-- ─ Text Colors Section ─ -->
<div class="space-y-4 pt-2">
  <h1 class="text-text-primary text-xl font-bold flex items-center gap-2">
    <IconTypography size={20} class="text-accent" />
    {$_("settings.theme.text_options")}
  </h1>

  <div class="grid grid-cols-1 gap-3">
    <!-- Primary Text Color -->
    <div
      class="flex items-center gap-4 p-4 rounded-xl border border-surface/8 bg-surface/3 hover:bg-surface/5 transition-all"
    >
      <div class="flex-1">
        <p class="text-sm font-semibold text-text-primary">
          {$_("settings.theme.text_primary_label")}
        </p>
        <p class="text-xs text-text-muted">
          {$_("settings.theme.text_primary_desc")}
        </p>
      </div>
      <div
        class="shrink-0 flex items-center gap-2 bg-surface/5 px-2.5 py-1.5 rounded-lg border border-surface/10"
      >
        <span class="text-xs font-mono text-text-secondary uppercase"
          >{themeStore.textPrimary}</span
        >
        <input
          type="color"
          value={themeStore.textPrimary}
          oninput={handleTextPrimaryChange}
          class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent p-0"
        />
      </div>
    </div>

    <!-- Secondary Text Color -->
    <div
      class="flex items-center gap-4 p-4 rounded-xl border border-surface/8 bg-surface/3 hover:bg-surface/5 transition-all"
    >
      <div class="flex-1">
        <p class="text-sm font-semibold text-text-primary">
          {$_("settings.theme.text_secondary_label")}
        </p>
        <p class="text-xs text-text-muted">
          {$_("settings.theme.text_secondary_desc")}
        </p>
      </div>
      <div
        class="shrink-0 flex items-center gap-2 bg-surface/5 px-3 py-2 rounded-lg border border-surface/10"
      >
        <span class="text-xs font-mono text-text-secondary uppercase"
          >{themeStore.textSecondary}</span
        >
        <input
          type="color"
          value={themeStore.textSecondary}
          oninput={handleTextSecondaryChange}
          class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent p-0"
        />
      </div>
    </div>
  </div>
</div>

<div class="grid grid-cols-2 gap-3 mt-1">
  <div
    class="p-3 rounded-lg border border-surface/5 bg-surface/2 flex flex-col gap-1"
  >
    <span class="text-xs text-text-muted uppercase tracking-wider"
      >{$_("settings.theme.preview_button")}</span
    >
    <button
      class="w-full py-2 bg-accent text-white rounded text-xs font-bold"
      >{$_("settings.theme.preview_button_primary")}</button
    >
  </div>
  <div
    class="p-3 rounded-lg border border-surface/5 bg-surface/2 flex flex-col gap-1"
  >
    <span class="text-xs text-text-muted uppercase tracking-wider"
      >{$_("settings.theme.preview_active")}</span
    >
    <div
      class="flex items-center gap-2 text-accent-light text-xs font-medium p-2 bg-accent-dim rounded"
    >
      <IconCheck size={14} />
      {$_("settings.theme.preview_active_item")}
    </div>
  </div>
</div>

<hr class="border-surface/8 my-1" />

<!-- ─ Background Section ─ -->
<div class="space-y-4">
  <h1 class="text-xl font-bold text-text-primary flex items-center gap-2">
    <IconPhoto size={20} class="text-accent" />
    {$_("settings.theme.bg_description")}
  </h1>

  <div class="grid grid-cols-1 gap-3">
    <!-- Option: Gradient -->
    <button
      class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
        {themeStore.bgType === 'gradient'
        ? 'border-accent bg-accent-dim'
        : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
      onclick={applyGradient}
    >
      <div
        class="w-12 h-12 rounded-lg shrink-0"
        style="background: #07070e;"
      ></div>
      <div class="flex-1">
        <p class="text-sm font-medium text-text-primary">
          {$_("settings.theme.bg_dark")}
        </p>
        <p class="text-xs text-text-muted">
          {$_("settings.theme.bg_dark_desc")}
        </p>
      </div>
      {#if themeStore.bgType === "gradient"}
        <IconCheck size={16} class="text-accent-light shrink-0" />
      {/if}
    </button>

    <!-- Option: Solid color -->
    <button
      class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
        {themeStore.bgType === 'solid'
        ? 'border-accent bg-accent-dim'
        : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
      onclick={applySolid}
    >
      <div
        class="w-12 h-12 rounded-lg shrink-0 border border-surface/10"
        style="background: {themeStore.bgType === 'solid' ? themeStore.bgValue : '#0d0d1a'};"
      ></div>
      <div class="flex-1">
        <p class="text-sm font-medium text-text-primary">
          {$_("settings.theme.bg_solid")}
        </p>
        <p class="text-xs text-text-muted">
          {$_("settings.theme.bg_solid_desc")}
        </p>
      </div>
      <div
        class="shrink-0 p-2 bg-surface/3 rounded-sm flex items-center gap-2"
        onclick={(e) => e.stopPropagation()}
        role="presentation"
      >
        <p class="text-xs">{$_("settings.theme.bg_solid_change")}</p>
        <input
          type="color"
          value={themeStore.bgType === 'solid' ? themeStore.bgValue : '#0d0d1a'}
          oninput={onSolidColorChange}
          class="w-8 h-8 rounded cursor-pointer border-0 bg-transparent p-0"
          aria-label={$_("settings.theme.bg_solid")}
        />
      </div>
      {#if themeStore.bgType === "solid"}
        <IconCheck size={16} class="text-accent-light shrink-0" />
      {/if}
    </button>

    <!-- Option: Custom image -->
    <button
      class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
        {themeStore.bgType === 'image'
        ? 'border-accent bg-accent-dim'
        : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
      onclick={handleSelectImage}
      disabled={loadingImage}
    >
      <div
        class="w-12 h-12 rounded-lg shrink-0 border border-surface/10 bg-bg-overlay flex items-center justify-center overflow-hidden"
      >
        {#if themeStore.bgType === 'image' && themeStore.bgValue}
          <span class="text-xs text-text-muted text-center px-1 leading-tight"
            >{themeStore.bgValue}</span
          >
        {:else}
          <IconPhoto size={20} class="text-text-muted" />
        {/if}
      </div>
      <div class="flex-1">
        <p class="text-sm font-medium text-text-primary">
          {$_("settings.theme.bg_image")}
        </p>
        <p class="text-xs text-text-muted">
          {loadingImage
            ? $_("settings.theme.bg_image_loading")
            : themeStore.bgType === 'image' && themeStore.bgValue
              ? $_("settings.theme.bg_image_click")
              : $_("settings.theme.bg_image_formats")}
        </p>
      </div>
      {#if themeStore.bgType === "image"}
        <IconCheck size={16} class="text-accent-light shrink-0" />
      {/if}
    </button>
  </div>
</div>
