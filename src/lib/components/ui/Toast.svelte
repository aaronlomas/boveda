<script lang="ts">
  import { toast } from "$lib/stores/toast.svelte";
  import { flip } from "svelte/animate";
  import ToastItem from "./ToastItem.svelte";
</script>

<!-- Toast Container — positioned at bottom-right, above everything -->
<div
  class="fixed bottom-6 right-6 z-200 flex flex-col-reverse gap-3 pointer-events-none"
  aria-live="polite"
  aria-atomic="false"
>
  {#each toast.items as t (t.id)}
    <div animate:flip={{ duration: 250 }}>
      <ToastItem {t} />
    </div>
  {/each}
</div>
