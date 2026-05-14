<script lang="ts">
  import { _ } from "svelte-i18n";
  import { 
    IconLock, 
    IconArrowLeft, 
    IconPlus,
    IconSearch,
    IconDialpad
  } from "@tabler/icons-svelte";
  import { globalState } from "$lib/stores/stores.svelte";
  import { toast } from "$lib/stores/toast.svelte";

  let search = $state("");
  let pins = $state<any[]>([]); // Mocked for now, will connect to backend later

  let filtered = $derived(
    pins.filter((p) => p.name?.toLowerCase().includes(search.toLowerCase()))
  );

  function handleNewPin() {
    toast.info($_("pin_security.coming_soon_modal"));
  }
</script>

<div
  class="max-w-6xl mx-auto animate-in fade-in slide-in-from-bottom-2 duration-300 pb-10"
>
  <!-- Header -->
  <header class="flex items-center justify-between mb-7 gap-4">
    <div class="flex gap-x-4">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-lg text-text-muted hover:text-text-primary transition-colors row-span-2 my-auto cursor-pointer"
        onclick={() => (globalState.activeView = "general")}
        aria-label="Back"
      >
        <IconArrowLeft size={20} />
      </button>
      <div>
        <h1
          class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent pointer-events-none"
        >
          {$_("pin_security.title")}
        </h1>
        <p class="text-text-muted text-sm mt-0.5 pointer-events-none">
          {$_("pin_security.desc")}
        </p>
      </div>
    </div>

    <button
      class="inline-flex items-center justify-center gap-2 py-2 px-4 rounded-lg text-sm font-medium cursor-pointer transition-all border border-transparent bg-accent text-white hover:scale-[0.98] shadow-md shadow-accent/20"
      onclick={handleNewPin}
    >
      <IconPlus size={16} />
      {$_("pin_security.new_pin")}
    </button>
  </header>

  <!-- Search -->
  <div
    class="flex items-center mb-6 border border-surface/10 rounded-lg text-text-primary px-4 py-3"
  >
    <div class="text-text-muted text-lg">
      <IconSearch size={18} />
    </div>
    <input
      class="w-full pl-4 bg-transparent text-sm placeholder:text-text-muted focus:outline-none transition-all"
      bind:value={search}
      placeholder={$_("pin_security.search_placeholder")}
    />
  </div>

  <!-- Empty State -->
  {#if filtered.length === 0}
    <div
      class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary"
    >
      <div class="text-accent/20 mb-2">
        <IconDialpad size={80} stroke={1.5} />
      </div>
      <h3 class="text-lg text-text-primary font-semibold">
        {search ? $_("dashboard.no_credentials") : $_("pin_security.no_pins")}
      </h3>
      <p class="text-text-muted">
        {search ? "" : $_("pin_security.no_pins_desc")}
      </p>
      {#if !search}
        <button
          class="inline-flex items-center justify-center gap-2 px-4 py-2 mt-2 rounded-lg text-sm font-medium cursor-pointer transition-all border border-transparent bg-accent text-white hover:bg-accent-hover shadow-lg shadow-accent/20"
          onclick={handleNewPin}
        >
          <IconPlus size={16} />
          {$_("pin_security.new_pin")}
        </button>
      {/if}
    </div>
  {:else}
    <!-- Grid for future PIN cards -->
    <div class="grid grid-cols-[repeat(auto-fill,minmax(320px,1fr))] gap-4">
      <!-- Mapped PIN cards will go here -->
    </div>
  {/if}
</div>
