<script lang="ts">
  /**
   * @component PinView
   * @description View for managing saved PINs. Encapsulates the logic
   * using the composable usePins, decoupling it from direct data retrieval.
   */
  import Button from "../core/primitives/Button.svelte";
  import Capsule from "../core/primitives/Capsule.svelte";
  import { onMount } from "svelte";
  import { _, locale } from "svelte-i18n";
  import {
    IconArrowLeft,
    IconPlus,
    IconSearch,
    IconDialpad,
  } from "@tabler/icons-svelte";
  import { uiState } from "$lib/stores/stores.svelte";
  import { modal } from "$lib/stores/modal.svelte";
  import { usePins } from "$lib/composables/usePins.svelte";
  import PinCard from "$lib/components/features/pins/PinCard.svelte";

  // ── Composable ────────────────────────────────────────────
  const pinService = usePins();
  let search = $state("");

  // ── Derived State ──────────────────────────────────────────────────────────
  let filtered = $derived(
    pinService.pins.filter((p) =>
      p.name?.toLowerCase().includes(search.toLowerCase()),
    ),
  );

  // ── Handlers ───────────────────────────────────────────────────────────────
  async function handleNewPin() {
    const added = await modal.openAddPin();
    if (added) {
      await pinService.refresh();
    }
  }

  onMount(() => {
    pinService.refresh();
  });
</script>

<div class="flex flex-col h-full">
  <!-- Header -->
  <header class="flex items-center justify-between mb-7 gap-4">
    <div class="flex gap-x-4">
      <button
        class="p-2 bg-surface/5 hover:bg-surface/10 rounded-lg text-text-muted hover:text-text-primary transition-colors row-span-2 my-auto cursor-pointer"
        onclick={() => (uiState.activeView = "general")}
        aria-label="Back"
      >
        <IconArrowLeft size={20} />
      </button>
      <div>
        <h1
          class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent pointer-events-none"
        >
          {$_("pin_security_mode.title")}
        </h1>
        <p class="text-text-muted text-sm mt-0.5 pointer-events-none">
          {$_("pin_security_mode.desc")}
        </p>
      </div>
    </div>

    <Button onclick={handleNewPin}>
      <IconPlus size={16} />
      {$_("pin_security.new_pin")}
    </Button>
  </header>

  <!-- Search -->
  <div
    class="flex items-center mb-6 border border-surface/10 rounded-lg text-text-primary px-4 py-3 focus-within:border-accent/50 transition-colors"
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

  {#if pinService.error}
    <div class="text-center py-16">
      <p
        class="text-danger bg-danger/10 p-4 rounded-xl border border-danger/20 inline-block"
      >
        {pinService.error}
      </p>
      <button
        class="block mx-auto mt-4 text-accent underline cursor-pointer"
        onclick={() => pinService.refresh()}
      >
        {$_("actions.retry")}
      </button>
    </div>
  {:else if filtered.length === 0}
    <div
      class="text-center py-20 px-5 flex flex-col items-center gap-3 text-text-secondary bg-surface/4 backdrop-blur-2xl rounded-2xl border border-surface/8 shadow-xl"
    >
      <div class="text-accent/20 mb-2">
        <IconDialpad size={80} stroke={1.5} />
      </div>
      <h3 class="text-lg text-text-primary font-semibold">
        {search
          ? $_("pin_security.no_coincidences")
          : $_("pin_security.no_pins")}
      </h3>
      <p class="text-text-muted">
        {search ? "" : $_("pin_security.no_pins_desc")}
      </p>
      {#if !search}
        <Button onclick={handleNewPin}>
          <IconPlus size={16} />
          {$_("pin_security.new_pin")}
        </Button>
      {/if}
    </div>
  {:else}
    <div class="grid grid-cols-[repeat(auto-fill,minmax(320px,1fr))] gap-4 flex-1 overflow-y-auto min-h-0 pb-4 content-start">
      {#each filtered as pinEntry (pinEntry.id)}
        <PinCard
          {pinEntry}
          locale={$locale ?? "es"}
          ondelete={(id) => pinService.delete(id)}
        />
      {/each}
    </div>
  {/if}
</div>
