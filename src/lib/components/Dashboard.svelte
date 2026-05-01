<script lang="ts">
  /* Cache bust comment */
  import { activeView } from "$lib/stores";
  import AccountsView from "./AccountsView.svelte";
  import GeneralView from "./GeneralView.svelte";
  import DocumentsView from "./DocumentsView.svelte";
  import AboutView from "./AboutView.svelte";
  import PinView from "./PinView.svelte";
  import TokensView from "./TokensView.svelte";
</script>

<div class="h-full w-full">
  {#if $activeView === 'general'}
    <GeneralView />
  {:else if $activeView === 'accounts'}
    <AccountsView />
  {:else if $activeView === 'documents'}
    <DocumentsView />
  {:else if $activeView === 'about'}
    <AboutView />
  {:else if $activeView === 'pin'}
    <PinView />
  {:else if $activeView === 'tokens'}
    <TokensView />
  {/if}
</div>

<style>
  /* Empty style block to ensure Vite doesn't misidentify the file style section */
</style>
