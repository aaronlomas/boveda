<script lang="ts">
  /* Cache bust comment */
  import { globalState } from "$lib/stores/stores.svelte";
  import AccountsView from "./AccountsView.svelte";
  import GeneralView from "./GeneralView.svelte";
  import DocumentsView from "./DocumentsView.svelte";
  import AboutView from "./AboutView.svelte";
  import PinView from "./PinView.svelte";
  import SettingsView from "./SettingsView.svelte";
</script>

<div class="h-full w-full">
  {#if globalState.activeView === "general"}
    <GeneralView />
  {:else if globalState.activeView === "accounts"}
    <AccountsView />
  {:else if globalState.activeView === "documents"}
    <DocumentsView />
  {:else if globalState.activeView === "about"}
    <AboutView />
  {:else if globalState.activeView === "pin"}
    <PinView />
  {:else if globalState.activeView === "settings"}
    <SettingsView />
  {/if}
</div>
