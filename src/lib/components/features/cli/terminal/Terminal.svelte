<script lang="ts">
  import { tick, onMount } from "svelte";
  import { logStore } from "$lib/stores/log.svelte"; // To potentially read logs or interact
  import { getOsUsername } from "$lib/utils/tauri";

  interface TerminalLine {
    id: string;
    text: string;
    isCommand: boolean;
  }

  let history = $state<TerminalLine[]>([
    { id: crypto.randomUUID(), text: "Bóveda Core CLI v1.0.0", isCommand: false },
    { id: crypto.randomUUID(), text: "Type 'help' for available commands.", isCommand: false }
  ]);
  let inputValue = $state("");
  let terminalContainer: HTMLDivElement | undefined = $state();
  let inputElement: HTMLInputElement | undefined = $state();
  let username = $state("");
  let prompt = $derived(username ? `${username}@boveda-core:~$` : "boveda-core:~$");

  onMount(async () => {
    try {
      username = await getOsUsername();
    } catch (e) {
      console.warn("Could not fetch OS username:", e);
    }
  });

  export function clear() {
    history = [];
  }

  $effect(() => {
    if (history.length && terminalContainer) {
      tick().then(() => {
        if (terminalContainer) {
          terminalContainer.scrollTop = terminalContainer.scrollHeight;
        }
      });
    }
  });

  function executeCommand(cmd: string) {
    const trimmed = cmd.trim();
    history.push({ id: crypto.randomUUID(), text: `${prompt} ${trimmed}`, isCommand: true });
    
    if (!trimmed) return;

    const parts = trimmed.split(" ");
    const command = parts[0].toLowerCase();

    switch (command) {
      case "clear":
        clear();
        break;
      case "help":
        history.push({ id: crypto.randomUUID(), text: "Available commands:", isCommand: false });
        history.push({ id: crypto.randomUUID(), text: "  clear  - Clears the terminal output", isCommand: false });
        history.push({ id: crypto.randomUUID(), text: "  help   - Shows this help message", isCommand: false });
        history.push({ id: crypto.randomUUID(), text: "  status - Displays core security status", isCommand: false });
        break;
      case "status":
        history.push({ id: crypto.randomUUID(), text: "System Status: SECURE", isCommand: false });
        history.push({ id: crypto.randomUUID(), text: "Crypto Engine: Active (AES-256-GCM / Argon2id)", isCommand: false });
        break;
      default:
        history.push({ id: crypto.randomUUID(), text: `Command not found: ${command}`, isCommand: false });
        break;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      executeCommand(inputValue);
      inputValue = "";
    }
  }

  // Auto-focus when mounted
  $effect(() => {
    if (inputElement) {
      inputElement.focus();
    }
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
    <div class="log-line rounded-sm {line.isCommand ? 'text-accent/90 font-medium' : 'text-text-primary/80'}">
      {line.text}
    </div>
  {/each}
  
  <div class="flex items-center mt-1">
    <span class="text-accent/90 font-medium mr-2 whitespace-pre">{prompt}</span>
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
