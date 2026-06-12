<script lang="ts">
  import { _ } from "svelte-i18n";
  import { IconEye, IconEyeOff } from "@tabler/icons-svelte";
  import { focus } from "$lib/utils/actions";
  import PasswordStrength from "../../modals/forms/PasswordStrength.svelte";

  let {
    password = $bindable(),
    confirmPassword = $bindable(),
    isNew,
    cooldown,
  } = $props<{
    password: string;
    confirmPassword: string;
    isNew: boolean;
    cooldown: number;
  }>();

  let showPassword = $state(false);
  let showConfirmPassword = $state(false);
</script>

<div class="grid gap-1">
  <label for="master-pw" class="text-xs text-text-primary">
    {$_("unlock_screen.master_password_label")}
  </label>

  <div
    class="flex border border-surface/10 rounded-lg px-4 py-2 gap-x-2 bg-transparent {cooldown >
    0
      ? 'opacity-50 grayscale'
      : ''}"
  >
  <!-- MASTER PASS INPUT -->
    <input
      id="master-pw"
      use:focus
      class="w-full text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:bg-transparent tracking-widest disabled:cursor-not-allowed"
      type={showPassword ? "text" : "password"}
      bind:value={password}
      placeholder={$_("unlock_screen.placeholder")}
      autocomplete="current-password"
      disabled={cooldown > 0}
    />
    <button
      type="button"
      class="bg-none border-none cursor-pointer text-text-muted hover:text-text-primary transition-all flex items-center disabled:opacity-50"
      onclick={() => (showPassword = !showPassword)}
      disabled={cooldown > 0}
    >
      {#if showPassword}
        <IconEyeOff size={18} />
      {:else}
        <IconEye size={18} />
      {/if}
    </button>
  </div>
</div>
{#if isNew}
  <div class="flex flex-col gap-1 mt-2">
    <label for="confirm-pw" class="text-xs text-text-primary">
      {$_("unlock_screen.confirm_password_label")}
    </label>

    <div
      class="flex border border-surface/10 rounded-lg px-4 py-2 gap-x-2 bg-transparent"
    >
      <input
        id="confirm-pw"
        class="w-full border-0 text-text-primary text-sm placeholder:text-text-muted focus:outline-none focus:bg-transparent tracking-widest"
        type={showConfirmPassword ? "text" : "password"}
        bind:value={confirmPassword}
        placeholder={$_("unlock_screen.placeholder")}
      />

      <button
        type="button"
        class="bg-none border-none cursor-pointer text-text-muted hover:text-text-primary transition-all flex items-center"
        onclick={() => (showConfirmPassword = !showConfirmPassword)}
      >
        {#if showConfirmPassword}
          <IconEyeOff size={18} />
        {:else}
          <IconEye size={18} />
        {/if}
      </button>
    </div>
    <div class="px-2">
      <PasswordStrength {password} />
    </div>
  </div>
{/if}
