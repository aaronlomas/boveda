<script lang="ts">
  /**
   * @component CredentialField
   * @description Campo reutilizable para visualizar información de credenciales (Usuario, Contraseña, Cuentas, etc.).
   * Soporta modo seguro (cifrado/ocultable), copiado al portapapeles y temporizadores de limpieza.
   */
  import { _ } from "svelte-i18n";
  import { IconEye, IconEyeOff, IconCopy } from "@tabler/icons-svelte";

  // Props
  let {
    label,
    value,
    isSecret = false,
    revealed = false,
    countdown = null,
    showRevealButton = true,
    placeholder = "••••••••••••••••",
    oncopy,
    ontogglereveal,
  } = $props<{
    label: string;
    value: string;
    isSecret?: boolean;
    revealed?: boolean;
    countdown?: number | null;
    showRevealButton?: boolean;
    placeholder?: string;
    oncopy: () => void;
    ontogglereveal?: () => void;
  }>();
</script>

<!-- ========================================================================= -->
<!-- CONTENEDOR DEL CAMPO -->
<!-- ========================================================================= -->
<div class="grid gap-2">
  <!-- Etiqueta del Campo -->
  <span class="text-xs text-text-muted uppercase tracking-wider font-bold">
    {label}
  </span>
  
  <!--Contenedor de usuario y contraseña + Botones de Control -->
  <div
    class="flex items-center gap-2 bg-surface/5 border border-surface/8 rounded-xl p-2 px-3 transition-colors hover:bg-surface/[0.07]"
  >
    <!-- Texto o Máscara -->
    <code
      class="flex-1 font-mono text-sm whitespace-nowrap overflow-hidden text-ellipsis tracking-wider"
      class:text-text-primary={isSecret && revealed}
      class:text-text-secondary={!(isSecret && revealed)}
    >
      {#if isSecret}
        {revealed && value ? value : placeholder}
      {:else}
        {value}
      {/if}
    </code>

    <!-- Botones de Interacción -->
    <div class="flex items-center gap-0.5 shrink-0">
      
      <!-- Botón Revelar (Opcional, para campos secretos) -->
      {#if isSecret && showRevealButton}
        <button
          class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer"
          onclick={ontogglereveal}
          aria-label={revealed ? $_("actions.hide") : $_("actions.show")}
          data-tooltip={revealed ? $_("actions.hide") : $_("actions.show")}
        >
          {#if revealed}
            <IconEyeOff size={16} />
          {:else}
            <IconEye size={16} />
          {/if}
        </button>
      {/if}

      <!-- Botón Copiar al Portapapeles -->
      <button
        class="p-2 text-text-muted hover:text-text-primary hover:bg-surface/10 rounded-md transition-all cursor-pointer
               {countdown !== null ? 'text-accent-light' : ''}"
        onclick={oncopy}
        aria-label={$_("actions.copy")}
        data-tooltip={countdown !== null
          ? $_("actions.status.clearing", { values: { seconds: countdown } })
          : $_("actions.copy")}
      >
        <div class="w-4 h-4 flex items-center justify-center">
          {#if countdown !== null}
            <!-- Contador Numérico Visual -->
            <span class="text-[10px] font-bold leading-none">{countdown}</span>
          {:else}
            <IconCopy size={16} />
          {/if}
        </div>
      </button>
    </div>
  </div>
</div>
