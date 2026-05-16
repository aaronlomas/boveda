<script lang="ts">
  import { _ } from "svelte-i18n";

  let { password }: { password: string } = $props();

  let strength = $derived(measureStrength(password));

  function measureStrength(pw: string) {
    if (!pw) return { score: 0, label: "", color: "#333" };
    let score = 0;
    if (pw.length >= 8) score++;
    if (pw.length >= 14) score++;
    if (/[A-Z]/.test(pw)) score++;
    if (/[0-9]/.test(pw)) score++;
    if (/[^A-Za-z0-9]/.test(pw)) score++;
    const labels = [
      "",
      $_("add_credential.strength.very_weak"),
      $_("add_credential.strength.weak"),
      $_("add_credential.strength.regular"),
      $_("add_credential.strength.strong"),
      $_("add_credential.strength.very_strong"),
    ];
    const colors = [
      "#333",
      "#ef4444",
      "#f97316",
      "#f59e0b",
      "#10b981",
      "#6ee7b7",
    ];
    return { score, label: labels[score], color: colors[score] };
  }
</script>

{#if password}
  <div class="flex items-center gap-2.5 mt-1">
    <div class="flex-1 h-1 rounded-sm bg-surface/10 overflow-hidden">
      <div
        class="h-full rounded-sm transition-[width,background] duration-300"
        style="width: {strength.score * 20}%; background: {strength.color}"
      ></div>
    </div>
    <span
      class="text-xs font-bold min-w-16"
      style="color: {strength.color}"
    >
      {strength.label}
    </span>
  </div>
{/if}
