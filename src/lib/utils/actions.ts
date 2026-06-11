/**
 * Action to focus an element when it is mounted.
 * This is preferred over 'autofocus' for accessibility reasons.
 */
export function focus(node: HTMLElement) {
  node.focus();
}

/**
 * Action to focus and select the text of an input element when mounted.
 * Useful for rename fields.
 */
export function selectOnFocus(node: HTMLInputElement) {
  node.focus();
  node.select();
}

// Global queue for typewriter effects to run sequentially
const typewriterQueue: (() => void)[] = [];
let isTyping = false;

function processQueue() {
  if (isTyping || typewriterQueue.length === 0) return;
  isTyping = true;
  const nextFn = typewriterQueue.shift();
  if (nextFn) nextFn();
}

/**
 * Action to apply a typewriter effect to a text node on mount.
 * The original text is extracted, cleared, and typed out sequentially.
 * Uses a global queue to ensure multiple elements type out one by one.
 */
export function typewriter(node: HTMLElement, { speed = 15, delay = 0 }: { speed?: number, delay?: number } = {}) {
  const text = node.textContent || '';
  node.textContent = '';
  
  let i = 0;
  let timeout: ReturnType<typeof setTimeout>;
  let isDestroyed = false;
  let isThisActive = false;
  
  function type() {
    if (isDestroyed) {
      if (isThisActive) {
        isThisActive = false;
        isTyping = false;
        processQueue();
      }
      return;
    }

    if (i < text.length) {
      node.textContent += text.charAt(i);
      i++;
      timeout = setTimeout(type, speed);
    } else {
      isThisActive = false;
      isTyping = false;
      processQueue();
    }
  }
  
  function start() {
    if (isDestroyed) return;
    isThisActive = true;
    if (delay > 0) {
      timeout = setTimeout(type, delay);
    } else {
      type();
    }
  }

  typewriterQueue.push(start);
  processQueue();
  
  return {
    destroy() {
      isDestroyed = true;
      clearTimeout(timeout);
      const index = typewriterQueue.indexOf(start);
      if (index > -1) {
        typewriterQueue.splice(index, 1);
      } else if (isThisActive) {
        isThisActive = false;
        isTyping = false;
        processQueue();
      }
    }
  };
}
