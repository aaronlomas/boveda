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
