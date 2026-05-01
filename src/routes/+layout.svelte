<script lang="ts">
  import "../layout.css";
  import { onMount } from "svelte";
  import { themeStore } from "$lib/theme";
  import { initI18n } from "$lib/i18n";
  import { isLoading, waitLocale } from "svelte-i18n";

  let initialized = false;

  onMount(async () => {
    // Start i18n init as early as possible
    await initI18n();
    // Wait for the locale to be loaded
    await waitLocale();
    // Initialize theme
    await themeStore.init();
    
    initialized = true;
  });
</script>

{#if initialized && !$isLoading}
  <slot />
{/if}
