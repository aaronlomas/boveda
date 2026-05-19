<script lang="ts">
  /**
   * @component AddCredentialModal
   * @description Formulario para agregar nuevas credenciales a la bóveda.
   * Maneja la validación y el envío de datos de usuario, contraseña y códigos de recuperación.
   * 
   * @param {Object} props
   * @param {() => void} props.onadded - Callback ejecutado exitosamente tras guardar la credencial.
   * @param {() => void} props.onclose - Callback para cerrar el modal.
   */
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
    form.handleSubmit();
  }
</script>

<Modal show={true} onclose={props.onclose} title={$_("add_credential.title")}>
  <form
    id="add-credential-form"
    class="flex flex-col gap-4"
    onsubmit={handleSubmit}
  >
    <CredentialFields
      bind:site={form.values.site}
      bind:username={form.values.username}
      bind:password={form.values.password}
      bind:recoveryCode={form.values.recoveryCode}
      bind:notes={form.values.notes}
      errors={form.errors}
    />

    {#if form.globalError}
      <Alert variant="destructive">{form.globalError}</Alert>
    {/if}
  </form>

  {#snippet footer()}
    <Button variant="ghost" onclick={props.onclose}>
      {$_("actions.cancel")}
    </Button>
    <Button type="submit" form="add-credential-form" disabled={form.loading}>
      {#if form.loading}
        <span
          class="w-3.5 h-3.5 border-2 border-surface/30 border-t-white rounded-full animate-spin mr-1.5"
        ></span>
        {$_("actions.status.saving")}
      {:else}
        {$_("actions.save")}
      {/if}
    </Button>
  {/snippet}
</Modal>
