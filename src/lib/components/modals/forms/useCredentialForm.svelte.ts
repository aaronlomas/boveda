import { addAccount } from "$lib/utils/tauri";

export function useCredentialForm(getCallbacks: () => { onadded?: () => void; onclose?: () => void }) {
  let site = $state("");
  let username = $state("");
  let password = $state("");
  let recoveryCode = $state("");
  let notes = $state("");
  let loading = $state(false);
  let error = $state("");

  async function submit(translations: { siteError: string; userError: string; pwError: string }) {
    error = "";
    if (!site.trim()) {
      error = translations.siteError;
      return;
    }
    if (!username.trim()) {
      error = translations.userError;
      return;
    }
    if (!password.trim()) {
      error = translations.pwError;
      return;
    }

    loading = true;
    try {
      await addAccount(site.trim(), username.trim(), password, recoveryCode.trim(), notes.trim());
      const callbacks = getCallbacks();
      callbacks.onadded?.();
      callbacks.onclose?.();
    } catch (e: any) {
      error = e.toString();
    } finally {
      loading = false;
    }
  }

  return {
    get site() { return site; },
    set site(v) { site = v; },
    get username() { return username; },
    set username(v) { username = v; },
    get password() { return password; },
    set password(v) { password = v; },
    get recoveryCode() { return recoveryCode; },
    set recoveryCode(v) { recoveryCode = v; },
    get notes() { return notes; },
    set notes(v) { notes = v; },
    get loading() { return loading; },
    get error() { return error; },
    submit
  };
}
