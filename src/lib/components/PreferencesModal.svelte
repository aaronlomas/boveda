<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { themeStore, PRESETS } from "$lib/theme";
  import type { BackgroundType, ColorScheme, ThemePreset } from "$lib/theme";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { IconX, IconPalette, IconPhoto, IconCheck, IconLanguage, IconTypography, IconSun, IconMoon, IconDeviceDesktop, IconAdjustmentsHorizontal } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import { changeLanguage } from "$lib/i18n";
  import { locale } from "svelte-i18n";

  const dispatch = createEventDispatcher();

  let activeSection: "theme" | "language" = "theme";
  
  // Local state for immediate binding
  let colorScheme = $themeStore.colorScheme;
  let accentColor = $themeStore.accentColor;
  let textPrimary = $themeStore.textPrimary;
  let textSecondary = $themeStore.textSecondary;
  let bgType = $themeStore.bgType;
  let solidColor = $themeStore.bgType === "solid" ? $themeStore.bgValue : "#0d0d1a";
  let imageFilename = $themeStore.bgType === "image" ? $themeStore.bgValue : "";
  let loadingImage = false;

  function close(e: MouseEvent) {
    if (e.target === e.currentTarget) dispatch("close");
  }

  async function handleAccentChange(e: Event) {
    accentColor = (e.target as HTMLInputElement).value;
    await themeStore.setAccentColor(accentColor);
  }

  async function handleColorSchemeChange(scheme: ColorScheme) {
    colorScheme = scheme;
    await themeStore.setColorScheme(scheme);
  }

  async function handleApplyPreset(preset: ThemePreset) {
    await themeStore.applyPreset(preset);
    colorScheme = $themeStore.colorScheme;
    accentColor = $themeStore.accentColor;
    textPrimary = $themeStore.textPrimary;
    textSecondary = $themeStore.textSecondary;
    bgType = $themeStore.bgType;
    solidColor = $themeStore.bgType === "solid" ? $themeStore.bgValue : "#0d0d1a";
    imageFilename = $themeStore.bgType === "image" ? $themeStore.bgValue : "";
  }

  async function handleTextColorsChange() {
    await themeStore.setTextColors(textPrimary, textSecondary);
  }

  async function handleSelectImage() {
    loadingImage = true;
    try {
      const filePath = await open({
        title: $_("global.select_bg_image_title"),
        filters: [{ name: $_("global.images_filter_name"), extensions: ["jpg", "jpeg", "png", "webp", "gif"] }]
      });
      if (filePath) {
        const filename = await invoke<string>("import_background_image", { srcPath: filePath });
        imageFilename = filename;
        await themeStore.setBackground("image", filename);
        bgType = "image";
      }
    } catch (e) {
      console.error("Error importing background:", e);
    } finally {
      loadingImage = false;
    }
  }

  async function applyGradient() {
    bgType = "gradient";
    await themeStore.setBackground("gradient", "");
  }

  async function applySolid() {
    bgType = "solid";
    await themeStore.setBackground("solid", solidColor);
  }

  async function onSolidColorChange(e: Event) {
    solidColor = (e.target as HTMLInputElement).value;
    if (bgType === "solid") {
      await themeStore.setBackground("solid", solidColor);
    }
  }

  async function handleRestore() {
    await themeStore.resetToDefaults();
    // Update local state to reflect defaults
    colorScheme = $themeStore.colorScheme;
    accentColor = $themeStore.accentColor;
    textPrimary = $themeStore.textPrimary;
    textSecondary = $themeStore.textSecondary;
    bgType = $themeStore.bgType;
    solidColor = $themeStore.bgType === "solid" ? $themeStore.bgValue : "#0d0d1a";
    imageFilename = $themeStore.bgType === "image" ? $themeStore.bgValue : "";
  }
</script>

