/**
 * Utilities for the rich text board
 */

export function execBoardCommand(command: string, value: string | undefined = undefined): void {
  if (typeof document !== "undefined") {
    document.execCommand(command, false, value);
  }
}

export function getCommandState(command: string): boolean {
  if (typeof document !== "undefined") {
    try {
      return document.queryCommandState(command);
    } catch (e) {
      return false;
    }
  }
  return false;
}

export function getAlignment(): "left" | "center" | "right" {
  if (typeof document !== "undefined") {
    if (document.queryCommandState("justifyCenter")) return "center";
    if (document.queryCommandState("justifyRight")) return "right";
  }
  return "left";
}

/**
 * Apply an exact point (pt) font-size to the current text selection.
 * Works by wrapping the selected Range in a <span> with an explicit
 * inline style, avoiding the browser's limited 1-7 scale.
 */
export function setFontSize(size: number): void {
  if (typeof document === "undefined") return;

  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return;

  const range = sel.getRangeAt(0);

  if (range.collapsed) {
    // No text selected — store the size for the next character typed
    // using a zero-width span so future typed text inherits the size.
    const span = document.createElement("span");
    span.style.fontSize = `${size}pt`;
    // Insert a zero-width space so the caret enters the span
    span.innerHTML = "\u200B";
    range.insertNode(span);
    // Move caret inside the span, after the ZWS
    const newRange = document.createRange();
    newRange.setStart(span.firstChild!, 1);
    newRange.collapse(true);
    sel.removeAllRanges();
    sel.addRange(newRange);
    return;
  }

  // Text is selected — wrap it
  try {
    const span = document.createElement("span");
    span.style.fontSize = `${size}pt`;
    // surroundContents throws if selection crosses block boundaries;
    // fall back to execCommand in that case.
    range.surroundContents(span);
  } catch {
    // Fallback for complex selections: clone content, wrap, delete & insert
    const fragment = range.extractContents();
    const span = document.createElement("span");
    span.style.fontSize = `${size}pt`;
    span.appendChild(fragment);
    range.insertNode(span);
  }

  // Restore selection
  sel.removeAllRanges();
  const newRange = document.createRange();
  const inserted = range.commonAncestorContainer.lastChild as Node;
  newRange.selectNodeContents(inserted ?? range.commonAncestorContainer);
  sel.addRange(newRange);
}

/**
 * Read the computed font-size at the current caret / selection anchor.
 * Converts browser's computed px back to pt (1px = 0.75pt).
 * Returns null when nothing can be determined.
 */
export function getFontSizeAtCaret(): number | null {
  if (typeof document === "undefined") return null;

  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return null;

  let node: Node | null = sel.getRangeAt(0).startContainer;
  // Walk up to the nearest Element
  if (node.nodeType === Node.TEXT_NODE) node = node.parentElement;

  if (!(node instanceof HTMLElement)) return null;

  const raw = window.getComputedStyle(node).fontSize; // e.g. "16px"
  const pxParsed = parseFloat(raw);
  if (isNaN(pxParsed)) return null;
  
  // Convert px to pt (1pt = 1.333px, so 1px = 0.75pt)
  return Math.round(pxParsed * 0.75);
}
