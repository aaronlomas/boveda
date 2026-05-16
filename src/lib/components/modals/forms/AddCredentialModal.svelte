<script lang="ts">
  import { _ } from "svelte-i18n";
  import Modal from "../../ui/primitives/Modal.svelte";
  import Button from "../../ui/primitives/Button.svelte";
  import Alert from "../../ui/feedback/Alert.svelte";
  import CredentialFields from "./CredentialFields.svelte";
  import { useCredentialForm } from "./useCredentialForm.svelte";

  let props = $props();
  const form = useCredentialForm(() => props);

  function handleSubmit(e: Event) {
    e.preventDefault();
    form.submit({
      siteError: $_("add_credential.error_site"),
      userError: $_("add_credential.error_user"),
      pwError: $_("add_credential.error_password"),
    });
  }
</script>

<Modal show={true} onclose={props.onclose} title={$_("add_credential.title")}>
  <form
    id="add-credential-form"
    class="flex flex-col gap-4"
    onsubmit={handleSubmit}
  >
    <CredentialFields
      bind:site={form.site}
      bind:username={form.username}
      bind:password={form.password}
      bind:recoveryCode={form.recoveryCode}
      bind:notes={form.notes}
    />

    {#if form.error}
      <Alert variant="destructive">{form.error}</Alert>
    {/if}
  </form>

  {#snippet footer()}
    <Button variant="ghost" onclick={onclose}>
      {$_("add_credential.cancel_button")}
    </Button>
    <Button type="submit" form="add-credential-form" disabled={form.loading}>
      {#if form.loading}
        <span
          class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin mr-1.5"
        ></span>
        {$_("add_credential.saving")}
      {:else}
        {$_("add_credential.save_button")}
      {/if}
    </Button>
  {/snippet}
</Modal>
