<script lang="ts">
  /**
   * @component CredentialCard
   * @description Contenedor principal de la tarjeta de credenciales.
   * Orquesta los componentes desacoplados de cabecera y campos de información, gestionando los estados seguros.
   */
  import { _ } from "svelte-i18n";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import type { Account } from "$lib/stores/stores.svelte";
  import { invoke } from "@tauri-apps/api/core";
  
  // Componentes Desacoplados
  import CredentialHeader from "./CredentialHeader.svelte";
  import CredentialField from "./CredentialField.svelte";

  //Props
  let {
    account,
    locale,
    ondelete,
    onrefresh,
  }: {
    account: Account;
    locale: string;
    ondelete: (id: string) => void;
    onrefresh?: () => void;
  } = $props();

  // =========================================================================
  // ESTADOS REACTIVOS
  // =========================================================================
  let revealed = $state(false);
  let decryptedPassword: string | null = $state(null);
  let decryptedRecoveryCode: string | null = $state(null);
  let decryptedNotes: string | null = $state(null);
  let copyTimer: number | null = $state(null);
  let recoveryCopyTimer: number | null = $state(null);
  let userCopyTimer: number | null = $state(null);

  // =========================================================================
  // AUXILIARES Y FORMATEADORES
  // =========================================================================
  /**
   * Formatea la fecha de adición de la credencial según el idioma seleccionado.
   */
  function formatDate(iso: string): string {
    try {
      return new Date(iso).toLocaleDateString(
        locale === "es" ? "es-ES" : "en-US",
        {
          day: "2-digit",
          month: "short",
          year: "numeric",
        },
      );
    } catch {
      return iso;
    }
  }

  // =========================================================================
  // COPILACIÓN Y PORTAPAPELES (SEGURIDAD)
  // =========================================================================
  /**
   * Escribe el texto de manera segura en el portapapeles nativo de la plataforma.
   */
  async function copyToClipboard(
    text: string,
    timerId: "pass" | "user" | "recovery",
  ): Promise<void> {
    try {
      await writeText(text);
    } catch (e) {
      console.warn("Tauri clipboard write failed:", e);
      return;
    }
    startCountdown(timerId);
  }

  /**
   * Descifra la contraseña en el backend de Rust y la copia de forma segura.
   */
  async function copyPassword() {
    try {
      const plain = await invoke<string>("decrypt_secret", {
        ciphertext: account.password_cipher,
      });
      await copyToClipboard(plain, "pass");
    } catch (e) {
      console.error("Failed to decrypt for copy", e);
    }
  }

  /**
   * Descifra el código de recuperación en el backend de Rust y lo copia de forma segura.
   */
  async function copyRecoveryCode() {
    if (!account.recovery_code_cipher) return;
    try {
      const plain = await invoke<string>("decrypt_secret", {
        ciphertext: account.recovery_code_cipher,
      });
      await copyToClipboard(plain, "recovery");
    } catch (e) {
      console.error("Failed to decrypt for copy", e);
    }
  }

  /**
   * Alterna la revelación en pantalla de los campos cifrados (contraseña, códigos, notas).
   */
  async function toggleReveal() {
    if (revealed) {
      revealed = false;
      decryptedPassword = null;
      decryptedRecoveryCode = null;
      decryptedNotes = null;
    } else {
      try {
        decryptedPassword = await invoke<string>("decrypt_secret", {
          ciphertext: account.password_cipher,
        });
        if (account.recovery_code_cipher) {
          decryptedRecoveryCode = await invoke<string>("decrypt_secret", {
            ciphertext: account.recovery_code_cipher,
          });
        }
        if (account.notes_cipher) {
          decryptedNotes = await invoke<string>("decrypt_secret", {
            ciphertext: account.notes_cipher,
          });
        }
        revealed = true;
      } catch (e) {
        console.error("Failed to decrypt", e);
      }
    }
  }

  /**
   * Inicia el temporizador de seguridad de 30 segundos tras copiar un dato sensible.
   */
  function startCountdown(timerId: "pass" | "user" | "recovery"): void {
    const SECONDS = 30;

    if (timerId === "pass") {
      copyTimer = SECONDS;
      const interval = setInterval(() => {
        if (copyTimer === null || copyTimer <= 1) {
          clearInterval(interval);
          copyTimer = null;
          writeText("").catch(() => {});
        } else {
          copyTimer--;
        }
      }, 1000);
    } else if (timerId === "recovery") {
      recoveryCopyTimer = SECONDS;
      const interval = setInterval(() => {
        if (recoveryCopyTimer === null || recoveryCopyTimer <= 1) {
          clearInterval(interval);
          recoveryCopyTimer = null;
          writeText("").catch(() => {});
        } else {
          recoveryCopyTimer--;
        }
      }, 1000);
    } else {
      userCopyTimer = SECONDS;
      const interval = setInterval(() => {
        if (userCopyTimer === null || userCopyTimer <= 1) {
          clearInterval(interval);
          userCopyTimer = null;
        } else {
          userCopyTimer--;
        }
      }, 1000);
    }
  }
</script>

<!-- ========================================================================= -->
<!-- RENDERING DE LA TARJETA DE CREDENCIAL -->
<!-- ========================================================================= -->
<div
  class="p-4 flex flex-col gap-4 transition-all bg-panel/30 backdrop-blur-2xl rounded-2xl border border-surface/8 hover:border-accent/30 hover:translate-y-[-2px] relative"
  data-card-id={account.id}
>
  <!-- Cabecera Modular Desacoplada -->
  <CredentialHeader
    {account}
    ondelete={() => ondelete(account.id)}
    {onrefresh}
  />

  <!-- Campo Modular: Contraseña -->
  <CredentialField
    label={$_("accounts.password_label")}
    value={decryptedPassword || ""}
    isSecret={true}
    {revealed}
    countdown={copyTimer}
    oncopy={copyPassword}
    ontogglereveal={toggleReveal}
  />

  <!-- Campo Modular: Código de Recuperación (Opcional) -->
  {#if account.recovery_code_cipher}
    <CredentialField
      label={$_("accounts.recovery_code_label")}
      value={decryptedRecoveryCode || ""}
      isSecret={true}
      {revealed}
      showRevealButton={false}
      placeholder="••••••••••••"
      countdown={recoveryCopyTimer}
      oncopy={copyRecoveryCode}
    />
  {/if}

  <!-- Campo Modular: Nombre de Usuario / Email -->
  <CredentialField
    label={$_("accounts.username_label")}
    value={account.username}
    oncopy={() => copyToClipboard(account.username, "user")}
  />

  <!-- Notas Adicionales (Opcional) -->
  {#if account.notes_cipher}
    {#if decryptedNotes}
      <div
        class="text-xs text-text-muted p-2 px-2.5 bg-panel/15 rounded-sm border-l-2 border-accent-dim whitespace-pre-wrap max-h-15 overflow-auto"
      >
        {decryptedNotes}
      </div>
    {:else}
      <div class="text-xs text-text-muted/40 italic p-1 px-2.5">
        [Notas bloqueadas]
      </div>
    {/if}
  {/if}

  <!-- Fecha de Creación -->
  <div class="text-xs text-text-muted text-right">
    {$_("accounts.added_at", {
      values: { date: formatDate(account.created_at) },
    })}
  </div>
</div>
