/**
 * Utilities for reading state from the current selection in the board
 */

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
