<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconEye, IconEyeOff } from "@tabler/icons-svelte";
  import PasswordStrength from "./PasswordStrength.svelte";
  import PasswordGenerator from "./PasswordGenerator.svelte";
  import Input from "../../ui/primitives/Input.svelte";

  interface Props {
    site?: string;
    username?: string;
    password?: string;
    recoveryCode?: string;
    notes?: string;
    errors?: Record<string, string>;
  }

  let {
    site = $bindable(""),
    username = $bindable(""),
    password = $bindable(""),
    recoveryCode = $bindable(""),
    notes = $bindable(""),
    errors = {}
  }: Props = $props();

  let showPw = $state(false);
  let showGenerator = $state(false);
</script>

<div class="flex flex-col gap-4">
  <!-- Site -->
  <Input
    id="add-site"
    label={$_("add_credential.site_label")}
    bind:value={site}
    placeholder={$_("add_credential.site_placeholder")}
    error={errors.site ? $_(`add_credential.${errors.site}`) : undefined}
  />

  <!-- Username -->
  <Input
    id="add-user"
    label={$_("add_credential.user_label")}
    bind:value={username}
    placeholder={$_("add_credential.user_placeholder")}
    autocomplete="off"
    error={errors.username ? $_(`add_credential.${errors.username}`) : undefined}
  />

  <!-- Password -->
  <div class="flex flex-col gap-1.5">
    <div class="flex items-center justify-between">
      <label for="add-pw" class="text-sm font-medium text-text-secondary">
        {$_("add_credential.password_label")}
      </label>
      <button
        type="button"
        class="bg-none border-none text-accent-light text-xs cursor-pointer transition-opacity hover:opacity-80"
        onclick={() => (showGenerator = !showGenerator)}
      >
        {showGenerator ? $_("add_credential.hide") : $_("add_credential.generate")}
      </button>
    </div>
    <div class="relative">
      <input
        id="add-pw"
        class="flex h-10 w-full rounded-md border {errors.password ? 'border-danger' : 'border-surface/10'} bg-surface/4 px-3 py-2 text-sm text-text-primary ring-offset-bg-primary placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:border-accent disabled:cursor-not-allowed disabled:opacity-50 pr-10"
        type={showPw ? "text" : "password"}
        bind:value={password}
        placeholder={$_("add_credential.placeholder") || "•••••••••••••••"}
        autocomplete="new-password"
      />
      <button
        type="button"
        class="absolute right-2.5 top-1/2 -translate-y-1/2 bg-none border-none cursor-pointer p-1 text-text-muted hover:text-text-primary opacity-60 hover:opacity-100 transition-all flex items-center"
        onclick={() => (showPw = !showPw)}
      >
        {#if showPw}
          <IconEyeOff size={18} />
        {:else}
          <IconEye size={18} />
        {/if}
      </button>
    </div>
    
    {#if errors.password}
      <span class="text-[11px] text-danger mt-1 animate-in fade-in slide-in-from-top-1">
        {$_(`add_credential.${errors.password}`)}
      </span>
    {/if}

    <PasswordStrength {password} />

    {#if showGenerator}
      <PasswordGenerator onuse={(pw) => { password = pw; showGenerator = false; }} />
    {/if}
  </div>

  <!-- Recovery Code -->
  <Input
    id="add-recovery"
    label={$_("add_credential.recovery_code_label")}
    bind:value={recoveryCode}
    placeholder={$_("add_credential.recovery_code_placeholder")}
    autocomplete="off"
  />

  <!-- Notes -->
  <div class="flex flex-col gap-1.5">
    <label for="add-notes" class="text-sm font-medium text-text-secondary">
      {$_("add_credential.notes_label")}
    </label>
    <textarea
      id="add-notes"
      class="flex min-h-[80px] w-full rounded-md border border-surface/10 bg-surface/4 px-3 py-2 text-sm text-text-primary ring-offset-bg-primary placeholder:text-text-muted focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:border-accent disabled:cursor-not-allowed disabled:opacity-50 resize-vertical"
      bind:value={notes}
      placeholder={$_("add_credential.notes_placeholder")}
      rows="3"
    ></textarea>
  </div>
</div>
