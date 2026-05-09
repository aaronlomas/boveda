<script lang="ts">
  import { _, locale } from "svelte-i18n";
  import { changeLanguage } from "$lib/i18n";
  import { IconCheck } from "@tabler/icons-svelte";
</script>

<!-- ─ Language Section ─ -->
<p class="text-xs text-text-muted">
  {$_("settings.language.description")}
</p>

<div class="grid grid-cols-1 gap-3">
  <!-- Option: Spanish -->
  <button
    class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
      {$locale === 'es'
      ? 'border-accent bg-accent-dim'
      : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
    onclick={() => changeLanguage("es")}
  >
    <div
      class="w-10 h-10 rounded-lg shrink-0 bg-surface/5 flex items-center justify-center overflow-hidden"
    >
      <svg viewBox="0 0 36 24" class="w-6 h-auto rounded-sm shadow-sm">
        <rect width="36" height="24" fill="#AD1519" />
        <rect width="36" height="12" y="6" fill="#FABD00" />
      </svg>
    </div>
    <div class="flex-1">
      <p class="text-sm font-medium text-text-primary">
        {$_("settings.language.spanish")}
      </p>
    </div>
    {#if $locale === "es"}
      <IconCheck size={16} class="text-accent-light shrink-0" />
    {/if}
  </button>

  <!-- Option: English -->
  <button
    class="flex items-center gap-4 p-4 rounded-xl border transition-all text-left w-full cursor-pointer
      {$locale === 'en'
      ? 'border-accent bg-accent-dim'
      : 'border-surface/8 bg-surface/3 hover:border-surface/20'}"
    onclick={() => changeLanguage("en")}
  >
    <div
      class="w-10 h-10 rounded-lg shrink-0 bg-surface/5 flex items-center justify-center overflow-hidden"
    >
      <svg viewBox="0 0 36 24" class="w-6 h-auto rounded-sm shadow-sm">
        <rect width="36" height="24" fill="#FFFFFF" />
        <path d="M0 0h36v1.846H0zM0 3.692h36v1.846H0zM0 7.385h36v1.846H0zM0 11.077h36v1.846H0zM0 14.769h36v1.846H0zM0 18.462h36v1.846H0zM0 22.154h36v1.846H0z" fill="#B22234" />
        <rect width="14.4" height="12.923" fill="#3C3B6E" />
        <path d="M1.325 1.5l.325.96h1.01l-.817.593.312.96-.83-.603-.83.603.312-.96-.817-.593H1l.325-.96z" fill="#FFFFFF" transform="scale(1.2)" />
      </svg>
    </div>
    <div class="flex-1">
      <p class="text-sm font-medium text-text-primary">
        {$_("settings.language.english")}
      </p>
    </div>
    {#if $locale === "en"}
      <IconCheck size={16} class="text-accent-light shrink-0" />
    {/if}
  </button>
</div>

