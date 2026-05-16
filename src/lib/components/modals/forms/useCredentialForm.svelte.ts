import { addAccount } from "$lib/utils/tauri";
import { useForm } from "$lib/validation/useForm.svelte";
import { credentialSchema, type CredentialForm } from "$lib/validation/schemas";

export function useCredentialForm(getCallbacks: () => { onadded?: () => void; onclose?: () => void }) {
  const form = useForm<CredentialForm>(
    credentialSchema,
    {
      site: "",
      username: "",
      password: "",
      recoveryCode: "",
      notes: ""
    },
    async (values) => {
      await addAccount(
        values.site.trim(),
        values.username.trim(),
        values.password,
        values.recoveryCode?.trim() || "",
        values.notes?.trim() || ""
      );
      const callbacks = getCallbacks();
      callbacks.onadded?.();
      callbacks.onclose?.();
    }
  );

  return form;
}
