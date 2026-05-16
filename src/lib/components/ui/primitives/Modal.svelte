<script lang="ts">
  /**
   * @component Modal
   * @description Componente de diálogo modal con transiciones suaves y accesibilidad mejorada.
   * 
   * @param {Object} props
   * @param {boolean} [props.show=false] - Controla la visibilidad del modal.
   * @param {() => void} [props.onclose] - Callback para cerrar el modal.
   * @param {import("svelte").Snippet} [props.children] - Contenido principal del modal.
   * @param {string} [props.title] - Título del modal.
   * @param {string} [props.description] - Descripción opcional para accesibilidad.
   * @param {import("svelte").Snippet} [props.footer] - Contenido para el pie del modal (ej. botones).
   * @param {string} [props.class=""] - Clases CSS adicionales para el contenedor.
   */
  import type { Snippet } from "svelte";
  import { fade, scale } from "svelte/transition";
  import { backOut } from "svelte/easing";
  import { IconX } from "@tabler/icons-svelte";

  interface Props {
    show?: boolean;
    onclose?: () => void;
    children?: Snippet;
    title?: string;
    description?: string;
    footer?: Snippet;
    class?: string;
  }

  let {
    show = false,
    onclose,
    children,
    title,
    description,
    footer,
    class: className = "",
  }: Props = $props();

  // Unique IDs for better accessibility
  const id = Math.random().toString(36).slice(2, 9);
  const titleId = $derived(title ? `modal-title-${id}` : undefined);
  const descId = $derived(description ? `modal-desc-${id}` : undefined);
</script>

{#if show}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
    <!-- Backdrop: Separate element to avoid bubbling issues -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="fixed inset-0 bg-bg-primary/80 backdrop-blur-sm"
      transition:fade={{ duration: 200 }}
      onclick={onclose}
      aria-hidden="true"
    ></div>

    <!-- Modal Content -->
    <div
      class="relative z-50 grid w-full max-w-lg gap-4 border border-surface/10 bg-surface/2 p-6 shadow-2xl sm:rounded-2xl {className}"
      transition:scale={{ duration: 300, start: 0.95, easing: backOut }}
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      aria-labelledby={titleId}
      aria-describedby={descId}
    >
      {#if title || description}
        <div class="flex flex-col space-y-1.5 text-center sm:text-left">
          {#if title}
            <h2 id={titleId} class="text-lg font-semibold leading-none tracking-tight text-text-primary">
              {title}
            </h2>
          {/if}
          {#if description}
            <p id={descId} class="text-sm text-text-muted">
              {description}
            </p>
          {/if}
        </div>
      {/if}

      {#if onclose}
        <button
          onclick={onclose}
          class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-bg-primary transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-surface/10 data-[state=open]:text-text-muted"
        >
          <IconX size={16} />
          <span class="sr-only">Cerrar</span>
        </button>
      {/if}

      <div class="py-4">
        {#if children}
          {@render children()}
        {/if}
      </div>

      {#if footer}
        <div class="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2">
          {@render footer()}
        </div>
      {/if}
    </div>
  </div>
{/if}