<!-- Backdrop -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class="fixed inset-0 bg-panel/60 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  on:click={close}
>
  <div
    class="w-[min(540px,95vw)] max-h-[90vh] bg-surface/4 backdrop-blur-2xl border border-surface/10 rounded-2xl shadow-2xl flex flex-col overflow-hidden"
    on:click|stopPropagation
  >
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-surface/8">
      <h2 class="text-[16px] font-semibold text-text-primary">{$_("settings.title")}</h2>
      <button
        class="p-1.5 rounded-lg text-text-muted hover:text-text-primary hover:bg-surface/8 transition-all"
        on:click={() => dispatch("close")}
        aria-label={$_("settings.close")}
      >
        <IconX size={18} />
      </button>
    </div>

    <!-- Tabs -->
    <div class="flex border-b border-surface/8">
      <button
        class="flex items-center gap-2 px-6 py-3 text-[13px] font-medium transition-all border-b-2 {activeSection === 'theme' ? 'border-accent text-accent-light' : 'border-transparent text-text-muted hover:text-text-secondary'}"
        on:click={() => (activeSection = "theme")}
      >
        <IconPalette size={15} /> {$_("settings.tabs.theme")}
      </button>
      <button
        class="flex items-center gap-2 px-6 py-3 text-[13px] font-medium transition-all border-b-2 {activeSection === 'language' ? 'border-accent text-accent-light' : 'border-transparent text-text-muted hover:text-text-secondary'}"
        on:click={() => (activeSection = "language")}
      >
        <IconLanguage size={15} /> {$_("settings.tabs.language")}
      </button>
    </div>

    <!-- Body -->
    <div class="p-6 flex flex-col gap-5 flex-1 overflow-y-auto custom-scrollbar bg-panel/20">

      {#if activeSection === "theme"}
        <!-- ─ Presets Section ─ -->
        <div class="space-y-4">
          <h2 class="text-text-primary text-[15px] font-bold flex items-center gap-2">
            <IconPalette size={18} class="text-accent" />
            {$_("settings.theme.presets_title")}
          </h2>
          <div class="flex overflow-x-auto gap-3 pb-2 custom-scrollbar">
            {#each PRESETS as preset}
              <button 
                class="min-w-[120px] p-3 rounded-xl border transition-all flex flex-col gap-3 text-left
                  {$themeStore.activePresetId === preset.id ? 'border-accent bg-accent-dim' : 'border-surface/8 bg-surface/3 hover:bg-surface/5'}"
                on:click={() => handleApplyPreset(preset)}
              >
                <div class="flex gap-2">
                  <div class="w-6 h-6 rounded-full border border-surface/10" style="background-color: {preset.accentColor}"></div>
                  <div class="w-6 h-6 rounded-full border border-surface/10" style="background-color: {preset.bgType === 'solid' ? preset.bgValue : (preset.colorScheme === 'dark' ? '#07070e' : '#f3f4f6')}"></div>
                </div>
                <span class="text-[12px] font-medium text-text-primary leading-tight">{$_(preset.nameKey)}</span>
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
            <button class="p-3 rounded-xl border transition-all flex flex-col items-center gap-2 {colorScheme === 'light' ? 'border-accent bg-accent-dim text-accent' : 'border-surface/8 bg-surface/3 text-text-muted hover:border-surface/20 hover:text-text-primary'}" on:click={() => handleColorSchemeChange('light')}>
              <IconSun size={24} />
              <span class="text-[12px] font-medium">{$_("settings.theme.mode_light")}</span>
            </button>
            <button class="p-3 rounded-xl border transition-all flex flex-col items-center gap-2 {colorScheme === 'dark' ? 'border-accent bg-accent-dim text-accent' : 'border-surface/8 bg-surface/3 text-text-muted hover:border-surface/20 hover:text-text-primary'}" on:click={() => handleColorSchemeChange('dark')}>
              <IconMoon size={24} />
              <span class="text-[12px] font-medium">{$_("settings.theme.mode_dark")}</span>
            </button>
            <button class="p-3 rounded-xl border transition-all flex flex-col items-center gap-2 {colorScheme === 'system' ? 'border-accent bg-accent-dim text-accent' : 'border-surface/8 bg-surface/3 text-text-muted hover:border-surface/20 hover:text-text-primary'}" on:click={() => handleColorSchemeChange('system')}>
              <IconDeviceDesktop size={24} />
              <span class="text-[12px] font-medium">{$_("settings.theme.mode_system")}</span>
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

          <div class="flex items-center gap-4 p-4 rounded-xl border border-accent/20 bg-accent-dim transition-all">
            <div 
              class="w-12 h-12 rounded-full shadow-lg ring-4 ring-surface/5 shrink-0" 
              style="background-color: {accentColor};"
            ></div>
            <div class="flex-1">
              <p class="text-[14px] font-bold text-text-primary">{$_("settings.theme.color_label")}</p>
              <p class="text-[11px] text-text-muted">{$_("settings.theme.color_desc")}</p>
            </div>
            <div class="shrink-0 flex items-center gap-2 bg-surface/5 px-2.5 py-1.5 rounded-lg border border-surface/10 hover:bg-surface/10 transition-all">
              <span class="text-[11px] font-mono text-text-secondary uppercase">{accentColor}</span>
              <input
                type="color"
                bind:value={accentColor}
                on:input={handleAccentChange}
                on:click|stopPropagation
                class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent p-0"
                aria-label={$_("settings.theme.color_label")}
              />
            </div>
          </div>
        </div>

        <!-- ─ Text Colors Section ─ -->
        <div class="space-y-4 pt-2">
          <h2 class="text-text-primary text-[15px] font-bold flex items-center gap-2">
            <IconTypography size={18} class="text-accent" />
            Personalización de Texto
          </h2>
          
          <div class="grid grid-cols-1 gap-3">
            <!-- Primary Text Color -->
            <div class="flex items-center gap-4 p-4 rounded-xl border border-surface/8 bg-surface/3 hover:bg-surface/5 transition-all">
              <div class="flex-1">
                <p class="text-[13px] font-semibold text-text-primary">{$_("settings.theme.text_primary_label")}</p>
                <p class="text-[11px] text-text-muted">{$_("settings.theme.text_primary_desc")}</p>
              </div>
              <div class="shrink-0 flex items-center gap-2 bg-surface/5 px-2.5 py-1.5 rounded-lg border border-surface/10">
                <span class="text-[11px] font-mono text-text-secondary uppercase">{textPrimary}</span>
                <input
                  type="color"
                  bind:value={textPrimary}
                  on:input={handleTextColorsChange}
                  class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent p-0"
                />
              </div>
            </div>

            <!-- Secondary Text Color -->
            <div class="flex items-center gap-4 p-4 rounded-xl border border-surface/8 bg-surface/3 hover:bg-surface/5 transition-all">
              <div class="flex-1">
                <p class="text-[13px] font-semibold text-text-primary">{$_("settings.theme.text_secondary_label")}</p>
                <p class="text-[11px] text-text-muted">{$_("settings.theme.text_secondary_desc")}</p>
              </div>
              <div class="shrink-0 flex items-center gap-2 bg-surface/5 px-2.5 py-1.5 rounded-lg border border-surface/10">
                <span class="text-[11px] font-mono text-text-secondary uppercase">{textSecondary}</span>
                <input
                  type="color"
                  bind:value={textSecondary}
                  on:input={handleTextColorsChange}
                  class="w-7 h-7 rounded cursor-pointer border-0 bg-transparent p-0"
                />
              </div>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-2 gap-3 mt-1">
            <div class="p-3 rounded-lg border border-surface/5 bg-surface/2 flex flex-col gap-1">
                <span class="text-[10px] text-text-muted uppercase tracking-wider">{$_("settings.theme.preview_button")}</span>
                <button class="w-full py-2 bg-accent text-white rounded text-[12px] font-bold">{$_("settings.theme.preview_button_primary")}</button>
            </div>
            <div class="p-3 rounded-lg border border-surface/5 bg-surface/2 flex flex-col gap-1">
                <span class="text-[10px] text-text-muted uppercase tracking-wider">{$_("settings.theme.preview_active")}</span>
                <div class="flex items-center gap-2 text-accent-light text-[12px] font-medium p-2 bg-accent-dim rounded">
                    <IconCheck size={14} /> {$_("settings.theme.preview_active_item")}
                </div>
            </div>
        </div>

        <hr class="border-surface/8 my-1" />

        <!-- ─ Background Section ─ -->
        <div class="space-y-4">
          <p class="text-xl font-bold text-text-primary flex items-center gap-2">
            <IconPhoto size={20} class="text-accent" />
            {$_("settings.theme.bg_description")}
          </p>

          <div class="grid grid-cols-1 gap-3">
            <!-- Option: Gradient -->
            <button
              class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
                {bgType === 'gradient' ? 'border-accent bg-accent-dim' : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
              on:click={applyGradient}
            >
              <div class="w-12 h-12 rounded-lg shrink-0" style="background: #07070e;"></div>
              <div class="flex-1">
                <p class="text-[13px] font-medium text-text-primary">{$_("settings.theme.bg_dark")}</p>
                <p class="text-[11px] text-text-muted">{$_("settings.theme.bg_dark_desc")}</p>
              </div>
              {#if bgType === "gradient"}
                <IconCheck size={16} class="text-accent-light shrink-0" />
              {/if}
            </button>

            <!-- Option: Solid color -->
            <button
              class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
                {bgType === 'solid' ? 'border-accent bg-accent-dim' : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
              on:click={applySolid}
            >
              <div class="w-12 h-12 rounded-lg shrink-0 border border-surface/10" style="background: {solidColor};"></div>
              <div class="flex-1">
                <p class="text-[13px] font-medium text-text-primary">{$_("settings.theme.bg_solid")}</p>
                <p class="text-[11px] text-text-muted">{$_("settings.theme.bg_solid_desc")}</p>
              </div>
              <div class="shrink-0 p-2 bg-surface/3 rounded-sm flex items-center gap-2" on:click|stopPropagation>
                <p class="text-[12px]">{$_("settings.theme.bg_solid_change")}</p>
                <input
                  type="color"
                  bind:value={solidColor}
                  on:input={onSolidColorChange}
                  on:change={applySolid}
                  class="w-8 h-8 rounded cursor-pointer border-0 bg-transparent p-0"
                  aria-label={$_("settings.theme.bg_solid")}
                />
              </div>
              {#if bgType === "solid"}
                <IconCheck size={16} class="text-accent-light shrink-0" />
              {/if}
            </button>

            <!-- Option: Custom image -->
            <button
              class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
                {bgType === 'image' ? 'border-accent bg-accent-dim' : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
              on:click={handleSelectImage}
              disabled={loadingImage}
            >
              <div
                class="w-12 h-12 rounded-lg shrink-0 border border-surface/10 bg-bg-overlay flex items-center justify-center overflow-hidden"
              >
                {#if imageFilename}
                  <span class="text-[10px] text-text-muted text-center px-1 leading-tight">{imageFilename}</span>
                {:else}
                  <IconPhoto size={20} class="text-text-muted" />
                {/if}
              </div>
              <div class="flex-1">
                <p class="text-[13px] font-medium text-text-primary">{$_("settings.theme.bg_image")}</p>
                <p class="text-[11px] text-text-muted">
                  {loadingImage ? $_("settings.theme.bg_image_loading") : imageFilename ? $_("settings.theme.bg_image_click") : $_("settings.theme.bg_image_formats")}
                </p>
              </div>
              {#if bgType === "image"}
                <IconCheck size={16} class="text-accent-light shrink-0" />
              {/if}
            </button>
          </div>
        </div>

      {:else if activeSection === "language"}
        <!-- ─ Language Section ─ -->
        <p class="text-[12px] text-text-muted">
          {$_("settings.language.description")}
        </p>

        <div class="grid grid-cols-1 gap-3">
          <!-- Option: Spanish -->
          <button
            class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
              {$locale === 'es' ? 'border-accent bg-accent-dim' : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
            on:click={() => changeLanguage('es')}
          >
            <div class="w-10 h-10 rounded-lg shrink-0 bg-surface/5 flex items-center justify-center text-lg">🇪🇸</div>
            <div class="flex-1">
              <p class="text-[13px] font-medium text-text-primary">{$_("settings.language.spanish")}</p>
            </div>
            {#if $locale === "es"}
              <IconCheck size={16} class="text-accent-light shrink-0" />
            {/if}
          </button>

          <!-- Option: English -->
          <button
            class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
              {$locale === 'en' ? 'border-accent bg-accent-dim' : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
            on:click={() => changeLanguage('en')}
          >
            <div class="w-10 h-10 rounded-lg shrink-0 bg-surface/5 flex items-center justify-center text-lg">🇺🇸</div>
            <div class="flex-1">
              <p class="text-[13px] font-medium text-text-primary">{$_("settings.language.english")}</p>
            </div>
            {#if $locale === "en"}
              <IconCheck size={16} class="text-accent-light shrink-0" />
            {/if}
          </button>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="px-6 py-4 border-t border-surface/8 flex justify-end gap-3 bg-panel/40">
      <button
        class="px-5 py-2 text-[13px] font-medium bg-surface/5 border border-surface/10 rounded-lg text-text-secondary hover:bg-surface/10 hover:text-text-primary transition-all"
        on:click={handleRestore}
      >
        {$_("settings.restore")}
      </button>
      <button
        class="px-5 py-2 text-[13px] font-medium bg-accent text-white rounded-lg hover:bg-accent-hover transition-all shadow-lg shadow-accent/20"
        on:click={() => dispatch("close")}
      >
        {$_("settings.close")}
      </button>
    </div>
  </div>
</div>
