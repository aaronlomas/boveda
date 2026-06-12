<script lang="ts">
  import { tick, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getOsUsername } from "$lib/utils/tauri";

  interface TerminalLine {
    id: string;
    text: string;
    isCommand: boolean;
  }

  let history = $state<TerminalLine[]>([
    { id: crypto.randomUUID(), text: "Bóveda Core CLI — type 'help' for available commands.", isCommand: false },
  ]);
  let inputValue = $state("");
  let terminalContainer: HTMLDivElement | undefined = $state();
  let inputElement: HTMLInputElement | undefined = $state();
  let username = $state("");
  let prompt = $derived(username ? `${username}@boveda:~$` : "boveda:~$");

  onMount(async () => {
    try {
      username = await getOsUsername();
    } catch {
      // username stays empty, prompt falls back gracefully
    }
  });

  export function clear() {
    history = [];
  }

  $effect(() => {
    if (history.length && terminalContainer) {
      tick().then(() => {
        if (terminalContainer) terminalContainer.scrollTop = terminalContainer.scrollHeight;
      });
    }
  });

  async function executeCommand(cmd: string) {
    const trimmed = cmd.trim();

    // Echo the command line
    history.push({ id: crypto.randomUUID(), text: `${prompt} ${trimmed}`, isCommand: true });

    if (!trimmed) return;

    const command = trimmed.split(" ")[0].toLowerCase();

    // Handle client-side only commands
    if (command === "clear") {
      clear();
      return;
    }

    // All other commands are dispatched to boveda-core via Tauri
    try {
      const lines = await invoke<string[]>("shell_query", { input: trimmed });
      for (const line of lines) {
        history.push({ id: crypto.randomUUID(), text: line, isCommand: false });
      }
    } catch (err) {
      history.push({
        id: crypto.randomUUID(),
        text: `error: ${err}`,
        isCommand: false,
      });
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      executeCommand(inputValue);
      inputValue = "";
    }
  }

  $effect(() => {
    if (inputElement) inputElement.focus();
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  bind:this={terminalContainer}
  class="flex flex-col flex-1 w-full px-4 overflow-y-auto font-mono text-sm leading-relaxed custom-scrollbar"
  onclick={() => inputElement?.focus()}
>
  {#each history as line (line.id)}
    <div
      class="log-line rounded-sm whitespace-pre {line.isCommand
        ? 'text-accent/90 font-medium'
        : 'text-text-primary/70'}"
    >
      {line.text}
    </div>
  {/each}

  <div class="flex items-center mt-1">
    <span class="text-accent/90 font-medium mr-2 whitespace-pre">{prompt} </span>
    <input
      bind:this={inputElement}
      bind:value={inputValue}
      onkeydown={handleKeydown}
      type="text"
      class="flex-1 bg-transparent outline-none text-text-primary/90 font-mono caret-accent"
      autocomplete="off"
      spellcheck="false"
    />
  </div>
</div>
