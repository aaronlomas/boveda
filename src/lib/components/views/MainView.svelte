<script lang="ts">
  /* Cache bust comment */
  import { uiState, type ViewId } from "$lib/stores/stores.svelte";
  import type { Component } from "svelte";
  import AccountsView from "./AccountsView.svelte";
  import GeneralView from "./GeneralView.svelte";
  import DocumentsView from "./DocumentsView.svelte";
  import AboutView from "./AboutView.svelte";
  import PinView from "./PinView.svelte";
  import SettingsView from "../settings-content/SettingsView.svelte";

  const viewMap: Partial<Record<ViewId, Component>> = {
    general: GeneralView,
    accounts: AccountsView,
    documents: DocumentsView,
    about: AboutView,
    pin: PinView,
    settings: SettingsView,
  };

  const ActiveView = $derived(viewMap[uiState.activeView]);
</script>

<div class="h-full w-full">
  {#if ActiveView}
    <ActiveView />
  {/if}
</div>
