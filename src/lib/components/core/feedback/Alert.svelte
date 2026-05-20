<script lang="ts">
  /**
   * @component Alert
   * @description Componente para mostrar mensajes de retroalimentación (errores, advertencias, éxitos).
   * 
   * @param {Object} props
   * @param {"default" | "destructive" | "success" | "warning"} [props.variant="default"] - El estilo visual del alert.
   * @param {string} [props.title] - Título opcional para el alert.
   * @param {import("svelte").Snippet} [props.children] - Contenido principal del alert.
   * @param {string} [props.class=""] - Clases CSS adicionales.
   */
  import type { Snippet } from "svelte";
  import { IconAlertCircle, IconAlertTriangle, IconCheck, IconInfoCircle } from "@tabler/icons-svelte";

  interface Props {
    variant?: "default" | "destructive" | "success" | "warning";
    title?: string;
    children?: Snippet;
    class?: string;
  }

  let {
    variant = "default",
    title,
    children,
    class: className = "",
  }: Props = $props();

  import { COLORS } from "$lib/config/design-tokens";

  const variants = {
    default: `${COLORS.surface[5]} ${COLORS.text.primary} border ${COLORS.surface.borderMedium}`,
    destructive: COLORS.status.danger,
    success: COLORS.status.success,
    warning: COLORS.status.warning,
  };

  const icons = {
    default: IconInfoCircle,
    destructive: IconAlertCircle,
    success: IconCheck,
    warning: IconAlertTriangle,
  };

  const Icon = $derived(icons[variant]);
</script>

<div class="relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground {variants[variant]} {className}" role="alert">
  <Icon size={16} />
  {#if title}
    <h5 class="mb-1 font-medium leading-none tracking-tight">{title}</h5>
  {/if}
  <div class="text-sm [&_p]:leading-relaxed opacity-90">
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>
