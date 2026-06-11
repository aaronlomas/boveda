export function portal(
  node: HTMLElement,
  target: string | HTMLElement = "body",
) {
  let targetNode: HTMLElement | null;

  if (typeof target === "string") {
    targetNode = document.querySelector(target);
  } else {
    targetNode = target;
  }

  if (targetNode) {
    targetNode.appendChild(node);
  } else {
    console.warn(`Portal target '${target}' not found. Falling back to body.`);
    document.body.appendChild(node);
  }

  return {
    destroy() {
      if (node.parentNode) {
        node.parentNode.removeChild(node);
      }
    },
  };
}
