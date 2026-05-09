<script lang="ts">
  import { untrack } from "svelte";
  import { toast, type Toast as ToastType } from "$lib/stores/toast.svelte";
  import {
    IconCheck,
    IconX,
    IconAlertTriangle,
    IconInfoCircle,
    IconAlertCircle,
  } from "@tabler/icons-svelte";
  import { fly } from "svelte/transition";

  let { t }: { t: ToastType } = $props();

  const ICONS = {
    success: IconCheck,
    error: IconAlertCircle,
    warning: IconAlertTriangle,
    info: IconInfoCircle,
  };

  const STYLES = {
    success: "border-l-4 border-l-success bg-success/10 text-success",
    error: "border-l-4 border-l-danger  bg-danger/10  text-danger",
    warning: "border-l-4 border-l-warning bg-warning/10 text-warning",
    info: "border-l-4 border-l-accent  bg-accent/10  text-accent-light",
  };

  let Icon = $derived(ICONS[t.kind]);

  let timer: number | null = null;
  let remaining = $state(untrack(() => t.duration));
  let startTime: number;

  function startTimer(ms: number) {
    if (ms <= 0) return;
    startTime = Date.now();
    timer = window.setTimeout(() => {
      toast.dismiss(t.id);
    }, ms);
  }

  function pauseTimer() {
    if (timer) {
      window.clearTimeout(timer);
      timer = null;
      remaining -= Date.now() - startTime;
    }
  }

  function resumeTimer() {
    if (remaining > 0) {
      startTimer(remaining);
    }
  }

  // Initial start
  $effect(() => {
    startTimer(t.duration);
    return () => {
      if (timer) window.clearTimeout(timer);
    };
  });
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  role="alert"
  in:fly={{ y: 20, duration: 300 }}
  out:fly={{ x: 20, duration: 250 }}
  onmouseenter={pauseTimer}
  onmouseleave={resumeTimer}
  class="pointer-events-auto flex items-center gap-3 min-w-72 max-w-sm px-4 py-3
         rounded-xl backdrop-blur-xl
         bg-panel/80 border border-surface/15
         {STYLES[t.kind]}"
>
  <!-- Icon -->
  <div class="shrink-0 mt-0.5">
    <Icon size={16} />
  </div>

  <!-- Message -->
  <p class="flex-1 text-sm font-medium text-text-primary leading-snug">
    {t.message}
  </p>

  <!-- Dismiss button -->
  <button
    class="shrink-0 text-text-muted hover:text-text-primary transition-colors cursor-pointer"
    onclick={() => toast.dismiss(t.id)}
    aria-label="Dismiss notification"
  >
    <IconX size={14} />
  </button>
</div>
