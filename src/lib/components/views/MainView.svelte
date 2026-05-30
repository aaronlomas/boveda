<script lang="ts">
  /**
   * @component MainView
   * @description El orquestador principal de vistas de la aplicación.
   * Alterna dinámicamente entre el dashboard general, cuentas, documentos, settings y más
   * basándose en el estado global (uiState.activeView).
   */
  /* Cache bust comment */
  import { uiState, type ViewId } from "$lib/stores/stores.svelte";
  import type { Component } from "svelte";
  import AccountsView from "./AccountsView.svelte";
  import GeneralView from "./GeneralView.svelte";
  import DocumentsView from "./DocumentsView.svelte";
  import AboutView from "./AboutView.svelte";
  import PinView from "./PinView.svelte";
  import SettingsView from "./SettingsView.svelte";
  import ImportView from "./ImportView.svelte";

  const viewMap: Partial<Record<ViewId, Component>> = {
    general: GeneralView,
    accounts: AccountsView,
    documents: DocumentsView,
    about: AboutView,
    pin: PinView,
    settings: SettingsView,
    import: ImportView,
  };

  const ActiveView = $derived(viewMap[uiState.activeView]);
</script>

<div class="h-full w-full">
  {#if ActiveView}
    <ActiveView />
  {/if}
</div>
