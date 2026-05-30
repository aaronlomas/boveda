<script lang="ts">
  import { uiState } from "$lib/stores/stores.svelte";
  import { IconKey, IconLock } from "@tabler/icons-svelte";
  import { _ } from "svelte-i18n";
  import Card from "$lib/components/core/primitives/Card.svelte";
</script>

<div
  class="grid gap-6"
>
  <div>
    <h1
      class="text-xl font-bold bg-linear-to-br from-text-primary to-accent-light bg-clip-text text-transparent"
    >
      {$_("general_view.general")}
    </h1>
    <p class="text-text-muted text-sm">
      {$_("general_view.desc")}
    </p>
  </div>

  <div class="grid grid-cols-3 gap-4">
    <!-- Card Accounts -->
    <Card
      title={$_("accounts_mode.title")}
      description={$_("accounts_mode.desc")}
      icon={IconKey}
      onclick={() => (uiState.activeView = "accounts")}
    />

    <!-- Card Save PIN -->
    <Card
      title={$_("pin_security_mode.title")}
      description={$_("pin_security_mode.desc")}
      icon={IconLock}
      onclick={() => (uiState.activeView = "pin")}
    />
  </div>
</div>
